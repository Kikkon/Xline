use std::sync::OnceLock;

use xlineapi::{
    AuthDisableResponse, AuthEnableResponse, AuthRoleAddResponse, AuthRoleDeleteResponse,
    AuthRoleGetResponse, AuthRoleGrantPermissionResponse, AuthRoleListResponse,
    AuthRoleRevokePermissionResponse, AuthStatusResponse, AuthUserAddResponse,
    AuthUserChangePasswordResponse, AuthUserDeleteResponse, AuthUserGetResponse,
    AuthUserGrantRoleResponse, AuthUserListResponse, AuthUserRevokeRoleResponse,
    DeleteRangeResponse, KeyValue, LeaseGrantResponse, LeaseKeepAliveResponse, LeaseLeasesResponse,
    LeaseRevokeResponse, LeaseTimeToLiveResponse, PutResponse, RangeResponse, ResponseHeader,
    TxnResponse,
};

/// The global printer type config
static PRINTER_TYPE: OnceLock<PrinterType> = OnceLock::new();

/// The type of the Printer
pub(crate) enum PrinterType {
    /// Simple printer, which print simplified result
    Simple,
    /// Filed printer, which print every fields of the result
    Field,
}

/// Set the type of the printer
pub(crate) fn set_printer_type(printer_type: PrinterType) {
    let _ignore = PRINTER_TYPE.get_or_init(|| printer_type);
}

/// The printer implementation trait
pub(crate) trait Printer {
    /// Print the simplified result
    fn simple(&self);
    /// Print every fields of the result
    fn field(&self);
    /// Print according to the config set
    fn print(&self) {
        match *PRINTER_TYPE
            .get()
            .unwrap_or_else(|| unreachable!("the printer type should be initialized"))
        {
            PrinterType::Simple => self.simple(),
            PrinterType::Field => self.field(),
        }
    }
}

impl Printer for PutResponse {
    fn simple(&self) {
        println!("OK");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        if let Some(pre_kv) = self.prev_kv.as_ref() {
            FieldPrinter::kv(pre_kv);
        }
    }
}

impl Printer for RangeResponse {
    fn simple(&self) {
        for kv in &self.kvs {
            SimplePrinter::utf8(&kv.key);
            SimplePrinter::utf8(&kv.value);
        }
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("kvs:");
        for kv in &self.kvs {
            FieldPrinter::kv(kv);
        }
        println!("more: {}, count: {}", self.more, self.count);
    }
}

impl Printer for DeleteRangeResponse {
    fn simple(&self) {
        println!("{}", self.deleted);
        for kv in &self.prev_kvs {
            SimplePrinter::utf8(&kv.key);
            SimplePrinter::utf8(&kv.value);
        }
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("kvs:");
        for kv in &self.prev_kvs {
            FieldPrinter::kv(kv);
        }
        println!("deleted: {}", self.deleted);
    }
}

impl Printer for TxnResponse {
    fn simple(&self) {
        println!("{}", if self.succeeded { "SUCCESS" } else { "FAILURE" });
        for resp_op in &self.responses {
            if let Some(resp_wrapper) = resp_op.response.as_ref() {
                match *resp_wrapper {
                    xlineapi::Response::ResponseRange(ref resp) => resp.print(),
                    xlineapi::Response::ResponsePut(ref resp) => resp.print(),
                    xlineapi::Response::ResponseDeleteRange(ref resp) => resp.print(),
                    xlineapi::Response::ResponseTxn(ref resp) => resp.print(),
                }
            }
        }
    }

    fn field(&self) {
        println!("succeed: {}", self.succeeded);
        for resp_op in &self.responses {
            if let Some(resp_wrapper) = resp_op.response.as_ref() {
                match *resp_wrapper {
                    xlineapi::Response::ResponseRange(ref resp) => resp.print(),
                    xlineapi::Response::ResponsePut(ref resp) => resp.print(),
                    xlineapi::Response::ResponseDeleteRange(ref resp) => resp.print(),
                    xlineapi::Response::ResponseTxn(ref resp) => resp.print(),
                }
            }
        }
    }
}

impl Printer for LeaseGrantResponse {
    fn simple(&self) {
        println!("{}", self.id);
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("lease id: {}, granted ttl: {}", self.id, self.ttl);
    }
}

impl Printer for LeaseKeepAliveResponse {
    fn simple(&self) {
        println!("{}", self.ttl);
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("lease id: {} keepalived with TTL: {}", self.id, self.ttl);
    }
}

impl Printer for LeaseLeasesResponse {
    fn simple(&self) {
        for lease in &self.leases {
            println!("{}", lease.id);
        }
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        for lease in &self.leases {
            println!("lease: {}", lease.id);
        }
    }
}

impl Printer for LeaseRevokeResponse {
    fn simple(&self) {
        println!("Revoked");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Revoked");
    }
}

