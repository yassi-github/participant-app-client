use crate::generate_body::user;
use crate::httpreq;

/// generate HTTP request params.
/// regist user by POST.
// pub fn regist_user(specified_macaddress: &crate::RegistArgs) -> Result<String, Box<dyn std::error::Error>> {
pub fn regist_user() -> Result<String, Box<dyn std::error::Error>> {

    let request_path: httpreq::RequestPath = Default::default();

    let params = httpreq::Params {
        method: httpreq::HttpMethod::Post,
        path: request_path.regist_path,
        body: Some(user::generate_request_body(None)?),
    };

    httpreq::fetch(params)
}
