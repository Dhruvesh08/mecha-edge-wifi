use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use wifi_ctrl::{
    sta::{self, NetworkResult, ScanResult},
    Result,
};

pub async fn get_wifi_list() -> Result<Vec<ScanResult>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting wifi-sta example");

    let mut setup = sta::WifiSetup::new()?;

    let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
    setup.set_socket_path(proposed_path);

    let broadcast = setup.get_broadcast_receiver();
    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, scan_wifi, _broadcast) = tokio::join!(
        async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {}", e);
            }
        },
        scan_wifi(requester),
        broadcast_listener(broadcast),
    );

    let wifi_list = scan_wifi.unwrap();
    Ok(wifi_list)
}

async fn scan_wifi(requester: sta::RequestClient) -> Result<Vec<ScanResult>> {
    info!("Requesting scan");
    let scan = requester.get_scan().await?;
    info!("Scan complete");
    info!("Shutting down");
    requester.shutdown().await?;
    Ok(scan.to_vec())
}

pub async fn get_known_wifi_list() -> Result<Vec<NetworkResult>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting wifi-sta example");

    let mut setup = sta::WifiSetup::new()?;

    let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
    setup.set_socket_path(proposed_path);

    let broadcast = setup.get_broadcast_receiver();
    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, known_wifi, _broadcast) = tokio::join!(
        async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {}", e);
            }
        },
        known_wifi(requester),
        broadcast_listener(broadcast),
    );

    let wifi_list = known_wifi.unwrap();
    Ok(wifi_list)
}

async fn known_wifi(requester: sta::RequestClient) -> Result<Vec<NetworkResult>> {
    info!("Requesting scan");
    let scan = requester.get_networks().await?;
    info!("Scan complete");
    info!("Shutting down");
    requester.shutdown().await?;
    Ok(scan)
}

pub async fn get_connect_wifi() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting Wifi Connection");

    dotenv().ok();
    info!("Getting Environment Variables");
    let ssid = std::env::var("SSID").expect("SSID not found");
    let psk = std::env::var("PSK").expect("PSK not found");

    info!("SSID: {}", ssid);
    info!("PSK: {}", psk);

    let mut setup = sta::WifiSetup::new()?;

    let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
    setup.set_socket_path(proposed_path);

    let broadcast = setup.get_broadcast_receiver();
    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, connect_wifi, _broadcast) = tokio::join!(
        async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {}", e);
            }
        },
        connect_wifi(requester, "ssid", "psk"),
        broadcast_listener(broadcast),
    );

    let wifi_list = connect_wifi.unwrap();
    Ok(wifi_list)
}

async fn connect_wifi(requester: sta::RequestClient, ssid: &str, psk: &str) -> Result {
    info!("Getting network id for network");

    let network_id = requester.add_network().await?;
    info!("Network id: {}", network_id);

    requester
        .set_network_ssid(network_id, ssid.to_string())
        .await?;
    requester
        .set_network_psk(network_id, psk.to_string())
        .await?;

    //select network
    requester.select_network(network_id).await?;

    requester.shutdown().await?;
    Ok(())
}

async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result {
    while let Ok(broadcast) = broadcast_receiver.recv().await {
        info!("Broadcast: {:?}", broadcast);
    }
    Ok(())
}
