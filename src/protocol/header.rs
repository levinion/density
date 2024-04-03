use super::reader::PacketReader;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResponseCode {
    NoError = 0,
    FormErr = 1,
    ServFail = 2,
    NxDomain = 3,
    NotImpl = 4,
    Refused = 5,
}

impl From<u8> for ResponseCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoError,
            1 => Self::FormErr,
            2 => Self::ServFail,
            3 => Self::NxDomain,
            4 => Self::NotImpl,
            5 => Self::Refused,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode {
    Query = 0,
    IQuery = 1,
    Status = 2,
    Update = 5,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Query,
            1 => Self::IQuery,
            2 => Self::Status,
            5 => Self::Update,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DnsHeader {
    pub id: u16, // 16bits

    // flags start, 16bits
    pub qr: bool,                   // 1bit, query/response flag
    pub opcode: OpCode,             // 4bits
    pub authoritative_answer: bool, // 1bit
    pub truncated_flag: bool,       // 1bit
    pub recursion_desired: bool,    // 1bit
    pub recursion_avaliable: bool,  // 1bit
    // 3bits, zero
    pub rcode: ResponseCode, // 4bits
    // flags end
    pub question_count: u16,          // 16bits
    pub answer_record_count: u16,     // 16bits
    pub name_server_count: u16,       // 16bits
    pub additional_record_count: u16, // 16bits
}

impl DnsHeader {
    pub fn parse(reader: &mut PacketReader) -> Self {
        let id = reader.read_u16();
        let flags = reader.read_u16();
        let qr = flags >> 15 > 0;
        let opcode = OpCode::from(((flags & 0xff) >> 11) as u8);
        let authoritative_answer = flags & (1 << 10) > 1;
        let truncated_flag = flags & (1 << 9) > 1;
        let recursion_desired = flags & (1 << 8) > 1;
        let recursion_avaliable = flags & (1 << 7) > 1;
        let rcode = ResponseCode::from((flags & 0x04) as u8);
        let question_count = reader.read_u16();
        let answer_record_count = reader.read_u16();
        let name_server_count = reader.read_u16();
        let additional_record_count = reader.read_u16();
        Self {
            id,
            qr,
            opcode,
            authoritative_answer,
            truncated_flag,
            recursion_desired,
            recursion_avaliable,
            rcode,
            question_count,
            answer_record_count,
            name_server_count,
            additional_record_count,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut v = vec![];
        v.append(&mut self.id.to_be_bytes().to_vec());
        let flags = (self.qr as u16) << 15
            | (self.opcode as u16) << 11
            | (self.authoritative_answer as u16) << 10
            | (self.truncated_flag as u16) << 9
            | (self.recursion_desired as u16) << 8
            | (self.recursion_avaliable as u16) << 7
            | (self.rcode as u16) << 3;
        v.append(&mut flags.to_be_bytes().to_vec());
        v.append(&mut self.question_count.to_be_bytes().to_vec());
        v.append(&mut self.answer_record_count.to_be_bytes().to_vec());
        v.append(&mut self.name_server_count.to_be_bytes().to_vec());
        v.append(&mut self.additional_record_count.to_be_bytes().to_vec());
        v
    }
}
