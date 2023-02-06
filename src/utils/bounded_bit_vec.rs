use std::fmt::Display;

#[derive(Debug, Clone)]
/// If data is 0b00000001, then the last pushed value is true.
/// get(0) will return true and all other will return false.
#[derive(PartialEq)]
pub struct BoundedBitVec {
    data: u8,
}

impl BoundedBitVec {
    pub fn new() -> Self {
        Self { data: 0b00000000 }
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
}

impl From<&BoundedBitVec> for [bool; 8] {
    fn from(value: &BoundedBitVec) -> Self {
        [
            value.get(0),
            value.get(1),
            value.get(2),
            value.get(3),
            value.get(4),
            value.get(5),
            value.get(6),
            value.get(7),
        ]
    }
}

impl Display for BoundedBitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", <[bool; 8]>::from(self))
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

impl From<&BoundedBitVec> for u8 {
    fn from(value: &BoundedBitVec) -> Self {
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
    fn display() {
        let Setup {
            empty,
            full,
            bounded_bit_vec,
        } = setup();

        assert_eq!(
            "[false, false, false, false, false, false, false, false]",
            format!("{empty}")
        );

        assert_eq!(
            "[true, true, true, true, true, true, true, true]",
            format!("{full}")
        );

        // bounded_bit_vec: 0b10110110
        assert_eq!(
            "[false, true, true, false, true, true, false, true]",
            format!("{bounded_bit_vec}")
        );
    }
}
