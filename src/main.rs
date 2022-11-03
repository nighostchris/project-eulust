mod solution;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len().ge(&2) {
        match (args[1]).parse::<i32>() {
            Ok(problem_id) => match problem_id {
                1 => solution::p1::multiples_of_3_or_5(1000),
                _ => {
                    println!("There is no solution to problem '{}'.", problem_id);
                }
            },
            Err(_) => {
                println!("Invalid problem id. Please retry.");
            }
        }
    } else {
        println!("Please specify the problem id to run the corresponding solution.");
    }
}