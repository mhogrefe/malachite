use malachite_base::num::arithmetic::traits::{Sign, UnsignedAbs};
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;
use Rational;

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
    if sign_cmp != Ordering::Equal || self_sign == Ordering::Equal {
        return Some(sign_cmp);
    }
    // Then check if one is < 1 and the other is > 1
    let self_cmp_one = x.numerator.cmp(&x.denominator);
    let other_cmp_one = other.cmp(&T::ONE);
    let one_cmp = self_cmp_one.cmp(&other_cmp_one);
    if one_cmp != Ordering::Equal {
        return Some(one_cmp);
    }
    // Then compare numerators and denominators
    let n_cmp = x.numerator.partial_cmp(other).unwrap();
    let d_cmp = x.denominator.cmp(&Natural::ONE);
    if n_cmp == Ordering::Equal && d_cmp == Ordering::Equal {
        return Some(Ordering::Equal);
    } else {
        let nd_cmp = n_cmp.cmp(&d_cmp);
        if nd_cmp != Ordering::Equal {
            return Some(nd_cmp);
        }
    }
    let first_prod_bits = x.numerator.significant_bits();
    let second_prod_bits = x.denominator.significant_bits() + other.significant_bits();
    if first_prod_bits < second_prod_bits - 1 {
        return Some(Ordering::Less);
    } else if first_prod_bits > second_prod_bits {
        return Some(Ordering::Greater);
    }
    // Finally, cross-multiply.
    Some(x.numerator.cmp(&(&x.denominator * Natural::from(*other))))
}

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Rational {
            /// Compares the absolute value of a `Rational` to the absolute value of a value of
            /// unsigned primitive integer type.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_cmp_abs_primitive_int` module.
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                partial_cmp_abs_unsigned(self, other)
            }
        }

        impl PartialOrdAbs<Rational> for $t {
            /// Compares the absolute value of a value of unsigned primitive integer type to the
            /// absolute value of a `Rational`.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_cmp_abs_primitive_int` module.
            #[inline]
            fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

fn partial_cmp_abs_signed<
    U: Copy + One + Ord + Sign + SignificantBits,
    S: Copy + Sign + UnsignedAbs<Output = U>,
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
    if sign_cmp != Ordering::Equal || self_sign == Ordering::Equal {
        return Some(sign_cmp);
    }
    // Then check if one is < 1 and the other is > 1
    let self_cmp_one = x.numerator.cmp(&x.denominator);
    let other_cmp_one = other_abs.cmp(&U::ONE);
    let one_cmp = self_cmp_one.cmp(&other_cmp_one);
    if one_cmp != Ordering::Equal {
        return Some(one_cmp);
    }
    // Then compare numerators and denominators
    let n_cmp = x.numerator.partial_cmp(&other_abs).unwrap();
    let d_cmp = x.denominator.cmp(&Natural::ONE);
    if n_cmp == Ordering::Equal && d_cmp == Ordering::Equal {
        return Some(Ordering::Equal);
    } else {
        let nd_cmp = n_cmp.cmp(&d_cmp);
        if nd_cmp != Ordering::Equal {
            return Some(nd_cmp);
        }
    }
    let first_prod_bits = x.numerator.significant_bits();
    let second_prod_bits = x.denominator.significant_bits() + other_abs.significant_bits();
    let bit_cmp = if first_prod_bits < second_prod_bits - 1 {
        Some(Ordering::Less)
    } else if first_prod_bits > second_prod_bits {
        Some(Ordering::Greater)
    } else {
        None
    };
    if let Some(bit_cmp) = bit_cmp {
        return Some(bit_cmp);
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
            /// Compares the absolute value of a `Rational` to the absolute value of a value of
            /// signed primitive integer type.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_cmp_abs_primitive_int` module.
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                partial_cmp_abs_signed(self, other)
            }
        }

        impl PartialOrdAbs<Rational> for $t {
            /// Compares the absolute value of a value of signed primitive integer type to the
            /// absolute value of a `Rational`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_cmp_abs_primitive_int` module.
            #[inline]
            fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
