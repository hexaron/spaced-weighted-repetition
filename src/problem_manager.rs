use std::{fmt::Display, ops::Range};

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use self::problem::Problem;

mod problem;

#[derive(Debug)]
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

    pub fn shuffle(&mut self) {
        self.problems.shuffle(&mut self.rng);
    }

    pub fn pose(&mut self) {
        let problem_index = self.get_most_relevant_problem_mut();
        let problem = &mut self.problems[problem_index];

        let correct = problem.pose();

        let to_index;

        if correct {
            let to_index_option = self
                .problems
                .iter()
                .enumerate()
                .skip(problem_index)
                .find(|(_, problem)| problem.get_player_p() > problem.get_player_p());

            to_index = match to_index_option {
                Some((i, _)) => i,
                None => (problem_index + 1).min(self.problems.len() - 1),
            };
        } else {
            to_index = 0;
        }

        let problem = self.problems.remove(problem_index);

        self.problems.insert(to_index, problem);
    }

    fn get_most_relevant_problem_mut(&mut self) -> usize {
        let r: f64 = self.rng.gen();

        (((self.problems.len() + 1) as f64).powf(r.powi(6)) - 1.0) as usize
    }

    pub fn total_p(&self) -> f64 {
        self.problems
            .iter()
            .map(|problem| problem.get_p() * problem.get_player_p())
            .sum()
    }

    pub fn debug_print_problem_order(&self, range: Range<usize>) {
        for (i, problem) in self.problems[range].iter().enumerate() {
            println!("{i: <2}: {problem}");
        }
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