impl Printer for LeaseTimeToLiveResponse {
    fn simple(&self) {
        println!("{}", self.ttl);
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!(
            "lease id: {}, ttl: {}, granted_ttl: {}",
            self.id, self.ttl, self.granted_ttl
        );

        for key in &self.keys {
            FieldPrinter::key(key);
        }
    }
}

impl Printer for AuthEnableResponse {
    fn simple(&self) {
        println!("Authentication enabled");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Authentication enabled");
    }
}

impl Printer for AuthDisableResponse {
    fn simple(&self) {
        println!("Authentication disabled");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Authentication disabled");
    }
}

impl Printer for AuthStatusResponse {
    fn simple(&self) {
        println!(
            "enabled: {}, revision: {}",
            self.enabled, self.auth_revision
        );
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!(
            "enabled: {}, revision: {}",
            self.enabled, self.auth_revision
        );
    }
}

impl Printer for AuthUserAddResponse {
    fn simple(&self) {
        println!("User added");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("User added");
    }
}

impl Printer for AuthUserDeleteResponse {
    fn simple(&self) {
        println!("User deleted");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("User deleted");
    }
}

impl Printer for AuthUserGetResponse {
    fn simple(&self) {
        for role in &self.roles {
            println!("{role}");
        }
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Roles: ");
        for role in &self.roles {
            print!("{role}");
        }
    }
}

impl Printer for AuthRoleGetResponse {
    fn simple(&self) {
        for perm in &self.perm {
            println!("Permission: {}", perm_type(perm.perm_type));
            SimplePrinter::utf8(&perm.key);
            if !perm.range_end.is_empty() {
                SimplePrinter::utf8(&perm.range_end);
            }
        }
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        for perm in &self.perm {
            println!("perm type: {}", perm_type(perm.perm_type));
            FieldPrinter::key(&perm.key);
            if !perm.range_end.is_empty() {
                FieldPrinter::range_end(&perm.range_end);
            }
        }
    }
}

/// Convert perm type to string
fn perm_type(perm: i32) -> String {
    match perm {
        0 => "Read",
        1 => "Write",
        2 => "ReadWrite",
        _ => "Unknown",
    }
    .to_owned()
}

impl Printer for AuthUserGrantRoleResponse {
    fn simple(&self) {
        println!("Role granted");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Role granted");
    }
}

impl Printer for AuthUserListResponse {
    fn simple(&self) {
        for user in &self.users {
            println!("{user}");
        }
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Users:");
        for user in &self.users {
            println!("{user}");
        }
    }
}

impl Printer for AuthUserChangePasswordResponse {
    fn simple(&self) {
        println!("Password updated");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Password updated");
    }
}

impl Printer for AuthUserRevokeRoleResponse {
    fn simple(&self) {
        println!("Role revoked");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Role revoked");
    }
}

impl Printer for AuthRoleAddResponse {
    fn simple(&self) {
        println!("Role added");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Role added");
    }
}

impl Printer for AuthRoleDeleteResponse {
    fn simple(&self) {
        println!("Role deleted");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Role deleted");
    }
}

impl Printer for AuthRoleGrantPermissionResponse {
    fn simple(&self) {
        println!("Permission granted");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Permission granted");
    }
}

impl Printer for AuthRoleListResponse {
    fn simple(&self) {
        for role in &self.roles {
            println!("{role}");
        }
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        for role in &self.roles {
            println!("{role}");
        }
    }
}

impl Printer for AuthRoleRevokePermissionResponse {
    fn simple(&self) {
        println!("Permission revoked");
    }

    fn field(&self) {
        FieldPrinter::header(self.header.as_ref());
        println!("Permission revoked");
    }
}

/// Simple Printer of common response types
struct SimplePrinter;

impl SimplePrinter {
    /// Print utf8 bytes as string
    fn utf8(vec: &[u8]) {
        println!("{}", String::from_utf8_lossy(vec));
    }
}

/// Field Printer of common response types
struct FieldPrinter;

impl FieldPrinter {
    /// Response header printer
    fn header(header: Option<&ResponseHeader>) {
        let Some(header) = header else { return };
        println!("header:");
        println!(
            "cluster_id: {}, member_id: {}, revision: {}, raft_term: {}",
            header.cluster_id, header.member_id, header.revision, header.raft_term
        );
    }

    /// Response key printer
    pub(crate) fn key(key: &[u8]) {
        println!("key: {}", String::from_utf8_lossy(key));
    }

    /// Response key printer
    pub(crate) fn range_end(range_end: &[u8]) {
        println!("range_end: {}", String::from_utf8_lossy(range_end));
    }

    #[allow(dead_code)]
    /// Response value printer
    pub(crate) fn value(value: &[u8]) {
        println!("value: {}", String::from_utf8_lossy(value));
    }

    /// Response key-value printer
    fn kv(kv: &KeyValue) {
        println!(
            "key: {}, value: {}",
            String::from_utf8_lossy(&kv.key),
            String::from_utf8_lossy(&kv.value)
        );
    }
}
