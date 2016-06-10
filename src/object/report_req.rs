#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct ReportReq {
    pub report_req: ReportReqBody
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct ReportReqBody {
    pub reportDefinition: String
}
