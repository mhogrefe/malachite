use crate::num::arithmetic::traits::{
    BinomialCoefficient, CheckedBinomialCoefficient, UnsignedAbs,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::OverflowingFrom;
use crate::num::exhaustive::primitive_int_increasing_inclusive_range;
use std::cmp::min;

fn checked_binomial_coefficient_unsigned<T: PrimitiveUnsigned>(n: T, mut k: T) -> Option<T> {
    if k > n {
        return Some(T::ZERO);
    }
    k = min(k, n - k);
    if k == T::ZERO {
        return Some(T::ONE);
    } else if k == T::ONE {
        return Some(n);
    } else if k == T::TWO {
        (if n.even() { n - T::ONE } else { n }).checked_mul(n >> 1)
    } else {
        // Some binomial coefficient algorithms have intermediate results greater than the final
        // result, risking overflow. This one does not.
        let mut product = n - k + T::ONE;
        let mut numerator = product;
        for i in primitive_int_increasing_inclusive_range(T::TWO, k) {
            numerator += T::ONE;
            let gcd = numerator.gcd(i);
            product /= i / gcd;
            product = product.checked_mul(numerator / gcd)?;
        }
        Some(product)
    }
}

fn checked_binomial_coefficient_signed<
    U: PrimitiveUnsigned,
    S: OverflowingFrom<U> + PrimitiveSigned + TryFrom<U> + UnsignedAbs<Output = U>,
>(
    n: S,
    k: S,
) -> Option<S> {
    if k < S::ZERO {
        return None;
    }
    if n >= S::ZERO {
        S::try_from(U::checked_binomial_coefficient(
            n.unsigned_abs(),
            k.unsigned_abs(),
        )?)
        .ok()
    } else {
        let k = k.unsigned_abs();
        let b = U::checked_binomial_coefficient(n.unsigned_abs() + k - U::ONE, k)?;
        if k.even() {
            S::try_from(b).ok()
        } else {
            let (b, overflow) = S::overflowing_from(b);
            if overflow {
                if b == S::MIN {
                    Some(S::MIN)
                } else {
                    None
                }
            } else {
                Some(-b)
            }
        }
    }
}

macro_rules! impl_binomial_coefficient_unsigned {
    ($t:ident) => {
        impl CheckedBinomialCoefficient for $t {
            #[inline]
            fn checked_binomial_coefficient(n: $t, k: $t) -> Option<$t> {
                checked_binomial_coefficient_unsigned(n, k)
            }
        }
    };
}
apply_to_unsigneds!(impl_binomial_coefficient_unsigned);

macro_rules! impl_binomial_coefficient_signed {
    ($t:ident) => {
        impl CheckedBinomialCoefficient for $t {
            #[inline]
            fn checked_binomial_coefficient(n: $t, k: $t) -> Option<$t> {
                checked_binomial_coefficient_signed(n, k)
            }
        }
    };
}
apply_to_signeds!(impl_binomial_coefficient_signed);

macro_rules! impl_binomial_coefficient_primitive_int {
    ($t:ident) => {
        impl BinomialCoefficient for $t {
            #[inline]
            fn binomial_coefficient(n: $t, k: $t) -> $t {
                $t::checked_binomial_coefficient(n, k).unwrap()
            }
        }
    };
}
apply_to_primitive_ints!(impl_binomial_coefficient_primitive_int);
