use num::arithmetic::traits::{DivRound, SaturatingSubAssign};
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{
    ExactFrom, PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, WrappingFrom,
};
use num::logic::traits::BitBlockAccess;
use rounding_modes::RoundingMode;
use std::marker::PhantomData;

/// A double-ended iterator over the base-$2^\ell$ digits of a primitive unsigned integer.
///
/// This `struct` is created by the `power_of_two_digits` function in this type's implementation of
/// `PowerOfTwoDigitIterable`. See its documentation for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PrimitivePowerOfTwoDigitIterator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    pub(crate) value: T,
    pub(crate) log_base: u64,
    pub(crate) some_remaining: bool,
    // If `n` is nonzero, this index initially points to the least-significant bit of the least-
    // significant digit, and is left-shifted by `next`.
    pub(crate) i: u64,
    // If `n` is nonzero, this mask initially points to the least-significant bit of the most-
    // significant nonzero digit, and is right-shifted by `next_back`.
    pub(crate) j: u64,
    phantom: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<<T as BitBlockAccess>::Bits>>
    Iterator for PrimitivePowerOfTwoDigitIterator<T, U>
{
    type Item = U;

    /// A function to iterate through the digits of a primitive unsigned integer in ascending order
    /// (least-significant first).
    ///
    /// The base is $2^\ell$ and the output type is `U`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::digits::power_of_two_digit_iterable::*;
    /// use malachite_base::num::conversion::traits::PowerOfTwoDigitIterable;
    ///
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
    /// assert_eq!(digits.next(), None);
    ///
    /// // 107 = 1101011b
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    /// assert_eq!(digits.next(), Some(3));
    /// assert_eq!(digits.next(), Some(2));
    /// assert_eq!(digits.next(), Some(2));
    /// assert_eq!(digits.next(), Some(1));
    /// assert_eq!(digits.next(), None);
    /// ```
    fn next(&mut self) -> Option<U> {
        if self.some_remaining {
            let digit = U::wrapping_from(self.value.get_bits(self.i, self.i + self.log_base));
            if self.i == self.j {
                self.some_remaining = false;
            }
            self.i += self.log_base;
            Some(digit)
        } else {
            None
        }
    }

    /// A function that returns the length of the digits iterator; that is, the value's significant
    /// digit count.
    ///
    /// The format is `(lower, Option<upper>)`, but in this case it's trivial to always have an
    /// exact bound.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::digits::power_of_two_digit_iterable::*;
    /// use malachite_base::num::conversion::traits::PowerOfTwoDigitIterable;
    ///
    /// let digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
    /// assert_eq!(digits.size_hint(), (0, Some(0)));
    ///
    /// let digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    /// assert_eq!(digits.size_hint(), (4, Some(4)));
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_digits = usize::exact_from(
            self.value
                .significant_bits()
                .div_round(self.log_base, RoundingMode::Ceiling),
        );
        (significant_digits, Some(significant_digits))
    }
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<<T as BitBlockAccess>::Bits>>
    DoubleEndedIterator for PrimitivePowerOfTwoDigitIterator<T, U>
{
    /// A function to iterate through the digits of a primitive unsigned integer in descending order
    /// (most-significant first).
    ///
    /// The base is $2^\ell$ and the output type is `U`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::digits::power_of_two_digit_iterable::*;
    /// use malachite_base::num::conversion::traits::PowerOfTwoDigitIterable;
    ///
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
    /// assert_eq!(digits.next_back(), None);
    ///
    /// // 107 = 1101011b
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    /// assert_eq!(digits.next_back(), Some(1));
    /// assert_eq!(digits.next_back(), Some(2));
    /// assert_eq!(digits.next_back(), Some(2));
    /// assert_eq!(digits.next_back(), Some(3));
    /// assert_eq!(digits.next_back(), None);
    /// ```
    fn next_back(&mut self) -> Option<U> {
        if self.some_remaining {
            if self.i == self.j {
                self.some_remaining = false;
            }
            let digit = U::wrapping_from(self.value.get_bits(self.j, self.j + self.log_base));
            self.j.saturating_sub_assign(self.log_base);
            Some(digit)
        } else {
            None
        }
    }
}

