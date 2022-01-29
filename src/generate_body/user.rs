use super::mac;
use crate::userdata;

/// generate body.
/// if `spectfied_macaddress` arg is empty,
/// this will get mac addr data from mac.rs .
/// otherwise, use mac addr data by `spectfied_macaddress` arg.
pub fn generate_request_body(spectfied_macaddress: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let mac_data: String;
    match spectfied_macaddress {
        Some(mac_addr) => {
            mac_data = mac_addr.to_string();
        },
        None => {
            mac_data = mac::gen_mac()?.to_string();
        },
    }

    let readed_data = userdata::input_yaml::read_settings()?;

    let id = readed_data.user.id;
    let name = readed_data.user.name;

    // sample POST body:
    // {"id":"19T999","name":"test-name","macaddress":"3c:06:30:43:3f:52"}
    // expected PUT body:
    // {"id:"19T999","macaddress":"3c:06:30:43:3f:52"}
    // But this structure can also be used to PUT `macaddr`. For now.
    let request_body = serde_json::json!({
        "id": id,
        "name": name,
        "macaddress": mac_data,
    }).to_string();


    Ok(request_body)
}
