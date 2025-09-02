use std::collections::HashMap;
use std::error::Error;
use std::net::{IpAddr, Shutdown};
use std::str::FromStr;
use std::sync::Mutex;

use crate::configuration::Configuration;
use async_std::io::{ReadExt, WriteExt};
use async_std::net::{SocketAddr, TcpListener, TcpStream};
use async_std::task;
use chrono::Local;
use env_logger;
use env_logger::Builder;
use log::{error, info, LevelFilter};
use once_cell::sync::Lazy;
use toml;
use utils::log::configuration::{init_logger, load_config_file};
// use crate::utils::configuration::{Configuration, init_logger, load_config_file};

// lazy_static! {
//     static ref CONFIG :Configuration = toml::from_str(load_config_file().as_str()).unwrap();
//     static ref MAP_SESSION_TCP: Mutex<HashMap<String, TcpStream>> = Mutex::new(HashMap::<String, TcpStream>::new());
// }

static CONFIG: Lazy<Configuration> =
    Lazy::<Configuration>::new(|| toml::from_str(load_config_file().as_str()).unwrap());
static MAP_SESSION_TCP: Lazy<Mutex<HashMap<String, TcpStream>>> =
    Lazy::new(|| Mutex::new(HashMap::<String, TcpStream>::new()));

async fn handle_cli_connection(
    cli_tcp_stream: TcpStream,
    cli_addr: SocketAddr,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // add in to tcp_stream list
    // let mut buffer: Vec<u8> = vec![0; CONFIG.tcp_server.buffer_size];
    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(CONFIG.tcp_server.buffer_size, 0u8);
    // cli_tcp_stream.set_nodelay(true).unwrap();
    let mut tcp_stream = cli_tcp_stream.clone();
    MAP_SESSION_TCP
        .lock()
        .unwrap()
        .insert(cli_addr.to_string(), cli_tcp_stream);
    loop {
        match tcp_stream.read(&mut buffer).await {
            Ok(n) => {
                // info!("buff len:{}",buffer.len());
                if n > 0 {
                    let received_data = String::from_utf8_lossy(&buffer[..n]);
                    // buffer.clear();
                    info!(
                        "Received[{}]:[{}]|{}|",
                        cli_addr.to_string(),
                        n,
                        received_data
                    );
                    if received_data.ends_with("[exit]") {
                        break;
                    }
                    // tcp_stream.write_all(format!("Received at:{}", Local::now().naive_local()).as_bytes()).await.unwrap();
                    // tcp_stream.flush().await.unwrap();
                    // tcp_stream.flush().await.unwrap();
                }
            }
            Err(e) => break,
        }
    }
    info!("Client [{}] disconnected!", cli_addr.to_string());
    let shutdown_ok = tcp_stream.shutdown(Shutdown::Both).is_ok();
    if shutdown_ok {
        info!("Shutdown stream!")
    } else {
        info!("Shutdown failed!!!")
    }
    let session_lock = MAP_SESSION_TCP.lock();
    match session_lock {
        Ok(mut mg) => {
            mg.remove(&cli_addr.to_string());
            info!("Session size:{}", mg.len());
        }
        Err(e) => {
            error!("Session Error: {}", e)
        }
    }
    Ok(())
}

#[test]
fn start_server() {
    init_logger();
    task::block_on(async {
        let ip_addr = IpAddr::from_str(&CONFIG.tcp_server.address).unwrap();
        let address = SocketAddr::new(ip_addr, CONFIG.tcp_server.port);
        info!(
            "Server is listening at {}:{} ...",
            address.ip().to_string(),
            address.port()
        );
        let tcp_server = TcpListener::bind(address).await.unwrap();
        while let (tcp_stream, cli_addr) = tcp_server.accept().await.unwrap() {
            info!("Client addr:{} connected", cli_addr.to_string());
            if let Err(e) = task::spawn(handle_cli_connection(tcp_stream, cli_addr)).await {
                error!("{:?}", e)
            }
        }
    });
}
