use std::collections::HashMap;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct FeatureFlagBody {
    pub key: String,
    pub value: bool,
    pub links: Option<HashMap<String, String>>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct FeatureFlag {
    featureFlag: FeatureFlagBody,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct FeatureFlagsBody {
    pub items: Vec<FeatureFlag>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct FeatureFlags {
    pub featureFlags: FeatureFlagsBody,
}

impl Into<String> for FeatureFlags {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
