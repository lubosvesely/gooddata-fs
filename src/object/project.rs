use std::collections::HashMap;
use rustc_serialize::json;

use gd::client::GoodDataClient;

pub use object::associated_permissions::*;
pub use object::associated_roles::*;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct ProjectContent {
    pub environment: Option<String>,
    pub cluster: Option<String>,
    pub guidedNavigation: Option<String>,
    pub isPublic: Option<String>,
    pub driver: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Project {
    pub project: ProjectBody,
}

impl Project {
    pub fn project(&self) -> &ProjectBody {
        &self.project
    }

    pub fn user_permissions(&self, client: &mut GoodDataClient) -> AssociatedPermissions {
        let mut res =
            client.get(self.project().links().as_ref().unwrap()["userPermissions"].to_string());
        let raw = client.get_content(&mut res);
        let obj: AssociatedPermissions = json::decode(&raw.to_string()).unwrap();

        return obj;
    }

    pub fn user_roles(&self, client: &mut GoodDataClient) -> AssociatedRoles {
        let mut res = client.get(self.project().links().as_ref().unwrap()["userRoles"].to_string());
        let raw = client.get_content(&mut res);
        let obj: AssociatedRoles = json::decode(&raw.to_string()).unwrap();

        return obj;
    }
}
