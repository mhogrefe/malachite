use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::conversion::continued_fraction::to_continued_fraction::RationalContinuedFraction;
use malachite_q::conversion::traits::ContinuedFraction;
use malachite_q::Rational;

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
            Some(Rational::from_continued_fraction_ref(&self.floor, &self.xs))
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
