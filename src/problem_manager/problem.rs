mod problems;

use std::fmt::Display;

use crate::utils::{bounded_bit_vec::BoundedBitVec, ToPercent};

impl BoundedBitVec {
    /// How likely is it, that he player will guess correctly.
    /// Give the most recent values more weight.
    pub fn to_p(&self) -> f64 {
        return (u8::from(self).reverse_bits() as f64) / 255f64;
    }
}

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

    pub fn pose(&mut self) {
        self.spacing = 0;

        println!("{}", self.from);

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        if input.trim() == self.to {
            self.history.push(true);

            println!("Correct! {}%", self.get_player_p().to_percent());
        } else {
            self.history.push(false);

            println!("Not correct! {}%", self.get_player_p().to_percent());
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

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Problem: {} -> {}, player_p: {}%, p: {}%",
            self.from,
            self.to,
            self.get_player_p().to_percent(),
            self.get_p().to_percent()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        empty: BoundedBitVec,
        full: BoundedBitVec,
        /// A non symmetric BoundedBitVec.
        bounded_bit_vec: BoundedBitVec,
    }

    fn setup() -> Setup {
        Setup {
            empty: 0b00000000.into(),
            full: 0b11111111.into(),
            bounded_bit_vec: 0b10110110.into(),
        }
    }

    #[test]
    fn to_p() {
        let Setup {
            empty,
            full,
            bounded_bit_vec,
        } = setup();

        assert_eq!(0f64, empty.to_p());
        assert_eq!(1f64, full.to_p());
        assert_eq!(0.625f64, bounded_bit_vec.to_p());
    }
}
