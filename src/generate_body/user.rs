use super::mac;
use crate::userdata;

/// generate body.
/// this uses mac addr data from mac.rs
pub fn generate_request_body() -> Result<String, Box<dyn std::error::Error>> {
    let mac_data = mac::gen_mac()?.to_string();
    let readed_data = userdata::input_yaml::read_settings()?;

    let id = readed_data.user.id;
    let name = readed_data.user.name;

    // sample POST body:
    // {"id":"19T999","name":"test-name","macaddress":"3c:06:30:43:3f:52"}
    let request_body = serde_json::json!({
        "id": id,
        "name": name,
        "macaddress": mac_data,
    }).to_string();


    Ok(request_body)
}
