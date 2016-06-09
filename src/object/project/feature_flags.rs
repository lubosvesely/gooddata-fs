use std::collections::HashMap;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FeatureFlagBody {
    pub key: Option<String>,
    pub value: Option<bool>,
    pub links: Option<HashMap<String, String>>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FeatureFlag {
    featureFlag: FeatureFlagBody,
}

#[allow(dead_code)]
impl Into<String> for FeatureFlag {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FeatureFlagsBody {
    pub items: Vec<FeatureFlag>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FeatureFlags {
    pub featureFlags: FeatureFlagsBody,
}

#[allow(dead_code)]
impl Into<String> for FeatureFlags {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
