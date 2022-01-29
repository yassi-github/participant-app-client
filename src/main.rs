//! The client-side command line tool
//! for [participant-app](https://github.com/higuruchi/participant-app).

use participant_app_client::{Cli, Action, httpreq, userdata, generate_body};
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
    let args = Cli::parse();

    let subcmd_result: Result<String, Box<dyn std::error::Error>>;
    let mut success_exit_message: String = String::from("");
    match args.action {
        // change-macaddr subcommand
        Action::ChangeMacaddr(change_macaddr_args) => {
            subcmd_result = httpreq::put::change_macaddress(&change_macaddr_args);

            let macaddr = &change_macaddr_args.macaddr;
            success_exit_message = format!("Changed MAC Address to: {}", macaddr);
        },
        // regist subcommand
        Action::Regist => {
            subcmd_result = httpreq::post::regist_user();

            let message_macaddr = format!("Registed MAC Address: {}", generate_body::mac::gen_mac().expect("Failed to get MAC address!").to_string());

            let readed_userdata = userdata::input_yaml::read_settings().expect("Failed to read config file!").user;
            success_exit_message = format!("
Domo, {stnum} {name}-san!
Information registed successfully.
{message_macaddr}

If you want to change your MAC address,
please run `participant-app-client change-macaddr --macaddr <CORRECT MACADDRESS>` command.
", stnum = readed_userdata.id, name = readed_userdata.name, message_macaddr = message_macaddr);
        },
        // get subcommand
        Action::Get(get_args) => {
            subcmd_result = httpreq::get::get_participants(get_args);
        },
    }

    // exit with status code.
    std::process::exit(match subcmd_result {
        Ok(response) => {
            let exit_message: String = match participant_app_client::read_resposne_message(&response) {
                // response JSON key was "message" (Error message)
                Ok(message) => {
                    // internal server error
                    if message == "Internal Server Error" {
                        String::from("Oops! An error has occurred in the API Server...\nYou might already registed your MAC address?")
                    // other error
                    } else {
                        format!("Error: {}", message)
                    }
                },
                // case: get, successed(e.g. {Status: .+})
                Err(_) => {                
                    match response.as_str() {
                        // regist or change-macaddr successed
                        "{\"Status\":true}\n" => {
                            success_exit_message
                        },
                        // get response
                        _ => {
                            response
                        }
                    }
                }
            };
            println!("{}", exit_message);
            0
        },
        Err(err) => {
            eprintln!("ERROR: `{}`", err);
            1
        }
    });
}
