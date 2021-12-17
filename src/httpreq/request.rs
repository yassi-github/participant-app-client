
/// example data
/// 
/// ```
/// id=19T999
/// name=kagawa-taro
/// macaddress=aa:aa:aa:aa:aa:aa
/// ```
pub struct BodyData {
    id: String,
    name: String,
}

/// example data
/// 
/// ```
/// address: 192.168.10.10
/// path: /user
/// header: Content-Type:application/x-www-form-urlencoded
/// ```
pub struct ServerDest {
    address: String,
    path: String,
    header: String,
}

// pub fn regist_user(data: BodyData, dest: ServerDest) -> Result<i32, String> {
pub fn regist_user(data: String, dest: crate::userdata::input_yaml::SettingsData) -> Result<i32, String> {
    // post like:
    // wget --post-data id=data.id\nname=data.name\nmacaddress=data.macaddr" dest.address+dest.path
    println!("POST REQUEST!!");

    // result
    let is_requested = false;
    if is_requested {
        // Err(ResistError::NotReachable)
        Err(String::from("hogeErr"))
    } else {
        Ok(0)
    }
}
