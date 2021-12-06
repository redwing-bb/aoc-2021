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

    for i in 0..80 {
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
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
