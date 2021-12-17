use super::user;
use crate::userdata::input_yaml;

pub fn generate_headerdata() -> Result<String, Box<dyn std::error::Error>> {
    let readed_data = input_yaml::read_settings()?;
    // like: /user
    let path = readed_data.dest_data.request_path;
    let address = readed_data.dest_data.server_dest;

    let body_data = user::generate_request_body()?;

    let request_body = format!(
        "POST {} HTTP/1.1\r\n\
         HOST: {}\r\n\
         Content-Type: application/x-www-form-urlencoded\r\n\
         Content-Length: {}\r\n\
         \r\n\
         ",
        path,
        address,
        body_data.len()
    );

    Ok(request_body)
}
