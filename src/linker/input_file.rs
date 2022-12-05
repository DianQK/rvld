use std::mem::size_of;

use crate::{Ehdr, File, LinkerError, LinkerResult, Shdr, Sym};

use super::elf::SectionType;

pub struct StringTable(Vec<u8>);

impl StringTable {
    pub fn get_str(&self, offset: usize) -> &str {
        let bytes = &self.0[offset..];
        let nul_position = bytes.iter().position(|&c| c == b'\0').unwrap();
        let bytes = &bytes[0..nul_position];
        std::str::from_utf8(bytes).unwrap()
    }
}

pub struct InputFile {
    file: File,
    pub elf_sections: Vec<Shdr>,
    pub elf_syms: Vec<Sym>,
    /// first global symbol index
    pub first_global: u32,
    /// section header string table
    pub sh_strtab: StringTable,
    // symbol string table
    pub symbol_strtab: StringTable,
}

const MAGIC: &[u8; 4] = b"\x7fELF";

impl InputFile {
    pub fn new(file: File) -> LinkerResult<Self> {
        let magic = &file.contents[0..4];
        if magic != MAGIC {
            return Err(LinkerError::NotELF);
        }
        let mut contents = file.contents.as_slice();
        let ehdr = Ehdr::try_from(contents)?;
        contents = &contents[ehdr.sh_off as usize..];
        let shdr = Shdr::try_from(contents)?;
        /* https://manpages.debian.org/stretch/manpages/elf.5.en.html
          If the number of entries in the section header table is larger than or equal to SHN_LORESERVE (0xff00),
          e_shnum holds the value zero and the real number of entries in the section header table is held in the sh_size member of the initial entry in section header table.
          Otherwise, the sh_size member of the initial entry in the section header table holds the value zero.
        */
        let num_sections = match ehdr.sh_num {
            0 => shdr.size,
            _ => ehdr.sh_num as u64,
        };
        let sh_strndx = if ehdr.index_is_in_extra_table() {
            shdr.link as usize
        } else {
            ehdr.sh_strndx as usize
        };
        let mut elf_sections = Vec::<Shdr>::with_capacity(num_sections as usize);
        elf_sections.push(shdr);
        for _ in 1..num_sections {
            contents = &contents[size_of::<Shdr>()..];
            let shdr = Shdr::try_from(contents)?;
            elf_sections.push(shdr);
        }

        let mut input_file = Self {
            file,
            elf_sections,
            elf_syms: Vec::new(),
            first_global: 0,
            sh_strtab: StringTable(Vec::new()),
            symbol_strtab: StringTable(Vec::new()),
        };
        input_file.sh_strtab = input_file.get_string_table_from_idx(sh_strndx);
        Ok(input_file)
    }

    pub fn get_bytes_from_shdr(&self, shdr: &Shdr) -> &[u8] {
        let end = shdr.offset + shdr.size;
        &self.file.contents[shdr.offset as usize..end as usize]
    }

    pub fn get_bytes_from_idx(&self, idx: usize) -> &[u8] {
        self.get_bytes_from_shdr(&self.elf_sections[idx])
    }

    pub fn get_string_table_from_idx(&self, idx: usize) -> StringTable {
        StringTable(self.get_bytes_from_idx(idx).to_vec())
    }

    pub fn find_section(&self, ty: SectionType) -> Option<&Shdr> {
        self.elf_sections.iter().find(|&shdr| shdr.r#type == ty)
    }

    pub fn find_section_position(&self, ty: SectionType) -> Option<usize> {
        self.elf_sections.iter().position(|shdr| shdr.r#type == ty)
    }
}
