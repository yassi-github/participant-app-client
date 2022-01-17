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
// const BUFFER_SIZE: usize = 12;

pub fn fetch(params: Params) -> Result<String, Box<dyn std::error::Error>> {
    let request_content_string: String;
    // let url_params: String;
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
            // url_params = String::new();
            "POST"
        }
    };

    // server address
    let address: String = input_yaml::read_settings()?.dest_data.server_dest;
    let address: std::net::SocketAddr = address.parse().expect("Invalid address");

    let mut stream = TcpStream::connect(&address)?;

    // get param ver
    // let http_request_string = format!("{} {}{} HTTP/1.1\r\nHOST: {}\r\n{}", method, params.path, url_params, address, request_content_string);
    let http_request_string = format!(
        // "{} {} HTTP/1.1\r\nHost: {}\r\n{}",
        "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n{}\r\n",
        method, params.path, address, request_content_string
    );
    stream.write(http_request_string.as_bytes())?;
    // let rq = stream.write_all(format!("{}", http_request_string).as_bytes())?;
    // println!("request: {:?}", rq);
    let mut response_vec = Vec::<u8>::new();
    // let mut count = 0;

    let mut reader = BufReader::new(&stream);
    // let mut buffer = vec![0u8, 26u8];
    const BUFFER_INIT_PADDING_BIT: u8 = 0;
    let mut buffer = vec![BUFFER_INIT_PADDING_BIT; BUFFER_SIZE];
    let mut left_buffer_len = BUFFER_SIZE;
    // println!("request_string: ```{}```",http_request_string);
    loop {
        // println!("start loop: {}", count);
        // count += 1;
        // let mut buffer = Vec::new();
        // let mut buffer = Vec::with_capacity(BUFFER_SIZE);
        // stream.take(BUFFER_SIZE as u64).read_to_end(&mut buffer)?;
        // reader.read_until(b'\n', &mut buffer).expect("failed to read from socket");
        // print!("{}", std::str::from_utf8(&buffer).expect("failed to convert to String"));
        // println!("LOOPING:\n\t{:?}\n\t{:?}\n\t{:?}\n\t{:?}", buffer, reader, reader.buffer(), response_vec);
        // debug
        // break;
        // println!("{:?} {:?} {:?}", buffer, reader, response_vec);

        match reader.read_exact(&mut buffer) {
            Ok(_) => {
                // do nothing
                // println!("OK: ```{:?}```, len: {}, {:?}", std::str::from_utf8(&buffer)?, buffer.len(), buffer);
                response_vec.extend_from_slice(&buffer);
                left_buffer_len = reader.buffer().len();
            }
            Err(e) => {
                // println!("err!");
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    // BUFFERSIZE 大: ０が残る（0を含めてlenで数えてる） => rpositionで消せる
                    // BUFFERSIZE 小: おｋ
                    // 出力: null文字なので見た目問題はない
                    // let non_zero_idx = left_buffer_len - 1;
                    // BUFFERSIZE 小(Ok のパターンがあった)のとき, reader.buffer()はしっかり残りのバイトだけ持ってくれるが，
                    // buffer はそれプラスよくわからんバイトを持ってやがるので，適切なidxまでのbufferに更新する．
                    // ただし，BUFFERSIZE 大(bufferを埋められず，いきなりここが実行された)のときは，left_buffer_len は BUFFER_SIZE であるので，
                    // bufferをここで更新しても何も変わらない(0u8を含むbufferのまま)．
                    buffer = buffer[..left_buffer_len].to_vec();
                    // println!("EOF(before): ```{:?}```, len: {}, {:?},\n\tREADER: {:?}, Bufer: {:?}", std::str::from_utf8(&buffer)?, buffer.len(), buffer, reader, reader.buffer());
                    // BUFFERSIZE 大: おｋ（repositionで消してる）
                    // BUFFERSIZE 小: はみ出す（ERRでいらんとこだしてる）
                    // bufferに残ってる 0u8 を消すための idx を求める
                    let non_zero_idx: usize = match buffer.iter().rposition(|&x| x != BUFFER_INIT_PADDING_BIT) {
                        Some(idx) => idx,
                        // Some(idx) => buffer.len(),
                        // full buffer size
                        None => BUFFER_SIZE,
                    };
                    // println!("EOF(after): ```{:?}```, len: {}, {:?}, {:?},\n\tREADER: {:?}, Bufer: {:?}", std::str::from_utf8(&buffer)?, buffer.len(), buffer, &buffer[..=non_zero_idx], reader, reader.buffer());
                    // if reader.len == 0 {
                    //     non_zero_idx = reader.len;
                    // }
                    // response_vec += std::str::from_utf8(&buffer[..non_zero_idx])?;
                    response_vec.extend_from_slice(&buffer[..=non_zero_idx]);
                    break;
                }
                // println!("OtherERROR!!: ```{:?}```", e);
            }
        }
        // println!("{:?} {:?} {:?}", buffer, reader, response_vec);
        // reader.read_to_end(&mut buffer)?;

        // if count > 100 {
        //     break;
        // }
        // println!("LOOPING:\n\t{:?}\n\t{:?}\n\t{:?}\n\t{:?}", buffer, reader, reader.buffer(), response_vec);
    }

    // if reader.stream_len()? > 0 {
    //     reader.read_to_end(&mut buffer)?;
    // }
    drop(stream);

    // return only the body
    Ok(std::str::from_utf8(&response_vec)?.split_once("\r\n\r\n").ok_or("Invalid response")?.1.to_string())
    // Ok(String::from(std::str::from_utf8(&response_vec.iter().flatten())?))
}
