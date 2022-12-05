use std::ops::Deref;

use crate::{File, InputFile, LinkerResult, Sym};

use super::elf::SectionType;

pub struct ObjectFile {
    input_file: InputFile,
    /// position of symtab section header
    pub symtab_sec_position: usize,
}

impl ObjectFile {
    pub fn new(file: File) -> LinkerResult<Self> {
        Self::parse(InputFile::new(file)?)
    }

    fn parse(mut input_file: InputFile) -> LinkerResult<ObjectFile> {
        let symtab_sec_position = input_file
            .find_section_position(SectionType::SHT_SYMTAB)
            .unwrap();
        let symtab_sec = &input_file.elf_sections[symtab_sec_position];
        input_file.first_global = symtab_sec.info;
        input_file.elf_syms = Sym::read_syms(input_file.get_bytes_from_shdr(symtab_sec))?;
        input_file.symbol_strtab = input_file.get_string_table_from_idx(symtab_sec.link as usize);
        Ok(ObjectFile {
            input_file,
            symtab_sec_position,
        })
    }
}

impl Deref for ObjectFile {
    type Target = InputFile;

    fn deref(&self) -> &Self::Target {
        &self.input_file
    }
}
