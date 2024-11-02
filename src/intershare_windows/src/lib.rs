mod ble;
pub mod nearby_server;
pub mod discovery;

pub use intershare_sdk::{ClipboardTransferIntent};
pub use intershare_sdk::connection_request::{ConnectionRequest, ReceiveProgressState, ReceiveProgressDelegate};
pub use intershare_sdk::Device;
pub use intershare_sdk::DiscoveryDelegate;
pub use intershare_sdk::encryption::EncryptedStream;
pub use intershare_sdk::nearby::{ConnectionMedium, SendProgressState, SendProgressDelegate, BleServerImplementationDelegate, L2CapDelegate, NearbyConnectionDelegate};
pub use intershare_sdk::nearby::ConnectionIntentType;
pub use intershare_sdk::protocol::communication::FileTransferIntent;
pub use intershare_sdk::stream::NativeStreamDelegate;
pub use intershare_sdk::transmission::TransmissionSetupError;
pub use intershare_sdk::errors::*;
pub use intershare_sdk::*;
pub use crate::discovery::{Discovery};
pub use crate::nearby_server::{NearbyServer};
pub use intershare_sdk::protocol::discovery::{BluetoothLeConnectionInfo, TcpConnectionInfo};

uniffi::include_scaffolding!("intershare_sdk");