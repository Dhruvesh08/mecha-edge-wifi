use env_logger::Env;
use log::{error, info};
use once_cell::sync::OnceCell;
use wifi_ctrl::{
    sta::{self, NetworkResult, ScanResult},
    Result,
};

static LOGGER_INITIALIZED: OnceCell<()> = OnceCell::new();

fn initialize_logger() {
    LOGGER_INITIALIZED.get_or_init(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    });
}

async fn with_wifi_connection<F, Fut, T>(operation: F) -> Result<T>
where
    F: FnOnce(sta::RequestClient) -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    initialize_logger();

    let mut setup = sta::WifiSetup::new()?;
    let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
    setup.set_socket_path(proposed_path);

    let broadcast = setup.get_broadcast_receiver();
    let requester = setup.get_request_client();
    let runtime = setup.complete();

    if let Err(e) = runtime.run().await {
        error!("Error: {}", e);
    }

    let result = operation(requester).await?;

    broadcast_listener(broadcast).await?;

    Ok(result)
}

pub async fn get_wifi_list() -> Result<Vec<ScanResult>> {
    with_wifi_connection(wifi_list).await
}

// pub async fn remove_wifi_network(network_id: usize) -> Result<()> {
//     with_wifi_connection(|requester| remove_wifi(requester, network_id)).await
// }

async fn wifi_list(requester: sta::RequestClient) -> Result<Vec<ScanResult>> {
    // wifi_list implementation
    info!("Requesting scan");
    let scan = requester.get_scan().await?;
    info!("Scan complete");
    info!("Shutting down");
    requester.shutdown().await?;
    Ok(scan.to_vec())
}

// async fn remove_wifi(requester: sta::RequestClient, network_id: usize) -> Result<()> {
//     // remove_wifi implementation
// }

async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result {
    while let Ok(broadcast) = broadcast_receiver.recv().await {
        info!("Broadcast: {:?}", broadcast);
    }
    Ok(())
}
