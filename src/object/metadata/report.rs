use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportMeta {
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
pub struct ReportContent {
    pub domains: Option<Vec<String>>,
    pub definitions: Option<Vec<String>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportBody {
    pub content: ReportContent,
    pub meta: ReportMeta,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Report {
    pub report: ReportBody,
}

// impl Into<String> for Report {
//     fn into(self) -> String {
//         format!("{}\n", json::as_pretty_json(&self).to_string())
//     }
// }

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportPaging {
    pub next: Option<String>,
    pub count: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsReportBody {
    pub paging: ReportPaging,
    pub items: Vec<Report>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsReport {
    pub objects: ObjectsReportBody,
}
