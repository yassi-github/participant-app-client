use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Configures {
    user_id: String,
    name: String,
    server_dest: String,
    regist_path: String,
    get_path: String,
}

pub struct SettingsData {
    pub user_data: UserData,
    pub dest_data: DestData,
}

pub struct UserData {
    pub user_id: String,
    pub name: String,
}

pub struct DestData {
    pub server_dest: String,
    pub regist_path: String,
    pub get_path: String,
}

/// read user conf from yaml file
///
/// example yaml format:
/// ```
/// user_id: '19T999'
/// name: "kagawa-taro"
/// server_dest: "192.168.12.10:8080"
/// regist_path: "/user"
/// get_path: "/participants/:year/:month/:day"
/// ```
pub fn read_settings() -> Result<SettingsData, Box<dyn std::error::Error>> {
    // read from yaml file
    // receive Configures struct from read_configures_from_yaml_file()
    let something_config = read_configures_from_yaml_file()?;

    let settings_data = SettingsData {
        user_data: UserData {
            user_id: something_config.user_id,
            name: something_config.name,
        },
        dest_data: DestData {
            server_dest: something_config.server_dest,
            regist_path: something_config.regist_path,
            get_path: something_config.get_path,
        },
    };


    Ok(settings_data)
}

// This function reads configures to String from yaml file.
// Filename is "participant-app.conf".
pub fn read_configures_from_yaml_file() -> Result<Configures, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("participant-app.conf")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let configures: Configures = serde_yaml::from_str(&contents)?;


    Ok(configures)
}
