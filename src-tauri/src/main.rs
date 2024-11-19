use std::time::Duration;
use std::io::{Write, Read,ErrorKind};
use mio::{Events, Interest, Poll, Token};
use mio_serial::{SerialPortBuilderExt, SerialPort};
use tauri::command;
use std::env;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/tty.PL2303G-USBtoUART14441140";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM6";
const DEFAULT_BAUD: u32 = 9600;

const SERIAL_TOKEN: Token = Token(0);

#[command]
fn list_ports() -> Result<Vec<String>, String> {
    // List available serial ports using mio_serial
    match mio_serial::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|port| port.port_name).collect()),
        Err(e) => Err(format!("Failed to list ports: {}", e)),
    }
}

#[tauri::command]
fn open_port(path: String) -> Result<String, String> {
    // Clone path here to avoid the borrow checker error
    let mut rx = mio_serial::new(path.clone(), DEFAULT_BAUD)  // Clone path here
        .open_native_async()
        .map_err(|e| format!("Failed to open port {}: {}", path, e))?;

    Ok(format!("Opened port: {}", path))
}

#[tauri::command]
fn write_to_port(path: String, data: String) -> Result<String, String> {
    println!("write_to_port called with path: {}, data: {}", path, data);

    let mut poll = Poll::new().map_err(|e| format!("Failed to create poll: {}", e))?;
    let mut events = Events::with_capacity(1);

    let mut tx = mio_serial::new(path.clone(), DEFAULT_BAUD)
        .open_native_async()
        .map_err(|e| format!("Failed to open port {}: {}", path, e))?;
    println!("Serial port opened successfully");

    // Write data to the serial port
    let output = data.as_bytes();
    tx.write_all(output)
        .map_err(|e| format!("Write failed: {}", e))?;
    println!("Data sent to port: {}", data);

    let mut buf = [0u8; 1024]; // You can increase the buffer size here if needed
    let mut retries = 0;
    let max_retries = 10;
    let mut response = String::new();
    let mut complete_response = String::new();

    loop {
        match tx.read(&mut buf) {
            Ok(count) => {
                if count > 0 {
                    // Append the new data to the complete response
                    complete_response.push_str(&String::from_utf8_lossy(&buf[..count]));

                    // Check if the response is complete (if there’s a specific ending pattern you can check for)
                    // Example: Check if you’ve received a newline, or a specific character to mark the end
                    if complete_response.ends_with("\n") {
                        break; // Exit the loop once complete response is received
                    }
                }
            }
            Err(e) => {
                if retries >= max_retries {
                    return Err(format!("Failed to read from port after {} retries: {}", max_retries, e));
                }
                retries += 1;
                println!("Error reading from serial port, retrying...: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(100)); // Retry after delay
            }
        }
    }

    // Return the complete response
    Ok(complete_response)
}

#[command]
fn read_from_port(path: String) -> Result<String, String> {

    let mut poll = Poll::new().map_err(|e| format!("Failed to create poll: {}", e))?;
    let mut events = Events::with_capacity(1);

    let mut rx = mio_serial::new(path.clone(), DEFAULT_BAUD)  
        .open_native_async()
        .map_err(|e| format!("Failed to open port {}: {}", path, e))?;

    poll.registry()
        .register(&mut rx, SERIAL_TOKEN, Interest::READABLE)
        .map_err(|e| format!("Failed to register serial port: {}", e))?;

    let mut buf = [0u8; 1024];
    let mut full_data = String::new();

    loop {
		println!("read_from_port called with path: {}", path);
        poll.poll(&mut events, None)
            .map_err(|e| format!("Poll failed: {}", e))?;

        for event in events.iter() {
            match event.token() {
                SERIAL_TOKEN => loop {
                    match rx.read(&mut buf) {
                        Ok(count) if count > 0 => {
                            full_data.push_str(&String::from_utf8_lossy(&buf[..count]));
                            // Check if the data contains a newline character to stop reading
                            if full_data.contains("\n") {
                                break;
                            }
                        }
                        Ok(_) => break,
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(e) => {
                            return Err(format!("Read failed: {}", e));
                        }
                    }
                },
                _ => {}
            }
        }

        // If we have received full data, break the loop
        if full_data.contains("\n") {
            break;
        }
    }

    Ok(full_data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_ports, open_port, write_to_port, read_from_port])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
