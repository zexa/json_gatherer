use serde_json;
use std::fs;
use std::io;
use std::path::PathBuf;

fn get_dir_json() -> io::Result<()> {
    let entries = fs::read_dir("./examples")? // ReadDir iterator, which yields DirEntry
        .map(|res| res.map(|e| e.path())) // yields PathBuf
        .map(|path| path.map(|p| get_file_json(p).unwrap()))
        .map(|| 
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in entries {
        println!("{:?}", entry);
    }

    Ok(())
}

fn convert_to_hashmap()

fn get_file_json(file: PathBuf) -> io::Result<(String, serde_json::Value)> {
    let stem = match file.file_stem() {
        None => panic!("Could not read file stem"),
        Some(file_stem) => match file_stem.to_str() {
            None => panic!("Could not convert OsStr to String"),
            Some(file_stem_str) => file_stem_str,
        },
    }
    .to_string();
    let json: serde_json::Value = serde_json::from_str(&fs::read_to_string(file)?)?;

    Ok((stem, json))
}

fn main() {
    get_dir_json();
}
