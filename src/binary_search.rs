use std::path::Path;
use super::utils::read_ints;
use std::io::{self, BufRead};

pub fn binary_search(key: i32, a: &[i32]) -> Option<usize> {
    let mut lo = 0;
    let mut hi = a.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if key < a[mid] {
            hi = mid - 1;
        } else if key > a[mid] {
            lo = mid + 1;
        } else {
            return Some(mid);
        }
    }

    None
}

pub fn main<P: AsRef<Path>>(path: P) {
    let mut whitelist = read_ints(path);

    whitelist.sort_unstable();

    let stdin = io::stdin();
    let iter = stdin
        .lock()
        .lines()
        .map(|l| l.expect("Could not read line"))
        .map(|s| s.trim().parse::<i32>().expect("Invalid number"));

    for key in iter {
        if let None = binary_searcu(key, &whitelist) {
            println!("{}", key);
        }
    }
}
