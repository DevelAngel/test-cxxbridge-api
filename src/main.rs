mod cxx;

use anyhow::Result;

use cxx::Device;

fn fetch_device(num: usize) {
    let device: Device = cxx::fetch_device(num).expect("device found");
    println!("Fetch device with num {num}:");
    println!("  Device OS: {:?}", device.os());
    println!("  Device Type: {:?}", device.dtype());
}

fn fetch_hsm(num: usize) {
    match cxx::fetch_hsm(num) {
        Ok(device) => {
            println!("Fetch HSM device with num {num}:");
            println!("  Device OS: {:?}", device.os());
            println!("  Device Type: {:?}", device.dtype());
        }
        Err(error) => {
            println!("Warning: {}", error);
        }
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");

    (1..=6).for_each(fetch_device);
    (0..=6).for_each(fetch_hsm);

    println!(".. no errors detected ..");
    Ok(())
}
