use std::{fmt::Display, ops::Range};

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use self::problem::Problem;

mod problem;

#[derive(Debug)]
pub struct ProblemManager {
    problems: Vec<Problem>,
    log_2_m: i32,
    rng: ThreadRng,
}

impl ProblemManager {
    pub fn new(shuffle: bool) -> Self {
        let mut rng = rand::thread_rng();

        let mut problems = Problem::all_new().unwrap();

        if shuffle {
            problems.shuffle(&mut rng);
        }

        // Storing in temporary variable creates copy.
        let m = problems.len() as f64;

        Self {
            problems,
            log_2_m: m.log2().round() as i32,
            rng,
        }
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

        let m = self.problems.len() as f64;

        (m * r.powi(self.log_2_m)) as usize
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
