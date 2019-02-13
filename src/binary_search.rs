use std::path::Path;
use super::utils::read_ints;

pub fn rank(key: i32, a: &[i32]) -> Option<usize> {
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
    let whitelist = read_ints(path);
    println!("{:?}", whitelist);

    /*
    Arrays.sort(whitelist);

    while (!StdIn.isEmpty()) {
        int key = StdIn.readInt();

        if (rank(key, whitelist) < 0)

        StdOut.println(key);
    }*/
}
