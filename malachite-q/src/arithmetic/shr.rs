use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{CheckedFrom, ExactFrom};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use Rational;

fn shr_unsigned_assign<T>(x: &mut Rational, bits: T)
where
    u64: CheckedFrom<T>,
{
    if *x == 0u32 {
        return;
    }
    let numerator_zeros = x.numerator.trailing_zeros().unwrap();
    let bits_64 = u64::exact_from(bits);
    if numerator_zeros >= bits_64 {
        x.numerator >>= bits_64;
    } else {
        x.denominator <<= bits_64 - numerator_zeros;
        x.numerator >>= numerator_zeros
    }
}

fn shr_unsigned_ref<T>(x: &Rational, bits: T) -> Rational
where
    u64: CheckedFrom<T>,
{
    if *x == 0u32 {
        return x.clone();
    }
    let numerator_zeros = x.numerator.trailing_zeros().unwrap();
    let bits_64 = u64::exact_from(bits);
    if numerator_zeros >= bits_64 {
        Rational {
            sign: x.sign,
            numerator: &x.numerator >> bits_64,
            denominator: x.denominator.clone(),
        }
    } else {
        Rational {
            sign: x.sign,
            numerator: &x.numerator >> numerator_zeros,
            denominator: &x.denominator << (bits_64 - numerator_zeros),
        }
    }
}

macro_rules! impl_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for Rational {
            type Output = Rational;

            /// Shifts an `Rational` right (divides it by a power of 2), taking the `Rational` by
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
            /// See the documentation of the `arithmetic::shr` module.
            #[inline]
            fn shr(mut self, bits: $t) -> Rational {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Rational {
            type Output = Rational;

            /// Shifts an `Rational` right (divides it by a power of 2), taking the `Rational` by
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
            /// See the documentation of the `arithmetic::shr` module.
            #[inline]
            fn shr(self, bits: $t) -> Rational {
                shr_unsigned_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Rational {
            /// Shifts an `Rational` right (divides it by a power of 2), in place.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
            ///
            /// # Examples
            /// See the documentation of the `arithmetic::shr` module.
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_unsigned_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_shr_unsigned);

fn shr_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Rational,
    bits: S,
) -> Rational
where
    &'a Rational: Shl<U, Output = Rational> + Shr<U, Output = Rational>,
{
    if bits >= S::ZERO {
        x >> bits.unsigned_abs()
    } else {
        x << bits.unsigned_abs()
    }
}

fn shr_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &mut Rational, bits: S)
where
    Rational: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x >>= bits.unsigned_abs();
    } else {
        *x <<= bits.unsigned_abs();
    }
}

macro_rules! impl_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for Rational {
            type Output = Rational;

            #[inline]
            fn shr(mut self, bits: $t) -> Rational {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Rational {
            type Output = Rational;

            #[inline]
            fn shr(self, bits: $t) -> Rational {
                shr_signed_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Rational {
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_signed(self, bits)
            }
        }
    };
}
apply_to_signeds!(impl_shr_signed);
