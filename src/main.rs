// main.rs (in your project)
//enable clippy
#![deny(clippy::all)]
mod wifi_lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let wifi_list = get_known_wifi_list().await?;
    // println!("Wi-Fi networks: {:?}", wifi_list);

    //get wifi list
    let wifi_list = wifi_lib::get_wifi_list().await?;

    //print all wifi networks
    println!("Wi-Fi networks: {:?}", wifi_list);

    Ok(())
}
