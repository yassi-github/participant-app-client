//! The client-side command line tool
//! for [participant-app](https://github.com/higuruchi/participant-app).

use participant_register::userdata::input_yaml;
use participant_register::generate_body::{mac, user};
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
    let body_data = user::generate_request_body();
    let server_dest = input_yaml::read_settings();
    
    // exit with status code.
    std::process::exit(match request::regist_user(body_data, server_dest) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("An Error Occured!\nError is: {:?}", err);
            1
        }
    });
}
