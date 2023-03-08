# SNI-Sniffer

Extract domains visited through SNI sniffing.

## Usage

```bash
apt-get install cargo
make # Or: cargo build --release
make install
sudo sni-sniffer --sniff eno1
```

## JSON Output
```
{
  "dest": "2.19.113.121",
  "dest_port": 443,
  "sni": [
    "ir.ebaystatic.com"
  ],
  "src": "192.168.0.139",
  "src_port": 49291,
  "ts": 1678300938.143
}
{
  "dest": "2.19.113.24",
  "dest_port": 443,
  "sni": [
    "www.ebay.com"
  ],
  "src": "192.168.0.139",
  "src_port": 35257,
  "ts": 1678300938.149
}
```

## Notes
For interface sniffing you must run the program as root.

## TO-DO
- [ ] Show Date/Time Request was made
- [ ] Choose output format
- [ ] Better formatted output

```
Date/Time Request was made,
Source Address,
Source Port,
SNI,
Remote Address,
Remote Port,
```

## Sources

This project is built based on the following sources.

- https://github.com/rusticata/rusticata/blob/master/src/tls.rs
- https://github.com/rusticata/pcap-parse/blob/master/src/main.rs
- https://github.com/libpnet/libpnet/blob/master/examples/packetdump.rs