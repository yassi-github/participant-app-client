//! The client-side command line tool
//! for [participant-app](https://github.com/higuruchi/participant-app).

use participant_register::httpreq::request;

/// Regist user infomation.
/// POST id, name, MAC address to /user
///
/// 1. Get user data from yaml file.
///     - id
///     - name
/// 1. Get MAC address form system using
/// [mac_address crate](https://docs.rs/mac_address/latest/mac_address/)
/// 1. Generate HTTP request body
/// 1. regist_user by POST Request
fn main() {
    // request_body_data is like:
    // "id={}\n\
    // name={}\n\
    // macaddress=aa:aa:aa:aa:aa:aa\n\
    // "
    // uncode.  :poop:
    // let request_header = header::generate_headerdata().unwrap();
    // let request_body_data = user::generate_request_body().unwrap();

    // exit with status code.
    std::process::exit(match request::regist_user() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("An Error Occured!\nError is: {:?}", err);
            1
        }
    });
}
