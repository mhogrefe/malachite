use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{CheckedFrom, ExactFrom};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use Rational;

fn shl_unsigned_assign<T>(x: &mut Rational, bits: T)
where
    u64: CheckedFrom<T>,
{
    if *x == 0u32 {
        return;
    }
    let denominator_zeros = x.denominator.trailing_zeros().unwrap();
    let bits_64 = u64::exact_from(bits);
    if denominator_zeros >= bits_64 {
        x.denominator >>= bits_64;
    } else {
        x.denominator >>= denominator_zeros;
        x.numerator <<= bits_64 - denominator_zeros;
    }
}

fn shl_unsigned_ref<T>(x: &Rational, bits: T) -> Rational
where
    u64: CheckedFrom<T>,
{
    if *x == 0u32 {
        return x.clone();
    }
    let denominator_zeros = x.denominator.trailing_zeros().unwrap();
    let bits_64 = u64::exact_from(bits);
    if denominator_zeros >= bits_64 {
        Rational {
            sign: x.sign,
            numerator: x.numerator.clone(),
            denominator: &x.denominator >> bits_64,
        }
    } else {
        Rational {
            sign: x.sign,
            numerator: &x.numerator << (bits_64 - denominator_zeros),
            denominator: &x.denominator >> denominator_zeros,
        }
    }
}

macro_rules! impl_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for Rational {
            type Output = Rational;

            /// Shifts an `Rational` left (multiplies it by a power of 2), taking the `Rational` by
            /// value.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
            ///
            /// # Examples
            /// See the documentation of the `arithmetic::shl` module.
            #[inline]
            fn shl(mut self, bits: $t) -> Rational {
                self <<= bits;
                self
            }
        }

        impl<'a> Shl<$t> for &'a Rational {
            type Output = Rational;

            /// Shifts an `Rational` left (multiplies it by a power of 2), taking the `Rational` by
            /// reference.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
            ///
            /// # Examples
            /// See the documentation of the `arithmetic::shl` module.
            #[inline]
            fn shl(self, bits: $t) -> Rational {
                shl_unsigned_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for Rational {
            /// Shifts an `Rational` left (multiplies it by a power of 2), in place.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
            ///
            /// # Examples
            /// See the documentation of the `arithmetic::shl` module.
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_unsigned_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_shl_unsigned);

fn shl_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Rational,
    bits: S,
) -> Rational
where
    &'a Rational: Shl<U, Output = Rational> + Shr<U, Output = Rational>,
{
    if bits >= S::ZERO {
        x << bits.unsigned_abs()
    } else {
        x >> bits.unsigned_abs()
    }
}

fn shl_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &mut Rational, bits: S)
where
    Rational: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x <<= bits.unsigned_abs();
    } else {
        *x >>= bits.unsigned_abs();
    }
}

macro_rules! impl_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for Rational {
            type Output = Rational;

            #[inline]
            fn shl(mut self, bits: $t) -> Rational {
                self <<= bits;
                self
            }
        }

        impl<'a> Shl<$t> for &'a Rational {
            type Output = Rational;

            #[inline]
            fn shl(self, bits: $t) -> Rational {
                shl_signed_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for Rational {
            fn shl_assign(&mut self, bits: $t) {
                shl_assign_signed(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_shl_signed);
