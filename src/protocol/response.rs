use super::{
    header::DnsHeader, question::DnsQuestion, reader::PacketReader, resource::DnsResource,
};

#[derive(Debug, Clone)]
pub struct DnsResponse {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    pub resource: DnsResource,
}

impl DnsResponse {
    #[allow(unused)]
    pub fn parse(data: &[u8]) -> Self {
        let mut reader = PacketReader::new(data);
        let header = DnsHeader::parse(&mut reader);
        let question = DnsQuestion::parse(&mut reader);
        let resource = DnsResource::parse(&mut reader);
        Self {
            header,
            question,
            resource,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut v = vec![];
        v.append(&mut self.header.into_bytes());
        v.append(&mut self.question.into_bytes());
        v.append(&mut self.resource.into_bytes());
        v
    }
}

#[cfg(test)]
mod test {

    use super::DnsResponse;

    #[test]
    fn test_parse_response() {
        let packet = [
            146, 4, 129, 0, 0, 1, 0, 1, 0, 0, 0, 0, 17, 114, 114, 53, 45, 45, 45, 115, 110, 45,
            111, 48, 57, 55, 122, 110, 115, 100, 11, 103, 111, 111, 103, 108, 101, 118, 105, 100,
            101, 111, 3, 99, 111, 109, 0, 0, 1, 0, 1, 17, 114, 114, 53, 45, 45, 45, 115, 110, 45,
            111, 48, 57, 55, 122, 110, 115, 100, 11, 103, 111, 111, 103, 108, 101, 118, 105, 100,
            101, 111, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0, 0, 2, 88, 0, 4, 127, 0, 0, 1,
        ];
        DnsResponse::parse(&packet);
    }
}
