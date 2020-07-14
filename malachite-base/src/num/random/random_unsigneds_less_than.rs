use num::basic::unsigneds::PrimitiveUnsigned;
use num::random::random_bit_chunks::RandomUnsignedBitChunks;

/// Uniformly generates random unsigned integers less than `limit`, unless `limit` is 0, in which
/// case any integer may be generated.
#[derive(Clone, Debug)]
pub struct RandomUnsignedsLessThan<T: PrimitiveUnsigned> {
    pub(crate) xs: RandomUnsignedBitChunks<T>,
    pub(crate) limit: T,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedsLessThan<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let x = self.xs.next();
            if self.limit == T::ZERO || x.unwrap() < self.limit {
                return x;
            }
        }
    }
}
