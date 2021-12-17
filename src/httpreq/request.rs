use crate::generate_body::user;
use crate::userdata::input_yaml;
/// example data
///
/// ```
/// id=19T999
/// name=kagawa-taro
/// macaddress=aa:aa:aa:aa:aa:aa
/// ```
// pub struct BodyData {
//     id: String,
//     name: String,
// }

/// example data
///
/// ```
/// address: 192.168.10.10
/// path: /user
/// header: Content-Type:application/x-www-form-urlencoded
/// ```
// pub struct ServerDest {
//     address: String,
//     path: String,
//     header: String,
// }
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

// pub fn regist_user(data: BodyData, dest: ServerDest) -> Result<i32, String> {
pub fn regist_user() -> Result<String, Box<dyn std::error::Error>> {
    // post like:
    // wget --post-data id=data.id\nname=data.name\nmacaddress=data.macaddr" dest.address+dest.path
    let readed_data = input_yaml::read_settings()?;
    // like: /user
    let path = readed_data.dest_data.request_path;

    let address: String = readed_data.dest_data.server_dest;
    let address: std::net::SocketAddr = address.parse().unwrap();

    let mut stream = TcpStream::connect(&address)?;

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
    // let headers = header::generate_headerdata()?;

    stream.write(format!("{}{}", request_body, body_data).as_bytes())?;
    // stream.write(b"POST / HTTP/1.1\r\n").unwrap();
    // stream.write(b"Host: 192.168.12.10:8080\r\n").unwrap();
    // stream.write(b"Content-Length: 17\r\n").unwrap();
    // stream.write(b"Content-Type: application/x-www-form-urlencoded\r\n").unwrap();
    // stream.write(b"\r\n").unwrap();
    // stream.write(b"from rust\nhello!\n").unwrap();

    let mut reader = BufReader::new(&stream);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    print!("{}", std::str::from_utf8(&buffer).unwrap());

    // return this:
    // "HTTP/1.0 200 OK\r\nServer: BaseHTTP/0.6 Python/3.8.10\r\nDate: Fri, 17 Dec 2021 18:14:34 GMT\r\nContent-type: text/html; charset=utf-8\r\n\r\nmethod: POST\nparams: {}\nbody  : from rust\nhello!"

    drop(stream);

    Ok(String::from("OK"))
}
