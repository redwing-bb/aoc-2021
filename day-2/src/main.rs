// https://adventofcode.com/2021/day/2

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut pos = 0;
    let mut depth = 0;

    let mut aim = 0;
    let mut pos2 = 0;
    let mut depth2 = 0;

    for line in reader.lines() {
        let input = line.unwrap();
        if input.contains("forward ") { 
            let n = &input[8..].parse().unwrap();
            pos += n;
            pos2 += n;
            depth2 += aim * n;
        } else if input.contains("down ") {
            let n = &input[5..].parse().unwrap();
            depth += n;
            aim += n;
        } else if input.contains("up ") {
            let n = &input[3..].parse().unwrap();
            depth -= n;
            aim -= n;
        }
    }

    println!("{} * {} = {}", pos, depth, pos*depth);
    println!("{} * {} = {}", pos2, depth2, pos2*depth2);

    Ok(())
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }
}
