use clap::Parser;
use pcap_parser::*;
use pcap_parser::traits::PcapReaderIterator;
use std::fs::File;
use pnet::packet::{ethernet, ipv4, udp, Packet};
use rocev2_hdr::{AethHdrPacket, BthHdrPacket};
use stats::Stats;

pub mod rocev2_hdr;
pub mod stats;

#[derive(Parser)]
struct Args{
    #[clap(short, long)]
    file: String
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.file).unwrap();
    let mut stats = Stats::new();
    let mut reader = LegacyPcapReader::new(65536, file).expect("PcapNGReader");
    loop {
        match reader.next() {
            Ok((offset, block)) => {
                match block {
                    PcapBlockOwned::LegacyHeader(_) => (),
                    PcapBlockOwned::Legacy(b) => {
                        if let Some(eth_hdr) = ethernet::EthernetPacket::new(b.data){
                            if eth_hdr.get_ethertype() ==  ethernet::EtherTypes::Ipv4{
                                let payload = eth_hdr.payload();
                                if let Some(ipv4_hdr) = ipv4::Ipv4Packet::new(payload){
                                    let payload = ipv4_hdr.payload();
                                    if let Some(udp_hdr) = udp::UdpPacket::new(payload){
                                        if udp_hdr.get_destination() == 4791{
                                            let payload = udp_hdr.payload();
                                            if let Some(bth_hdr) = BthHdrPacket::new(payload){
                                                let aeth_packet = if bth_hdr.get_opcode() == 17{
                                                    let payload = bth_hdr.payload();
                                                    let a = AethHdrPacket::new(payload).unwrap();
                                                    Some(a)
                                                }else{
                                                    None
                                                };
                                                stats.add(&ipv4_hdr, &udp_hdr, &bth_hdr, aeth_packet, b.origlen);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    PcapBlockOwned::NG(_) => panic!("unexpected NG data"),
                }
                reader.consume(offset);
            },
            Err(PcapError::Eof) => break,
            Err(PcapError::Incomplete(_)) => {
                reader.refill().unwrap();
            },
            Err(e) => panic!("error while reading: {:?}", e),
        }
    }
    println!("{}", stats);
}
