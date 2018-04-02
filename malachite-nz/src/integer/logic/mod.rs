use integer::Integer;

impl Integer {
    //TODO test
    pub fn count_ones(&self) -> Option<u64> {
        if self.sign {
            Some(self.abs.count_ones())
        } else {
            None
        }
    }
}

pub mod bit_access;
pub mod not;
pub mod significant_bits;
pub mod trailing_zeros;
