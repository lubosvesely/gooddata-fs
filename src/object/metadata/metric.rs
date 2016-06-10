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
    pub meta: super::MetadataMeta,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Metric {
    pub metric: MetricBody,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsMetricBody {
    pub paging: super::MetadataPaging,
    pub items: Vec<Metric>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsMetric {
    pub objects: ObjectsMetricBody,
}
