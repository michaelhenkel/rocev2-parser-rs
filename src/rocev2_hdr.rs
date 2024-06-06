use std::mem;
use pnet_macros::Packet;
use pnet_macros_support::types::*;

#[derive(Packet)]
pub struct BthHdr{
    pub opcode: u8,
    pub sol_event: u8,
    pub part_key: u16be,
    pub res: u8,
    pub dest_qpn: u24be,
    pub ack: u8,
    pub psn_seq: u24be,
    #[payload]
    pub payload: Vec<u8>
}
impl BthHdr {
    pub const LEN: usize = mem::size_of::<BthHdr>();
}

#[derive(Packet)]
pub struct AethHdr{
    pub syndrom: u8,
    pub message: u24be,
    #[payload]
    pub payload: Vec<u8>
}
impl AethHdr {
    pub const LEN: usize = mem::size_of::<BthHdr>();
}

pub struct InvariantCrc{
    pub crc: u32,
}
impl InvariantCrc {
    pub const LEN: usize = mem::size_of::<InvariantCrc>();
}

pub struct DethHdr{
    pub queue_key: u32,
    pub res: u8,
    pub src_qpn: [u8;3],
}
impl DethHdr {
    pub const LEN: usize = mem::size_of::<DethHdr>();
}

pub struct MadHdr{
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub status: u16,
    pub class_specific: u16,
    pub transaction_id: [u8;8],
    pub attribute_id: u16,
    pub res: u16,
    pub attribute_modifier: u32,
}
impl MadHdr {
    pub const LEN: usize = mem::size_of::<MadHdr>();
}

pub struct CmConnectRequest{
    pub local_comm_id: u32,
    pub res: u32,
    pub ip_cm_service_id: IpCmServiceId,
    pub local_ca_guid: [u8;8],
    pub res2: u32,
    pub local_q_key: u32,
    pub local_qpn: [u8;3],
    pub responder_resources: u8,
    pub local_eecn: [u8;3],
    pub initiator_depth: u8,
    pub remote_eecn: [u8;3],
    pub remote_cm_response_timeout: u8,
    pub starting_psn: [u8;3],
    pub local_cm_response_timeout: u8,
    pub partition_key: u16,
    pub path_packet_payload_mtu: u8,
    pub max_cm_retries: u8,
    pub primary_local_port_lid: u16,
    pub primary_remote_port_lid: u16,
    pub ghost1: [u8;10],
    pub ghost2: u16,
    pub primary_local_port_gid: u32,
    pub ghost3: [u8;10],
    pub ghost4: u16,
    pub primary_remote_port_gid: u32,
    pub primary_flow_label: [u8;3],
    pub ghost5: u8,
    pub primary_traffic_class: u8,
    pub primary_hop_limit: u8,
    pub primary_subnet_local: u8,
    pub primary_local_ack_timeout: u8,
    pub alternate_local_port_lid: u16,
    pub alternate_remote_port_lid: u16,
    pub alternate_local_port_gid: [u8;16],
    pub alternate_remote_port_gid: [u8;16],
    pub alternate_flow_label: [u8;3],
    pub ghost6: u8,
    pub alternate_traffic_class: u8,
    pub alternate_hop_limit: u8,
    pub alternate_subnet_local: u8,
    pub alternate_local_ack_timeout: u8,
    pub ip_cm_private_data: IpCmPrivateData,
}
impl CmConnectRequest {
    pub const LEN: usize = mem::size_of::<CmConnectRequest>();
}

pub struct IpCmPrivateData{
    pub ip_cm_major_minor_version: u8,
    pub ip_cm_ip_version: u8,
    pub ip_cm_source_port: u16,
    pub ghost1: [u8;12],
    pub ip_cm_source_ip: u32,
    pub ghost2: [u8;12],
    pub ip_cm_destination_ip: u32,
    pub ip_cm_consumer_private_data: [u32;14],
}
impl IpCmPrivateData {
    pub const LEN: usize = mem::size_of::<IpCmPrivateData>();
}

pub struct IpCmServiceId{
    pub prefix: [u8;5],
    pub protocol: u8,
    pub destination_port: u16,
}
impl IpCmServiceId {
    pub const LEN: usize = mem::size_of::<IpCmServiceId>();
}

