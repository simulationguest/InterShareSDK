pub use intershare_sdk::errors::*;
use intershare_sdk::protocol::discovery::device_discovery_message::Content;
pub use intershare_sdk::protocol::discovery::{
    BluetoothLeConnectionInfo, DeviceDiscoveryMessage, TcpConnectionInfo,
};
use intershare_sdk::protocol::prost::Message;
pub use intershare_sdk::stream::NativeStreamDelegate;
pub use intershare_sdk::{
    nearby::{
        BleServerImplementationDelegate, L2CapDelegate, NearbyConnectionDelegate, NearbyServer,
        SendProgressDelegate,
    },
    Device,
};

#[derive(uniffi::Object)]
pub struct InternalNearbyServer {
    handler: NearbyServer,
}

#[uniffi::export(async_runtime = "tokio")]
impl InternalNearbyServer {
    #[uniffi::constructor]
    pub fn new(
        my_device: Device,
        file_storage: String,
        delegate: Option<Box<dyn NearbyConnectionDelegate>>,
    ) -> Self {
        let server = NearbyServer::new(my_device, file_storage, delegate);

        Self { handler: server }
    }

    pub fn get_current_ip(&self) -> Option<String> {
        return self.handler.get_current_ip();
    }

    pub fn add_l2_cap_client(&self, delegate: Box<dyn L2CapDelegate>) {
        self.handler.add_l2_cap_client(delegate);
    }

    pub fn add_ble_implementation(
        &self,
        ble_implementation: Box<dyn BleServerImplementationDelegate>,
    ) {
        self.handler
            .add_bluetooth_implementation(ble_implementation);
    }

    pub fn change_device(&self, new_device: Device) {
        self.handler.change_device(new_device);
    }

    pub fn set_ble_connection_details(&self, ble_details: BluetoothLeConnectionInfo) {
        self.handler.set_bluetooth_le_details(ble_details)
    }

    pub fn set_tcp_details(&self, tcp_details: TcpConnectionInfo) {
        self.handler.set_tcp_details(tcp_details)
    }

    pub async fn get_advertisement_data(&self) -> Vec<u8> {
        if self.handler.variables.read().await.advertise {
            return DeviceDiscoveryMessage {
                content: Some(Content::DeviceConnectionInfo(
                    self.handler
                        .variables
                        .read()
                        .await
                        .device_connection_info
                        .clone(),
                )),
            }
            .encode_length_delimited_to_vec();

            // self.mut_variables.write().await.discovery_message = message;
        } else {
            // return DeviceDiscoveryMessage {
            //     content: Some(
            //         Content::OfflineDeviceId(
            //             self.handler.variables
            //                 .read()
            //                 .await
            //                 .device_connection_info.device?.id.clone()
            //         )
            //     ),
            // }.encode_length_delimited_to_vec();
        }

        return vec![];
    }

    pub async fn start(&self) {
        self.handler.start().await;
    }

    pub async fn restart_server(&self) {
        self.handler.restart_server().await;
    }

    pub fn handle_incoming_ble_connection(
        &self,
        connection_id: String,
        native_stream: Box<dyn NativeStreamDelegate>,
    ) {
        return self
            .handler
            .handle_incoming_ble_connection(connection_id, native_stream);
    }

    pub async fn send_files(
        &self,
        receiver: Device,
        file_paths: Vec<String>,
        progress_delegate: Option<Box<dyn SendProgressDelegate>>,
    ) -> Result<(), ConnectErrors> {
        return self
            .handler
            .send_files(receiver, file_paths, progress_delegate)
            .await;
    }

    pub fn stop(&self) {
        self.handler.stop();
    }

    pub fn handle_incoming_connection(&self, native_stream_handle: Box<dyn NativeStreamDelegate>) {
        self.handler
            .handle_incoming_connection(native_stream_handle);
    }
}
