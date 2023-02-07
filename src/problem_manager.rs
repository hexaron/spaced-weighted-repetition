use std::{fmt::Display, ops::Range};

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use self::problem::Problem;

mod problem;

#[derive(Debug)]
pub struct ProblemManager {
    problems: Vec<Problem>,
    log_2_m: i32,
    rng: ThreadRng,
    last_problem_id: Option<usize>,
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
            last_problem_id: None,
        }
    }

    pub fn pose(&mut self) {
        let problem_index = self.get_relevant_problem_index();

        let problem = &mut self.problems[problem_index];

        let correct = problem.pose();

        // correct => bubble sort back (swap with lower p value).
        // not correct => move to front (keep ordering of all other elements).
        if correct {
            let problem_p = problem.get_player_p();

            // First problem after `problem`, that has a lower player p value.
            let to_index_option = self
                .problems
                .iter()
                .enumerate()
                .skip(problem_index)
                .find(|(_, problem)| problem.get_player_p() < problem_p);

            if let Some((to_index, _)) = to_index_option {
                self.problems.swap(problem_index, to_index);
            }
        } else {
            let problem = self.problems.remove(problem_index);

            self.problems.insert(0, problem);
        }
    }

    fn get_relevant_problem_index(&mut self) -> usize {
        let mut problem_index = self.generate_random_index();
        let mut problem_id = self.problems[problem_index].get_id();

        // Do not choose the same problem twice in a row.
        if let Some(last_problem_id) = self.last_problem_id {
            while problem_id == last_problem_id {
                problem_index = self.generate_random_index();
                problem_id = self.problems[problem_index].get_id();
            }
        }

        self.last_problem_id = Some(problem_id);

        problem_index
    }

    fn generate_random_index(&mut self) -> usize {
        // 0 <= r < 1
        let r: f64 = self.rng.gen();

        let formula = 1;

        match formula {
            0 => {
                let m = self.problems.len() as f64;

                (m * r.powi(self.log_2_m)) as usize
            }
            1 => {
                // OVERFLOW:
                // usize fits inside f64.
                let m = self.problems.len() as f64;

                // DIV 0:
                // r < 1.
                let x = 1.0 / (1.0 - r) - 1.0;

                if x >= m {
                    0
                } else {
                    // OVERFLOW:
                    // x is now < m and m came from usize.
                    x as usize
                }
            }
            _ => panic!(),
        }
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
