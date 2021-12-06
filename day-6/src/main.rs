// https://adventofcode.com/2021/day/6

use std::fs;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    // read input
    let input = fs::read_to_string(FILE).expect("no u");
    let mut fish: Vec<u32> = input.trim().split(",")
        .map(|f| f.parse().unwrap())
        .collect();

    for _i in 0..80 {
        let mut babies: Vec<u32> = Vec::new();
        for x in 0..fish.len() {
            if fish[x] == 0 {
                babies.push(8);
                fish[x] = 6;
            } else {
                fish[x] -= 1;
            }
        }
        fish.append(&mut babies);
        //println!("{}, {}, {:?}", i+1, fish.len(), fish);
    }
    println!("{}", fish.len());


    // part 2 
    let input = fs::read_to_string(FILE).expect("no u");
    let fish: Vec<u32> = input.trim().split(",")
        .map(|f| f.parse().unwrap())
        .collect();

    let mut timers: [u64; 9] = [0; 9];
    for f in fish {
        timers[f as usize] += 1;
    }

    println!("{:?}", timers);

    for _i in 0..256 {
        let babies = timers[0];
        timers.rotate_left(1);
        timers[6] += babies;
    }

    let p2: u64 = timers.iter().sum();
    println!("{:?}", p2);

}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
