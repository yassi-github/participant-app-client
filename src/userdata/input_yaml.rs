use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct SettingsData {
    pub user: UserData,
    pub server: DestData,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DestData {
    pub destination: String,
    pub regist_path: String,
    pub get_path: String,
}

/// read user conf from yaml file
///
/// example yaml format:
/// ```
/// user:
///     id: '19T991'
///     name: "kagawa-sabro"
/// server:
///     destination: "192.168.12.14:1323"
///     regist_path: "/user"
///     get_path: "/participants/:year/:month/:day"
/// ```
pub fn read_settings() -> Result<SettingsData, Box<dyn std::error::Error>> {
    // read from yaml file
    // receive Configures struct from read_configures_from_yaml_file()
    const CONFIG_FILE_PATH: &str = "./participant-app-client.conf";
    let something_config = read_configures_from_yaml_file(CONFIG_FILE_PATH)?;


    Ok(something_config)
}

// This function reads configures to String from yaml file.
// Filename is "participant-app.conf".
pub fn read_configures_from_yaml_file(config_file_name: &str) -> Result<SettingsData, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(config_file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let configures: SettingsData = serde_yaml::from_str(&contents)?;


    Ok(configures)
}
