use crate::ble::ble_client::BleClient;
pub use intershare_sdk::discovery::Discovery as InternalDiscovery;
use intershare_sdk::{Device, DiscoveryDelegate};
use std::sync::{Arc, Mutex};

pub struct Discovery {
    internal_discovery: Arc<Mutex<InternalDiscovery>>,
}

impl Discovery {
    pub fn new(delegate: Option<Box<dyn DiscoveryDelegate>>) -> Self {
        let internal_discovery = Arc::new(Mutex::new(
            InternalDiscovery::new(delegate)
                .expect("Failed to initialize internal discovery"),
        ));

        let ble_implementation = BleClient::new(internal_discovery.clone());

        {
            let mut internal_discovery_mut = internal_discovery.lock().unwrap();
            internal_discovery_mut.add_ble_implementation(Box::new(ble_implementation));
        }

        Self {
            internal_discovery,
        }
    }

    pub fn get_devices(&self) -> Vec<Device> {
        let internal_discovery = self.internal_discovery.lock().unwrap();
        internal_discovery.get_devices()
    }

    pub fn start(&self) {
        let internal_discovery = self.internal_discovery.lock().unwrap();
        internal_discovery.start()
    }

    pub fn stop(&self) {
        let internal_discovery = self.internal_discovery.lock().unwrap();
        internal_discovery.stop()
    }
}