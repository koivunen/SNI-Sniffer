#[macro_use]
extern crate log;
extern crate pnet;
extern crate pcap_parser;
extern crate clap;
use clap::{Arg,App,ArgGroup,crate_version};

mod common;
mod tls;
mod interface;
mod pcap;

fn main() {
    let matches = App::new("SNI Sniffer")
        .version(crate_version!())
        .author("David Z. <david@dzhy.dev>")
        .about("Extract domains visited through SNI sniffing.")
        .arg(Arg::with_name("SNIFF")
             .help("Sniff on interface")
             .short("s")
             .long("sniff"))
             .arg(Arg::with_name("PCAP")
                  .help("Parse pcap file")
                  .short("p")
                  .long("pcap"))
             .group(ArgGroup::with_name("switch")
                              .args(&["SNIFF", "PCAP"])
                              .required(true))
             //.group(ArgGroup::with_name("switch")
            //        .args(&["json", "text"])
            //        .required(false))
        .arg(Arg::with_name("INPUT")
             .help("<NETWORK INTERFACE>/<PCAP FILE>")
             .required(true)
             .index(1))
        .get_matches();

    if matches.is_present("SNIFF") {
        let iface_name = matches.value_of("INPUT").unwrap();
        interface::process(&iface_name);
    } else if matches.is_present("PCAP") {
        pcap::process(&matches.value_of("INPUT").unwrap());
    }
}