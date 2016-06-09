#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectCreateContent {
    pub environment: String,
    pub guidedNavigation: String,
    pub authorizationToken: String,
    pub driver: String,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectCreateMeta {
    pub summary: String,
    pub title: String,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectCreateBody {
    pub content: ProjectCreateContent,
    pub meta: ProjectCreateMeta,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectCreate {
    pub project: ProjectCreateBody,
}
