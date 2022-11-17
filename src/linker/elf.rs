use crate::{linker::bytes_reader::BytesReader, LinkerError};

#[derive(Debug)]
pub struct Ehdr {
    pub ident: [u8; 16],
    pub r#type: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u64,
    pub ph_off: u64,
    pub sh_off: u64,
    pub flags: u32,
    pub eh_size: u16,
    pub ph_ent_size: u16,
    pub ph_num: u16,
    pub sh_ent_size: u16,
    pub sh_num: u16,
    pub sh_strndx: u16,
}

impl TryFrom<&[u8]> for Ehdr {
    type Error = LinkerError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let ident: [u8; 16] = value[0..16].try_into()?;
        let mut reader = BytesReader::new(&value[16..]);
        Ok(Self {
            ident,
            r#type: reader.read_u16(),
            machine: reader.read_u16(),
            version: reader.read_u32(),
            entry: reader.read_u64(),
            ph_off: reader.read_u64(),
            sh_off: reader.read_u64(),
            flags: reader.read_u32(),
            eh_size: reader.read_u16(),
            ph_ent_size: reader.read_u16(),
            ph_num: reader.read_u16(),
            sh_ent_size: reader.read_u16(),
            sh_num: reader.read_u16(),
            sh_strndx: reader.read_u16(),
        })
    }
}

#[derive(Debug)]
pub struct Shdr {
    pub name: u32,
    pub r#type: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addr_align: u64,
    pub ent_size: u64,
}

impl TryFrom<&[u8]> for Shdr {
    type Error = LinkerError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BytesReader::new(value);
        Ok(Self {
            name: reader.read_u32(),
            r#type: reader.read_u32(),
            flags: reader.read_u64(),
            addr: reader.read_u64(),
            offset: reader.read_u64(),
            size: reader.read_u64(),
            link: reader.read_u32(),
            info: reader.read_u32(),
            addr_align: reader.read_u64(),
            ent_size: reader.read_u64(),
        })
    }
}
