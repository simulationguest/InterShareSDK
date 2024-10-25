use intershare_sdk::nearby::BleServerImplementationDelegate;
pub use intershare_sdk::nearby::NearbyServer as InternalNearbyServer;
use intershare_sdk::protocol::discovery::device_discovery_message::Content;
use intershare_sdk::protocol::discovery::DeviceDiscoveryMessage;
use intershare_sdk::protocol::prost::Message;
use intershare_sdk::{BLE_CHARACTERISTIC_UUID, BLE_SERVICE_UUID};
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use windows::{
    core::{Result as WinResult, GUID},
    Devices::Bluetooth::GenericAttributeProfile::*,
    Devices::Radios::*,
    Foundation::TypedEventHandler,
    Storage::Streams::*,
};

fn setup_gatt_server(nearby_server: Arc<InternalNearbyServer>) -> WinResult<GattServiceProvider> {
    let service_uuid = GUID::from(BLE_SERVICE_UUID);

    let service_provider_result: GattServiceProviderResult = GattServiceProvider::CreateAsync(service_uuid)?.get()?;
    let gatt_service_provider = service_provider_result.ServiceProvider()?;

    let characteristic_uuid = GUID::from(BLE_CHARACTERISTIC_UUID);

    let characteristic_parameters = GattLocalCharacteristicParameters::new()?;
    characteristic_parameters.SetCharacteristicProperties(
        GattCharacteristicProperties::Read
    )?;

    characteristic_parameters.SetReadProtectionLevel(GattProtectionLevel::Plain)?;

    let characteristic_result: GattLocalCharacteristicResult = gatt_service_provider
        .Service()?
        .CreateCharacteristicAsync(characteristic_uuid, &characteristic_parameters)?
        .get()?;

    let gatt_characteristic = characteristic_result.Characteristic()?;
    let nearby_server_clone = Arc::clone(&nearby_server);

    let read_requested_handler = TypedEventHandler::new(
        move |_sender: &Option<GattLocalCharacteristic>, args: &Option<GattReadRequestedEventArgs>| {
            if let Some(args) = args {
                let deferral = args.GetDeferral()?;
                let request: GattReadRequest = args.GetRequestAsync()?.get()?;

                let value = DeviceDiscoveryMessage {
                    content: Some(
                        Content::DeviceConnectionInfo(
                            nearby_server_clone.variables.blocking_read().device_connection_info.clone()
                        )
                    ),
                }.encode_length_delimited_to_vec();

                let writer = DataWriter::new()?;
                writer.WriteBytes(&value)?;
                let buffer = writer.DetachBuffer()?;
                request.RespondWithValue(&buffer)?;
                deferral.Complete()?;
            }
            Ok(())
        },
    );

    gatt_characteristic.ReadRequested(&read_requested_handler)?;

    return Ok(gatt_service_provider);
}

pub struct BleServer {
    gatt_service_provider: GattServiceProvider
}

impl BleServer {
    pub fn new(nearby_server: Arc<InternalNearbyServer>) -> Result<Self, Box<dyn Error>> {
        let gatt_service_provider = setup_gatt_server(nearby_server)?;

        return Ok(Self {
            gatt_service_provider
        });
    }

    pub fn ble_activated() -> bool {
        let radio_access_status = Radio::RequestAccessAsync()
            .expect("Failed to request access")
            .get()
            .expect("Failed to await result for RequestAccessAsync");

        return radio_access_status.0 == 1;
    }
}

impl Debug for BleServer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return Ok(());
    }
}

impl BleServerImplementationDelegate for BleServer {
    fn start_server(&self) {
        let adv_parameters = GattServiceProviderAdvertisingParameters::new().expect("Failed to create new GattServiceProviderAdvertisingParameters");
        adv_parameters.SetIsConnectable(true).expect("Failed to set IsConnectable");
        adv_parameters.SetIsDiscoverable(true).expect("Failed to set IsDiscoverable");
        self.gatt_service_provider.StartAdvertisingWithParameters(&adv_parameters).expect("Failed to start Advertising");
    }

    fn stop_server(&self) {
        self.gatt_service_provider.StopAdvertising().expect("Failed to stop advertising");
    }
}