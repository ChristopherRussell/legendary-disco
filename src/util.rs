use anyhow::Result;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn not_implemented_error(day: &str) -> anyhow::Error {
    anyhow::anyhow!(format!("{} not implemented", day))
}

pub fn get_input_file_reader(name: &str) -> Result<io::BufReader<File>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let filename = format!("input/{}.txt", name);
    path.push(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}
