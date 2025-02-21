use std::time::Duration;
use std::io::{Write, Read, ErrorKind};
use mio::{Events, Interest, Poll, Token};
use mio_serial::{SerialPortBuilderExt, SerialPort};
use tauri::command;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/tty.PL2303G-USBtoUART14441140";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM6";
const DEFAULT_BAUD: u32 = 9600;
const SERIAL_TOKEN: Token = Token(0);

const DEFAULT_BUFFER_SIZE: usize = 1024;
const DEFAULT_RETRY_DELAY: u64 = 100; 
const MAX_RETRIES: usize = 10;


#[command]
fn list_ports() -> Result<Vec<String>, String> {
    match mio_serial::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|port| port.port_name).collect()),
        Err(e) => Err(format!("Failed to list ports: {}", e)),
    }
}

#[tauri::command]
fn open_port(path: String) -> Result<String, String> {
    let mut port = mio_serial::new(path.clone(), DEFAULT_BAUD)
        .open_native_async()
        .map_err(|e| format!("Failed to open port {}: {}", path, e))?;
    Ok(format!("Opened port: {}", path))
}

#[tauri::command]
fn write_to_port(path: String, data: String) -> Result<String, String> {
    println!("write_to_port called with path: {}, data: {}", path, data);

    let mut tx = mio_serial::new(path.clone(), DEFAULT_BAUD)
        .open_native()
        .map_err(|e| format!("Failed to open port {}: {}", path, e))?;
    println!("Serial port opened successfully");

    // Write data to the port
    tx.write_all(data.as_bytes())
        .map_err(|e| format!("Failed to write to port: {}", e))?;
    println!("Data sent to port: {}", data);

    // Read response from the port
    let mut buf = [0u8; DEFAULT_BUFFER_SIZE];
    let mut complete_response = String::new();
    let mut retries = 0;

    while retries < MAX_RETRIES {
        match tx.read(&mut buf) {
            Ok(count) if count > 0 => {
                let chunk = String::from_utf8_lossy(&buf[..count]);
                complete_response.push_str(&chunk);

                // Check if the response is complete
                if complete_response.ends_with("\n") {
                    println!("Complete response received.");
                    return Ok(complete_response);
                }
            }
            Ok(_) => {
                println!("No data available to read, retrying...");
            }
            Err(e) => {
                println!("Error reading from port: {}", e);
                if retries >= MAX_RETRIES {
                    return Err(format!(
                        "Failed to read from port after {} retries: {}",
                        MAX_RETRIES, e
                    ));
                }
            }
        }

        retries += 1;
        std::thread::sleep(Duration::from_millis(DEFAULT_RETRY_DELAY));
    }

    Err("Failed to receive complete response within retry limit.".to_string())
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_ports,
            open_port,
            write_to_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
