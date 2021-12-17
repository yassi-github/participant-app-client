use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Configures {
    user_id: String,
    name: String,
    request_path: String,
}

pub struct SettingsData {
    pub user_data: UserData,
    pub request_path: PathData,
}

pub struct UserData {
    pub user_id: String,
    pub name: String,
}

pub struct PathData {
    pub request_path: String,
}

/// read user conf from yaml file
///
/// example yaml format:
/// ```
/// userid: 19T999
/// name: kagawa-taro
/// request_path: /user
/// ```
pub fn read_settings() -> Result<SettingsData, Box<dyn std::error::Error>> {
    // read from yaml file
    // receive Configures struct from read_configures_from_yaml_file()
    let something_config = read_configures_from_yaml_file()?;
    // let something_config = match read_configures_from_yaml_file() {
    //     Ok(config) => config,
    //     Err(err) => return Err(err),
    // };
    // println!("{:?}, userid: {:?}, name: {:?}", something_config, something_config.userid, something_config.name);

    // let readed_data_user = get_userdata();
    // let readed_data_path = get_request_path();

    let settings_data = SettingsData {
        user_data: UserData {
            user_id: something_config.user_id,
            name: something_config.name,
        },
        request_path: PathData {
            request_path: something_config.request_path,
        },
    };

    // let readed_data = SettingsData {
    //     userdata: readed_data_user,
    //     request_path: readed_data_path,
    // };

    Ok(settings_data)
}

// This function reads configures to String from yaml file.
// Filename is "participant-app.conf".
// To deserialise yaml file to String, use 'serde_yaml' crate.
pub fn read_configures_from_yaml_file() -> Result<Configures, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("participant-app.conf")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let configures: Configures = serde_yaml::from_str(&contents)?;
    Ok(configures)
}

// pub fn get_userdata() -> UserData {
//     //
//     let readed_data_user = UserData {
//         userid: String::from("19T999"),
//         name: String::from("kagawa-taro"),
//     };

//     readed_data_user
// }

// pub fn get_request_path() -> PathData {
//     //
//     let readed_data_path = PathData {
//         request_path: String::from("/user"),
//     };

//     readed_data_path
// }
