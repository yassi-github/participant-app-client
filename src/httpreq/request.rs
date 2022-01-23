use crate::generate_body::user;
use crate::userdata::input_yaml;
use crate::httpreq;


pub fn regist_user() -> Result<String, Box<dyn std::error::Error>> {

    let params = httpreq::Params {
        method: httpreq::HttpMethod::Post,
        path: input_yaml::read_settings()?.server.regist_path,
        body: Some(user::generate_request_body()?),
    };


    httpreq::fetch(params)
}
