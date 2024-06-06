use std::{collections::HashMap, fmt::Display};
use serde::{ser::SerializeMap, Serialize, Serializer};
use serde_yaml;
use pnet::packet::{ipv4::Ipv4Packet, udp::UdpPacket};

use crate::rocev2_hdr::{BthHdrPacket, IbvOpcode, AethHdrPacket};

#[derive(Serialize)]
pub struct Stats{
    #[serde(flatten)]
    flows: Flows,
    packets: u64,
}

impl Stats{
    pub fn new() -> Stats{
        Stats{
            flows: Flows::new(),
            packets: 0,
        }
    }
    pub fn add(&mut self, ipv4_packet: &Ipv4Packet, udp_packet: &UdpPacket, bth_packet: &BthHdrPacket, aeth_packets: Option<AethHdrPacket>, length: u32){
        let src_ip = ipv4_packet.get_source().to_string();
        let dst_ip = ipv4_packet.get_destination().to_string();
        let src_port = udp_packet.get_source();
        let dst_port = udp_packet.get_destination();
        let qp = bth_packet.get_dest_qpn();
        let op = bth_packet.get_opcode();
        let flow_key = FlowKey{
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            qp,
        };
        let flow_value = self.flows.0.entry(flow_key).or_insert(FlowValue::default());
        flow_value.inc("packets".to_string(), None);
        flow_value.inc("bytes".to_string(), Some(length as u64));
        self.packets += 1;
        if let Some(op_code) = IbvOpcode::from_u8(op){
            let transport = op_code.to_string();
            match op_code{
                IbvOpcode::Rc(rc) => {
                    let operation = rc.to_string();
                    let key = format!("{}_{}", transport, operation);
                    flow_value.inc(key, None);
                    if let Some(aeth_packet) = aeth_packets{
                        let syndrom = aeth_packet.get_syndrom();

                        // get 2nd and 3rd bits from left
                        let mask = 0b01100000;
                        let ack_opcode = (syndrom & mask) >> 5;
                        if ack_opcode == 3 {
                            // get 5 bits from right
                            let mask = 0b00011111;
                            let error_code = syndrom & mask;
                            if error_code == 0 {
                                flow_value.inc("out_of_sequence".to_string(), None);
                            } else {
                                flow_value.inc("error".to_string(), None);
                            }
                        }
                    }
                },
                IbvOpcode::Uc(uc) => {
                    let operation = uc.to_string();
                    let key = format!("{}_{}", transport, operation);
                    flow_value.inc(key, None);
                },
                IbvOpcode::Ud(ud) => {
                    let operation = ud.to_string();
                    let key = format!("{}_{}", transport, operation);
                    flow_value.inc(key, None);
                },
                IbvOpcode::Rd(rd) => {
                    let operation = rd.to_string();
                    let key = format!("{}_{}", transport, operation);
                    flow_value.inc(key, None);
                }
            }
        }
    }
}

impl Display for Stats{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let yaml = serde_yaml::to_string(&self).unwrap();
        write!(f, "{}", yaml)
    }
}

#[derive(Eq, Hash, PartialEq, Serialize)]
pub struct FlowKey{
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub qp: u32,
}

impl Display for FlowKey{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} -> {}:{} qp:{}", self.src_ip, self.src_port, self.dst_ip, self.dst_port, self.qp)
    }
}

#[derive(Eq, PartialEq, Serialize, Default)]
pub struct FlowValue(HashMap<String, u64>);

impl FlowValue{
    fn inc(&mut self, key: String, value: Option<u64>){
        self.0.entry(key).and_modify(|e| {*e += value.unwrap_or(1)}).or_default();
    }
}

#[derive(Eq, PartialEq)]
pub struct Flows(HashMap<FlowKey,FlowValue>);

impl Flows{
    pub fn new() -> Flows{
        Flows(HashMap::new())
    }
}

impl Serialize for Flows{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }

}