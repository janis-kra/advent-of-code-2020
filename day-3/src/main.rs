use std::fs;

fn main() {
    let puzzle = read_puzzle();
    let mut trees = 1;
    let lines = puzzle.lines();

    trees *= count_trees(&puzzle, 1, 1);
    trees *= count_trees(&puzzle, 3, 1);
    trees *= count_trees(&puzzle, 5, 1);
    trees *= count_trees(&puzzle, 7, 1);
    trees *= count_trees(&puzzle, 1, 2);
    
    println!("Trees: {}", trees);
}

fn count_trees(puzzle: &String, steps_right: i32, steps_down: i32) -> i32 {
    let mut position = 0;
    let mut trees = 0;
    let mut rows = puzzle.lines();
    let mut row = rows.next();

    while row != None {
        if row.is_some() {
            let line = row.unwrap();
            if line.chars().nth(position % line.len()).unwrap().to_string() == "#" {
                trees += 1;
            }
        }
        
        position += steps_right as usize;

        for i in 0..steps_down {
            row = rows.next();
        }
    }
    trees
}

fn read_puzzle() -> String {
    println!("Reading puzzle input...");
    fs::read_to_string("puzzle")
        .expect("Error :(")
}
