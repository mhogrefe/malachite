// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Sign, UnsignedAbs};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

#[allow(clippy::unnecessary_wraps)]
fn partial_cmp_unsigned<T: Copy + One + Ord + Sign + SignificantBits>(
    x: &Rational,
    other: &T,
) -> Option<Ordering>
where
    Natural: From<T> + PartialOrd<T>,
{
    // First check signs
    let self_sign = x.sign();
    let other_sign = other.sign();
    let sign_cmp = self_sign.cmp(&other_sign);
    if sign_cmp != Equal || self_sign == Equal {
        return Some(sign_cmp);
    }
    // Then check if one is < 1 and the other is > 1
    let self_cmp_one = x.numerator.cmp(&x.denominator);
    let other_cmp_one = other.cmp(&T::ONE);
    let one_cmp = self_cmp_one.cmp(&other_cmp_one);
    if one_cmp != Equal {
        return Some(one_cmp);
    }
    // Then compare numerators and denominators
    let n_cmp = x.numerator.partial_cmp(other).unwrap();
    let d_cmp = x.denominator.cmp(&Natural::ONE);
    if n_cmp == Equal && d_cmp == Equal {
        return Some(Equal);
    }
    let nd_cmp = n_cmp.cmp(&d_cmp);
    if nd_cmp != Equal {
        return Some(nd_cmp);
    }
    // Then compare floor ∘ log_2 ∘ abs
    let log_cmp = x
        .floor_log_base_2_abs()
        .cmp(&i64::exact_from(other.significant_bits() - 1));
    if log_cmp != Equal {
        return Some(if x.sign { log_cmp } else { log_cmp.reverse() });
    }
    // Finally, cross-multiply.
    Some(x.numerator.cmp(&(&x.denominator * Natural::from(*other))))
}

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrd<$t> for Rational {
            /// Compares a [`Rational`] to an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                partial_cmp_unsigned(self, other)
            }
        }

        impl PartialOrd<Rational> for $t {
            /// Compares an unsigned primitive integer to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

#[allow(clippy::unnecessary_wraps)]
fn partial_cmp_signed<
    U: Copy + One + Ord + SignificantBits,
    S: Copy + Sign + SignificantBits + UnsignedAbs<Output = U>,
>(
    x: &Rational,
    other: &S,
) -> Option<Ordering>
where
    Natural: From<U> + PartialOrd<U>,
{
    // First check signs
    let self_sign = x.sign();
    let other_sign = other.sign();
    let sign_cmp = self_sign.cmp(&other_sign);
    if sign_cmp != Equal || self_sign == Equal {
        return Some(sign_cmp);
    }
    let other_abs = other.unsigned_abs();
    // Then check if one is < 1 and the other is > 1
    let self_cmp_one = x.numerator.cmp(&x.denominator);
    let other_cmp_one = other_abs.cmp(&U::ONE);
    let one_cmp = self_cmp_one.cmp(&other_cmp_one);
    if one_cmp != Equal {
        return Some(if x.sign { one_cmp } else { one_cmp.reverse() });
    }
    // Then compare numerators and denominators
    let n_cmp = x.numerator.partial_cmp(&other_abs).unwrap();
    let d_cmp = x.denominator.cmp(&Natural::ONE);
    if n_cmp == Equal && d_cmp == Equal {
        return Some(Equal);
    }
    let nd_cmp = n_cmp.cmp(&d_cmp);
    if nd_cmp != Equal {
        return Some(if x.sign { nd_cmp } else { nd_cmp.reverse() });
    }
    // Then compare floor ∘ log_2 ∘ abs
    let log_cmp = x
        .floor_log_base_2_abs()
        .cmp(&i64::exact_from(other.significant_bits() - 1));
    if log_cmp != Equal {
        return Some(if x.sign { log_cmp } else { log_cmp.reverse() });
    }
    // Finally, cross-multiply.
    let prod_cmp = x
        .numerator
        .cmp(&(&x.denominator * Natural::from(other_abs)));
    Some(if x.sign { prod_cmp } else { prod_cmp.reverse() })
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrd<$t> for Rational {
            /// Compares a [`Rational`] to a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                partial_cmp_signed(self, other)
            }
        }

        impl PartialOrd<Rational> for $t {
            /// Compares a signed primitive integer to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
