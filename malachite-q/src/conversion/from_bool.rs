use crate::Rational;
use malachite_base::num::basic::traits::{One, Zero};

impl From<bool> for Rational {
    #[inline]
    fn from(b: bool) -> Rational {
        if b {
            Rational::ONE
        } else {
            Rational::ZERO
        }
    }
}
