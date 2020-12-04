use std::fs;
use std::process;

fn main() {
    println!("Reading puzzle input...");
    let puzzle = fs::read_to_string("puzzle")
        .expect("Error :(");

    for line1 in puzzle.lines() {
        let number1 = line1.parse::<i32>().unwrap();
        for line2 in puzzle.lines() {
            let number2 = line2.parse::<i32>().unwrap();
            let sum = number1 + number2;
            if sum == 2020 {
                println!("Solution found!\n{}\n{}", line1, line2);
                println!("Multplied value:\n{}", number1 * number2);
                process::exit(0);
            };
        }
    }

    println!("No solution found :(");
    process::exit(1);
}
