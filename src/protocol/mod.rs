use crate::protocol::response::DnsResponse;

use self::{header::DnsHeader, question::DnsQuestion, reader::PacketReader, resource::DnsResource};

mod header;
mod question;
mod reader;
mod resource;
mod response;

pub use question::QType;
pub use resource::RData;

#[derive(Debug, Clone)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: DnsQuestion,
}

impl DnsPacket {
    pub fn parse(data: &[u8]) -> Self {
        let mut reader = PacketReader::new(data);
        let header = DnsHeader::parse(&mut reader);
        let question = DnsQuestion::parse(&mut reader);
        Self { header, question }
    }

    pub fn gen_response(&self, rdata: RData) -> DnsResponse {
        let mut header = self.header;
        let question = self.question.clone();
        let resource = DnsResource::new(&self.question, rdata);
        header.answer_record_count += 1;
        header.qr = true;

        DnsResponse {
            header,
            question,
            resource,
        }
    }
}
