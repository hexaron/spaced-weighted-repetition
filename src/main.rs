use std::io::Write;

use colored::Colorize;

use crate::problem_manager::ProblemManager;

mod problem_manager;
mod utils;

fn main() {
    let mut problem_manager = ProblemManager::new(true);

    loop {
        print!("{esc}c", esc = 27 as char);

        problem_manager.pose();

        std::io::stdin()
            .read_line(&mut String::new())
            .expect("error: unable to read user input");
    }
}
