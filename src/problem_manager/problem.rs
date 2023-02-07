mod problems;

use std::fmt::Display;

use colored::Colorize;

use crate::utils::{bounded_bit_vec::BoundedBitVec, ToPercent};

impl BoundedBitVec {
    /// How likely is it, that he player will guess correctly.
    /// Give the most recent values more weight.
    pub fn to_p(&self) -> f64 {
        return (u8::from(self).reverse_bits() as f64) / 255f64;
    }
}

#[derive(Debug, PartialEq)]
pub struct Problem {
    id: usize,
    from: String,
    to: String,
    history: BoundedBitVec,
    /// How likely is it that this translation will be useful, i.e. how likely is `from` to appear
    /// in a random word/sentence.
    p: f64,
}

impl Problem {
    pub fn new(id: usize, from: &str, to: &str, p: f64) -> Self {
        Self {
            id,
            from: from.to_owned(),
            to: to.to_owned(),
            history: BoundedBitVec::new(),
            p,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn pose(&mut self) -> bool {
        // Empty history means either 8 times wrong in a row.
        // Or simply empty history.
        // Getting it wrong 8 times in a row is nearly impossible.
        // Therefore we can assume that this problem has never been posed.
        if self.history.is_empty() {
            println!("{}", "New Hiragana!".blue());
            println!("{} -> {}", self.from, self.to);

            // We push true, to mark this as seen.
            self.history.push(true);

            // We return false, because the player did not guess correctly.
            // This will move the problem to the beginning of the problem manager.
            return false;
        }

        println!("Translate:");
        println!("{}", self.from);

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        if input.trim() == self.to {
            self.history.push(true);

            println!(
                "{}  ({}%)",
                "Correct!".green(),
                self.get_player_p().to_percent()
            );

            return true;
        } else {
            self.history.push(false);

            println!(
                "{} -> {}  ({}%)",
                "Wrong!".on_red(),
                self.to.bold(),
                self.get_player_p().to_percent()
            );

            return false;
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
        assert_eq!(0.42745098039215684f64, bounded_bit_vec.to_p());
    }
}
