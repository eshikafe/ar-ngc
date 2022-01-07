// PFCP Stack
// Copyright (c) 2022 vPlane Telecom
// Compliance: 3GPP TS 29.244 version 16.4.0 Release 16

use std::convert::TryInto;
use std::fmt;
use std::io;
use std::net::{IpAddr, SocketAddr};
use tokio::net::UdpSocket;

const PFCP_PORT: u16 = 8805;

// 7.2.2 Message Header
struct Header {
    flag: u8, // Version(3), Spare(2), FO(1), MP(1), S(1)
    msg_type: u8,
    msg_len: u16,
    seid: Option<u64>,
    seq_num: u32, // 3 bytes
    msg_prio: Option<u8>,
}

pub async fn server(ip_addr: IpAddr) -> io::Result<()> {
    let socket_addr = SocketAddr::new(ip_addr, PFCP_PORT);
    let sock = UdpSocket::bind(socket_addr).await?;
    let mut buf = [0u8; 1024];
    println!("PFCP: server started on {}", socket_addr);
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;

        //let len = sock.send_to(&buf[..len], addr).await?;
        //println!("{:?} bytes sent", len);
    }
}
