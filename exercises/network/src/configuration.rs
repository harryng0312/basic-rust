use serde::Deserialize;

pub const CONFIG_FILE: &str = "config.toml";
#[derive(Debug, Deserialize)]
pub struct Configuration {
    #[serde(rename = "tcp_server")]
    pub tcp_server: TcpServerCfg,
    #[serde(rename = "tcp_client")]
    pub tcp_client: TcpClientCfg,
    #[serde(rename = "udp_server")]
    pub udp_server: UdpServerCfg,
    #[serde(rename = "udp_client")]
    pub udp_client: UdpClientCfg,
}

#[derive(Debug, Deserialize)]
pub struct TcpServerCfg {
    pub address: String,
    pub port: u16,
    pub buffer_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct TcpClientCfg {
    pub address: String,
    pub port: u16,
    pub buffer_size: usize,
}
#[derive(Debug, Deserialize)]
pub struct UdpServerCfg {
    pub address: String,
    pub port: u16,
    pub buffer_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct UdpClientCfg {
    pub address: String,
    pub port: u16,
    pub buffer_size: usize,
}
