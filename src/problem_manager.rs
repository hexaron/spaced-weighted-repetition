use std::fmt::Display;

use rand::{rngs::ThreadRng, seq::SliceRandom};

use self::problem::Problem;

mod problem;

pub struct ProblemManager {
    problems: Vec<Problem>,
    rng: ThreadRng,
}

impl ProblemManager {
    pub fn new() -> Self {
        Self {
            problems: Problem::all_new().unwrap(),
            rng: rand::thread_rng(),
        }
    }

    pub fn pose(&mut self) {
        let problem = self.get_most_relevant_problem_mut();

        problem.pose();
        println!();
    }

    fn get_most_relevant_problem_mut(&mut self) -> &mut Problem {
        let mut problem_improvements = Vec::new();

        for (index, problem) in self.problems.iter().enumerate() {
            // Assuming the problem would be answered correctly, how big of a difference would that
            // make?
            let p_update = problem.get_player_p_with_assumption() - problem.get_player_p();

            let total_p_update = problem.get_p() * p_update;

            problem_improvements.push((index, total_p_update));
        }

        problem_improvements.shuffle(&mut self.rng);

        // Sort descending.
        problem_improvements.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

        &mut self.problems[problem_improvements[0].0]
    }

    pub fn total_p(&self) -> f64 {
        self.problems
            .iter()
            .map(|problem| problem.get_p() * problem.get_player_p())
            .sum()
    }
}

impl Display for ProblemManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ProblemManager:")?;

        for problem in &self.problems {
            writeln!(f, "{problem}")?;
        }

        Ok(())
    }
}
