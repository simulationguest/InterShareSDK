use objc::{msg_send, sel, sel_impl};
use objc::runtime::{BOOL, NO, Object, Sel, YES};
use objc_foundation::{INSArray, INSData, INSString, NSArray, NSData, NSObject, NSString};
use protocol::discovery::{DeviceDiscoveryMessage};
use crate::apple::constants::POWERED_ON_IVAR;
use crate::apple::converter::IntoBool;
use crate::apple::ffi::{CBATTError, CBManagerState};
use protocol::prost::{Message};
use crate::apple::DISCOVERY_MESSAGE;

pub extern "C" fn peripheral_manager_did_update_state(
    delegate: &mut Object,
    _cmd: Sel,
    peripheral: *mut Object,
) {
    println!("peripheral_manager_did_update_state");

    unsafe {
        let state: CBManagerState = msg_send![peripheral, state];
        match state {
            CBManagerState::CBManagerStateUnknown => {
                println!("CBManagerStateUnknown");
            }
            CBManagerState::CBManagerStateResetting => {
                println!("CBManagerStateResetting");
            }
            CBManagerState::CBManagerStateUnsupported => {
                println!("CBManagerStateUnsupported");
            }
            CBManagerState::CBManagerStateUnauthorized => {
                println!("CBManagerStateUnauthorized");
            }
            CBManagerState::CBManagerStatePoweredOff => {
                println!("CBManagerStatePoweredOff");
                delegate.set_ivar::<BOOL>(POWERED_ON_IVAR, NO);
            }
            CBManagerState::CBManagerStatePoweredOn => {
                println!("CBManagerStatePoweredOn");
                delegate.set_ivar(POWERED_ON_IVAR, YES);
            }
        };
    }
}

pub extern "C" fn peripheral_manager_did_start_advertising_error(
    _delegate: &mut Object,
    _cmd: Sel,
    _peripheral: *mut Object,
    error: *mut Object,
) {
    if error.is_null() {
        return;
    }

    println!("peripheral_manager_did_start_advertising_error");
    println!("Type: {:?}", error);
    if error.into_bool() {
        let localized_description: *mut Object = unsafe { msg_send![error, localizedDescription] };
        let string = localized_description as *mut NSString;
        println!("{:?}", unsafe { (*string).as_str() });
    }
}


pub extern "C" fn peripheral_manager_did_add_service_error(
    _delegate: &mut Object,
    _cmd: Sel,
    _peripheral: *mut Object,
    _service: *mut Object,
    error: *mut Object,
) {
    println!("peripheral_manager_did_add_service_error");
    if error.into_bool() {
        let localized_description: *mut Object = unsafe { msg_send![error, localizedDescription] };
        let string = localized_description as *mut NSString;
        println!("{:?}", unsafe { (*string).as_str() });
    }
}

pub extern "C" fn peripheral_manager_did_receive_read_request(
    _delegate: &mut Object,
    _cmd: Sel,
    peripheral: *mut Object,
    request: *mut Object,
) {
    unsafe {
        let data = NSData::from_vec(DISCOVERY_MESSAGE.clone()).share();

        let _: Result<(), ()> = msg_send![request, setValue:data.clone()];
        let _: Result<(), ()> = msg_send![peripheral, respondToRequest:request withResult:CBATTError::CBATTErrorSuccess];
    }
}

pub extern "C" fn peripheral_manager_did_receive_write_requests(
    _delegate: &mut Object,
    _cmd: Sel,
    peripheral: *mut Object,
    requests: *mut Object,
) {
    unsafe {
        for request in (*(requests as *mut NSArray<NSObject>)).to_vec() {
            let value : *mut NSData = msg_send![request, value];
            let value = (*value).bytes();

            let discovery_message = DeviceDiscoveryMessage::decode_length_delimited(value);
            let Ok(message) = discovery_message else {
                break;
            };
            //
            // match message.device_data {
            //     Some(DeviceData::Device(device_data)) => {
            //         add_device(device_data);
            //     },
            //     Some(DeviceData::DeviceId(device_id)) => {
            //         remove_device(device_id);
            //     },
            //     _ => {}
            // }

            let _: Result<(), ()> = msg_send![peripheral, respondToRequest:request
                                        withResult:CBATTError::CBATTErrorSuccess];
        }
    }
}