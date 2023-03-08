use std::net::IpAddr;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use crate::tls::{self,TlsParser,STREAM_TOSERVER};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

/// Print SNI if exists
pub fn parse_tls(src: IpAddr, dst: IpAddr, tcp: &TcpPacket) {
    let payload = tcp.payload();
    if payload.len() == 0 { return; }

    if tls::tls_probe(&payload) {
        let mut parser = TlsParser::new(&payload);

        parser.parse(&payload, STREAM_TOSERVER);
        if parser.sni.len() > 0 {
            let print_json= true; // TODO
            if print_json {

            let obj = json!({
                "src":src,
                "src_port":tcp.get_source(),
                "dest_port": tcp.get_destination(),
                "dest":dst,
                "sni":parser.sni,
                "ts": SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64 / 1000.0
            });
            println!("{}", serde_json::to_string_pretty(&obj).unwrap());
            
            } else {
                println!("TCP {:?}:{} -> {:?}:{}",
                    src, tcp.get_source(),
                    dst, tcp.get_destination());
                println!("SNI: {:#?}", parser.sni);

            }
            
        }
    }
}