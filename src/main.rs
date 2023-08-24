// // main.rs (in your project)
// mod wifi_lib;
// use wifi_lib::get_wifi_list;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let wifi_list = get_wifi_list().await?;
//     println!("Wi-Fi networks: {:?}", wifi_list);
//     Ok(())
// }

//defaul rust hello world program

use log::info;
use wifi_lib::{Broadcast, WifiManager};

mod wifi_lib;

#[tokio::main]
async fn main() {
    let mut wifi_manager = WifiManager::new("wlan0").unwrap();
    // Scan for wifi networks and print the results
    let scan_results = wifi_manager.scan_wifi().await.unwrap();
    for scan_result in scan_results.iter() {
        info!("   {:?}", scan_result);
    }

    loop {
        let broadcast = wifi_manager.listen_broadcast().await.unwrap();
        match broadcast {
            Broadcast::Connected => info!("Connected to wifi"),
            Broadcast::Disconnected => info!("Disconnected from wifi"),
            Broadcast::NetworkNotFound => todo!(),
            Broadcast::WrongPsk => todo!(),
            Broadcast::Ready => todo!(),
        }
    }
}
