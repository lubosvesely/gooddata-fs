pub use object::project::*;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Projects {
    pub projects: Vec<Project>,
}
