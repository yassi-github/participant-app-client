use super::mac;
use crate::userdata;
/// generate body.
/// this uses mac addr data from mac.rs
pub fn generate_request_body() -> String {
    let mac_data = mac::gen_mac();
    let user_data = userdata::input_yaml::read_settings();
//     let request_body_ = String::from(r#"id=19T999
// name=kagawa-taro
// macaddress=aa:aa:aa:aa:aa:aa"#);
    let id = String::from("19T999");
    let name = String::from("kagawa-taro");
    let request_body = format!("id={}\n\
                                name={}\n\
                                macaddress=aa:aa:aa:aa:aa:aa\n\
                                ", id, name);

    request_body
}
