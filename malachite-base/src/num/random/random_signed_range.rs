use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::BitAccess;
use num::random::random_unsigned_range::RandomUnsignedRange;
use num::random::{random_unsigned_range, random_unsigned_range_to_max};
use random::seed::Seed;
use std::fmt::Debug;

#[doc(hidden)]
pub trait HasRandomSignedRange: Sized {
    type UnsignedRange: Clone + Debug;

    fn new_unsigned_range(seed: Seed, a: Self, b: Self) -> Self::UnsignedRange;

    fn new_unsigned_range_to_max(seed: Seed, a: Self) -> Self::UnsignedRange;

    fn next_value(xs: &mut Self::UnsignedRange) -> Option<Self>;
}

macro_rules! impl_has_random_signed_range {
    ($u: ident, $s: ident) => {
        impl HasRandomSignedRange for $s {
            type UnsignedRange = RandomUnsignedRange<$u>;

            fn new_unsigned_range(seed: Seed, mut a: $s, mut b: $s) -> RandomUnsignedRange<$u> {
                if a >= b {
                    panic!("a must be less than b. a: {}, b: {}", a, b);
                }
                a.flip_bit($u::WIDTH - 1);
                b.flip_bit($u::WIDTH - 1);
                random_unsigned_range(seed, $u::wrapping_from(a), $u::wrapping_from(b))
            }

            fn new_unsigned_range_to_max(seed: Seed, mut a: $s) -> RandomUnsignedRange<$u> {
                a.flip_bit($u::WIDTH - 1);
                random_unsigned_range_to_max(seed, $u::wrapping_from(a))
            }

            fn next_value(xs: &mut Self::UnsignedRange) -> Option<$s> {
                xs.next().map(|mut u| {
                    u.flip_bit($u::WIDTH - 1);
                    $s::wrapping_from(u)
                })
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_has_random_signed_range);

/// Uniformly generates random signed integers in the half-open interval $[a, b)$.
///
/// This `struct` is created by the `random_signed_range` method. See its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomSignedRange<T: HasRandomSignedRange> {
    pub(crate) xs: T::UnsignedRange,
}

impl<T: HasRandomSignedRange> Iterator for RandomSignedRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        T::next_value(&mut self.xs)
    }
}
