use std::collections::HashMap;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct AssociatedRolesBody {
    pub roles: Vec<String>,
    pub links: Option<HashMap<String, String>>,
}
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct AssociatedRoles {
    pub associatedRoles: AssociatedRolesBody,
}

impl Into<String> for AssociatedRoles {
    fn into(self) -> String {
        json::as_pretty_json(&self).to_string()
    }
}
