use rustc_serialize;
use rustc_serialize::json;

mod associated_permissions;
mod associated_roles;
mod feature_flags;
mod permissions;
mod project_content;
mod project_meta;
mod create;

use std::collections::HashMap;
use gd::client::GoodDataClient;

pub use self::associated_permissions::*;
pub use self::associated_roles::*;
pub use self::feature_flags::*;
pub use self::project_content::*;
pub use self::project_meta::*;
pub use self::create::*;

pub use super::metadata::*;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ProjectBody {
    pub content: ProjectContent,
    pub links: Option<HashMap<String, String>>,
    pub meta: ProjectMeta,
}

impl ProjectBody {
    pub fn links(&self) -> &Option<HashMap<String, String>> {
        &self.links
    }

    pub fn meta(&self) -> &ProjectMeta {
        &self.meta
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Project {
    pub project: ProjectBody,
}

impl Project {
    pub fn pid(&self) -> String {
        self.get_link("self").split("/").last().unwrap().to_string()
    }

    pub fn project(&self) -> &ProjectBody {
        &self.project
    }

    pub fn get_link(&self, link_name: &str) -> String {
        self.project().links().as_ref().unwrap()[link_name].to_string()
    }

    pub fn feature_flags(&self, client: &mut GoodDataClient) -> Option<FeatureFlags> {
        client.get_link_obj::<FeatureFlags>(self.get_link("projectFeatureFlags"))
    }

    pub fn user_permissions(&self, client: &mut GoodDataClient) -> Option<AssociatedPermissions> {
        client.get_link_obj::<AssociatedPermissions>(self.get_link("userPermissions"))
    }

    pub fn user_roles(&self, client: &mut GoodDataClient) -> Option<AssociatedRoles> {
        client.get_link_obj::<AssociatedRoles>(self.get_link("userRoles"))
    }

    pub fn get_metadata<T: rustc_serialize::Decodable>(&self,
                                                       client: &mut GoodDataClient,
                                                       md_type: String)
                                                       -> T {
        let uri = format!("/gdc/md/{}/objects/query?category={}&limit=50",
                          self.pid(),
                          md_type);
        println!("{:?}", uri);
        let mut res = client.get(uri);
        let raw = client.get_content(&mut res);
        println!("Project::get_metadata() - {} {:?}", md_type, raw);

        json::decode::<T>(&raw).unwrap()
    }
}
