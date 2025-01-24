// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

#[allow(clippy::unnecessary_wraps)]
fn partial_cmp_abs_unsigned<T: Copy + One + Ord + Sign + SignificantBits>(
    x: &Rational,
    other: &T,
) -> Option<Ordering>
where
    Natural: From<T> + PartialOrd<T>,
{
    // First check if either value is zero
    let self_sign = x.numerator_ref().sign();
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
        return Some(log_cmp);
    }
    // Finally, cross-multiply.
    Some(x.numerator.cmp(&(&x.denominator * Natural::from(*other))))
}

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Rational {
            /// Compares the absolute values of a [`Rational`] and an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                partial_cmp_abs_unsigned(self, other)
            }
        }

        impl PartialOrdAbs<Rational> for $t {
            /// Compares the absolute values of an unsigned primitive integer and a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

#[allow(clippy::unnecessary_wraps)]
fn partial_cmp_abs_signed<
    U: Copy + One + Ord + Sign + SignificantBits,
    S: Copy + Sign + SignificantBits + UnsignedAbs<Output = U>,
>(
    x: &Rational,
    other: &S,
) -> Option<Ordering>
where
    Natural: From<U> + PartialOrd<U>,
{
    // First check if either value is zero
    let self_sign = x.numerator_ref().sign();
    let other_abs = other.unsigned_abs();
    let other_sign = other_abs.sign();
    let sign_cmp = self_sign.cmp(&other_sign);
    if sign_cmp != Equal || self_sign == Equal {
        return Some(sign_cmp);
    }
    // Then check if one is < 1 and the other is > 1
    let self_cmp_one = x.numerator.cmp(&x.denominator);
    let other_cmp_one = other_abs.cmp(&U::ONE);
    let one_cmp = self_cmp_one.cmp(&other_cmp_one);
    if one_cmp != Equal {
        return Some(one_cmp);
    }
    // Then compare numerators and denominators
    let n_cmp = x.numerator.partial_cmp(&other_abs).unwrap();
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
        return Some(log_cmp);
    }
    // Finally, cross-multiply.
    Some(
        x.numerator
            .cmp(&(&x.denominator * Natural::from(other_abs))),
    )
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Rational {
            /// Compares the absolute values of a [`Rational`] and a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                partial_cmp_abs_signed(self, other)
            }
        }

        impl PartialOrdAbs<Rational> for $t {
            /// Compares the absolute values of a signed primitive integer and a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
