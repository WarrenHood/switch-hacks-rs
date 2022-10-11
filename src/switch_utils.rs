use rusb::{self, Context, Device, DeviceDescriptor, DeviceHandle, Direction, UsbContext};
use std::{error::Error, time::Duration};

pub struct Switch {
    read_endpoint: Endpoint,
    write_endpoint: Endpoint,
    switch_handle: DeviceHandle<Context>,
}

impl Switch {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let ctx = rusb::Context::new()?;

        let switch_device = ctx
            .open_device_with_vid_pid(0x057E, 0x3000)
            .expect("Couldn't find switch device")
            .device();
        let switch_descriptor = switch_device
            .device_descriptor()
            .expect("Could not get switch device descriptor");
        let switch_handle = switch_device.open().unwrap();

        let read_endpoint = get_switch_enpoint(&switch_descriptor, &switch_device, Direction::In)
            .expect("Could not get read endpoint");
        let write_endpoint = get_switch_enpoint(&switch_descriptor, &switch_device, Direction::Out)
            .expect("Could not get write endoint");

        Ok(Switch {
            read_endpoint,
            write_endpoint,
            switch_handle,
        })
    }

    fn send_command(&mut self, command: String) -> Result<(), Box<dyn Error>> {
        send_command(&mut self.switch_handle , &self.write_endpoint, command)
    }

    pub fn write_dword(&mut self, address: u32, value: u32) -> Result<(), Box<dyn Error>>{
        let converted_value = u32::from_le_bytes((value as u32).to_be_bytes()); // For some reason this seems to need big endian representation
        println!("poke 0x{:08x} 0x{:08x}", address, converted_value);
        self.send_command(format!("poke 0x{:08x} 0x{:08x}", address, converted_value))
    }

    pub fn read_bytes(&mut self, address: u32, buf: &mut [u8], length: u32) -> Result<(), Box<dyn Error>> {
        println!("peek 0x{:08x} 0x{:08x}", address, length);
        self.send_command(format!("peek 0x{:08x} 0x{:08x}", address, length))?;
        receive_bytes(&mut self.switch_handle, &self.read_endpoint, buf, length)?;
        Ok(())
    }
}

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn get_switch_enpoint(
    switch_descriptor: &DeviceDescriptor,
    switch_device: &Device<Context>,
    direction: Direction,
) -> Option<Endpoint> {
    for i in 0..switch_descriptor.num_configurations() {
        let config_desc = match switch_device.config_descriptor(i) {
            Ok(d) => d,
            Err(_) => continue,
        };

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == direction {
                        return Some(Endpoint {
                            config: config_desc.number(),
                            iface: interface.number(),
                            setting: interface_desc.setting_number(),
                            address: endpoint_desc.address(),
                        });
                    }
                }
            }
        }
    }
    None
}

fn configure_endpoint(
    switch_handle: &mut DeviceHandle<Context>,
    endpoint: &Endpoint,
) -> Result<(), Box<dyn Error>> {
    switch_handle.set_active_configuration(endpoint.config)?;
    switch_handle.claim_interface(endpoint.iface)?;
    switch_handle.set_alternate_setting(endpoint.iface, endpoint.setting)?;
    Ok(())
}

fn send_command(
    switch_handle: &mut DeviceHandle<Context>,
    write_endpoint: &Endpoint,
    command: String,
) -> Result<(), Box<dyn Error>> {
    configure_endpoint(switch_handle, &write_endpoint)?;
    let bytes_to_send = ((command.len() + 2) as u32).to_le_bytes();
    switch_handle.write_bulk(
        write_endpoint.address,
        &bytes_to_send,
        Duration::from_secs(5),
    )?;
    switch_handle.write_bulk(
        write_endpoint.address,
        (command + "\r\n").as_bytes(),
        Duration::from_secs(5),
    )?;
    Ok(())
}

fn receive_bytes(
    switch_handle: &mut DeviceHandle<Context>,
    read_endpoint: &Endpoint,
    buf: &mut [u8],
    length: u32
) -> Result<(), Box<dyn Error>> {
    configure_endpoint(switch_handle, &read_endpoint)?;

    let mut size_recv: [u8; 4] = [0; 4];
    switch_handle.read_bulk(read_endpoint.address, &mut size_recv, Duration::from_secs(30))?;

    if u32::from_le_bytes(size_recv) != length {
        println!("Warning: Receiving {} bytes from switch... Expected: {}... aborting", u32::from_le_bytes(size_recv), length);
    }
    // println!("Receiving {} bytes from switch... Expected: {}", u32::from_le_bytes(size_recv), length);

    switch_handle.read_bulk(
        read_endpoint.address,
        buf,
        Duration::from_secs(30),
    )?;


    Ok(())
}