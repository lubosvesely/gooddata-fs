pub use object::project::*;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Projects {
    pub projects: Vec<Project>,
}

// impl Projects {
//     fn project(&self, idx: usize) -> &Project {
//         &self.projects[idx]
//     }
// }
