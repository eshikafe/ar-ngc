use std::fmt;

// 5.4 EPC specific GTP-C header
#[derive(Default)]
struct GtpHeader {
    flags: u8, //version(3), P(1), T(1), MP(1), spare(2)
    msg_type: u8,
    msg_len: u16,
    teid: Option<u32>,
    seq_num: u32, // 24 bits
    msg_prio: Option<u8>,
}

// 5.6 GTPv2-C Message
#[allow(dead_code)]
struct GtpMessage {
    header: GtpHeader,
    info_element: Vec<u8>,
}

// struct GtpEchoRequest {
//     recovery GtpTlvRecovery,
//     sending_node_features GtpTlvNodeFeatures,
// }

// 6 GTP-C Message Types and Message Formats

enum MessageType {
    EchoRequest,                   // 1
    EchoResponse,                  // 2
    VersionNotSupportedIndication, // 3
    CreateSessionRequest,          // 32
    CreateSessionResponse,         // 33
    DeleteSessionRequest,          // 36
    DeleteSessionResponse,         // 37
}

const ECHO_REQUEST: u8 = 1;
const ECHO_RESPONSE: u8 = 2;
const VERSION_NOT_SUPPORTED_INDICATION: u8 = 3;

// S11, S5/S8
const CREATE_SESSION_REQUEST: u8 = 32;
const CREATE_SESSION_RESPONSE: u8 = 33;
const DELETE_SESSION_REQUEST: u8 = 36;
const DELETE_SESSION_RESPONSE: u8 = 37;


impl GtpHeader {
    fn parse(t_pdu: &[u8]) -> Self {
        let mut header = Self {
            flags: t_pdu[0],
            msg_type: t_pdu[1],
            msg_len: u16::from_be_bytes(t_pdu[2..4].try_into().unwrap()),
            ..Default::default()
        };
        let teid_flag = (t_pdu[0] & 0b00001000) >> 3;
        let mp_flag = (t_pdu[0] & 0b00000100) >> 2;
        match teid_flag {
            1 => {
                header.teid = Some(u32::from_be_bytes(t_pdu[4..8].try_into().unwrap()));
                header.seq_num =
                    ((t_pdu[8] as u32) << 16) + ((t_pdu[9] as u32) << 8) + t_pdu[10] as u32;
                if mp_flag == 1 {
                    header.msg_prio = Some((t_pdu[11] & 0b11110000) >> 4);
                } else {
                    header.msg_prio = None;
                }
            }
            _ => {
                header.teid = None;
                header.seq_num =
                    ((t_pdu[4] as u32) << 16) + ((t_pdu[5] as u32) << 8) + t_pdu[6] as u32;
                // if mp_flag == 1 {
                //     header.msg_prio = Some((t_pdu[7] & 0b11110000) >> 4);
                // } else {
                header.msg_prio = None;
                // }
            }
        }

        header
    }

    // Get GTP version
    fn version(&self) -> Result<u8, &'static str> {
        let ver: u8 = (self.flags & 0b11100000) >> 5;
        match ver {
            1 | 2 => Ok(ver),
            _ => Err("Unsupported GTP version: {ver}"),
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
        match self.msg_prio {
            Some(mp) => mp,
            None => 0,
        }
    }

    fn get_msg_type(&self) -> Result<&'static str, &'static str> {
        match self.msg_type {
            CREATE_SESSION_REQUEST => Ok("Create Session Request"),
            CREATE_SESSION_RESPONSE => Ok("Create Session Response"),
            ECHO_REQUEST => Ok("Echo Request"),
            ECHO_RESPONSE => Ok("Echo Response"),
            VERSION_NOT_SUPPORTED_INDICATION => Ok("Version Not Supported Indication"),
            DELETE_SESSION_REQUEST => Ok("Delete Session Request"),
            DELETE_SESSION_RESPONSE => Ok("Delete Session Response"),
            _ => Err("Unspported message"),
        }
    }
}

impl fmt::Display for GtpHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n Version: {}(0x{:x})\n Message Type: {}({:?})\n Message Length: {}\n Teid: {:?}\n Sequence Number: {}\n Message Priority: {:?}",
            self.flags,self.flags, self.msg_type, self.get_msg_type(), self.msg_len, self.teid, self.seq_num, self.msg_prio
        )
    }
}

fn dump_header(gtp_pkt: &[u8]) {
    let gtp_header = GtpHeader::parse(gtp_pkt);
    println!("Dump header: {}", gtp_header);
    println!("GTP Version: {:?}", gtp_header.version());
    println!("P Flag: {}", gtp_header.p_flag());
    println!("TEID Flag: {}", gtp_header.t_flag());
    println!("MP Flag: {}", gtp_header.mp_flag());
}

