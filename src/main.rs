use rvld::{File, LinkerError, LinkerResult, ObjectFile};

fn main() -> LinkerResult<()> {
    let args = std::env::args();
    let path = args.into_iter().nth(1).ok_or(LinkerError::WrongArgs)?;
    let contents = std::fs::read(&path)?;
    let file = File::new(path, contents);
    let object_file = ObjectFile::new(file)?;
    println!(
        "There are {} section headers.",
        object_file.elf_sections.len()
    );
    println!("first global: {}", object_file.first_global);
    println!("sym len: {}", object_file.elf_syms.len());
    for sym in &object_file.elf_syms {
        println!("{}", object_file.symbol_strtab.get_str(sym.name as usize));
    }
    Ok(())
}
