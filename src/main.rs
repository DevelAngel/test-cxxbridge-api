mod cxx;

use anyhow::Result;

use cxx::Device;

fn fetch_device(num: usize) {
    let device = Device::fetch_device(num).expect("device found");
    println!("Fetch device with num {num}:");
    println!("  Device OS: {:?}", device.os());
    println!("  Device Type: {:?}", device.dtype());
}

fn fetch_hsm(num: usize) {
    match Device::fetch_hsm(num) {
        Ok(device) => {
            println!("Fetch HSM device with num {num}:");
            println!("  Device OS: {:?}", device.os());
            println!("  Device Type: {:?}", device.dtype());
            // rust impl
            match device.sign_slot(1) {
                Ok(sig) => {
                    println!("  Device Sign: 0x{}", hex::encode(&sig));
                }
                Err(error) => {
                    println!("  Warning(sign_slot): {}", error);
                }
            }
            // c++ impl
            match device.sign(1) {
                Ok(sig) => {
                    println!("  Device Sign: 0x{}", hex::encode(&sig));
                }
                Err(error) => {
                    println!("  Warning(sign): {}", error);
                }
            }
        }
        Err(error) => {
            println!("Warning: {}", error);
        }
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");

    (1..=6).for_each(fetch_device);

    // 0 -> Warning: num < 1 not allowed
    // 4 -> Warning: HSM device 4 not found
    // 5 -> Warning: HSM device 5 not found
    // 6 -> Warning: HSM device 6 not found
    (0..=6).for_each(fetch_hsm);

    println!(".. no errors detected ..");
    Ok(())
}
