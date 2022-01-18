use super::mac;
use crate::userdata;
use form_urlencoded;

/// generate body.
/// this uses mac addr data from mac.rs
pub fn generate_request_body() -> Result<String, Box<dyn std::error::Error>> {
    let mac_data = mac::gen_mac()?.to_string();
    let readed_data = userdata::input_yaml::read_settings()?;

    let id = readed_data.user_data.user_id;
    let name = readed_data.user_data.name;

    // sample POST body:
    // id=19T999&name=kagawataro&macaddress=aa%3Aaa%3Aaa%3Aaa%3Aaa%3Aaa
    let request_body = form_urlencoded::Serializer::new(String::new())
        .append_pair("id", &id)
        .append_pair("name", &name)
        .append_pair("macaddress", &mac_data)
        .finish();


    Ok(request_body)
}
