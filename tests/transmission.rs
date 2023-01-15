use std::thread;
use data_rct::discovery::DeviceInfo;
use data_rct::transmission::Transmission;

#[test]
pub fn transmission_send() {
    let foreign_device: DeviceInfo = DeviceInfo {
        id: "B53CCB62-7DAB-4403-9FEB-F336834DB41F".to_string(),
        name: "Rust Device 1".to_string(),
        port: 0,
        device_type: "computer".to_string(),
        ip_address: "127.0.0.1".to_string()
    };

    let my_device = DeviceInfo {
        id: "A689B035-B4AC-461F-8408-5CF1A5570592".to_string(),
        name: "Rust Device 2".to_string(),
        port: 0,
        device_type: "computer".to_string(),
        ip_address: "127.0.0.1".to_string()
    };

    let receive_transmission = Transmission::new(foreign_device).unwrap();
    let foreign_device = receive_transmission.device_info.clone();

    thread::spawn(move || {
        loop {
            receive_transmission.accept().unwrap().unwrap();
        }
    });

    let transmission = Transmission::new(my_device).unwrap();
    let _encrypted_stream = transmission.open(&foreign_device).unwrap();
}


#[test]
pub fn transmission_receive() {
    let my_device: DeviceInfo = DeviceInfo {
        id: "B53CCB62-7DAB-4403-9FEB-F336834DB41F".to_string(),
        name: "Rust Device 1".to_string(),
        port: 0,
        device_type: "computer".to_string(),
        ip_address: "127.0.0.1".to_string()
    };

    let foreign_device = DeviceInfo {
        id: "A689B035-B4AC-461F-8408-5CF1A5570592".to_string(),
        name: "Rust Device 2".to_string(),
        port: 0,
        device_type: "computer".to_string(),
        ip_address: "127.0.0.1".to_string()
    };

    let receive_transmission = Transmission::new(my_device.clone()).unwrap();

    let my_device_clone = receive_transmission.device_info.clone();
    let foreign_device_clone = foreign_device.clone();

    thread::spawn(move || {
        let transmission = Transmission::new(foreign_device_clone).unwrap();
        let _encrypted_stream = transmission.open(&my_device_clone).unwrap();
    });

    let transmission_request = receive_transmission.accept().unwrap().unwrap();
    assert_eq!(transmission_request.sender_id, foreign_device.id);
    assert_eq!(transmission_request.sender_name, foreign_device.name);
    assert!(transmission_request.uuid.len() > 0);
}