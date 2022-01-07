// GTP Stack
// Copyright (c) 2022 Austin Aigbe
// 3GPP TS 29.274 version 16.6.0 Release 16

use std::convert::TryInto;
use std::fmt;
use std::io;
use std::net::{IpAddr, SocketAddr};
use tokio::net::UdpSocket;

// GTP ports
const GTPV2C_PORT: u16 = 2123;

// 5.4 EPC specific GTP-C header
#[derive(Default)]
struct Gtpv2CHeader {
    flags: u8, //version(3), P(1), T(1), MP(1), spare(2)
    msg_type: u8,
    msg_len: u16,
    teid: Option<u32>,
    seq_num: u32,
    spare: u8,
}

// 5.6 GTPv2-C Message
struct Gtpv2CMessage {
    header: Gtpv2CHeader,
    info_elem: Vec<u8>,
}

// 6 GTP-C Message Types and Message Formats
const ECHO_REQUEST: u8 = 1;
const ECHO_RESPONSE: u8 = 2;
const VERSION_NOT_SUPPORTED_INDICATION: u8 = 3;

// S11, S5/S8
const CREATE_SESSION_REQUEST: u8 = 32;
const CREATE_SESSION_RESPONSE: u8 = 33;
const DELETE_SESSION_REQUEST: u8 = 36;
const DELETE_SESSION_RESPONSE: u8 = 37;

impl Gtpv2CHeader {
    fn header(t_pdu: [u8; 1024]) -> Self {
        let mut header = Self {
            flags: t_pdu[0],
            msg_type: t_pdu[1],
            msg_len: u16::from_be_bytes(t_pdu[2..4].try_into().unwrap()),
            ..Default::default()
        };
        let t_flag = (t_pdu[0] & 0b00001000) >> 3;
        match t_flag {
            1 => {
                // let mp_flag = (t_pdu[0] & 0b00000100) >> 2;
                header.teid = Some(u32::from_be_bytes(t_pdu[4..8].try_into().unwrap()));
                header.seq_num =
                    ((t_pdu[8] as u32) << 16) + ((t_pdu[9] as u32) << 8) + t_pdu[10] as u32;
                header.spare = t_pdu[11];
            }
            _ => {
                header.teid = None;
                header.seq_num =
                    ((t_pdu[4] as u32) << 16) + ((t_pdu[5] as u32) << 8) + t_pdu[6] as u32;
                header.spare = t_pdu[7];
            }
        }

        header
    }

    // GTP-C version
    fn version(&self) -> Result<u8, &'static str> {
        let ver: u8 = (self.flags & 0b11100000) >> 5;
        match ver {
            1 | 2 => Ok(ver),
            _ => Err("Invalid GTP packet received"),
        }
    }

    // Is message piggybacked?
    fn p_flag(&self) -> u8 {
        (self.flags & 0b00010000) >> 4
    }

    // TEID flag: indicates if TEID field is present in the GTP-C header or not
    fn t_flag(&self) -> u8 {
        (self.flags & 0b00001000) >> 3
    }

    // If the "MP" flag is set to "1",
    // then bits 8 to 5 of octet 12 shall indicate the message priority
    fn mp_flag(&self) -> u8 {
        (self.flags & 0b00000100) >> 2
    }

    fn get_msg_priority(&self) -> u8 {
        (self.flags & 0b11110000) >> 4
    }

    fn get_msg_type(&self) -> Result<&'static str, &'static str> {
        match self.msg_type {
            CREATE_SESSION_REQUEST => Ok("Create Session Request"),
            _ => Err("Unspported message"),
        }
    }
}

impl fmt::Display for Gtpv2CHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n Version: {}(0x{:x})\n Message Type: {}({:?})\n Message Length: {}\n Teid: {:?}\n Sequence Number: {}\n Spare: {}",
            self.flags,self.flags, self.msg_type, self.get_msg_type(), self.msg_len, self.teid, self.seq_num, self.spare
        )
    }
}

pub async fn server(ip_addr: IpAddr) -> io::Result<()> {
    let socket_addr = SocketAddr::new(ip_addr, GTPV2C_PORT);
    let sock = UdpSocket::bind(socket_addr).await?;
    let mut buf = [0u8; 1024];
    println!("GTPStack: GTPv2-C server started on {}", socket_addr);
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let gtp_data = Gtpv2CHeader::header(buf);
        println!("Dump header: {}", gtp_data);
        println!("GTP Version: {:?}", gtp_data.version());
        println!("P Flag: {}", gtp_data.p_flag());
        println!("TEID Flag: {}", gtp_data.t_flag());
        if gtp_data.mp_flag() == 1 {
            println!("Message priority: {}", gtp_data.get_msg_priority());
        }

        //let len = sock.send_to(&buf[..len], addr).await?;
        //println!("{:?} bytes sent", len);
    }
}
