use num::basic::integers::PrimitiveInteger;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::BitAccess;
use num::random::random_unsigned_range::{RandomUnsignedInclusiveRange, RandomUnsignedRange};
use num::random::{random_unsigned_inclusive_range, random_unsigned_range};
use random::seed::Seed;
use std::fmt::Debug;

#[doc(hidden)]
pub trait HasRandomSignedRange: Sized {
    type UnsignedValue: PrimitiveUnsigned;

    fn new_unsigned_range(seed: Seed, a: Self, b: Self)
        -> RandomUnsignedRange<Self::UnsignedValue>;

    fn new_unsigned_inclusive_range(
        seed: Seed,
        a: Self,
        b: Self,
    ) -> RandomUnsignedInclusiveRange<Self::UnsignedValue>;

    fn from_unsigned_value(x: Self::UnsignedValue) -> Self;
}

macro_rules! impl_has_random_signed_range {
    ($u: ident, $s: ident) => {
        impl HasRandomSignedRange for $s {
            type UnsignedValue = $u;

            fn new_unsigned_range(seed: Seed, mut a: $s, mut b: $s) -> RandomUnsignedRange<$u> {
                a.flip_bit($u::WIDTH - 1);
                b.flip_bit($u::WIDTH - 1);
                random_unsigned_range(seed, $u::wrapping_from(a), $u::wrapping_from(b))
            }

            fn new_unsigned_inclusive_range(
                seed: Seed,
                mut a: $s,
                mut b: $s,
            ) -> RandomUnsignedInclusiveRange<$u> {
                a.flip_bit($u::WIDTH - 1);
                b.flip_bit($u::WIDTH - 1);
                random_unsigned_inclusive_range(seed, $u::wrapping_from(a), $u::wrapping_from(b))
            }

            fn from_unsigned_value(mut u: $u) -> $s {
                u.flip_bit($u::WIDTH - 1);
                $s::wrapping_from(u)
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
    pub(crate) xs: RandomUnsignedRange<T::UnsignedValue>,
}

impl<T: HasRandomSignedRange> Iterator for RandomSignedRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next().map(T::from_unsigned_value)
    }
}

/// Uniformly generates random signed integers in the closed interval $[a, b]$.
///
/// This `struct` is created by the `random_signed_inclusive_range` method. See its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct RandomSignedInclusiveRange<T: HasRandomSignedRange> {
    pub(crate) xs: RandomUnsignedInclusiveRange<T::UnsignedValue>,
}

impl<T: HasRandomSignedRange> Iterator for RandomSignedInclusiveRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next().map(T::from_unsigned_value)
    }
}