pub struct CmConnectReply{
    pub local_comm_id: u32,
    pub remote_comm_id: u32,
    pub local_q_key: u32,
    pub local_qpn: [u8;3],
    pub res: u8,
    pub local_ee_context_number: [u8;3],
    pub res2: u8,
    pub starting_psn: [u8;3],
    pub res3: u8,
    pub responder_resources: u8,
    pub initiator_depth: u8,
    pub target_ack_delay: u8,
    pub rnr_retry_count: u8,
    pub local_ca_guid: [u8;8],
    pub private_data_1: [u32;32],
    pub private_data_2: [u32;17],
}
impl CmConnectReply {
    pub const LEN: usize = mem::size_of::<CmConnectReply>();
}

pub struct CmReadyToUse{
    pub local_comm_id: u32,
    pub remote_comm_id: u32,
    pub private_data_1: [u32;32],
    pub private_data_2: [u32;24],

}
impl CmReadyToUse {
    pub const LEN: usize = mem::size_of::<CmReadyToUse>(); 
}

pub struct CmDisconnectRequest{
    pub local_comm_id: u32,
    pub remote_comm_id: u32,
    pub remote_qpn_eecn: [u8;3],
    pub res: u8,
    pub private_data_1: [u32;32],
    pub private_data_2: [u32;23],
}
impl CmDisconnectRequest {
    pub const LEN: usize = mem::size_of::<CmDisconnectRequest>();
}

pub struct CmDisconnectReply{
    pub local_comm_id: u32,
    pub remote_comm_id: u32,
    pub private_data_1: [u32;32],
    pub private_data_2: [u32;24],
}
impl CmDisconnectReply {
    pub const LEN: usize = mem::size_of::<CmDisconnectReply>();
}

#[repr(u8)]
pub enum IbvOpcodeTransportTypes {
    Rc = 0x00,
    Uc = 0x20,
    Rd = 0x40,
    Ud = 0x60,
}


#[repr(u8)]
pub enum IbvOpcodeOperations {
    SendFirst = 0x00,
    SendMiddle = 0x01,
    SendLast = 0x02,
    SendLastWithImmediate = 0x03,
    SendOnly = 0x04,
    SendOnlyWithImmediate = 0x05,
    RdmaWriteFirst = 0x06,
    RdmaWriteMiddle = 0x07,
    RdmaWriteLast = 0x08,
    RdmaWriteLastWithImmediate = 0x09,
    RdmaWriteOnly = 0x0a,
    RdmaWriteOnlyWithImmediate = 0x0b,
    RdmaReadRequest = 0x0c,
    RdmaReadResponseFirst = 0x0d,
    RdmaReadResponseMiddle = 0x0e,
    RdmaReadResponseLast = 0x0f,
    RdmaReadResponseOnly = 0x10,
    Acknowledge = 0x11,
    AtomicAcknowledge = 0x12,
    CompareSwap = 0x13,
    FetchAdd = 0x14,
}



#[repr(u8)]
pub enum IbvOpcodeRc {
    SendFirst = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::SendFirst as u8,
    SendMiddle = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::SendMiddle as u8,
    SendLast = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::SendLast as u8,
    SendLastWithImmediate = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::SendLastWithImmediate as u8,
    SendOnly = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::SendOnly as u8,
    SendOnlyWithImmediate = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::SendOnlyWithImmediate as u8,
    RdmaWriteFirst = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaWriteFirst as u8,
    RdmaWriteMiddle = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaWriteMiddle as u8,
    RdmaWriteLast = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaWriteLast as u8,
    RdmaWriteLastWithImmediate = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaWriteLastWithImmediate as u8,
    RdmaWriteOnly = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaWriteOnly as u8,
    RdmaWriteOnlyWithImmediate = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaWriteOnlyWithImmediate as u8,
    RdmaReadRequest = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaReadRequest as u8,
    RdmaReadResponseFirst = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaReadResponseFirst as u8,
    RdmaReadResponseMiddle = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaReadResponseMiddle as u8,
    RdmaReadResponseLast = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaReadResponseLast as u8,
    RdmaReadResponseOnly = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::RdmaReadResponseOnly as u8,
    Acknowledge = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::Acknowledge as u8,
    AtomicAcknowledge = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::AtomicAcknowledge as u8,
    CompareSwap = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::CompareSwap as u8,
    FetchAdd = IbvOpcodeTransportTypes::Rc as u8 | IbvOpcodeOperations::FetchAdd as u8,
}



