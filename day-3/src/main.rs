// https://adventofcode.com/2021/day/3

use std::fs;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    let input = fs::read_to_string(FILE).expect("no u");
    let lines: Vec<_> = input.lines().collect();
    let w = lines[0].len();

    let mut g = mode_bits(&lines);
    let e = flip_bits(&g);

    println!("{}, {}", &g, &e);  
    println!("{}", get_decimal(&g) * get_decimal(&e));  // part 1 answer

    // -- Part 2 ------------------------------------

    let mut o2: Vec<&str> = lines.iter().cloned()
            .filter(|line| line.starts_with(&g[..1]))
            .collect();
    for x in 1..w {
        if o2.len() == 1 { break; }
        g = mode_bits(&o2);
        o2 = o2.iter().cloned()
            .filter(|line| line.chars().nth(x).unwrap() == g.chars().nth(x).unwrap())
            .collect();
    }

    let mut co2: Vec<&str> = lines.iter().cloned()
        .filter(|line| line.starts_with(&e[..1]))
        .collect();
    for x in 1..w {
        if co2.len() == 1 { break; }
        g = flip_bits(&mode_bits(&co2));
        co2 = co2.iter().cloned()
            .filter(|line| line.chars().nth(x).unwrap() == g.chars().nth(x).unwrap())
            .collect();
    }

    println!("{}, {}", o2[0], co2[0]);
    println!("{}", get_decimal(&o2[0]) * get_decimal(&co2[0]));  // part 2 answer
}

fn flip_bits(s: &str) -> String {
    s.chars()
        .map(|c| if c == '1' {
            '0'
        } else {
            '1'
        })
    .collect()
}

fn mode_bits(input: &Vec<&str>) -> String {
    let h = input.len();
    let w = input[0].len();
    let mut acc:Vec<usize> = vec![0; w];
    let mut s = String::new();
    for line in input {
        for (i, bit) in line.chars().enumerate() {
            if bit == '1' { acc[i] += 1 }
        }
    }
    for b in acc.iter() {
        if h-b > *b {
            s.push_str("0");
        } else {
            s.push_str("1");
        }
    }
    s
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
