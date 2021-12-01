// https://adventofcode.com/2021/day/1

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut r0: u32;
    let mut r1: u32 = 0;
    let mut count: u32 = 0;

    let mut s0: u32;
    let mut s1: u32 = 0;
    let mut s2: u32 = 0;
    let mut w0: u32;
    let mut w1: u32 = 0;
    let mut count2: u32 = 0;

    for line in reader.lines() {

        r0 = r1;
        r1 = line.unwrap().parse().unwrap();

        if r0 != 0 {
            if r1 > r0 { count +=1; }
        }

        s0 = s1; 
        s1 = s2;
        s2 = r1;

        if s0 != 0 {
            w0 = w1;
            w1 = s0 + s1 + s2;
            if w0 != 0 {
                if w1 > w0 { count2 +=1; }
            }
        }
    
    }

    println!("{}", count);
    println!("{}", count2);
    Ok(())
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }
}
