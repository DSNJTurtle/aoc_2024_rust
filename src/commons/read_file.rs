use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_to_vec_str(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(lines)
}
