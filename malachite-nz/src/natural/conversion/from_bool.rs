use crate::natural::Natural;
use malachite_base::num::basic::traits::{One, Zero};

impl From<bool> for Natural {
    #[inline]
    fn from(b: bool) -> Natural {
        if b {
            Natural::ONE
        } else {
            Natural::ZERO
        }
    }
}
