// https://adventofcode.com/2021/day/7

use std::fs;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";
//const FILE: &str = "input3.txt";

fn main() {

    // read input
    let input = fs::read_to_string(FILE).expect("no u");
    let lines: Vec<Vec<&str>> = input.trim()
        .split(&['|', '\n'][..])
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut p1: u32 = 0;
    for display in lines.iter().skip(1).step_by(2) {
        let segments: Vec<usize> = display.iter()
            .map(|s| s.len())
            .collect();

        // println!("{:?}", display);

        for s in segments.iter() {
            match s {
                2 | 3 | 4 | 7 => p1 += 1,
                _ => (),
            }
        }
    }
    println!("{}", p1);  // part 1 answered

}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
