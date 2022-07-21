use malachite_base::num::arithmetic::traits::ReciprocalAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use crate::Rational;

pub fn from_continued_fraction_alt(floor: Integer, xs: Vec<Natural>) -> Rational {
    if xs.is_empty() {
        Rational::from(floor)
    } else {
        let mut x = Rational::ZERO;
        let mut first = true;
        for n in xs.into_iter().rev() {
            if first {
                first = false;
            } else {
                x.reciprocal_assign();
            }
            x += Rational::from(n);
        }
        if !first {
            x.reciprocal_assign();
        }
        x + Rational::from(floor)
    }
}
