use std::mem::size_of;

use crate::{Ehdr, File, LinkerError, LinkerResult, Shdr};

pub struct InputFile {
    pub file: File,
    pub elf_sections: Vec<Shdr>,
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
        let num_sections = if ehdr.sh_num != 0 {
            ehdr.sh_num as u64
        } else {
            shdr.size
        };
        let mut elf_sections = Vec::<Shdr>::with_capacity(num_sections as usize);
        elf_sections.push(shdr);
        for _ in 1..num_sections {
            contents = &contents[size_of::<Shdr>()..];
            let shdr = Shdr::try_from(contents)?;
            elf_sections.push(shdr);
        }
        Ok(Self { file, elf_sections })
    }
}
