use crate::ble::ble_server::BleServer;
use intershare_sdk::nearby::NearbyConnectionDelegate;
pub use intershare_sdk::nearby::NearbyServer as InternalNearbyServer;
use intershare_sdk::Device;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[uniffi::export()]
pub struct NearbyServer {
    runtime: Runtime,
    internal_nearby_server: Arc<InternalNearbyServer>
}

impl NearbyServer {
    pub fn new(my_device: Device, delegate: Option<Box<dyn NearbyConnectionDelegate>>) -> NearbyServer {
        let nearby = Arc::new(InternalNearbyServer::new(my_device, "TODO".to_string(), delegate));
        let ble_server = BleServer::new(Arc::clone(&nearby))
            .expect("Failed to initialize BLE Server");

        nearby.add_bluetooth_implementation(Box::new(ble_server));

        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();

        Self {
            runtime,
            internal_nearby_server: nearby
        }
    }

    pub fn start(&self) {
        self.runtime.block_on(self.internal_nearby_server.start());
    }
}