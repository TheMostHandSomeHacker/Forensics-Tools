use crate::common::{ZIP_CVE_2023_31102, WINRAR_CVE_2023_38831};

use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

fn convert_to_interger(version: String) -> i32{
    
    let version_no_dots: String = version.chars().filter(|&c| c != '.').collect();

    // Parse the string as an interger
    if let Ok(version_int) = version_no_dots.parse::<i32>() {
        return version_int;
    } else {
        return 0;
    }
}

pub fn check_7zip() -> i32{
    let key_path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";

    let zip_verions: i32;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey(key_path).expect("Failed to open registry");

    for key_name in key.enum_keys().map(|key_name| key_name.unwrap()) {
        let subkey = key.open_subkey(&key_name).expect("Failed to open subkey");
        let display_name: String = subkey.get_value("DisplayName").unwrap_or_default();

        if display_name.contains("7-Zip"){
            let display_version: String = subkey.get_value("DisplayVersion").unwrap_or_default();
            zip_verions = convert_to_interger(display_version);
            if zip_verions < ZIP_CVE_2023_31102 {
                return 3;
            }
            return 2;
        }
    }
    1
}

pub fn check_winrar() -> i32{
    let key_path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";

    let winrar_version: i32;
    // Open the registry key for reading
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey(key_path).expect("Failed to open registry");

    // Iterate over subkeys and collect application data
    for key_name in key.enum_keys().map(|key_name| key_name.unwrap()) {
        let subkey = key.open_subkey(&key_name).expect("Failed to open subkey");
        let display_name: String = subkey.get_value("DisplayName").unwrap_or_default();

        if display_name.contains("WinRAR") {
            let display_version: String = subkey.get_value("DisplayVersion").unwrap_or_default();
            winrar_version = convert_to_interger(display_version);
            if winrar_version < WINRAR_CVE_2023_38831 {
                return 3
            }
            return 2;
        }
    }
    1
                                            
}
