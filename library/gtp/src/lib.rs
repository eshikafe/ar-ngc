// GTP Stack
// Copyright (c) 2022 VPlane Telecoms
// Compliance:
// 3GPP TS 29.274 (GTPv2-C) version 16.6.0 Release 16
// 3GPP TS 29.281 (GTPv1-U) version 16.6.0 Release 16

// use std::convert::TryInto;
use std::io;
use std::net::{IpAddr, SocketAddr};
use tokio::net::UdpSocket;
mod message;
mod types;

use self::message::*;

// GTP ports
const GTPV2C_PORT: u16 = 2123;
const GTPV1U_PORT: u16 = 2152;

pub async fn server(ip_addr: IpAddr) -> io::Result<()> {
    let socket_addr = SocketAddr::new(ip_addr, GTPV2C_PORT);
    let sock = UdpSocket::bind(socket_addr).await?;
    let mut buf = [0u8; 1024];
    println!("GTPStack: GTPv2-C server started on {}", socket_addr);
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("Client IP address: {:?}", addr);
        dump_header(&buf);

        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} number of bytes sent", len);
    }
}

// use gtp;
// use std::net::{IpAddr, Ipv4Addr};

// #[tokio::main]
// async fn main() {
//     let s11_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
//     gtp::server(s11_addr).await;
// }
