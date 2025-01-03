// Written by sigroot

use std::io::{Error, ErrorKind};
use std::result::Result;
use std::time::Duration;

const VERSION_STATEMENT: &str = "Sig FW LED Matrix FW V1.0\0\0\0\0\0\0\0";

pub fn get_ports() -> Option<Vec<serialport::SerialPortInfo>> {
    let mut ports = match serialport::available_ports(){
        Ok(x) => x,
        Err(_) => Vec::new(),
    };
    let mut i = 0;
    for p in ports.clone() {
        if !(p.port_name.contains("COM") || p.port_name.contains("ACM")) {
            ports.remove(i);
            i -= 1;
        }
        i += 1;
    }
    if ports.len() > 0 {
        Some(ports)
    } else {
        None
    }
}

pub fn get_matrix_port(baud_rate: u32, time_out: u64) -> Result<Box<dyn serialport::SerialPort>, Error> {
    let ports = get_ports();
    let port_info;
    match ports {
        Some(x) => {
            if x.len() == 1 {
                port_info = x[0].clone();
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, "Too many ports found!"));
            }
        },
        None => return Err(Error::new(ErrorKind::NotFound, "No ACM or Com ports found!")),
    }
    
    print!("{:?} ", port_info.port_name);
    println!("Test 1");
    let mut port = serialport::new(port_info.port_name, baud_rate).timeout(Duration::from_millis(time_out)).open()?;
    println!("Test 2");
    port.write(&[127])?;

    println!("Test 3");
    let mut read_buffer: Vec<u8> = vec![0; 32];
    port.read(&mut read_buffer)?;
    println!("Test 4 {:?}", std::str::from_utf8(&read_buffer));
    match std::str::from_utf8(&read_buffer) {
        Ok(x) => {
            if x == VERSION_STATEMENT {
                return Ok(port);
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, "Incorrect version statement from port!"));
            }
        }
        Err(_) => {
            return Err(Error::new(ErrorKind::InvalidInput, "Incorrect string format from port!"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports_available() {
        let ports = get_ports().expect("No ACM or COM ports found!");
        for p in ports {
            let mut vid = 0;
            let mut pid = 0;
            let mut sn = "None".to_string();
            let mut man = "None".to_string();
            let mut prod = "None".to_string();
            let ptype = match p.port_type {
                serialport::SerialPortType::UsbPort(info) => {
                    vid = info.vid;
                    pid = info.pid;
                    sn = match info.serial_number {
                        Some(x) => x,
                        None => "None".to_string(),
                    };
                    man = match info.manufacturer {
                        Some(x) => x,
                        None => "None".to_string(),
                    };
                    prod = match info.product {
                        Some(x) => x,
                        None => "None".to_string(),
                    };
                    "USB"},
                serialport::SerialPortType::PciPort => "PCI",
                serialport::SerialPortType::BluetoothPort => "BT",
                serialport::SerialPortType::Unknown => "Unknown",
            };
            println!("{}\t{}", p.port_name, ptype);
            if ptype == "USB" {
                println!("\t{} {} {:?} {:?} {:?}", vid, pid, sn, man, prod);
            }
        }
        assert!(true);
    }

    #[test]
    fn port_correct() {
        let port = get_matrix_port(1000000, 10000);
        match port {
            Ok(x) => {let port = x;},
            Err(x) => {
                println!("{:?}", x);
                assert!(false);
            },
        }
        assert!(true); 
    }
}