#[repr(u8)]
pub enum IbvOpcodeUc {
    SendFirst = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::SendFirst as u8,
    SendMiddle = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::SendMiddle as u8,
    SendLast = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::SendLast as u8,
    SendLastWithImmediate = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::SendLastWithImmediate as u8,
    SendOnly = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::SendOnly as u8,
    SendOnlyWithImmediate = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::SendOnlyWithImmediate as u8,
    RdmaWriteFirst = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::RdmaWriteFirst as u8,
    RdmaWriteMiddle = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::RdmaWriteMiddle as u8,
    RdmaWriteLast = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::RdmaWriteLast as u8,
    RdmaWriteLastWithImmediate = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::RdmaWriteLastWithImmediate as u8,
    RdmaWriteOnly = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::RdmaWriteOnly as u8,
    RdmaWriteOnlyWithImmediate = IbvOpcodeTransportTypes::Uc as u8 | IbvOpcodeOperations::RdmaWriteOnlyWithImmediate as u8,
}

#[repr(u8)]
pub enum IbvOpcodeRd {
    SendFirst = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::SendFirst as u8,
    SendMiddle = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::SendMiddle as u8,
    SendLast = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::SendLast as u8,
    SendLastWithImmediate = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::SendLastWithImmediate as u8,
    SendOnly = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::SendOnly as u8,
    SendOnlyWithImmediate = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::SendOnlyWithImmediate as u8,
    RdmaWriteFirst = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaWriteFirst as u8,
    RdmaWriteMiddle = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaWriteMiddle as u8,
    RdmaWriteLast = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaWriteLast as u8,
    RdmaWriteLastWithImmediate = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaWriteLastWithImmediate as u8,
    RdmaWriteOnly = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaWriteOnly as u8,
    RdmaWriteOnlyWithImmediate = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaWriteOnlyWithImmediate as u8,
    RdmaReadRequest = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaReadRequest as u8,
    RdmaReadResponseFirst = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaReadResponseFirst as u8,
    RdmaReadResponseMiddle = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaReadResponseMiddle as u8,
    RdmaReadResponseLast = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaReadResponseLast as u8,
    RdmaReadResponseOnly = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::RdmaReadResponseOnly as u8,
    Acknowledge = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::Acknowledge as u8,
    AtomicAcknowledge = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::AtomicAcknowledge as u8,
    CompareSwap = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::CompareSwap as u8,
    FetchAdd = IbvOpcodeTransportTypes::Rd as u8 | IbvOpcodeOperations::FetchAdd as u8,
}

#[repr(u8)]
pub enum IbvOpcodeUd {
    SendOnly = IbvOpcodeTransportTypes::Ud as u8 | IbvOpcodeOperations::SendOnly as u8,
    SendOnlyWithImmediate = IbvOpcodeTransportTypes::Ud as u8 | IbvOpcodeOperations::SendOnlyWithImmediate as u8,
}

