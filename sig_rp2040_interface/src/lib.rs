// Written by sigroot

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports_available() {
        let ports = get_matrix_ports().expect("No ACM or COM ports found");
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
    fn ports_matrix
}
