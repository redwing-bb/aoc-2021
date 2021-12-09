// https://adventofcode.com/2021/day/9

use std::fs;
use array2d::Array2D;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    // read input
    let input = fs::read_to_string(FILE).expect("no u");
    let lines: Vec<_> = input.lines().collect();

    let cols = lines[0].len();
    let rows = lines.len();

    let lines: String = lines.join("");
    let lines: Vec<u32> = lines.chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect();

    let height_map: Array2D<u32> = Array2D::from_row_major(&lines, rows, cols);

    let mut low_points: Vec<u32> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if check_low(&height_map, r, c) {
                low_points.push(height_map[(r,c)]);
            }
        }
    }
    println!("{:?}", low_points.iter().sum::<u32>() + low_points.len() as u32);
}

fn check_low(map: &Array2D<u32>, x: usize, y: usize) -> bool {
    
    let n = map[(x,y)];
    let u = if x == 0 { true } else { 
        n < map[(x-1,y)]
    };
    let d = if x == map.num_rows() - 1 { true } else {
        n < map[(x+1,y)]
    };
    let l = if y == 0 { true } else {
        n < map[(x,y-1)]
    };
    let r = if y == map.num_columns() - 1 { true } else {
        n < map[(x,y+1)]
    };

    u && d && l && r
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
