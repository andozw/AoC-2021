use std::fs::File;
use std::io::BufReader;

pub fn get_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}