impl IbvOpcodeRc {
    pub fn from_u8(value: u8) -> Option<IbvOpcodeRc> {
        match value {
            0x00 => Some(IbvOpcodeRc::SendFirst),
            0x01 => Some(IbvOpcodeRc::SendMiddle),
            0x02 => Some(IbvOpcodeRc::SendLast),
            0x03 => Some(IbvOpcodeRc::SendLastWithImmediate),
            0x04 => Some(IbvOpcodeRc::SendOnly),
            0x05 => Some(IbvOpcodeRc::SendOnlyWithImmediate),
            0x06 => Some(IbvOpcodeRc::RdmaWriteFirst),
            0x07 => Some(IbvOpcodeRc::RdmaWriteMiddle),
            0x08 => Some(IbvOpcodeRc::RdmaWriteLast),
            0x09 => Some(IbvOpcodeRc::RdmaWriteLastWithImmediate),
            0x0a => Some(IbvOpcodeRc::RdmaWriteOnly),
            0x0b => Some(IbvOpcodeRc::RdmaWriteOnlyWithImmediate),
            0x0c => Some(IbvOpcodeRc::RdmaReadRequest),
            0x0d => Some(IbvOpcodeRc::RdmaReadResponseFirst),
            0x0e => Some(IbvOpcodeRc::RdmaReadResponseMiddle),
            0x0f => Some(IbvOpcodeRc::RdmaReadResponseLast),
            0x10 => Some(IbvOpcodeRc::RdmaReadResponseOnly),
            0x11 => Some(IbvOpcodeRc::Acknowledge),
            0x12 => Some(IbvOpcodeRc::AtomicAcknowledge),
            0x13 => Some(IbvOpcodeRc::CompareSwap),
            0x14 => Some(IbvOpcodeRc::FetchAdd),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String{
        match self{
            Self::SendFirst => "send_first".to_string(),
            Self::SendMiddle => "send_middle".to_string(),
            Self::SendLast => "send_last".to_string(),
            Self::SendLastWithImmediate => "send_last_with_immediate".to_string(),
            Self::SendOnly => "send_only".to_string(),
            Self::SendOnlyWithImmediate => "send_only_with_immediate".to_string(),
            Self::RdmaWriteFirst => "rdma_write_first".to_string(),
            Self::RdmaWriteMiddle => "rdma_write_middle".to_string(),
            Self::RdmaWriteLast => "rdma_write_last".to_string(),
            Self::RdmaWriteLastWithImmediate => "rdma_write_last_with_immediate".to_string(),
            Self::RdmaWriteOnly => "rdma_write_only".to_string(),
            Self::RdmaWriteOnlyWithImmediate => "rdma_write_only_with_immediate".to_string(),
            Self::RdmaReadRequest => "rdma_read_request".to_string(),
            Self::RdmaReadResponseFirst => "rdma_read_response_first".to_string(),
            Self::RdmaReadResponseMiddle => "rdma_read_response_middle".to_string(),
            Self::RdmaReadResponseLast => "rdma_read_response_last".to_string(),
            Self::RdmaReadResponseOnly => "rdma_read_response_only".to_string(),
            Self::Acknowledge => "acknowledge".to_string(),
            Self::AtomicAcknowledge => "atomic_acknowledge".to_string(),
            Self::CompareSwap => "compare_swap".to_string(),
            Self::FetchAdd => "fetch_add".to_string(),
        }
    }
}

impl IbvOpcodeUc {
    pub fn from_u8(value: u8) -> Option<IbvOpcodeUc> {
        match value {
            0x00 => Some(IbvOpcodeUc::SendFirst),
            0x01 => Some(IbvOpcodeUc::SendMiddle),
            0x02 => Some(IbvOpcodeUc::SendLast),
            0x03 => Some(IbvOpcodeUc::SendLastWithImmediate),
            0x04 => Some(IbvOpcodeUc::SendOnly),
            0x05 => Some(IbvOpcodeUc::SendOnlyWithImmediate),
            0x06 => Some(IbvOpcodeUc::RdmaWriteFirst),
            0x07 => Some(IbvOpcodeUc::RdmaWriteMiddle),
            0x08 => Some(IbvOpcodeUc::RdmaWriteLast),
            0x09 => Some(IbvOpcodeUc::RdmaWriteLastWithImmediate),
            0x0a => Some(IbvOpcodeUc::RdmaWriteOnly),
            0x0b => Some(IbvOpcodeUc::RdmaWriteOnlyWithImmediate),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String{
        match self{
            Self::SendFirst => "send_first".to_string(),
            Self::SendMiddle => "send_middle".to_string(),
            Self::SendLast => "send_last".to_string(),
            Self::SendLastWithImmediate => "send_last_with_immediate".to_string(),
            Self::SendOnly => "send_only".to_string(),
            Self::SendOnlyWithImmediate => "send_only_with_immediate".to_string(),
            Self::RdmaWriteFirst => "rdma_write_first".to_string(),
            Self::RdmaWriteMiddle => "rdma_write_middle".to_string(),
            Self::RdmaWriteLast => "rdma_write_last".to_string(),
            Self::RdmaWriteLastWithImmediate => "rdma_write_last_with_immediate".to_string(),
            Self::RdmaWriteOnly => "rdma_write_only".to_string(),
            Self::RdmaWriteOnlyWithImmediate => "rdma_write_only_with_immediate".to_string(),
        }
    }
}

impl IbvOpcodeRd {
    pub fn from_u8(value: u8) -> Option<IbvOpcodeRd> {
        match value {
            0x00 => Some(IbvOpcodeRd::SendFirst),
            0x01 => Some(IbvOpcodeRd::SendMiddle),
            0x02 => Some(IbvOpcodeRd::SendLast),
            0x03 => Some(IbvOpcodeRd::SendLastWithImmediate),
            0x04 => Some(IbvOpcodeRd::SendOnly),
            0x05 => Some(IbvOpcodeRd::SendOnlyWithImmediate),
            0x06 => Some(IbvOpcodeRd::RdmaWriteFirst),
            0x07 => Some(IbvOpcodeRd::RdmaWriteMiddle),
            0x08 => Some(IbvOpcodeRd::RdmaWriteLast),
            0x09 => Some(IbvOpcodeRd::RdmaWriteLastWithImmediate),
            0x0a => Some(IbvOpcodeRd::RdmaWriteOnly),
            0x0b => Some(IbvOpcodeRd::RdmaWriteOnlyWithImmediate),
            0x0c => Some(IbvOpcodeRd::RdmaReadRequest),
            0x0d => Some(IbvOpcodeRd::RdmaReadResponseFirst),
            0x0e => Some(IbvOpcodeRd::RdmaReadResponseMiddle),
            0x0f => Some(IbvOpcodeRd::RdmaReadResponseLast),
            0x10 => Some(IbvOpcodeRd::RdmaReadResponseOnly),
            0x11 => Some(IbvOpcodeRd::Acknowledge),
            0x12 => Some(IbvOpcodeRd::AtomicAcknowledge),
            0x13 => Some(IbvOpcodeRd::CompareSwap),
            0x14 => Some(IbvOpcodeRd::FetchAdd),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String{
        match self{
            Self::SendFirst => "send_first".to_string(),
            Self::SendMiddle => "send_middle".to_string(),
            Self::SendLast => "send_last".to_string(),
            Self::SendLastWithImmediate => "send_last_with_immediate".to_string(),
            Self::SendOnly => "send_only".to_string(),
            Self::SendOnlyWithImmediate => "send_only_with_immediate".to_string(),
            Self::RdmaWriteFirst => "rdma_write_first".to_string(),
            Self::RdmaWriteMiddle => "rdma_write_middle".to_string(),
            Self::RdmaWriteLast => "rdma_write_last".to_string(),
            Self::RdmaWriteLastWithImmediate => "rdma_write_last_with_immediate".to_string(),
            Self::RdmaWriteOnly => "rdma_write_only".to_string(),
            Self::RdmaWriteOnlyWithImmediate => "rdma_write_only_with_immediate".to_string(),
            Self::RdmaReadRequest => "rdma_read_request".to_string(),
            Self::RdmaReadResponseFirst => "rdma_read_response_first".to_string(),
            Self::RdmaReadResponseMiddle => "rdma_read_response_middle".to_string(),
            Self::RdmaReadResponseLast => "rdma_read_response_last".to_string(),
            Self::RdmaReadResponseOnly => "rdma_read_response_only".to_string(),
            Self::Acknowledge => "acknowledge".to_string(),
            Self::AtomicAcknowledge => "atomic_acknowledge".to_string(),
            Self::CompareSwap => "compare_swap".to_string(),
            Self::FetchAdd => "fetch_add".to_string(),
        }
    }
}

impl IbvOpcodeUd {
    pub fn from_u8(value: u8) -> Option<IbvOpcodeUd> {
        match value {
            0x00 => Some(IbvOpcodeUd::SendOnly),
            0x01 => Some(IbvOpcodeUd::SendOnlyWithImmediate),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String{
        match self{
            Self::SendOnly => "send_only".to_string(),
            Self::SendOnlyWithImmediate => "send_only_with_immediate".to_string(),
        }
    }
}

pub enum IbvOpcode {
    Rc(IbvOpcodeRc),
    Uc(IbvOpcodeUc),
    Rd(IbvOpcodeRd),
    Ud(IbvOpcodeUd),
}

impl IbvOpcode {
    pub fn from_u8(value: u8) -> Option<IbvOpcode> {
        match value & 0x60 {
            0x00 => Some(IbvOpcode::Rc(IbvOpcodeRc::from_u8(value)?)),
            0x20 => Some(IbvOpcode::Uc(IbvOpcodeUc::from_u8(value)?)),
            0x40 => Some(IbvOpcode::Rd(IbvOpcodeRd::from_u8(value)?)),
            0x60 => Some(IbvOpcode::Ud(IbvOpcodeUd::from_u8(value)?)),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String{
        match self{
            Self::Rc(_) => "rc".to_string(),
            Self::Rd(_) => "ud".to_string(),
            Self::Uc(_) => "uc".to_string(),
            Self::Ud(_) => "ud".to_string()
        }
    }
}