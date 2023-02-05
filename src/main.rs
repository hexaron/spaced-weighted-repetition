use crate::problem::Problem;

mod problem;
mod utils;

fn main() {
    let mut problems = Problem::all_new();

    println!("{problems:?}");

    for problem in &mut problems {
        problem.pose();
    }
}
