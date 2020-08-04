use num::basic::unsigneds::PrimitiveUnsigned;
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
