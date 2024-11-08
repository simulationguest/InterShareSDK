use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use windows::{
    core::{Result, GUID},
    Devices::Bluetooth::{
        Advertisement::{
            BluetoothLEAdvertisementFilter, BluetoothLEAdvertisementReceivedEventArgs,
            BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStatus,
            BluetoothLEScanningMode,
        },
        BluetoothLEDevice,
        GenericAttributeProfile::{
            GattCommunicationStatus,
        },
    },
    Foundation::TypedEventHandler,
    Storage::Streams::DataReader,
};
use tokio::runtime::Handle;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
use intershare_sdk::{BLE_CHARACTERISTIC_UUID, BLE_SERVICE_UUID};
use intershare_sdk::discovery::BleDiscoveryImplementationDelegate;
use crate::discovery::InternalDiscovery;


pub struct BleClient {
    internal_discovery: Arc<Mutex<InternalDiscovery>>,
    scanning: Arc<AtomicBool>,
}

impl BleDiscoveryImplementationDelegate for BleClient {
    fn start_scanning(&self) {
        let internal_discovery = self.internal_discovery.clone();
        let scanning = self.scanning.clone();

        scanning.store(true, Ordering::Relaxed);

        // Start a new thread to run the async code
        std::thread::spawn(move || {
            // Initialize the COM library for use by the calling thread
            // windows::core::initialize_mta().unwrap();
            unsafe {
                CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
            }

            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();

            let handle = rt.handle().clone();

            rt.block_on(async {
                if let Err(e) = BleClient::scan_and_connect(internal_discovery, scanning, handle).await {
                    eprintln!("Error during scanning: {:?}", e);
                }
            });
        });
    }

    fn stop_scanning(&self) {
        self.scanning.store(false, Ordering::Relaxed);
    }

}

impl BleClient {
    pub fn new(internal_discovery: Arc<Mutex<InternalDiscovery>>) -> Self {
        Self {
            internal_discovery,
            scanning: Arc::new(AtomicBool::new(false)),
        }
    }

    async fn scan_and_connect(
        internal_discovery: Arc<Mutex<InternalDiscovery>>,
        scanning: Arc<AtomicBool>,
        runtime_handle: Handle
    ) -> Result<()> {
        let watcher = BluetoothLEAdvertisementWatcher::new()?;

        // Set up the filter for the service UUID
        let filter = BluetoothLEAdvertisementFilter::new()?;
        filter.Advertisement()?.ServiceUuids()?.Append(GUID::from(BLE_SERVICE_UUID))?;
        watcher.SetAdvertisementFilter(&filter)?;

        // Set scanning mode to Active
        watcher.SetScanningMode(BluetoothLEScanningMode::Active)?;

        let discovered_devices = Arc::new(Mutex::new(Vec::new()));
        let discovered_devices_clone = discovered_devices.clone();
        let internal_discovery_clone = internal_discovery.clone();

        let handler = TypedEventHandler::new(
            move |_: &Option<BluetoothLEAdvertisementWatcher>,
                  args: &Option<BluetoothLEAdvertisementReceivedEventArgs>| {
                let args = args.as_ref().unwrap();
                let ble_address = args.BluetoothAddress()?;
                let discovered_devices = discovered_devices_clone.clone();
                let internal_discovery = internal_discovery_clone.clone();

                // Spawn a task to handle the connection and data retrieval
                runtime_handle.spawn(async move {
                    // Check if the device has already been discovered
                    {
                        let devices = discovered_devices.lock().unwrap();
                        if devices.contains(&ble_address) {
                            return;
                        }
                    }

                    // Add the device to the discovered list
                    {
                        let mut devices = discovered_devices.lock().unwrap();
                        devices.push(ble_address);
                    }

                    if let Err(e) =
                        BleClient::connect_and_read_characteristic(ble_address, internal_discovery)
                            .await
                    {
                        eprintln!("Error connecting to device: {:?}", e);
                    }
                });

                Ok(())
            },
        );

        watcher.Received(&handler)?;
        watcher.Start()?;

        // Wait until scanning is stopped
        while scanning.load(Ordering::Relaxed) {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }

        // Stop the watcher
        if watcher.Status()? == BluetoothLEAdvertisementWatcherStatus::Started {
            watcher.Stop()?;
        }

        Ok(())
    }

    async fn connect_and_read_characteristic(
        ble_address: u64,
        internal_discovery: Arc<Mutex<InternalDiscovery>>,
    ) -> Result<()> {
        // Connect to the device
        let device = BluetoothLEDevice::FromBluetoothAddressAsync(ble_address)?.get()?;

        // Get the GATT services
        let services_result = device.GetGattServicesForUuidAsync(GUID::from(BLE_SERVICE_UUID))?.get()?;
        if services_result.Status()? != GattCommunicationStatus::Success {
            eprintln!("Failed to get GATT services");
            return Ok(());
        }
        let services = services_result.Services()?;

        if services.Size()? == 0 {
            eprintln!("No services found");
            return Ok(());
        }

        let service = services.GetAt(0)?;

        // Get the characteristics
        let characteristics_result = service.GetCharacteristicsForUuidAsync(GUID::from(BLE_CHARACTERISTIC_UUID))?.get()?;
        if characteristics_result.Status()? != GattCommunicationStatus::Success {
            eprintln!("Failed to get characteristics");
            return Ok(());
        }
        let characteristics = characteristics_result.Characteristics()?;

        if characteristics.Size()? == 0 {
            eprintln!("No characteristics found");
            return Ok(());
        }

        let characteristic = characteristics.GetAt(0)?;

        // Read the characteristic value
        let read_result = characteristic.ReadValueAsync()?.get()?;
        if read_result.Status()? != GattCommunicationStatus::Success {
            eprintln!("Failed to read characteristic");
            return Ok(());
        }
        let value = read_result.Value()?;
        let reader = DataReader::FromBuffer(&value)?;
        let length = reader.UnconsumedBufferLength()? as usize;
        let mut buffer = vec![0u8; length];
        reader.ReadBytes(&mut buffer)?;

        // Process the data
        {
            let mut discovery = internal_discovery.lock().unwrap();
            discovery.parse_discovery_message(buffer, Some(device.DeviceId()?.to_string()));
        }

        Ok(())
    }
}

impl Debug for BleClient {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
