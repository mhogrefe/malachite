use num::basic::unsigneds::PrimitiveUnsigned;
use num::random::random_primitive_integers::RandomPrimitiveIntegers;
use num::random::random_unsigneds_less_than::RandomUnsignedsLessThan;

/// Uniformly generates random unsigned integers in the half-open interval $[a, b)$.
///
/// This `struct` is created by the `random_unsigned_range` method. See its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedRange<T: PrimitiveUnsigned> {
    pub(crate) xs: RandomUnsignedsLessThan<T>,
    pub(crate) a: T,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedRange<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.xs.next().map(|x| x + self.a)
    }
}

/// Uniformly generates random unsigned integers in the closed interval $[a, b]$.
///
/// This `struct` is created by the `random_unsigned_inclusive_range` method. See its documentation
/// for more.
#[derive(Clone, Debug)]
pub enum RandomUnsignedInclusiveRange<T: PrimitiveUnsigned> {
    NotAll(RandomUnsignedsLessThan<T>, T),
    All(RandomPrimitiveIntegers<T>),
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedInclusiveRange<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        match self {
            RandomUnsignedInclusiveRange::NotAll(xs, a) => xs.next().map(|x| x + *a),
            RandomUnsignedInclusiveRange::All(xs) => xs.next(),
        }
    }
}