/// This allows for some optimizations, _e.g._ when collecting into a `Vec`.
impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<<T as BitBlockAccess>::Bits>>
    ExactSizeIterator for PrimitivePowerOfTwoDigitIterator<T, U>
{
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<<T as BitBlockAccess>::Bits>>
    PowerOfTwoDigitIterator<U> for PrimitivePowerOfTwoDigitIterator<T, U>
{
    /// A function to retrieve base-$2^\ell$ digits by index.
    ///
    /// Indexing at or above the significant digit count returns zero. The output type is `U`.
    ///
    /// This function is stateless. It doesn't affect, and isn't affected by, the iterator's
    /// position.
    ///
    /// $f(x, \ell, i) = d_i$, where $0 \leq d_i < 2^\ell$ for all $i$ and
    /// $$
    /// \sum_{i=0}^\infty2^{\ell i}d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::digits::power_of_two_digit_iterable::*;
    /// use malachite_base::num::conversion::traits::{
    ///     PowerOfTwoDigitIterable,
    ///     PowerOfTwoDigitIterator
    /// };
    ///
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
    /// assert_eq!(digits.get(0), 0);
    ///
    /// // 107 = 1101011b
    /// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    /// assert_eq!(digits.get(0), 3);
    /// assert_eq!(digits.get(1), 2);
    /// assert_eq!(digits.get(2), 2);
    /// assert_eq!(digits.get(100), 0);
    /// ```
    fn get(&self, index: u64) -> U {
        let i = index * self.log_base;
        U::wrapping_from(self.value.get_bits(i, i + self.log_base))
    }
}

fn _power_of_two_digits<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    x: T,
    log_base: u64,
) -> PrimitivePowerOfTwoDigitIterator<T, U> {
    assert_ne!(log_base, 0);
    if log_base > U::WIDTH {
        panic!(
            "type {:?} is too small for a digit of width {}",
            U::NAME,
            log_base
        );
    }
    let significant_digits = x
        .significant_bits()
        .div_round(log_base, RoundingMode::Ceiling);
    PrimitivePowerOfTwoDigitIterator {
        value: x,
        log_base,
        some_remaining: significant_digits != 0,
        i: 0,
        j: significant_digits.saturating_sub(1) * log_base,
        phantom: PhantomData,
    }
}

macro_rules! impl_power_of_two_digit_iterable {
    ($t:ident) => {
        macro_rules! impl_power_of_two_digit_iterable_inner {
            ($u:ident) => {
                impl PowerOfTwoDigitIterable<$u> for $t {
                    type PowerOfTwoDigitIterator = PrimitivePowerOfTwoDigitIterator<$t, $u>;

                    /// Returns a double-ended iterator over the base-$2^\ell$ digits of a primitive
                    /// unsigned integer.
                    ///
                    /// The forward order is ascending, so that less-significant digits appear
                    /// first. There are no trailing zeros going forward, or leading zeros going
                    /// backward. The type of the digits is `$u`.
                    ///
                    /// If it's necessary to get a `Vec` of all the digits, consider using
                    /// `to_power_of_to_digits_asc` or `to_power_of_two_digits_desc` instead.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is larger than the width of `$u`.
                    ///
                    /// # Examples
                    /// See the documentation of the
                    /// `num::conversion::digits::power_of_two_digit_iterable`  module.
                    #[inline]
                    fn power_of_two_digits(
                        self,
                        log_base: u64,
                    ) -> PrimitivePowerOfTwoDigitIterator<$t, $u> {
                        _power_of_two_digits(self, log_base)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_power_of_two_digit_iterable_inner);
    };
}
apply_to_unsigneds!(impl_power_of_two_digit_iterable);
