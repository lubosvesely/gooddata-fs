use std::collections::HashMap;
use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct AssociatedRolesBody {
    pub roles: Vec<String>,
    pub links: Option<HashMap<String, String>>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct AssociatedRoles {
    pub associatedRoles: AssociatedRolesBody,
}

#[allow(dead_code)]
impl Into<String> for AssociatedRoles {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
