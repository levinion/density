use std::net::{Ipv4Addr, Ipv6Addr};

use super::{
    question::{DnsQuestion, QClass, QType},
    reader::PacketReader,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DnsResource {
    name: String,
    typ: QType,
    class: QClass,
    ttl: u32,
    rd_length: u16,
    rdata: RData,
}

impl DnsResource {
    #[allow(unused)]
    pub fn parse(reader: &mut PacketReader) -> Self {
        let name = reader.read_name();
        let typ = QType::from(reader.read_u16());
        let class = QClass::from(reader.read_u16());
        let ttl = reader.read_u32();
        let rd_length = reader.read_u16();
        let rdata = RData::parse(typ, reader);
        Self {
            name,
            typ,
            class,
            ttl,
            rd_length,
            rdata,
        }
    }

    pub fn new(question: &DnsQuestion, rdata: RData) -> Self {
        let rd_length = match rdata {
            RData::A(_) => 4,
            RData::AAAA(_) => 16,
        };
        Self {
            name: question.qname.clone(),
            typ: question.qtype,
            class: question.qclass,
            ttl: 600,
            rd_length,
            rdata,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut v = vec![];
        self.name
            .split('.')
            .map(|name| (name.len(), name.as_bytes()))
            .for_each(|(size, bytes)| {
                v.push(size as u8);
                v.append(&mut bytes.to_vec());
            });
        v.push(0);
        let typ = self.typ as u16;
        v.append(&mut typ.to_be_bytes().to_vec());
        let class = self.class as u16;
        v.append(&mut class.to_be_bytes().to_vec());
        v.append(&mut self.ttl.to_be_bytes().to_vec());
        v.append(&mut self.rd_length.to_be_bytes().to_vec());
        v.append(&mut self.rdata.into_bytes());
        v
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum RData {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
}

impl RData {
    fn parse(typ: QType, reader: &mut PacketReader) -> Self {
        match typ {
            QType::A => {
                let a = reader.read();
                let b = reader.read();
                let c = reader.read();
                let d = reader.read();
                let addr = Ipv4Addr::new(a, b, c, d);
                Self::A(addr)
            }
            QType::AAAA => {
                let a = reader.read_u16();
                let b = reader.read_u16();
                let c = reader.read_u16();
                let d = reader.read_u16();
                let e = reader.read_u16();
                let f = reader.read_u16();
                let g = reader.read_u16();
                let h = reader.read_u16();
                let addr = Ipv6Addr::new(a, b, c, d, e, f, g, h);
                Self::AAAA(addr)
            }
            _ => unimplemented!(),
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        match self {
            RData::A(addr) => addr.octets().to_vec(),
            RData::AAAA(addr) => addr.octets().to_vec(),
        }
    }
}
