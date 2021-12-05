// https://adventofcode.com/2021/day/4

use std::fs;
use array2d::Array2D;

const FILE: &str = "input.txt";
//const FILE: &str = "input2.txt";

fn main() {

    // read input
    let input = fs::read_to_string(FILE).expect("no u");
    let mut lines: Vec<_> = input.lines().collect();

    // get the numbers drawn by the bingo caller
    let numbers = get_numbers(lines[0].clone());

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

    let mut winners: Vec<usize> = Vec::new();
    let mut win_at: Vec<u32> = Vec::new();
    for num in numbers {
        // mark boards
        for board in 0..boards.len() {
            if !winners.contains(&board) {
                for x in 0..5 {
                    for y in 0..5 {
                        if boards[board][(x, y)] == num {
                            marks[board][(x, y)] = true;
                            // println!("match found: {}", num);
                        }
                    }
                }
            }
        }

        // check for bingos
        for board in 0..boards.len() {
            if !winners.contains(&board) {
                // rows
                for y in 0..5 {
                    let score =  marks[board].row_iter(y)
                        .filter(|square| **square == true)
                        .count();
                    if score == 5 {
                        println!("winning card: #{}", board);
                        winners.push(board);
                        win_at.push(num);
                    }
                }
                // columns
                for x in 0..5 {
                    let score = marks[board].column_iter(x)
                        .filter(|square| **square == true)
                        .count();
                    if score == 5 {
                        println!("winning card: #{}", board);
                        winners.push(board);
                        win_at.push(num);
                    }
                }
            }
        }
    }

    // part 1
    for row in boards[winners[0]].as_rows() {
        println!("{:?}", row);
    }
    for row in marks[winners[0]].as_rows() {
        println!("{:?}", row);
    }
    let mut score: u32 = 0;
    for y in 0..5 {
        for x in 0..5 {
            if marks[winners[0]][(y,x)] == false {
                score += boards[winners[0]][(y,x)];
            }
        }
    }
    println!("{}, {}, {}", score, win_at[0], score*win_at[0]);

    // part 2
    score = 0;
    for row in boards[winners[*winners.last().unwrap()]].as_rows() {
        println!("{:?}", row);
    }
    for row in marks[winners[*winners.last().unwrap()]].as_rows() {
        println!("{:?}", row);
    }
    for y in 0..5 {
        for x in 0..5 {
            if marks[*winners.last().unwrap()][(y,x)] == false {
                score += boards[*winners.last().unwrap()][(y,x)];
            }
        }
    }
    println!("{}, {}, {}", score, win_at.last().unwrap(), score*win_at.last().unwrap());
}


fn get_numbers(input: &str) -> Vec<u32> {
    let numbers: Vec<u32> = input.split(",")
        .map(|n| n.parse().unwrap())
        .collect();
        numbers
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() { assert_eq!(4 + 4, 8); }

}
