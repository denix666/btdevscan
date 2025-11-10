#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Manager;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Mac address of the device you want to scan
    #[arg(short, long)]
    mac_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mac = args.mac_address.to_uppercase();

    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().next().ok_or("❌ No Bluetooth adapters found")?;

    println!("🔍 Scanning BLE-devices...");
    central.start_scan(ScanFilter::default()).await?;
    time::sleep(Duration::from_secs(3)).await;

    let peripherals = central.peripherals().await?;
    let peripheral = peripherals
        .into_iter()
        .find(|p| p.address().to_string() == mac)
        .ok_or("❌ Device not found")?;

    println!("📡 Connecting to {}", mac);
    peripheral.connect().await?;
    peripheral.discover_services().await?;

    println!("✅ Connected. Services and characteristics:");

    for service in peripheral.services() {
        println!("🧩 Service UUID: {}", service.uuid);
        for characteristic in service.characteristics {
            println!("  ↳ Characteristic UUID: {}", characteristic.uuid);
            println!("    Properties: {:?}", characteristic.properties);
            println!("    Descriptors: {}", characteristic.descriptors.len());
        }
    }

    peripheral.disconnect().await?;
    println!("🔌 Dosconnected.");

    Ok(())
}
