use crate::userdata::input_yaml;

pub fn generate_headerdata() -> Result<String, Box<dyn std::error::Error>> {
    let readed_data = input_yaml::read_settings()?;
    // like: /user
    let path = readed_data.dest_data.request_path;

    let request_body = format!(
        "POST {} HTTP/1.1\r\n\
        Host: {}\r\n\
        Content-Type: application/x-www-form-urlencoded\r\n\
        \r\n\
        ",
        path, readed_data.dest_data.server_dest
    );

    Ok(request_body)
}
