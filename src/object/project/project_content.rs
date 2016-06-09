use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ProjectContent {
    pub environment: Option<String>,
    pub cluster: Option<String>,
    pub guidedNavigation: Option<String>,
    pub isPublic: Option<String>,
    pub driver: Option<String>,
    pub state: Option<String>,
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl ProjectContent {
    pub fn environment(&self) -> &Option<String> {
        &self.environment
    }

    pub fn cluster(&self) -> &Option<String> {
        &self.cluster
    }

    pub fn guidedNavigation(&self) -> &Option<String> {
        &self.guidedNavigation
    }

    pub fn isPublic(&self) -> &Option<String> {
        &self.isPublic
    }

    pub fn driver(&self) -> &Option<String> {
        &self.driver
    }

    pub fn state(&self) -> &Option<String> {
        &self.state
    }
}

#[allow(dead_code)]
impl Into<String> for ProjectContent {
    fn into(self) -> String {
        json::as_pretty_json(&self).to_string()
    }
}
