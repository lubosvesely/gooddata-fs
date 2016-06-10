#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportContent {
    pub domains: Option<Vec<String>>,
    pub definitions: Option<Vec<String>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportBody {
    pub content: ReportContent,
    pub meta: super::MetadataMeta,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Report {
    pub report: ReportBody,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsReportBody {
    pub paging: super::MetadataPaging,
    pub items: Vec<Report>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsReport {
    pub objects: ObjectsReportBody,
}
