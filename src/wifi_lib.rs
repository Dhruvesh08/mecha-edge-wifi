// wifi_lib/src/lib.rs

use env_logger::Env;
use log::{error, info};
use wifi_ctrl::{sta, Result};

pub use sta::{Broadcast, KeyMgmt, NetworkResult, ScanResult, SelectResult};

// Define a struct that represents a wifi manager
pub struct WifiManager {
    requester: sta::RequestClient,
    broadcast_receiver: sta::BroadcastReceiver,
}

impl WifiManager {
    // Implement a constructor that takes a network interface name as an argument
    pub fn new(interface_name: &str) -> Result<Self> {
        let mut setup = sta::WifiSetup::new()?;
        let proposed_path = format!("/var/run/wpa_supplicant/{}", interface_name);
        info!("Setting socket path to {}", proposed_path);
        setup.set_socket_path(proposed_path);
        let broadcast_receiver = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();
        // Spawn a task that runs the runtime in the background
        tokio::spawn(async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {e}");
            }
        });
        Ok(Self {
            requester,
            broadcast_receiver,
        })
    }

    // Implement a method that scans for wifi networks
    pub async fn scan_wifi(&self) -> Result<Vec<ScanResult>> {
        info!("Requesting scan");
        let scan = self.requester.get_scan().await?;
        info!("Scan complete");
        Ok(scan.to_vec())
    }

    // Implement a method that connects to a wifi network by id
    pub async fn connect_wifi(&self, network_id: usize) -> Result<SelectResult> {
        info!("Selecting network {}", network_id);
        let result = self.requester.select_network(network_id).await?;
        info!("Select result: {}", result);
        Ok(result)
    }

    // Implement a method that forgets a wifi network by id
    pub async fn forget_wifi(&self, network_id: usize) -> Result {
        info!("Removing network {}", network_id);
        self.requester.remove_network(network_id).await?;
        info!("Network removed");
        Ok(())
    }

    // Implement a method that listens for broadcast events
    pub async fn listen_broadcast(&mut self) -> Result<Broadcast> {
        let broadcast = self.broadcast_receiver.recv().await.unwrap();
        info!("Broadcast: {:?}", broadcast);
        Ok(broadcast)
    }
}


