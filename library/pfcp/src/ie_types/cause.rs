use super::*;

// Table 8.2.1-1: Cause values
pub const RESERVED: u8 = 0;
pub const REQUEST_ACCEPTED: u8 = 1;
pub const MORE_USAGE_REPORT_TO_SEND: u8 = 2;
// Spare: 3 - 63
pub const REQUEST_REJECTED: u8 = 64;
pub const SESSION_CONTEXT_NOT_FOUND: u8 = 65;
pub const MANDATORY_IE_MISSING: u8 = 66;
pub const CONDITIONAL_IE_MISSING: u8 = 67;
pub const INVALID_LENGTH: u8 = 68;
pub const MANDATORY_IE_INCORRECT: u8 = 69;
pub const INVALID_FORWARDING_POLICY: u8 = 70;
pub const INVALID_F_TEID_ALLOCATION_OPTION: u8 = 71;
pub const NO_ESTABLISHED_PFCP_ASSOCIATION: u8 = 72;
pub const RULE_CREATION_MODIFICATION_FAILURE: u8 = 73;
pub const PFCP_ENTITY_IN_CONGESTION: u8 = 74;
pub const NO_RESOURCES_AVAILABLE: u8 = 75;
pub const SERVICE_NOT_SUPPORTED: u8 = 76;
pub const SYSTEM_FAILURE: u8 = 77;
pub const REDIRECTION_REQUESTED: u8 = 78;
pub const ALL_DYNAMIC_ADDRESS_ARE_OCCUPIED: u8 = 79;
// Spare: 80 - 255

#[derive(Debug, Default)]
pub struct Cause {
    ie_type: u16,
    ie_len: u16,
    cause: u8,
}

impl Cause {
    pub fn decode(buf: &mut [u8], len: u16) -> Self {
        let mut element = Cause {
            ie_type: CAUSE_TYPE,
            ie_len: len,
            ..Default::default()
        };
        element.cause = buf[0];
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.cause);
        element_vec
    }
}