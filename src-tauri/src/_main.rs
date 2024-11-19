use std::io::{self, Write};
use std::time::Duration;
use clap::{App, Arg};
use serialport::{self, SerialPort};

fn main() {
    // Define the argument parser
    let matches = App::new("SerialPortApp")
        .version("1.0")
        .about("Reads and writes data to a serial port")
        .arg(
            Arg::new("port")
                .help("The serial port to connect to")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("baud")
                .help("The baud rate to connect at")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("data")
                .help("The data to write to the serial port")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Retrieve the argument values
    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();
    let data = matches.value_of("data").unwrap();

    // Your code for working with the serial port here
    println!("Connecting to {} at {} baud...", port_name, baud_rate);
    println!("Sending data: {}", data);
}

// Function to list available serial ports
fn list_ports() {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("No serial ports found.");
            } else {
                println!("Available serial ports:");
                for p in ports {
                    println!("{}", p.port_name);
                }
            }
        }
        Err(e) => eprintln!("Error listing ports: {}", e),
    }
}

// Function to write data to a serial port and read the response
fn write_data(port_name: &str, baud_rate: u32, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Open the serial port
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()?;

    // Write data to the serial port
    port.write_all(data)?;

    // Read the response from the serial port
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    match port.read(serial_buf.as_mut_slice()) {
        Ok(bytes_read) => {
            // Trim the response to the number of bytes actually read
            serial_buf.truncate(bytes_read);
            Ok(serial_buf)
        }
        Err(e) => Err(Box::new(e)),
    }
}

// Validator for baud rate input
fn valid_baud(val: &str) -> Result<u32, String> {
    val.parse::<u32>()
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}
