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

    let max = crabs.iter().max().unwrap();
    let min = crabs.iter().min().unwrap();
    let mean:u32 = crabs.iter().sum::<u32>() / crabs.len() as u32;

    let mut best: u32 = *max * crabs.len() as u32;
    for p in 0..=mean {
        let f = fuel_to_reach(p as u32, &crabs);
        if f < best {
            best = f;
        }
    }

    println!("{}", best);
}

fn fuel_to_reach(position: u32, crabs: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for crab in crabs.iter() {
        if crab > &position {
            sum += crab-position;
        } else if crab < &position {
            sum += position-crab;
        }
    }
    sum
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
