// https://adventofcode.com/2021/day/3

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::char;

fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut acc0: [u32; 12] = [0; 12];
    let mut acc1: [u32; 12] = [0; 12];
    let mut gamma: [u8; 12] = [0; 12];
    let mut epsilon: [u8; 12] = [0; 12];

    for line in reader.lines() {
        let input = line.unwrap();
        for (i, val) in input.chars().enumerate() {
            match val {
                '0' => acc0[i] += 1,
                '1' => acc1[i] += 1,
                _ => panic!("invalid accumulator value"),
            }
        }
        println!("{:?}", acc0);
        println!("{:?}", acc1);
    }

    for n in 0..acc0.len() {
        if acc0[n] > acc1[n] {
            gamma[n] = 0;
            epsilon[n] = 1;
        } else if acc0[n] < acc1[n] {
            gamma[n] = 1;
            epsilon[n] = 0;
        } else {
            panic!("don't know what to do if 0 and 1 are equal");
        }
    }

    println!("{:?}", gamma);
    println!("{:?}", epsilon);

    let mut s = String::from("");
    for b in gamma {
        s.push(char::from_digit(b as u32, 10).unwrap());
    }
    println!("{}", s);
    let g = u32::from_str_radix(&s, 2).unwrap();

    let mut s = String::from("");
    for b in epsilon {
        s.push(char::from_digit(b as u32, 10).unwrap());
    }
    println!("{}", s);
    let e = u32::from_str_radix(&s, 2).unwrap();

    println!("gamma: {}", g);
    println!("epsilon: {}", e);

    println!("{}", g*e);

    Ok(())
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }
}
