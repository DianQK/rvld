use std::mem::size_of;

use crate::{linker::bytes_reader::BytesReader, LinkerError, LinkerResult};

// https://codebrowser.dev/glibc/glibc/elf/elf.h.html
/// Index is in extra table.
pub const SHN_XINDEX: u16 = 0xffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionType(pub u32);

// https://codebrowser.dev/glibc/glibc/elf/elf.h.html#427
/// Legal values for sh_type (section type).
impl SectionType {
    pub const SHT_NULL: SectionType = SectionType(0); /* Section header table entry unused */
    pub const SHT_PROGBITS: SectionType = SectionType(1); /* Program data */
    pub const SHT_SYMTAB: SectionType = SectionType(2); /* Symbol table */
    pub const SHT_STRTAB: SectionType = SectionType(3); /* String table */
    pub const SHT_RELA: SectionType = SectionType(4); /* Relocation entries with addends */
    pub const SHT_HASH: SectionType = SectionType(5); /* Symbol hash table */
    pub const SHT_DYNAMIC: SectionType = SectionType(6); /* Dynamic linking information */
    pub const SHT_NOTE: SectionType = SectionType(7); /* Notes */
    pub const SHT_NOBITS: SectionType = SectionType(8); /* Program space with no data (bss) */
    pub const SHT_REL: SectionType = SectionType(9); /* Relocation entries, no addends */
    pub const SHT_SHLIB: SectionType = SectionType(10); /* Reserved */
    pub const SHT_DYNSYM: SectionType = SectionType(11); /* Dynamic linker symbol table */
    pub const SHT_INIT_ARRAY: SectionType = SectionType(14); /* Array of constructors */
    pub const SHT_FINI_ARRAY: SectionType = SectionType(15); /* Array of destructors */
    pub const SHT_PREINIT_ARRAY: SectionType = SectionType(16); /* Array of pre-constructors */
    pub const SHT_GROUP: SectionType = SectionType(17); /* Section group */
    pub const SHT_SYMTAB_SHNDX: SectionType = SectionType(18); /* Extended section indices */
    pub const SHT_NUM: SectionType = SectionType(19); /* Number of defined types.  */
    pub const SHT_LOOS: SectionType = SectionType(0x60000000); /* Start OS-specific.  */
    pub const SHT_GNU_ATTRIBUTES: SectionType = SectionType(0x6ffffff5); /* Object attributes.  */
    pub const SHT_GNU_HASH: SectionType = SectionType(0x6ffffff6); /* GNU-style hash table.  */
    pub const SHT_GNU_LIBLIST: SectionType = SectionType(0x6ffffff7); /* Prelink library list */
    pub const SHT_CHECKSUM: SectionType = SectionType(0x6ffffff8); /* Checksum for DSO content.  */
    pub const SHT_LOSUNW: SectionType = SectionType(0x6ffffffa); /* Sun-specific low bound.  */
    pub const SHT_SUNW_MOVE: SectionType = SectionType(0x6ffffffa);
    pub const SHT_SUNW_COMDAT: SectionType = SectionType(0x6ffffffb);
    pub const SHT_SUNW_SYMINFO: SectionType = SectionType(0x6ffffffc);
    pub const SHT_GNU_VERDEF: SectionType = SectionType(0x6ffffffd); /* Version definition section.  */
    pub const SHT_GNU_VERNEED: SectionType = SectionType(0x6ffffffe); /* Version needs section.  */
    pub const SHT_GNU_VERSYM: SectionType = SectionType(0x6fffffff); /* Version symbol table.  */
    pub const SHT_HISUNW: SectionType = SectionType(0x6fffffff); /* Sun-specific high bound.  */
    pub const SHT_HIOS: SectionType = SectionType(0x6fffffff); /* End OS-specific type */
    pub const SHT_LOPROC: SectionType = SectionType(0x70000000); /* Start of processor-specific */
    pub const SHT_HIPROC: SectionType = SectionType(0x7fffffff); /* End of processor-specific */
    pub const SHT_LOUSER: SectionType = SectionType(0x80000000); /* Start of application-specific */
    pub const SHT_HIUSER: SectionType = SectionType(0x8fffffff); /* End of application-specific */
}

#[derive(Debug, Clone)]
#[repr(C)]
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

impl Ehdr {
    pub fn index_is_in_extra_table(&self) -> bool {
        self.sh_strndx == SHN_XINDEX
    }
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

/// Section header
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Shdr {
    pub name: u32,
    pub r#type: SectionType,
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
            r#type: SectionType(reader.read_u32()),
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

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Sym {
    pub name: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub val: u16,
    pub size: u64,
}

impl Sym {
    const SIZE: usize = size_of::<Sym>();

    pub fn read_syms(mut bytes: &[u8]) -> LinkerResult<Vec<Sym>> {
        let nums = bytes.len() / Sym::SIZE;
        let mut elf_syms = Vec::<Sym>::with_capacity(nums);
        for _ in 0..nums {
            let sym = Sym::try_from(bytes)?;
            elf_syms.push(sym);
            bytes = &bytes[Sym::SIZE..];
        }
        Ok(elf_syms)
    }
}

impl TryFrom<&[u8]> for Sym {
    type Error = LinkerError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BytesReader::new(value);
        Ok(Self {
            name: reader.read_u32(),
            info: reader.read_u8(),
            other: reader.read_u8(),
            shndx: reader.read_u16(),
            val: reader.read_u16(),
            size: reader.read_u64(),
        })
    }
}
