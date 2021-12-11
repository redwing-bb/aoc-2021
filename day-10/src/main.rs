// https://adventofcode.com/2021/day/10

use std::fs;
use std::collections::HashMap;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    let mut scores: HashMap<char, u32> = HashMap::new();
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('}', 1197);
    scores.insert('>', 25137);

    let mut scores2: HashMap<char, u64> = HashMap::new();
    scores2.insert(')', 1);
    scores2.insert(']', 2);
    scores2.insert('}', 3);
    scores2.insert('>', 4);

    let mut score2_list: Vec<u64> = Vec::new();

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
            if c == '<' { ends.push('>');
            } else {
                let e = ends.pop().unwrap();
                if e != c {

                    //println!("unmatched, expected {}, found {}", e, c);
                    score += scores.get(&c).unwrap();
                    ends.clear();
                    break
                }
            }
        }
        // ======= part 2 =======
        let mut score2: u64 = 0;
        ends.reverse();
        for e in ends {
            score2 *= 5;
            score2 += scores2.get(&e).unwrap();
        }
        if score2 > 0 {
            score2_list.push(score2);
        }

 
    }
    println!("{}", score); // part 1

    score2_list.sort();
    println!("{}", score2_list[score2_list.len() / 2]); // part 2
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
