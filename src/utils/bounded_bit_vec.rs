#[derive(Debug, Clone)]
/// If data is 0b00000001, then the last pushed value is true.
/// get(0) will return true and all other will return false.
pub struct BoundedBitVec {
    data: u8,
}

impl BoundedBitVec {
    pub fn new() -> Self {
        Self { data: 0 }
    }

    pub fn push(&mut self, value: bool) {
        if value {
            self.data = (self.data << 1) + 1;
        } else {
            self.data = self.data << 1;
        }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < 8);

        (self.data >> index) % 2 == 1
    }

    /// Take the average of all 1s and 0s.
    /// This is the probability that a randomly chosen element is 1.
    pub fn to_p(&self) -> f64 {
        (self.data.count_ones() as f64) / 8f64
    }
}

impl From<u8> for BoundedBitVec {
    fn from(value: u8) -> Self {
        BoundedBitVec { data: value }
    }
}

impl From<BoundedBitVec> for u8 {
    fn from(value: BoundedBitVec) -> Self {
        value.data
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
    fn get() {
        let Setup {
            empty,
            full,
            bounded_bit_vec,
        } = setup();

        for index in 0..8 {
            assert_eq!(false, empty.get(index));
        }

        for index in 0..8 {
            assert_eq!(true, full.get(index));
        }

        assert_eq!(false, bounded_bit_vec.get(0));
        assert_eq!(true, bounded_bit_vec.get(1));
        assert_eq!(true, bounded_bit_vec.get(2));
        assert_eq!(false, bounded_bit_vec.get(3));
        assert_eq!(true, bounded_bit_vec.get(4));
        assert_eq!(true, bounded_bit_vec.get(5));
        assert_eq!(false, bounded_bit_vec.get(6));
        assert_eq!(true, bounded_bit_vec.get(7));
    }

    #[test]
    fn push() {
        let mut bounded_bit_vec = BoundedBitVec::new();

        assert_eq!(0b00000000, u8::from(bounded_bit_vec.clone()));

        bounded_bit_vec.push(true);

        assert_eq!(0b00000001, u8::from(bounded_bit_vec.clone()));

        bounded_bit_vec.push(true);

        assert_eq!(0b00000011, u8::from(bounded_bit_vec.clone()));

        bounded_bit_vec.push(false);

        assert_eq!(0b00000110, u8::from(bounded_bit_vec.clone()));
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
