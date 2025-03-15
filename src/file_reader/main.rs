use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};


fn main() -> std::io::Result<()> {
  // Criar um arquivo em .txt
  let file_path = "output.exe";
  let file = File::create(file_path)?;

  let mut writer = BufWriter::new(file);

  writer.write_all(b"Hello, World!\n")?;

  writer.flush()?;

  // Ler seu conteúdo e exibir

  let contents = fs::read_to_string(file_path)
  .expect("File isn't reading properly.");

  println!("Content is:\n {contents}");

  Ok(())
}