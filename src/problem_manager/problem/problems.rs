use super::Problem;

impl Problem {
    pub fn all_new() -> Vec<Self> {
        vec![Self::new("a", "1", 0.25f64), Self::new("b", "2", 0.75f64)]
    }
}
