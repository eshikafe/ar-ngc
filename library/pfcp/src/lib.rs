// Copyright (c) 2022 VPlane Telecoms
// PFCP Stack for 5G core
// Compliance: 3GPP TS 29.244 version 16.6.0 Release 16

// use std::fmt;
use std::io;
pub mod header;

pub use header::*;

pub const PFCP_UDP_PORT: u16 = 8805;

pub const MAX_NUM_OF_PDR: u8 = 16
pub const MAX_NUM_OF_FAR: u8 = 16
pub const MAX_NUM_OF_URR: u8 = 16
pub const MAX_NUM_OF_QER: u8 =  4
pub const MAX_NUM_OF_BAR: u8 = 1

use std::net::{IpAddr, SocketAddr};
use tokio::net::UdpSocket;

pub async fn server(ip_addr: IpAddr) -> io::Result<()> {
    let socket_addr = SocketAddr::new(ip_addr, PFCP_UDP_PORT);
    let sock = UdpSocket::bind(socket_addr).await?;
    let mut buf = [0u8; 1024];
    println!("PFCP: server started on {}", socket_addr);
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;

        //let len = sock.send_to(&buf[..len], addr).await?;
        //println!("{:?} bytes sent", len);
    }
}


#[tokio::#[test]
use pfcp;
use std::net::{IpAddr, Ipv4Addr};
fn test() {
    let n4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    pfcp::server(n4_addr).await;
}]




