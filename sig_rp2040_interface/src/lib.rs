// Written by sigroot

use std::io::{Error, ErrorKind};
use std::result::Result;
use std::time::Duration;

const VERSION_STATEMENT: &str = "Sig FW LED Matrix FW V1.0\0\0\0\0\0\0\0";

pub struct LedMatrixInterface <'a> {
    pwm_matrix: &'a mut[[u8;9];34],
    scale_matrix: &'a mut[[u8;9];34],
    led_matrix_port: dyn serialport::SerialPort,
}

impl LedMatrixInterface <'_> {
    pub fn set_pwm (&mut self, input_matrix: &[[u8;9];34]) {
         for i in 0..self.pwm_matrix.len() {
            for j in 0..self.pwm_matrix[i].len() {
                self.pwm_matrix[i][j] = input_matrix[i][j];
            }
         }
    }

    pub fn set_scale (&mut self, input_matrix: &[[u8;9];34]) {
         for i in 0..self.scale_matrix.len() {
            for j in 0..self.scale_matrix[i].len() {
                self.scale_matrix[i][j] = input_matrix[i][j];
            }
         }
    }

    pub fn write_pwm (&mut self) {
        let mut write_buffer = Vec::<u8>::with_capacity(307); 
        write_buffer.extend_from_slice(&[b'w']);
        for i in 0..self.pwm_matrix.len() {
            for j in 0..self.pwm_matrix[i].len() {
                write_buffer.extend_from_slice(&[self.pwm_matrix[i][j]]);
            }
        }
        let _ = self.led_matrix_port.write_all(write_buffer.as_slice());
    }

    pub fn write_scale (&mut self) {
        let mut write_buffer = Vec::<u8>::with_capacity(307); 
        write_buffer.extend_from_slice(&[b's']);
        for i in 0..self.scale_matrix.len() {
            for j in 0..self.scale_matrix[i].len() {
                write_buffer.extend_from_slice(&[self.pwm_matrix[i][j]]);
            }
        }
        let _ = self.led_matrix_port.write_all(write_buffer.as_slice());
    }

    pub fn write (&mut self) {
        self.write_scale();
        self.write_pwm();
    }
}

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
    
    let mut port = serialport::new(port_info.port_name, baud_rate).timeout(Duration::from_millis(time_out)).open()?;
    port.write(&[127])?;

    let mut read_buffer: Vec<u8> = vec![0; 32];
    port.read(&mut read_buffer)?;
    match std::str::from_utf8(&read_buffer) {
        Ok(x) => {
            if x == VERSION_STATEMENT {
                return Ok(port);
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, format!("Incorrect version statement from port!: {:?}", read_buffer.as_slice())));
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
            Ok(x) => {println!("{:?}",x);},
            Err(x) => {
                println!("{:?}", x);
                assert!(false);
            },
        }
        assert!(true); 
    }
}
