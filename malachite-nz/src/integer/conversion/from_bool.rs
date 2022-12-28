use crate::integer::Integer;
use malachite_base::num::basic::traits::{One, Zero};

impl From<bool> for Integer {
    #[inline]
    fn from(b: bool) -> Integer {
        if b {
            Integer::ONE
        } else {
            Integer::ZERO
        }
    }
}
