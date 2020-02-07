use serde_json;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;

struct JsonFile {
    stem: String,
    content: String,
}

fn get_dir_json(P: &Path) -> io::Result<()> {
    let entries = fs::read_dir(P)?
        .map(|res| res.map(|e| e.path()))
        .map(|path| path.map(|p| get_file_json(p).unwrap()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(())
}

fn serialize_json_file(input: Vec<JsonFile>) -> io::Result<serde_json::Value> {
    let hash: HashMap<String, serde_json::Value> = HashMap::new();
    in
     
    Ok()
} 

fn get_file_json(file: PathBuf) -> io::Result<JsonFile> {
    Ok(JsonFile {
        stem: match file.file_stem() {
            None => panic!("Could not read file stem"),
            Some(file_stem) => match file_stem.to_str() {
                None => panic!("Could not convert OsStr to String"),
                Some(file_stem_str) => file_stem_str,
            }
        }.to_string(),
        content: fs::read_to_string(file)?
    })
}

fn main() {
    get_dir_json(Path::new("/home/zexa/Projects/get-files/src/examples"))
        .expect("Could not read directory");
}
