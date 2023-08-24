// main.rs (in your project)
mod wifi_lib;
use wifi_lib::get_known_wifi_list;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wifi_list = get_known_wifi_list().await?;
    println!("Wi-Fi networks: {:?}", wifi_list);
    Ok(())
}
