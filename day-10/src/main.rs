// https://adventofcode.com/2021/day/10

use std::fs;
use std::collections::HashMap;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    /*
): 3 points.
]: 57 points.
}: 1197 points.
>: 25137 points.
    */

    let mut scores: HashMap<char, u32> = HashMap::new();
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('}', 1197);
    scores.insert('>', 25137);
    // read input
    let input = fs::read_to_string(FILE).expect("no u");

    let mut score: u32 = 0;
    for line in input.lines() {
        let mut ends: Vec<char> = Vec::new();
        let data = line.chars();
        for c in data {
            if c == '(' { ends.push(')'); } else
            if c == '[' { ends.push(']'); } else
            if c == '{' { ends.push('}'); } else
            if c == '<' { ends.push('>'); } else {
                let e = ends.pop().unwrap();
                if e != c {
                    println!("unmatched, expected {}, found {}", e, c);
                    score += scores.get(&c).clone().unwrap();
                    break
                }
            }
        }
    }
    println!("{}", score);
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
