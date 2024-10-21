use std::fmt::Debug;
pub use prost;

pub mod discovery {
    include!(concat!(env!("OUT_DIR"), "/inter_share_sdk.discovery.rs"));
}

pub mod communication {
    include!(concat!(env!("OUT_DIR"), "/inter_share_sdk.communication.rs"));
}

pub trait DiscoveryDelegate: Send + Sync + Debug {
    fn device_added(&self, value: discovery::Device);
    fn device_removed(&self, device_id: String);
}
