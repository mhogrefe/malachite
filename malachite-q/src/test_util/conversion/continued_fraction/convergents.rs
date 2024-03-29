use crate::conversion::continued_fraction::to_continued_fraction::RationalContinuedFraction;
use crate::conversion::traits::ContinuedFraction;
use crate::Rational;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[derive(Clone, Debug)]
pub struct ConvergentsAlt {
    first: bool,
    floor: Integer,
    xs: Vec<Natural>,
    cf: RationalContinuedFraction,
}

impl Iterator for ConvergentsAlt {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        if self.first {
            self.first = false;
            Some(Rational::from(&self.floor))
        } else if let Some(n) = self.cf.next() {
            self.xs.push(n);
            Some(Rational::from_continued_fraction_ref(
                &self.floor,
                self.xs.iter(),
            ))
        } else {
            self.xs.clear();
            None
        }
    }
}

pub fn convergents_alt(x: Rational) -> ConvergentsAlt {
    let (floor, cf) = x.continued_fraction();
    ConvergentsAlt {
        first: true,
        floor,
        xs: Vec::new(),
        cf,
    }
}
