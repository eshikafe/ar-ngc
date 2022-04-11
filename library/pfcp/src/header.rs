// Copyright (c) 2022 VPlane Telecoms

use std::convert::TryInto;

// 7.2.2 Message Header
#[derive(Debug, Default)]
pub struct PfcpHeader {
    flags: u8, // Version(3), Spare(2), FO(1), MP(1), S(1)
    msg_type: u8,
    msg_length: u16,
    seid: Option<u64>, // Session Endpoint Identifier
    seq_num: u32,      // 3 bytes
    msg_priority: Option<u8>,
}

impl PfcpHeader {
    pub fn parse(buf: &[u8]) -> PfcpHeader {
        let mut header = PfcpHeader {
            flags: buf[0],
            msg_type: buf[1],
            msg_length: u16::from_be_bytes(buf[2..4].try_into().unwrap()),
            ..Default::default()
        };
        let seid_flag = buf[0] & 0b0000_0001;
        let mp_flag = (buf[0] & 0b0000_0010) >> 1;
        match seid_flag {
            1 => {
                header.seid = Some(u64::from_be_bytes(buf[4..12].try_into().unwrap()));
                header.seq_num =
                    ((buf[12] as u32) << 16) + ((buf[13] as u32) << 8) + buf[14] as u32;
                if mp_flag == 1 {
                    header.msg_priority = Some((buf[15] & 0b11110000) >> 4);
                } else {
                    header.msg_priority = None;
                }
            }
            _ => {
                header.seid = None;
                header.seq_num = ((buf[4] as u32) << 16) + ((buf[5] as u32) << 8) + buf[6] as u32;
                header.msg_priority = None;
            }
        }
        header
    }
    pub fn pack(self) -> Vec<u8> {
        let mut header_vec: Vec<u8> = Vec::new();
        let flags: u8 = self.flags;
        header_vec.push(flags);
        header_vec.push(self.msg_type);
        header_vec.append(&mut self.msg_length.to_be_bytes().to_vec());
        /*if self.s {
            header_vec.append(&mut self.seid);
        }*/
        if let Some(seid) = self.seid {
            header_vec.append(&mut seid.to_be_bytes().to_vec());
        }
        header_vec.push(((self.seq_num >> 16) & 0xFF).try_into().unwrap());
        header_vec.push(((self.seq_num >> 8) & 0xFF).try_into().unwrap());
        header_vec.push((self.seq_num & 0xFF).try_into().unwrap());
        if let Some(priority) = self.msg_priority {
            header_vec.push(priority);
        } else {
            header_vec.push(0x00);
        }
        header_vec
    }
}
