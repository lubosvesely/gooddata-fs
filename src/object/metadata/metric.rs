use rustc_serialize::json;


#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricTreePosition {
    pub column: Option<u8>,
    pub line: Option<u8>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricTreeNode {
    pub content: Option<Vec<MetricTreeNode>>,
    pub position: MetricTreePosition,
    // pub type: Option<String>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricMeta {
    pub author: Option<String>,
    pub uri: Option<String>,
    pub tags: Option<String>,
    pub created: Option<String>,
    pub identifier: Option<String>,
    pub deprecated: Option<String>,
    pub summary: Option<String>,
    pub isProduction: Option<u8>,
    pub title: Option<String>,
    pub category: Option<String>,
    pub updated: Option<String>,
    pub contributor: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricContent {
    pub folders: Option<Vec<String>>,
    pub format: Option<String>,
    pub tree: MetricTreeNode,
    pub expression: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricBody {
    pub content: MetricContent,
    pub meta: MetricMeta,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Metric {
    pub metric: MetricBody,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricPaging {
    pub next: Option<String>,
    pub count: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsMetricBody {
    pub paging: MetricPaging,
    pub items: Vec<Metric>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsMetric {
    pub objects: ObjectsMetricBody,
}
