use super::Problem;

impl Problem {
    pub fn all_new() -> std::io::Result<Vec<Problem>> {
        let mut problems = Vec::new();

        let dict = std::fs::read_to_string("src/dict")?;

        let n = dict.lines().filter(|line| !line.is_empty()).count();
        let p = (n as f64).recip();

        for line in dict.lines() {
            if line.is_empty() {
                continue;
            }

            let pair: Vec<_> = line.split(" -- ").collect();

            assert_eq!(pair.len(), 2);

            let (hiragana, roman) = (pair[0], pair[1]);

            // TODO (2023-02-06): Do not skip these.
            if hiragana.chars().into_iter().count() == 2 {
                continue;
            }

            problems.push(Problem::new(hiragana, roman, p));
        }

        Ok(problems)
    }
}
