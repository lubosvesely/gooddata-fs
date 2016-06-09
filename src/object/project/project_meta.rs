use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ProjectMeta {
    pub created: Option<String>,
    pub summary: Option<String>,
    pub updated: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub contributor: Option<String>,
}

#[allow(dead_code)]
impl ProjectMeta {
    pub fn created(&self) -> &Option<String> {
        &self.created
    }

    pub fn summary(&self) -> &Option<String> {
        &self.summary
    }

    pub fn updated(&self) -> &Option<String> {
        &self.updated
    }

    pub fn author(&self) -> &Option<String> {
        &self.author
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn contributor(&self) -> &Option<String> {
        &self.contributor
    }
}

#[allow(dead_code)]
impl Into<String> for ProjectMeta {
    fn into(self) -> String {
        json::as_pretty_json(&self).to_string()
    }
}
