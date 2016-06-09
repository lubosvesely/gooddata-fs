use rustc_serialize::json;
use object::project::Project;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Projects {
    pub projects: Vec<Project>,
}

#[allow(dead_code)]
impl Projects {
    pub fn projects(&self) -> &Vec<Project> {
        &self.projects
    }
}

impl Into<String> for Projects {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
