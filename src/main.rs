use std::{env, fs};

use rvld::{File, InputFile, LinkerError, LinkerResult};

fn main() -> LinkerResult<()> {
    let args = env::args();
    let path = args.into_iter().nth(1).ok_or(LinkerError::WrongArgs)?;
    let contents = fs::read(&path)?;
    let file = File::new(path, contents);
    let input_file = InputFile::new(file)?;
    println!(
        "There are {} section headers.",
        input_file.elf_sections.len()
    );
    Ok(())
}
