mod protocol;

use crate::protocol::DnsPacket;
use crate::protocol::QType;
use crate::protocol::RData;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:53").await.unwrap();
    loop {
        let mut buf = [0; 1024];
        let (n, addr) = socket.recv_from(&mut buf).await.unwrap();
        let packet = DnsPacket::parse(&buf[..n]);
        println!("received: {:?}", &packet);
        let res = match packet.question.qtype {
            QType::A => packet.gen_response(RData::A(Ipv4Addr::LOCALHOST)),
            QType::AAAA => packet.gen_response(RData::AAAA(Ipv6Addr::LOCALHOST)),
            _ => continue,
        };
        println!("send: {:?}", &res);
        socket.send_to(&res.into_bytes(), addr).await.unwrap();
    }
}
