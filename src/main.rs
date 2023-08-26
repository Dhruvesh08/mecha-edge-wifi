// main.rs (in your project)
mod wifi_lib;
use wifi_lib::{get_connect_wifi, remove_wifi_network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let wifi_list = get_known_wifi_list().await?;
    // println!("Wi-Fi networks: {:?}", wifi_list);
    let wifi_list = remove_wifi_network(2).await?;

    Ok(())
}
