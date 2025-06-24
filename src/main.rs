use test_cxxbridge::{Device, Error, Linux};

use anyhow::{Context, Result};
use tracing_subscriber::EnvFilter;

fn fetch_device(num: usize) {
    match Device::fetch_device(num).context(format!("failed to fetch device with num {num}")) {
        Ok(device) => {
            println!("Fetch device with num {num}:");
            println!("  Device OS: {:?}", device.os());
            println!("  Device Type: {:?}", device.dtype());
            // println!("  Name: {}", device.name()); // not possible for AnyOS
        }
        Err(error) => {
            println!("Warning: {error:#}");
        }
    }
}

fn fetch_hsm(num: usize) {
    match Device::fetch_hsm(num).context(format!("failed to fetch hsm with num {num}")) {
        Ok(device) => {
            println!("Fetch HSM device with num {num}:");
            println!("  Device OS: {}", device.os());
            println!("  Device Type: {}", device.dtype());
            // println!("  Name: {}", device.name()); // not possible for AnyOS
            println!("  Max Slots: {}", device.max_slots());
            for n in 0..=device.max_slots() {
                // rust impl
                match device
                    .sign_slot(n)
                    .context(format!("failed to sign with slot {n}"))
                {
                    Ok(sig) => {
                        println!("  Device Sign(sign_slot): 0x{sig}", sig = hex::encode(&sig));
                    }
                    Err(error) => {
                        println!("  Warning(sign_slot): {error:#}");
                    }
                }
                // c++ impl
                match device
                    .sign(n)
                    .map_err(Error::from)
                    .context(format!("failed to sign with slot {n}"))
                {
                    Ok(sig) => {
                        println!("  Device Sign(sign):      0x{sig}", sig = hex::encode(&sig));
                    }
                    Err(error) => {
                        println!("  Warning(sign): {error:#}");
                    }
                }
            }
        }
        Err(error) => {
            println!("Warning: {error:#}");
        }
    }
}

fn fetch_hsm_and_create_key(num: usize) {
    match Device::fetch_hsm(num).context(format!("failed to fetch hsm with num {num}")) {
        Ok(mut device) => {
            println!("Fetch HSM device with num {num}:");
            for n in 0..=device.max_slots() {
                match device
                    .create_key(n)
                    .map_err(Error::from)
                    .context(format!("failed to create key for slot {n}"))
                {
                    Ok(()) => {
                        println!("  Key created for Slot {n}");
                    }
                    Err(error) => {
                        println!("  Warning(create_key): {error:#}");
                    }
                }
            }
        }
        Err(error) => {
            println!("Warning: {error:#}");
        }
    }
}

fn fetch_linux_hsm(num: usize) {
    match Device::fetch_hsm_with::<Linux>(num)
        .context(format!("failed to fetch linux hsm with num {num}"))
    {
        Ok(device) => {
            println!("Fetch linux hsm device with num {num}:");
            println!("  Device OS: {}", device.os());
            println!("  Device Type: {}", device.dtype());
            println!("  Name: {}", device.name());
        }
        Err(error) => {
            println!("Warning: {error:#}");
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

    (0..=7).for_each(fetch_device);
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
