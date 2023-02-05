use crate::utils::bounded_bit_vec::BoundedBitVec;

#[derive(Debug)]
pub struct Problem {
    from: String,
    to: String,
    history: BoundedBitVec,
    spacing: u32,
    /// How likely is it that this translation will be useful, i.e. how likely is `from` to appear
    /// in a random word/sentence.
    p: f64,
}

impl Problem {
    pub fn new(from: &str, to: &str, p: f64) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            history: BoundedBitVec::new(),
            spacing: 0,
            p,
        }
    }

    pub fn all_new() -> Vec<Self> {
        vec![Self::new("a", "1", 1f64)]
    }

    pub fn pose(&mut self) {
        self.spacing = 0;

        println!("{}", self.from);

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        if input.trim() == self.to {
            self.history.push(true);

            println!("Correct! {:.0}%", self.get_player_p() * 100f64);
        } else {
            self.history.push(false);

            println!("Not correct! {:.0}%", self.get_player_p() * 100f64);
        }
    }

    /// Returns the probability of the player getting this Problem right, based on the results from
    /// the history, assuming that the next guess would be correct.
    ///
    /// Compare this method to `get_player_p`, which does not assume the next guess being correct.
    pub fn get_player_p_with_assumption(&self) -> f64 {
        let mut new_history = self.history.clone();

        new_history.push(true);

        new_history.to_p()
    }

    /// Returns the probability of the player getting this Problem right, based on the results from
    /// the history.
    ///
    /// This method does not assume the next guess being correct.
    /// Compare this to `get_player_p_with_assumption`.
    pub fn get_player_p(&self) -> f64 {
        self.history.to_p()
    }

    /// How likely is it that this translation will be useful, i.e. how likely is `from` to appear
    /// in a random word/sentence.
    pub fn get_p(&self) -> f64 {
        self.p
    }
}
