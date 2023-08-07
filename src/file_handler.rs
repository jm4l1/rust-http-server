use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, BufWriter};
use std::path::Path;

pub fn read_file(path: &Path) -> Option<Vec<u8>> {
    let f = OpenOptions::new().read(true).open(path);
    match f {
        Ok(file) => {
            let mut f = BufReader::new(file);
            let mut buffer = Vec::<u8>::new();
            match f.read_to_end(&mut buffer) {
                Ok(_) => return Some(buffer),
                Err(err) => {
                    eprintln!("Error opening {:?} - {}", path.to_str(), err);
                    return None;
                }
            };
        }
        Err(err) => {
            eprintln!("Error opening {:?} - {}", path.to_str(), err);
            return None;
        }
    }
}
