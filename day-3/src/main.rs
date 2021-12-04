// https://adventofcode.com/2021/day/3

use std::fs;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    let input = fs::read_to_string(FILE).expect("no u");
    let h = input.matches("\n").count();

    let mut g = String::new();
    let mut e = String::new();

    let acc = count_bits(&input);
    for b in acc.iter() {
        if b >= &(h/2) { 
            g.push_str("1");
            e.push_str("0");
        } else {
            g.push_str("0");
            e.push_str("1");
        }
    }

    println!("{}", get_decimal(&g) * get_decimal(&e));  // part 1 answer

}


fn count_bits(input: &str) -> Vec<usize> {
    let w: usize = input.find("\n").unwrap();
    let mut acc:Vec<usize> = vec![0; w];
    for line in input.lines() {
        for (i, bit) in line.trim().chars().enumerate() {
            if bit == '1' { acc[i] += 1 }
        }
    }
    acc
}

fn get_decimal(s: &str) -> u32 {
    u32::from_str_radix(&s, 2).unwrap()
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
