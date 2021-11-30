// https://adventofcode.com/2021/day/1

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
    }

    println!("Hello, world!");

    Ok(())
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }
}
