use num::arithmetic::traits::{CheckedLogTwo, DivAssignMod};
use num::basic::traits::{One, Zero};
use num::conversion::traits::{ConvertibleFrom, Digits, ExactFrom, PowerOfTwoDigits, WrappingFrom};

pub fn _unsigned_to_digits_asc_naive<
    T: Copy + DivAssignMod<T, ModOutput = T> + Eq + ExactFrom<U> + Zero,
    U: Copy + One + Ord + WrappingFrom<T>,
>(
    x: &T,
    base: U,
) -> Vec<U> {
    assert!(base > U::ONE);
    let mut digits = Vec::new();
    let mut remainder = *x;
    let base = T::exact_from(base);
    while remainder != T::ZERO {
        digits.push(U::wrapping_from(remainder.div_assign_mod(base)));
    }
    digits
}

fn _unsigned_to_digits_asc<
    T: ConvertibleFrom<U>
        + Copy
        + DivAssignMod<T, ModOutput = T>
        + Eq
        + ExactFrom<U>
        + PowerOfTwoDigits<U>
        + Zero,
    U: CheckedLogTwo + Copy + One + Ord + WrappingFrom<T>,
>(
    x: &T,
    base: &U,
) -> Vec<U> {
    assert!(T::convertible_from(*base));
    if *x == T::ZERO {
        Vec::new()
    } else if let Some(log_base) = base.checked_log_two() {
        x.to_power_of_two_digits_asc(log_base)
    } else {
        _unsigned_to_digits_asc_naive(x, *base)
    }
}

fn _unsigned_to_digits_desc<
    T: ConvertibleFrom<U>
        + Copy
        + DivAssignMod<T, ModOutput = T>
        + Eq
        + ExactFrom<U>
        + PowerOfTwoDigits<U>
        + Zero,
    U: CheckedLogTwo + Copy + One + Ord + WrappingFrom<T>,
>(
    x: &T,
    base: &U,
) -> Vec<U> {
    assert!(T::convertible_from(*base));
    if *x == T::ZERO {
        Vec::new()
    } else if let Some(log_base) = base.checked_log_two() {
        x.to_power_of_two_digits_desc(log_base)
    } else {
        let mut digits = _unsigned_to_digits_asc_naive(x, *base);
        digits.reverse();
        digits
    }
}

macro_rules! impl_digits {
    ($t:ident) => {
        macro_rules! impl_digits_inner {
            ($u:ident) => {
                impl Digits<$u> for $t {
                    /// Returns a `Vec` containing the digits of `self` in ascending order (least-
                    /// to most-significant).
                    ///
                    /// The type of each digit is `$u`, and `base` must be convertible to $t$. If
                    /// `self` is 0, the `Vec` is empty; otherwise, it ends with a nonzero digit.
                    ///
                    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or
                    /// $d_{k-1} \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{k-1}b^i d_i = x.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(n)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `self.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `base` is less than 2 or greater than `$t::MAX`.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::general_digits`
                    /// module.
                    #[inline]
                    fn to_digits_asc(&self, base: &$u) -> Vec<$u> {
                        _unsigned_to_digits_asc(self, base)
                    }

                    /// Returns a `Vec` containing the digits of `self` in descending order (most-
                    /// to least-significant).
                    ///
                    /// The type of each digit is `$u`, and `base` must be convertible to $t$. If
                    /// `self` is 0, the `Vec` is empty; otherwise, it begins with a nonzero digit.
                    ///
                    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or
                    /// $d_{k-1} \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{k-1}b^i d_{k-i-1} = x.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(n)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `self.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `base` is less than 2 or greater than `$t::MAX`.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::general_digits`
                    /// module.
                    #[inline]
                    fn to_digits_desc(&self, base: &$u) -> Vec<$u> {
                        _unsigned_to_digits_desc(self, base)
                    }

                    #[inline]
                    fn from_digits_asc<I: Iterator<Item = $u>>(_base: &$u, _digits: I) -> $t {
                        unimplemented!()
                    }

                    fn from_digits_desc<I: Iterator<Item = $u>>(_base: &$u, _digits: I) -> $t {
                        unimplemented!()
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_digits_inner);
    };
}
apply_to_unsigneds!(impl_digits);
