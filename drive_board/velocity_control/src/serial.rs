use serde::{Deserialize, Serialize};
use serialport::{SerialPort, SerialPortInfo, SerialPortType::UsbPort};

pub mod canadapter;

#[derive(Debug)]
pub struct USBDevInfo<'a> {
  pub vid: Option<u16>,
  pub pid: Option<u16>,
  pub manufacturer: Option<&'a str>,
  pub product: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct USBPort {
  port_name: String,
  vid: u16,
  pid: u16,
  serial_number: String,
  manufacturer: String,
  product: String,
}

impl USBPort {
  pub fn new(p: &SerialPortInfo) -> Option<USBPort> {
    if let SerialPortInfo{port_type:UsbPort(upi), ..} = p {
      Some(USBPort {
        port_name: p.port_name.clone(),
        vid: upi.vid,
        pid: upi.pid,
        serial_number: match &upi.serial_number {Some(i) => i.clone(), None => String::from("")},
        manufacturer: match &upi.manufacturer {Some(i) => i.clone(), None => String::from("")},
        product: match &upi.product {Some(i) => i.clone(), None => String::from("")},
      })
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct ConnectionProps<'a> {
  pub baud_rate: u32,
  pub data_bits: Option<usize>,
  pub flow_control: Option<&'a str>,
  pub parity: Option<&'a str>,
  pub stop_bits: Option<usize>,
  pub timeout: Option<u64>,
}

pub fn list_ports() -> Option<Vec<SerialPortInfo>> {
  let ports: Vec<SerialPortInfo> = match serialport::available_ports() {
      Ok(p) => p,
      Err(_) => return None
  };
  Some(ports)
}

pub fn list_usb_devices() -> Option<Vec<USBPort>> {
  let ports: Option<Vec<SerialPortInfo>> = list_ports();
  match ports {
    Some(i) => {
      let usbs: Vec<USBPort> = i.iter().map(|x| USBPort::new(x))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
      match usbs.len() {
        0 => None,
        _ => Some(usbs)
      }
    },
    None => None
  }
}

pub fn search_usb_devices(dev: USBDevInfo) -> Option<Vec<SerialPortInfo>> {
  let ports = list_ports();
  match ports {
    Some(i) => {
      let port_specified: Vec<SerialPortInfo> = i
        .into_iter()
        .filter(|i|
          if let SerialPortInfo{port_type:UsbPort(upi), ..} = i {
            let mut is_match: bool = true;
            if let Some(x) = dev.vid {is_match &= upi.vid == x};
            if let Some(x) = dev.pid {is_match &= upi.pid == x};
            if let Some(x) = dev.manufacturer { if let Some(i) = &upi.manufacturer {is_match &= i == x} else {is_match = false} };
            if let Some(x) = dev.product { if let Some(i) = &upi.product {is_match &= i == x} else {is_match = false} };
            is_match
          } else {
            false
          })
        .collect();
      match port_specified.len() {
        0 => None,
        _ => Some(port_specified)
      }
    },
    None => None
  }
}

pub fn open_serialport(path: String, props: ConnectionProps) -> Result<Box<dyn SerialPort>, serialport::Error> {
  let data_bits = match props.data_bits {
    Some(value) => match value {
        5 => serialport::DataBits::Five,
        6 => serialport::DataBits::Six,
        7 => serialport::DataBits::Seven,
        8 => serialport::DataBits::Eight,
        _ => serialport::DataBits::Eight,
    },
    None => serialport::DataBits::Eight,
  };

  let flow_control = match props.flow_control {
    Some(value) => match value {
      "Software" => serialport::FlowControl::Software,
      "Hardware" => serialport::FlowControl::Hardware,
      "None" => serialport::FlowControl::None,
      _ => serialport::FlowControl::None,
    },
    None => serialport::FlowControl::None,
  };

  let parity = match props.parity {
    Some(value) => match value {
      "Odd" => serialport::Parity::Odd,
      "Even" => serialport::Parity::Even,
      "None" => serialport::Parity::None,
      _ => serialport::Parity::None,
    },
    None => serialport::Parity::None,
  };

  let stop_bits = match props.stop_bits {
    Some(value) => match value {
      1 => serialport::StopBits::One,
      2 => serialport::StopBits::Two,
      _ => serialport::StopBits::Two,
    },
    None => serialport::StopBits::Two,
  };

  serialport::new(path.clone(), props.baud_rate)
    .data_bits(data_bits)
    .flow_control(flow_control)
    .parity(parity)
    .stop_bits(stop_bits)
    .timeout(std::time::Duration::from_millis(props.timeout.unwrap_or(10)))
    .open()
}