// https://adventofcode.com/2021/day/4

use std::fs;
use array2d::Array2D;

//const FILE: &str = "input.txt";
const FILE: &str = "input2.txt";

fn main() {

    let input = fs::read_to_string(FILE).expect("no u");
    let mut lines: Vec<_> = input.lines().collect();

    let draws = get_draws(lines[0].clone());

    lines[0] = "";
    let lines: String = lines.join(" ");
    let lines: Vec<u32> = lines.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let lines: Vec<&[u32]> = lines.chunks(25).collect();

    let boards: Vec<Array2D<u32>> = lines.iter()
        .map(|line| Array2D::from_row_major(&line, 5, 5))
        .collect();

    let mut marks: Vec<Array2D<bool>> = Vec::new();
    for n in 0..boards.len() {
        marks.push(Array2D::filled_with(false, 5, 5));
    }

//    println!("{}, {:?}", boards.len(), boards);
//    println!("{}, {:?}", marks.len(), marks);
    for n in draws {
        println!("{}", n);
    }
}

fn get_draws(input: &str) -> Vec<u32> {
    let draws: Vec<u32> = input.split(",")
        .map(|n| n.parse().unwrap())
        .collect();
        draws
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
