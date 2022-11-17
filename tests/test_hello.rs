use rvld::{File, InputFile, LinkerResult};

#[test]
fn test_hello() -> LinkerResult<()> {
    let path = "hello.o".to_string();
    let contents = include_bytes!("./hello.o");
    let file = File::new(path, contents.to_vec());
    let input_file = InputFile::new(file)?;
    assert_eq!(12, input_file.elf_sections.len());
    Ok(())
}
