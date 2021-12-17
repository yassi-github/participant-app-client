use super::mac;
use crate::userdata;
/// generate body.
/// this uses mac addr data from mac.rs
pub fn generate_request_body() -> Result<String, Box<dyn std::error::Error>> {
    let mac_data = mac::gen_mac();
    let readed_data = userdata::input_yaml::read_settings()?;

    let id = readed_data.user_data.user_id;
    let name = readed_data.user_data.name;
    let request_body = format!(
        "id={}\n\
        name={}\n\
        macaddress=aa:aa:aa:aa:aa:aa\n\
        ",
        id, name
    );

    //     let request_body_ = String::from(
    //         r#"id=19T999
    // name=kagawa-taro
    // macaddress=aa:aa:aa:aa:aa:aa"#,
    //     );

    //     assert_eq!(request_body, request_body_);

    Ok(request_body)
}
