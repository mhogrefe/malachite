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

pub mod assign_bit;
pub mod clear_bit;
pub mod flip_bit;
pub mod from_sign_and_limbs;
pub mod from_twos_complement_limbs;
pub mod get_bit;
pub mod not;
pub mod set_bit;
pub mod sign_and_limbs;
pub mod significant_bits;
pub mod trailing_zeros;
pub mod twos_complement_limbs;
