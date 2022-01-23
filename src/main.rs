//! The client-side command line tool
//! for [participant-app](https://github.com/higuruchi/participant-app).

use participant_app_client::{Cli, Action, httpreq};
use clap::Parser;

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

    let args = Cli::parse();

    let subcmd_result: Result<String, Box<dyn std::error::Error>>;
    match args.action {
        // regist subcommand
        Action::Regist => {
            //(仮)
            subcmd_result = httpreq::request::regist_user();
            // exit_message:
            //    Hello, {stnum} {name}!
            //    Information registed successfully.
            //    Registed MAC Address: {macaddr}
            //    Have a nice day!
            // expected:
            // subcmd_result = httpreq::request::regist_user(regist_args.name, regist_args.stnum);
        },
        // get subcommand
        Action::Get(get_args) => {
            subcmd_result = httpreq::get::get_participants(get_args);
        },
    }

    // exit with status code.
    std::process::exit(match subcmd_result {
        Ok(exit_message) => {
            println!("{}", exit_message);
            0
        },
        Err(err) => {
            eprintln!("ERROR: `{}`", err);
            1
        }
    });
}
