use std::time::Duration;
use std::io::{Write, Read};
use serialport::{available_ports, SerialPort};

#[tauri::command]
fn list_ports() -> Result<Vec<String>, String> {
    match serialport::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|port| port.port_name).collect()),
        Err(e) => Err(format!("Failed to list ports: {}", e)),
    }
}

#[tauri::command]
fn open_port(port_name: String) -> Result<String, String> {
    let port = serialport::new(&port_name, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .map_err(|e| format!("Failed to open port {}: {}", port_name, e))?;

    Ok(format!("Opened port: {}", port_name))
}


#[tauri::command]
fn write_to_port(port_name: String, data: String) -> Result<String, String> {
    let mut port = serialport::new(&port_name, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .map_err(|e| format!("Failed to open port {}: {}", port_name, e))?;

    let output = data.as_bytes();
    port.write(output).map_err(|e| format!("Write failed: {}", e))?;

    Ok(format!("Written to port {}: {}", port_name, data))
}


#[tauri::command]
fn read_from_port(port_name: String) -> Result<String, String> {
    let mut port = serialport::new(&port_name, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .map_err(|e| format!("Failed to open port {}: {}", port_name, e))?;

    let mut serial_buf: Vec<u8> = vec![0; 255];
    let mut full_data = String::new();

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(bytes_read) if bytes_read > 0 => {
                let result = String::from_utf8_lossy(&serial_buf[..bytes_read]).to_string();
                full_data.push_str(&result);

                if full_data.contains("\n") {
                    break; 
                }
            }
            Ok(_) => {
                println!("No data received, continuing...");
            }
            Err(e) => {
                return Err(format!("Read failed: {}", e));
            }
        }
    }


    let parsed_values: Vec<&str> = full_data.split(',').collect();
 
    Ok(full_data)
}




fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_ports, open_port, write_to_port, read_from_port])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
