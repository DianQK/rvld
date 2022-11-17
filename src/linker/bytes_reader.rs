pub struct BytesReader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> BytesReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(
            self.bytes[self.offset..(self.offset + 2)]
                .try_into()
                .unwrap(),
        );
        self.offset += 2;
        value
    }

    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes(
            self.bytes[self.offset..(self.offset + 4)]
                .try_into()
                .unwrap(),
        );
        self.offset += 4;
        value
    }

    pub fn read_u64(&mut self) -> u64 {
        let value = u64::from_le_bytes(
            self.bytes[self.offset..(self.offset + 8)]
                .try_into()
                .unwrap(),
        );
        self.offset += 8;
        value
    }
}
