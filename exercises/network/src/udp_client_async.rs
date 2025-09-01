use async_std::io::{ReadExt, Write, WriteExt};
use async_std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use async_std::task;
use std::collections::HashMap;
use std::env::join_paths;
use std::error::Error;
use std::net::{IpAddr, Shutdown};
use std::str::FromStr;
use std::sync::Mutex;
// use lazy_static::lazy_static;
use crate::configuration::Configuration;
use env_logger;
use env_logger::Builder;
use log::{error, info, LevelFilter};
use once_cell::sync::Lazy;
use toml;
use utils::log::configuration::{init_logger, load_config_file};
// use crate::macros::configuration::{Configuration, init_logger, load_config_file};

// lazy_static! {
//     static ref CONFIG :Configuration = toml::from_str(load_config_file().as_str()).unwrap();
// }

static CONFIG: Lazy<Configuration> =
    Lazy::<Configuration>::new(|| toml::from_str(load_config_file().as_str()).unwrap());

async fn handle_serv_connection(
    mut cli_tcp_stream: TcpStream,
    cli_addr: &SocketAddr,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // tcp_client.set_nodelay(true).unwrap();
    info!(
        "Client send to {}:{} ...",
        cli_addr.to_string(),
        cli_addr.port()
    );
    // tcp_client.write_all(String::from("Hello server").as_bytes()).await.unwrap();
    // tcp_client.flush().await.unwrap();
    // tcp_client.write_all(String::from("Thử cả dữ liệu UTF8 xem thế nào").as_bytes()).await.unwrap();
    // // tcp_client.flush().await.unwrap();
    // tcp_client.write_all(String::from("[exit]").as_bytes()).await.unwrap();
    // tcp_client.flush().await.unwrap();
    let mut str = String::new();
    let _stdin = async_std::io::stdin();
    loop {
        str.clear();
        match _stdin.read_line(&mut str).await {
            Ok(count) => {
                str = str.trim().to_string();
                if str == "exit" {
                    break;
                }
                match cli_tcp_stream.write_all(str.as_bytes()).await {
                    Err(e) => {
                        error!("{:?}", e)
                    }
                    _ => {}
                }
            }
            Err(e) => {
                error!("{:?}", e)
            }
        }
    }
    let shutdown_ok = cli_tcp_stream.shutdown(Shutdown::Both).is_ok();
    if shutdown_ok {
        info!("Shutdown is ok!")
    } else {
        info!("Shutdown is failed!!!")
    }
    info!(
        "Client is disconnected from {}:{} ...",
        cli_addr.to_string(),
        cli_addr.port()
    );
    Ok(())
}

#[test]
/// Adds two numbers together.
///
/// # Examples
///
/// ```shell
/// $ cargo test --color=always --package basic-rust --bin basic-rust macros::tcp_client_async::start_client
/// ```
fn start_client() {
    init_logger();
    task::block_on(async {
        let cli_ip_addr = IpAddr::from_str(&CONFIG.udp_client.address).unwrap();
        let cli_address = SocketAddr::new(cli_ip_addr, CONFIG.udp_client.port + 1);
        let udp_client = UdpSocket::bind(cli_address).await.unwrap();

        let srv_ip_addr = IpAddr::from_str(&CONFIG.udp_server.address).unwrap();
        let srv_address = SocketAddr::new(srv_ip_addr, CONFIG.udp_server.port);
        udp_client.connect(srv_address).await.unwrap();

        let msg = "Đây là dữ liệu thử nghiệm truyền qua socket".as_bytes();

        let mut buff = vec![0u8; CONFIG.udp_client.buffer_size];
        // buff.clone_from_slice(msg);
        // buff.truncate(msg.len());
        let _ = udp_client.send(msg).await.unwrap();
        info!(
            "Sent to {:?}: {}",
            srv_address,
            String::from_utf8_lossy(msg).to_string()
        );
        buff.fill(0u8);
        let recv_count = udp_client.recv(&mut buff).await.unwrap();
        let echo_msg = String::from_utf8_lossy(&buff[..recv_count]).to_string();
        info!("Received from {:?}: {}", srv_address, echo_msg);
    });
}
