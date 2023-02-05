use crate::problem::Problem;

mod problem;

fn main() {
    let mut problems = Problem::all_new();

    println!("{problems:?}");

    for problem in &mut problems {
        problem.state();
    }
}
