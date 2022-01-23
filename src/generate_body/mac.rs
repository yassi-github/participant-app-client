use mac_address;
use mac_address::MacAddressError;

pub fn gen_mac() -> Result<mac_address::MacAddress, MacAddressError> {

    let mac_data = mac_address::get_mac_address()?;


    match mac_data {
        Some(m) => Ok(m),
        None => return Err(MacAddressError::InternalError),
    }
}
