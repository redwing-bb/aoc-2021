// https://adventofcode.com/2021/day/7

use std::fs;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    // read input
    let input = fs::read_to_string(FILE).expect("no u");
    let crabs: Vec<u64> = input.trim().split(",")
        .map(|f| f.parse().unwrap())
        .collect();

    let max = crabs.iter().max().unwrap();
    let mean:u64 = crabs.iter().sum::<u64>() / crabs.len() as u64;

    let mut best: u64 = *max * crabs.len() as u64;
    for p in 0..=mean {
        let f = fuel_to_reach(p as u64, &crabs);
        if f < best {
            best = f;
        }
    }

    println!("{}", best); // part 1


    let half = (*max / 2) + 1;
    best = (half * (half+1))/2 * crabs.len() as u64;
    for p in 0..=best {
        let f = fuel_to_reach_p2(p as u64, &crabs);
        if f < best {
            best = f;
        }
        println!("{}, {}", best, f);
    }

    println!("{}", best); // part 2
}

fn fuel_to_reach(position: u64, crabs: &Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for crab in crabs.iter() {
        if crab > &position {
            sum += crab-position;
        } else if crab < &position {
            sum += position-crab;
        }
    }
    sum
}


fn fuel_to_reach_p2(position: u64, crabs: &Vec<u64>) -> u64 {
    let mut sum: u64 = 0; 
    for crab in crabs.iter() {
        let mut d = 0;
        if crab > &position {
            d = crab-position;
        } else if crab < &position {
            d = position-crab;
        } 

        if d > 0 { sum += (d * (d+1))/2 }
    }
    sum
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
