use rustc_serialize::json;

pub use object::permissions::*;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct AssociatedPermissions {
    pub associatedPermissions: Permissions,
}

impl Into<String> for AssociatedPermissions {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
