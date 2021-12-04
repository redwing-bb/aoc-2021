// https://adventofcode.com/2021/day/3

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::char;

fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut acc0: [u32; 12] = [0; 12]; // count of the 0 values in each line
    let mut acc1: [u32; 12] = [0; 12]; // count of the 1 values in each line
    let mut in_len = 0;
    let mut gamma: [u8; 12] = [0; 12];
    let mut epsilon: [u8; 12] = [0; 12];

    // get the input data, counting the input bits to appropriate accumulators
    for line in reader.lines() {
        let input = line.unwrap();
        in_len = input.trim().len();
        for (i, val) in input.trim().chars().enumerate() {
            match val {
                '0' => acc0[i] += 1,
                '1' => acc1[i] += 1,
                _ => panic!("invalid accumulator value '{}'", val),
            }
        }
    }

    // compare each position in the accumulators, building gamma and epsilon
    for n in 0..in_len {
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

    println!("{:?}", &gamma[0..in_len]);
    println!("{:?}", &epsilon[0..in_len]);
    println!("{}", get_decimal(&gamma[0..in_len]) * get_decimal(&epsilon[0..in_len]));  // part 1 answer


    Ok(())
}


fn stringify_bitfield(bits: &[u8]) -> String {
    // read bits from array into a string
    let mut s = String::from("");
    for b in bits {
        s.push(char::from_digit(*b as u32, 10).unwrap());
    }
    s
}
fn get_decimal(bits: &[u8]) -> u32 {
    let s = stringify_bitfield(bits);
    u32::from_str_radix(&s, 2).unwrap()
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
