pub struct PacketReader {
    data: Vec<u8>,
    pos: usize,
}

impl PacketReader {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            pos: 0,
        }
    }

    pub fn read(&mut self) -> u8 {
        let byte = self.data.get(self.pos).unwrap();
        self.pos += 1;
        *byte
    }

    pub fn read_u16(&mut self) -> u16 {
        let head = self.read();
        let tail = self.read();
        (head as u16) << 8 | tail as u16
    }

    pub fn read_u32(&mut self) -> u32 {
        let head = self.read_u16();
        let tail = self.read_u16();
        (head as u32) << 16 | tail as u32
    }

    pub fn read_name(&mut self) -> String {
        let mut s = String::new();
        loop {
            let length = self.read();
            if length == 0 {
                let _ = s.pop();
                break;
            }
            for _ in 0..length {
                s.push(self.read() as char);
            }
            s.push('.');
        }
        s
    }
}
