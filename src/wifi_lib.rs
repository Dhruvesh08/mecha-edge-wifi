// wifi_lib/src/lib.rs

use env_logger::Env;
use log::{error, info};
use wifi_ctrl::{sta::{self, ScanResult}, Result};

pub async fn get_wifi_list() -> Result<Vec<ScanResult>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting wifi-sta example");

    let mut setup = sta::WifiSetup::new()?;

    let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
    setup.set_socket_path(proposed_path);

    let broadcast = setup.get_broadcast_receiver();
    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, app, _broadcast) = tokio::join!(
        async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {}", e);
            }
        },
        app(requester),
        broadcast_listener(broadcast),
    );

    let wifi_list = app.unwrap();
    Ok(wifi_list)
}

async fn app(requester: sta::RequestClient) -> Result<Vec<ScanResult>> {
    info!("Requesting scan");
    let scan = requester.get_scan().await?;
    info!("Scan complete");
    info!("Shutting down");
    requester.shutdown().await?;
    Ok(scan.to_vec())
}

async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result {
    while let Ok(broadcast) = broadcast_receiver.recv().await {
        info!("Broadcast: {:?}", broadcast);
    }
    Ok(())
}
