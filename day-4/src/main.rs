// https://adventofcode.com/2021/day/4

use std::fs;
use array2d::Array2D;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    // read input
    let input = fs::read_to_string(FILE).expect("no u");
    let mut lines: Vec<_> = input.lines().collect();

    // get the number draws for the bingo caller
    let draws = get_draws(lines[0].clone());

    // build the bingo cards
    lines[0] = "";
    let lines: String = lines.join(" ");
    let lines: Vec<u32> = lines.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let lines: Vec<&[u32]> = lines.chunks(25).collect();

    let boards: Vec<Array2D<u32>> = lines.iter()
        .map(|line| Array2D::from_row_major(&line, 5, 5))
        .collect();

    // build the score cards 
    let mut marks: Vec<Array2D<bool>> = Vec::new();
    for _n in 0..boards.len() {
        marks.push(Array2D::filled_with(false, 5, 5));
    }

//    println!("{}, {:?}", boards.len(), boards);
//    println!("{}, {:?}", marks.len(), marks);

    let mut winner = 0;
    let mut call = 0;
    'drawing: for draw in draws {
        // mark boards
        for z in 0..boards.len() {
            for x in 0..5 {
                for y in 0..5 {
                    if boards[z][(x, y)] == draw {
                        marks[z][(x, y)] = true;
                        // println!("match found: {}", draw);
                    }
                }
            }
        }

        // check for bingos
        for z in 0..boards.len() {
            // check rows
            for y in 0..5 {
                let score =  marks[z].row_iter(y)
                    .filter(|square| **square == true)
                    .count();
                if score == 5 {
                    println!("winning card: #{}", z);
                    winner = z;
                    call = draw;
                    break 'drawing;
                }
            }
            // check columns
            for x in 0..5 {
                let score = marks[z].column_iter(x)
                    .filter(|square| **square == true)
                    .count();
                if score == 5 {
                    println!("winning card: #{}", z);
                    winner = z;
                    call = draw;
                    break 'drawing;
                }
            }
        }
    }
    // get winning board
    for row in boards[winner].as_rows() {
        println!("{:?}", row);
    }
    for row in marks[winner].as_rows() {
        println!("{:?}", row);
    }

    let mut score: u32 = 0;
    for y in 0..5 {
        for x in 0..5 {
            if marks[winner][(y,x)] == false {
                score += boards[winner][(y,x)];
            }
        }
    }
    println!("{}, {}, {}", score, call, score*call);
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
