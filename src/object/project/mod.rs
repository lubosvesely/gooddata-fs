mod associated_permissions;
mod associated_roles;
mod feature_flags;
mod project_content;
mod project_meta;
mod create;

use std::collections::HashMap;
use gd::connector::Connector;

pub use self::associated_permissions::*;
pub use self::associated_roles::*;
pub use self::feature_flags::*;
pub use self::project_content::*;
pub use self::project_meta::*;
pub use self::create::*;


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
    pub fn project(&self) -> &ProjectBody {
        &self.project
    }

    pub fn get_link(&self, link_name: &str) -> String {
        self.project().links().as_ref().unwrap()[link_name].to_string()
    }

    pub fn feature_flags(&self, client: &mut Connector) -> Option<FeatureFlags> {
        client.object_by_get::<FeatureFlags>(self.get_link("projectFeatureFlags"))
    }

    pub fn user_permissions(&self, client: &mut Connector) -> Option<AssociatedPermissions> {
        client.object_by_get::<AssociatedPermissions>(self.get_link("userPermissions"))
    }

    pub fn user_roles(&self, client: &mut Connector) -> Option<AssociatedRoles> {
        client.object_by_get::<AssociatedRoles>(self.get_link("userRoles"))
    }
}
