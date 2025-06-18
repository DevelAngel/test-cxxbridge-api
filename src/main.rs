use test_cxxbridge::{Device, Linux};

use anyhow::Result;
use tracing_subscriber::EnvFilter;

fn fetch_device(num: usize) {
    let device = Device::fetch_device(num).expect("device found");
    println!("Fetch device with num {num}:");
    println!("  Device OS: {:?}", device.os());
    println!("  Device Type: {:?}", device.dtype());
    // println!("  Name: {}", device.name()); // not possible for AnyOS
}

fn fetch_hsm(num: usize) {
    match Device::fetch_hsm(num) {
        Ok(device) => {
            println!("Fetch HSM device with num {num}:");
            println!("  Device OS: {}", device.os());
            println!("  Device Type: {}", device.dtype());
            // println!("  Name: {}", device.name()); // not possible for AnyOS
            println!("  Max Slots: {}", device.max_slots());
            for n in 0..=device.max_slots() {
                // rust impl
                match device.sign_slot(n) {
                    Ok(sig) => {
                        println!("  Device Sign(sign_slot): 0x{}", hex::encode(&sig));
                    }
                    Err(error) => {
                        println!("  Warning(sign_slot): {}", error);
                    }
                }
                // c++ impl
                match device.sign(n) {
                    Ok(sig) => {
                        println!("  Device Sign(sign):      0x{}", hex::encode(&sig));
                    }
                    Err(error) => {
                        println!("  Warning(sign): {}", error);
                    }
                }
            }
        }
        Err(error) => {
            println!("Warning: {}", error);
        }
    }
}

fn fetch_hsm_and_create_key(num: usize) {
    match Device::fetch_hsm(num) {
        Ok(mut device) => {
            println!("Fetch HSM device with num {num}:");
            for n in 0..=device.max_slots() {
                match device.create_key(n) {
                    Ok(()) => {
                        println!("  Key created for Slot {n}");
                    }
                    Err(error) => {
                        println!("  Warning(create_key): {}", error);
                    }
                }
            }
        }
        Err(error) => {
            println!("Warning: {}", error);
        }
    }
}

fn fetch_linux_hsm(num: usize) {
    match Device::fetch_hsm_with::<Linux>(num) {
        Ok(device) => {
            println!("Fetch HSM Linux device with num {num}:");
            println!("  Device OS: {}", device.os());
            println!("  Device Type: {}", device.dtype());
            println!("  Name: {}", device.name());
        }
        Err(error) => {
            println!("Warning: {}", error);
        }
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .compact()
        .init();

    tracing::info!("Hello, world!");

    (1..=6).for_each(fetch_device);
    println!("--------------------------------");

    // 0 -> Warning: invalid device number 0
    // 4 -> Warning: HSM device 4 not found
    // 5 -> Warning: HSM device 5 not found
    // 6 -> Warning: HSM device 6 not found
    (0..=6).for_each(fetch_hsm);
    println!("--------------------------------");

    (1..=3).for_each(fetch_hsm_and_create_key);
    println!("--------------------------------");
    (1..=3).for_each(fetch_hsm);
    println!("--------------------------------");
    (1..=3).for_each(fetch_linux_hsm);
    println!("--------------------------------");

    println!(".. no errors detected ..");
    Ok(())
}
