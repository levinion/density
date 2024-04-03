use super::reader::PacketReader;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DnsQuestion {
    pub qname: String,
    pub qtype: QType,
    pub qclass: QClass,
}

impl DnsQuestion {
    pub fn parse(reader: &mut PacketReader) -> Self {
        let qname = reader.read_name();
        let qtype = QType::from(reader.read_u16());
        let qclass = QClass::from(reader.read_u16());
        Self {
            qname,
            qtype,
            qclass,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut v = vec![];
        self.qname
            .split('.')
            .map(|name| (name.len(), name.as_bytes()))
            .for_each(|(size, bytes)| {
                v.push(size as u8);
                v.append(&mut bytes.to_vec());
            });
        v.push(0);
        let typ = self.qtype as u16;
        v.append(&mut typ.to_be_bytes().to_vec());
        let class = self.qclass as u16;
        v.append(&mut class.to_be_bytes().to_vec());
        v
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,
    DS = 43,
    RRSIG = 46,
    NSEC = 47,
    DNSKEY = 48,
}

impl From<u16> for QType {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            5 => Self::CNAME,
            6 => Self::SOA,
            12 => Self::PTR,
            15 => Self::MX,
            16 => Self::TXT,
            28 => Self::AAAA,
            33 => Self::SRV,
            43 => Self::DS,
            46 => Self::RRSIG,
            47 => Self::NSEC,
            48 => Self::DNSKEY,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QClass {
    In = 1,
}

impl From<u16> for QClass {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::In,
            _ => unreachable!(),
        }
    }
}
