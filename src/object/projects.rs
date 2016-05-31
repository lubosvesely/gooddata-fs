use rustc_serialize::json;

pub use object::project::*;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Projects {
    pub projects: Vec<Project>,
}

impl Into<String> for Projects {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
