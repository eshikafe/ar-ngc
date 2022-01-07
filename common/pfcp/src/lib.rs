// PFCP Stack
// Copyright (c) 2022 Austin Aigbe
// 3GPP TS 29.244 version 16.4.0 Release 16

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
