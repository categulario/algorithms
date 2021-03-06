use std::env;

use algorithms::binary_search;

fn main() {
    let mut args = env::args();
    args.next();

    binary_search::main(args.next().expect("Need a file argument"));
}
