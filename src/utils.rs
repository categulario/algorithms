use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

pub fn read_ints<P: AsRef<Path>>(path: P) -> Vec<i32> {
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        .map(|l| l.expect("Error al leer"))
        .map(|s| s.trim().parse().expect("Could not parse"))
        .collect()
}
