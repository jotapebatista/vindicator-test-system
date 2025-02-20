use mio_serial::{SerialPort};
use std::io::{Read, Write};
use std::time::Duration;
use tauri::command;
use log::{info, error};
use thiserror::Error;


#[cfg(windows)]
const DEFAULT_TTY: &str = "COM6";

const DEFAULT_BAUD: u32 = 9600;
const DEFAULT_BUFFER_SIZE: usize = 1024;
const DEFAULT_RETRY_DELAY: u64 = 100;
const MAX_RETRIES: usize = 10;

#[derive(Debug, Error)]
enum SerialError {
    #[error("Failed to list ports: {0}")]
    ListPortsError(String),

    #[error("Failed to open port {0}: {1}")]
    OpenPortError(String, String),

    #[error("Failed to write to port: {0}")]
    WriteError(String),

    #[error("Failed to read from port after {0} retries: {1}")]
    ReadError(usize, String),

    #[error("No response received within retry limit.")]
    NoResponse,
}

struct SerialPortManager;

impl SerialPortManager {
    fn list_ports() -> Result<Vec<String>, SerialError> {
        mio_serial::available_ports()
            .map(|ports| ports.into_iter().map(|port| port.port_name).collect())
            .map_err(|e| SerialError::ListPortsError(e.to_string()))
    }

	fn open_port(path: &str) -> Result<Box<dyn SerialPort>, SerialError> {
		mio_serial::new(path, DEFAULT_BAUD)
			.open_native()
			.map(|port| Box::new(port) as Box<dyn SerialPort>)
			.map_err(|e| SerialError::OpenPortError(path.to_string(), e.to_string()))
	}

    fn write_to_port(path: &str, data: &str) -> Result<String, SerialError> {
        info!("write_to_port called with path: {}, data: {}", path, data);

        let mut port = Self::open_port(path)?;
        info!("Serial port opened successfully");

        port.write_all(data.as_bytes())
            .map_err(|e| SerialError::WriteError(e.to_string()))?;
        info!("Data sent to port: {}", data);

        let mut buf = [0u8; DEFAULT_BUFFER_SIZE];
        let mut response = String::new();
        let mut retries = 0;

        while retries < MAX_RETRIES {
            match port.read(&mut buf) {
                Ok(count) if count > 0 => {
                    let chunk = String::from_utf8_lossy(&buf[..count]);
                    response.push_str(&chunk);

                    if response.ends_with('\n') {
                        info!("Complete response received.");
                        return Ok(response);
                    }
                }
                Ok(_) => {
                    info!("No data available to read, retrying...");
                }
                Err(e) => {
                    error!("Error reading from port: {}", e);
                    if retries >= MAX_RETRIES {
                        return Err(SerialError::ReadError(MAX_RETRIES, e.to_string()));
                    }
                }
            }

            retries += 1;
            std::thread::sleep(Duration::from_millis(DEFAULT_RETRY_DELAY));
        }

        Err(SerialError::NoResponse)
    }
}

#[command]
fn list_ports() -> Result<Vec<String>, String> {
    SerialPortManager::list_ports().map_err(|e| e.to_string())
}

#[tauri::command]
fn open_port(path: String) -> Result<String, String> {
    SerialPortManager::open_port(&path)
        .map(|_| format!("Opened port: {}", path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn write_to_port(path: String, data: String) -> Result<String, String> {
    SerialPortManager::write_to_port(&path, &data).map_err(|e| e.to_string())
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_ports,
            open_port,
            write_to_port
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
