use std::io::prelude::*;
use serialport::{SerialPort, SerialPortInfo};

use std::sync::mpsc::Sender;
use crate::serial::{self, USBDevInfo, ConnectionProps, open_serialport};
use std::{thread, time};

pub const DEVINFO: USBDevInfo = USBDevInfo {
  vid: Some(1155),
  pid: Some(22336),
  manufacturer: Some("STMicroelectronics"),
  product: Some("STM32 Virtual ComPort"),
};

pub const CONNPROPS: ConnectionProps = ConnectionProps {
  baud_rate: 115200,
  data_bits: Some(8),
  flow_control: Some("None"),
  parity: Some("None"),
  stop_bits: Some(2),
  timeout: Some(100),
};

pub const TX_DELIMITER: &str = "\r\n";
pub const RX_DELIMITER: &str = "\r\n";

pub struct CANAdapter {
  port: Box<dyn SerialPort>,
  pub sender_notify: Option<Sender<usize>>,
  pub sender_data: Option<Sender<usize>>,
}

impl CANAdapter {

  pub fn open() -> Result<Self, Box<dyn std::error::Error>>{
    let usb_port: SerialPortInfo = match serial::search_usb_devices(DEVINFO) {
        Some(ports) => ports[0].clone(),
        None => panic!("NO PORT FOUND")
    };

    let port: Box<dyn SerialPort> = match open_serialport(usb_port.port_name, CONNPROPS) {
        Ok(port) => port,
        Err(_) => panic!("CONNECTION FAILED")
    };

    Ok(Self{ port, sender_notify: None, sender_data: None })
  }

  pub fn close(self) -> Result<(), Box<dyn std::error::Error>>{
    self.port.clear(serialport::ClearBuffer::All)?;
    std::mem::drop(self);
    Ok(())
  }

  pub fn write(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    match self.port.write(msg.as_bytes()) {
      Ok(_) => {
        std::io::stdout().flush().unwrap();
        Ok(())
      },
      Err(_) => {return Err("FAILED TO WRITE PORT".into())}
    }
  }

  pub fn drive_enable(&mut self, channel: u8) -> Result<(), Box<dyn std::error::Error>>{
    let msg: String = format!("_sendresponse {channel} 129 cmdval double 0{TX_DELIMITER}");
    self.write(&msg)?;

    thread::sleep(time::Duration::from_millis(100));
    
    let msg: String = format!("_sendresponse {channel} 129 drive double 1{TX_DELIMITER}");
    self.write(&msg)
  }

  pub fn drive_disable(&mut self, channel: u8) -> Result<(), Box<dyn std::error::Error>>{
    let msg: String = format!("_sendresponse {channel} 129 drive double 0{TX_DELIMITER}");
    self.write(&msg)?;

    thread::sleep(time::Duration::from_millis(300));

    let msg: String = format!("_sendresponse {channel} 129 cmdval double 0{TX_DELIMITER}");
    self.write(&msg)
  }

  pub fn send_reference(&mut self, channel: u8, value: f64) -> Result<(), Box<dyn std::error::Error>>{
    let msg: String = format!("_sendresponse {channel} 129 cmdval double {value}{TX_DELIMITER}");
    self.write(&msg)
  }

}
