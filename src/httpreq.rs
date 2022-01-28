pub mod get;
pub mod request;

use crate::userdata::input_yaml;

use std::io::{BufReader, Read, Write};
use std::net::TcpStream;


#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::Get
    }
}

#[derive(Default)]
pub struct Params {
    pub method: HttpMethod,
    pub path: String,
    pub body: Option<String>,
}

const BUFFER_SIZE: usize = 256;

pub fn fetch(params: Params) -> Result<String, Box<dyn std::error::Error>> {
    let request_content_string: String;
    let method = match params.method {
        HttpMethod::Get => {
            // WILL create get url params like:
            // url_params = String::from("?key=value&key2=value2");
            request_content_string = params.body.unwrap_or_default();
            "GET"
        }
        HttpMethod::Post => {
            // create post request body
            request_content_string = match params.body {
                Some(body) => format!(
"Content-Type: application/x-www-form-urlencoded\r\n\
Content-Length: {}\r\n\
\r\n\
{}",
                    body.len(),
                    body
                ),
                None => String::new(),
            };
            "POST"
        }
    };

    // server address
    let address: String = input_yaml::read_settings()?.server.destination;
    let address: std::net::SocketAddr = address.parse().expect("Invalid address");

    let mut stream = TcpStream::connect(&address)?;

    let http_request_string = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n{}\r\n",
        method, params.path, address, request_content_string
    );
    stream.write(http_request_string.as_bytes())?;
    let mut response_vec = Vec::<u8>::new();

    let mut reader = BufReader::new(&stream);
    const BUFFER_INIT_PADDING_BIT: u8 = 0;
    let mut buffer = vec![BUFFER_INIT_PADDING_BIT; BUFFER_SIZE];
    let mut left_buffer_len = BUFFER_SIZE;
    loop {

        match reader.read_exact(&mut buffer) {
            Ok(_) => {
                response_vec.extend_from_slice(&buffer);
                left_buffer_len = reader.buffer().len();
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    // BUFFERSIZE 小(Ok のパターンがあった)のとき, reader.buffer()はしっかり残りのバイトだけ持ってくれるが，
                    // buffer はそれプラスよくわからんバイトを持ってやがるので，適切なidx(left_buffer_len)までのbufferに更新する．
                    // ただし，BUFFERSIZE 大(bufferを埋められず，いきなりここが実行された)のときは，left_buffer_len は BUFFER_SIZE であるので，
                    // bufferをここで更新しても何も変わらない(0u8を含むbufferのまま)．
                    buffer = buffer[..left_buffer_len].to_vec();
                    // bufferが空のとき，スライスできないのでここで終了
                    if buffer.len() == 0 {
                        break;
                    }
                    // bufferに残ってる 0u8 を消すための idx を求める
                    let non_zero_idx: usize = match buffer.iter().rposition(|&x| x != BUFFER_INIT_PADDING_BIT) {
                        Some(idx) => idx,
                        // buffer が全て 0u8 の場合．
                        None => break,
                    };
                    response_vec.extend_from_slice(&buffer[..=non_zero_idx]);
                    break;
                }
            }
        }
    }

    drop(stream);

    // return only the body
    Ok(std::str::from_utf8(&response_vec)?.split_once("\r\n\r\n").ok_or("Invalid response")?.1.to_string())
}
