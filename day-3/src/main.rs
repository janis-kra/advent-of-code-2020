use std::fs;

fn main() {
    let puzzle = read_puzzle();
    let rows = puzzle.lines();
    let mut position = 0;
    let mut trees = 0;

    for row in rows {
        if row.chars().nth(position % row.len()).unwrap().to_string() == "#" {
            trees += 1;
        }
        position += 3;
    }
    println!("Trees: {}", trees);
}

fn read_puzzle() -> String {
    println!("Reading puzzle input...");
    fs::read_to_string("puzzle")
        .expect("Error :(")
}
