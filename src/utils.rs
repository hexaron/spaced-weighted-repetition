pub mod bounded_bit_vec;

pub trait ToPercent {
    fn to_percent(&self) -> u8;
}

impl ToPercent for f64 {
    fn to_percent(&self) -> u8 {
        (self * 100f64) as u8
    }
}
