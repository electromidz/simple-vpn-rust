use log::{error, info};
use std::io::Result;
use std::net::{SocketAddr, UdpSocket};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let server_ip = "172.19.24.18:51820";
    info!("Starting server  on IP :: {}", server_ip);

    let mut config = tun::Configuration::default();
    config.address("10.10.10.10").netmask("255.255.255.0").up();
    let mut device = tun::create(&config)?;
    info!("TUN device created with IP: 10.10.10.10");

    let mut buf = [0u8; 1500];
    let socket = UdpSocket::bind(server_ip)?;
    info!("UDP socket bound successfully");

    loop {
        let (data, socketAddr) = socket.recv_from(&mut buf)?;
        info!("data form udp {:?}", data);
        info!("from sockerAddr udp {:?}", socketAddr);
    }
}

