use crate::generate_body::user;
use crate::httpreq;

/// generate HTTP request params.
/// Change registed macaddress to `specified_macaddress` by PUT.
pub fn change_macaddress(specified_macaddress: &crate::ChangeMacaddrArgs) -> Result<String, Box<dyn std::error::Error>> {

    let request_path: httpreq::RequestPath = Default::default();

    let params = httpreq::Params {
        method: httpreq::HttpMethod::Put,
        path: request_path.change_macaddr_path,
        body: Some(user::generate_request_body(Some(&specified_macaddress.macaddr))?),
    };

    httpreq::fetch(params)
}
