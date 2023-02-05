use self::problem::Problem;

mod problem;

pub struct ProblemManager {
    problems: Vec<Problem>,
}

impl ProblemManager {
    pub fn new() -> Self {
        Self {
            problems: Problem::all_new(),
        }
    }

    pub fn pose(&mut self) {
        let problem = self.get_most_relevant_problem_mut();

        problem.pose();
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
