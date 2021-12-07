// https://adventofcode.com/2021/day/7

use std::fs;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    // read input
    let input = fs::read_to_string(FILE).expect("no u");
    let crabs: Vec<u32> = input.trim().split(",")
        .map(|f| f.parse().unwrap())
        .collect();

}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
