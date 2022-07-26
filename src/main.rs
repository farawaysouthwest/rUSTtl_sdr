use rusb::{Context, Device, DeviceHandle, Result, UsbContext};
use std::time::Duration;

const VID: u16 = 0x0bda;
const PID: u16 = 0x2838;

fn main() -> Result<()> {
    let mut context = Context::new()?;
    let (mut device, mut handle) =
    open_device(&mut context, VID, PID).expect("Failed to read USB device.");

    print_device_info(&mut handle)?;

    Ok(())
}


fn open_device<T:UsbContext>(
    context: &mut T,
    vid:u16,
    pid: u16
) -> Option<(Device<T>, DeviceHandle<T>)> {

    // get all devices
    let devices = match context.devices(){
        Ok(d) => d,
        Err(_) => return None
    };

    // loop through devices
    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        // match up id's
        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(_) => continue,
                
            }
        }
    }

    // if not found, exit.
    None
}

fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", handle.active_configuration()?);

    if !languages.is_empty() {
        let language = languages[0];
        println!(
            "Language: {:?}", language);

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
            );
    
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );

        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
    
    }

    Ok(())
}