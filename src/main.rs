use crate::problem_manager::ProblemManager;

mod problem_manager;
mod utils;

fn main() {
    let mut problem_manager = ProblemManager::new();

    loop {
        problem_manager.pose();

        println!();
        println!("{problem_manager}");
    }
}
