use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use windows::core::{Result, PCWSTR};
use windows::Win32::Devices::DeviceAndDriverInstallation::*;

#[derive(Serialize, Deserialize)]
struct DeviceInfo {
    class: String,
    enumerator: String,
    description: String,
    manufacturer: String,
    hardware_id: String,
    compatible_id: String,
    class_guid: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() == 2 {
        args[1].as_str()
    } else {
        "device_info.json"
    };

    unsafe {
        let mut file = File::create(filename).expect("create file Failed");
        match get_info() {
            Ok(s) => write!(file, "{}", s).expect("write file failed"),
            Err(e) => println!("{}", e),
        }
    }
}

unsafe fn get_info() -> Result<String> {
    let h_dev_info = SetupDiGetClassDevsW(
        None,
        None,
        None,
        DIGCF_PRESENT | DIGCF_ALLCLASSES,
    )?;
    let mut dev_info_data = SP_DEVINFO_DATA::default();
    dev_info_data.cbSize = std::mem::size_of::<SP_DEVINFO_DATA>() as u32;
    let mut buf: [u8; 1024] = [0; 1024];
    let mut reqsize = 0;
    macro_rules! get_property {
        ($p:expr) => {
            if SetupDiGetDeviceRegistryPropertyW(
                h_dev_info,
                &mut dev_info_data,
                $p,
                None,
                Some(&mut buf),
                Some(&mut reqsize),
            )
            .as_bool()
            {
                PCWSTR(buf.as_ptr().cast::<u16>())
                    .to_string()
                    .expect("failed to construct pcwstr")
            } else {
                "".to_string()
            }
        };
    }

    let mut i = 0;
    let mut final_output: Vec<DeviceInfo> = Vec::new();
    while SetupDiEnumDeviceInfo(h_dev_info, i, &mut dev_info_data).as_bool() {
        final_output.push(DeviceInfo {
            class: get_property!(SPDRP_CLASS),
            enumerator: get_property!(SPDRP_ENUMERATOR_NAME),
            description: get_property!(SPDRP_DEVICEDESC),
            manufacturer: get_property!(SPDRP_MFG),
            hardware_id: get_property!(SPDRP_HARDWAREID),
            compatible_id: get_property!(SPDRP_COMPATIBLEIDS),
            class_guid: get_property!(SPDRP_CLASSGUID),
        });
        i += 1;
    }

    SetupDiDestroyDeviceInfoList(h_dev_info);

    Ok(serde_json::to_string_pretty(&final_output)
        .unwrap_or("construct json string failed".into()))
}
