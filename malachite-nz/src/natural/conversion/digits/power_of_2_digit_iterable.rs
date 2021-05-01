use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, DivRound, FloorLogBase2, ModPowerOf2, PowerOf2, SaturatingSubAssign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::digits::power_of_2_digit_iterable::*;
use malachite_base::num::conversion::traits::{
    ExactFrom, PowerOf2DigitIterable, PowerOf2DigitIterator,
};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode;
use natural::logic::bit_block_access::limbs_slice_get_bits;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::{min, Ordering};
use std::marker::PhantomData;
use std::slice::Chunks;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FitsInLimbIterator<'a, T>(FILIterator<'a, T>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct FILIterator<'a, T> {
    significant_digits: usize,
    limbs: &'a [Limb],
    log_base: u64,
    some_remaining: bool,
    limb_i: usize,
    limb_j: usize,
    // This index initially points to the least-significant digit, and is incremented by next().
    i: u64,
    // This index initially points to the most-significant nonzero digit, and is decremented by
    // next_back().
    j: u64,
    mask: Limb,
    phantom: PhantomData<*const T>,
}

impl<'a, T: PrimitiveUnsigned> Iterator for FILIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.some_remaining {
            let digit = T::wrapping_from((self.limbs[self.limb_i] >> self.i) & self.mask);
            if self.limb_i == self.limb_j && self.i == self.j {
                self.some_remaining = false;
            }
            self.i += self.log_base;
            if self.i == Limb::WIDTH {
                self.i = 0;
                self.limb_i += 1;
            }
            Some(digit)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_digits = self.significant_digits;
        (significant_digits, Some(significant_digits))
    }
}

impl<'a, T: PrimitiveUnsigned> DoubleEndedIterator for FILIterator<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.some_remaining {
            if self.limb_i == self.limb_j && self.i == self.j {
                self.some_remaining = false;
            }
            let digit = T::wrapping_from((self.limbs[self.limb_j] >> self.j) & self.mask);
            if self.j == 0 {
                self.j = Limb::WIDTH - self.log_base;
                self.limb_j.saturating_sub_assign(1);
            } else {
                self.j -= self.log_base;
            }
            Some(digit)
        } else {
            None
        }
    }
}

impl<'a, T: PrimitiveUnsigned> ExactSizeIterator for FILIterator<'a, T> {}

impl<'a, T: PrimitiveUnsigned> PowerOf2DigitIterator<T> for FILIterator<'a, T> {
    fn get(&self, index: u64) -> T {
        let log_log_base = self.log_base.floor_log_base_2();
        let log_ratio = Limb::LOG_WIDTH - log_log_base;
        let limb_index = usize::exact_from(index >> log_ratio);
        let digit_index = index.mod_power_of_2(log_ratio);
        if limb_index < self.limbs.len() {
            T::wrapping_from((self.limbs[limb_index] >> (digit_index << log_log_base)) & self.mask)
        } else {
            T::ZERO
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SizeOfLimbIterator<'a, T>(SOLIterator<'a, T>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct SOLIterator<'a, T> {
    limbs: &'a [Limb],
    some_remaining: bool,
    // This index initially points to the least-significant digit, and is incremented by next().
    i: usize,
    // This index initially points to the most-significant nonzero digit, and is decremented by
    // next_back().
    j: usize,
    phantom: PhantomData<*const T>,
}

impl<'a, T: PrimitiveUnsigned> Iterator for SOLIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.some_remaining {
            let digit = T::wrapping_from(self.limbs[self.i]);
            if self.i == self.j {
                self.some_remaining = false;
            }
            self.i += 1;
            Some(digit)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_digits = self.limbs.len();
        (significant_digits, Some(significant_digits))
    }
}

impl<'a, T: PrimitiveUnsigned> DoubleEndedIterator for SOLIterator<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.some_remaining {
            let digit = T::wrapping_from(self.limbs[self.j]);
            if self.i == self.j {
                self.some_remaining = false;
            }
            self.j.saturating_sub_assign(1);
            Some(digit)
        } else {
            None
        }
    }
}

impl<'a, T: PrimitiveUnsigned> ExactSizeIterator for SOLIterator<'a, T> {}

impl<'a, T: PrimitiveUnsigned> SOLIterator<'a, T> {
    fn get(&self, index: u64) -> T {
        let index = usize::exact_from(index);
        if index < self.limbs.len() {
            T::wrapping_from(self.limbs[index])
        } else {
            T::ZERO
        }
    }
}

#[derive(Clone, Debug)]
pub struct MultipleOfLimbIterator<'a, T>(MOLIterator<'a, T>);

#[derive(Clone, Debug)]
struct MOLIterator<'a, T> {
    significant_digits: usize,
    log_ratio: u64,
    limbs: &'a [Limb],
    chunks: Chunks<'a, Limb>,
    phantom: PhantomData<*const T>,
}

impl<'a, T: PrimitiveUnsigned> Iterator for MOLIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.chunks.next().map(T::from_other_type_slice)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_digits = self.significant_digits;
        (significant_digits, Some(significant_digits))
    }
}

impl<'a, T: PrimitiveUnsigned> DoubleEndedIterator for MOLIterator<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.chunks.next_back().map(T::from_other_type_slice)
    }
}

impl<'a, T: PrimitiveUnsigned> ExactSizeIterator for MOLIterator<'a, T> {}

impl<'a, T: PrimitiveUnsigned> PowerOf2DigitIterator<T> for MOLIterator<'a, T> {
    fn get(&self, index: u64) -> T {
        let start_index = usize::exact_from(index << self.log_ratio);
        if start_index >= self.limbs.len() {
            T::ZERO
        } else {
            let end_index = min(
                self.limbs.len(),
                start_index + usize::power_of_2(self.log_ratio),
            );
            T::from_other_type_slice(&self.limbs[start_index..end_index])
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IrregularIterator<'a, T>(IIterator<'a, T>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct IIterator<'a, T> {
    significant_digits: usize,
    limbs: &'a [Limb],
    log_base: u64,
    some_remaining: bool,
    // This index initially points to the least-significant digit, and is incremented by next().
    i: u64,
    // This index initially points to the most-significant nonzero digit, and is decremented by
    // next_back().
    j: u64,
    phantom: PhantomData<*const T>,
}

impl<'a, T: PrimitiveUnsigned> Iterator for IIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.some_remaining {
            let digit = self.get(self.i);
            if self.i == self.j {
                self.some_remaining = false;
            }
            self.i += 1;
            Some(digit)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_digits = self.significant_digits;
        (significant_digits, Some(significant_digits))
    }
}

impl<'a, T: PrimitiveUnsigned> DoubleEndedIterator for IIterator<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.some_remaining {
            let digit = self.get(self.j);
            if self.i == self.j {
                self.some_remaining = false;
            }
            self.j.saturating_sub_assign(1);
            Some(digit)
        } else {
            None
        }
    }
}

impl<'a, T: PrimitiveUnsigned> ExactSizeIterator for IIterator<'a, T> {}

impl<'a, T: PrimitiveUnsigned> IIterator<'a, T> {
    fn get(&self, index: u64) -> T {
        let start = index * self.log_base;
        let limb_start = usize::exact_from(start >> Limb::LOG_WIDTH);
        let len = self.limbs.len();
        let mut result = T::ZERO;
        if limb_start >= len {
            return result;
        }
        let mut result_index = 0;
        let mut limb_index = start & Limb::WIDTH_MASK;
        for &limb in &self.limbs[limb_start..] {
            let remaining_result_bits = self.log_base - result_index;
            let remaining_limb_bits = Limb::WIDTH - limb_index;
            if remaining_limb_bits <= remaining_result_bits {
                result |= T::wrapping_from(limb >> limb_index) << result_index;
                result_index += remaining_limb_bits;
                limb_index = 0;
            } else {
                result |=
                    T::wrapping_from((limb >> limb_index).mod_power_of_2(remaining_result_bits))
                        << result_index;
                break;
            }
        }
        result
    }
}

/// A double-ended iterator over the digits of a `Natural`, where the base is a power of 2. The
/// base-2 logarithm of the base is specified. The type of each digit is `T`, and `log_base` must be
/// no larger than the width of `T`. The forward order is ascending (least-significant first). The
/// iterator does not iterate over the implicit leading zero digits.
///
/// This struct also supports retrieving digits by index. This functionality is completely
/// independent of the iterator's state. Indexing the implicit leading zero digits is allowed.
#[derive(Clone, Debug)]
pub enum NaturalPowerOf2DigitPrimitiveIterator<'a, T: PrimitiveUnsigned> {
    Small(PrimitivePowerOf2DigitIterator<Limb, T>),
    FitsInLimb(FitsInLimbIterator<'a, T>),
    SizeOfLimb(SizeOfLimbIterator<'a, T>),
    MultipleOfLimb(MultipleOfLimbIterator<'a, T>),
    Irregular(IrregularIterator<'a, T>),
}

impl<'a, T: PrimitiveUnsigned> Iterator for NaturalPowerOf2DigitPrimitiveIterator<'a, T> {
    type Item = T;

    /// A function to iterate through the digits of a `Natural` in ascending order (least-
    /// significant first), where the base is a power of 2.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2).next(), None);
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2);
    /// assert_eq!(digits.next(), Some(3));
    /// assert_eq!(digits.next(), Some(2));
    /// assert_eq!(digits.next(), Some(2));
    /// assert_eq!(digits.next(), Some(1));
    /// assert_eq!(digits.next(), None);
    /// ```
    fn next(&mut self) -> Option<T> {
        match *self {
            NaturalPowerOf2DigitPrimitiveIterator::Small(ref mut xs) => xs.next(),
            NaturalPowerOf2DigitPrimitiveIterator::FitsInLimb(ref mut xs) => xs.0.next(),
            NaturalPowerOf2DigitPrimitiveIterator::SizeOfLimb(ref mut xs) => xs.0.next(),
            NaturalPowerOf2DigitPrimitiveIterator::MultipleOfLimb(ref mut xs) => xs.0.next(),
            NaturalPowerOf2DigitPrimitiveIterator::Irregular(ref mut xs) => xs.0.next(),
        }
    }

    /// A function that returns the length of the digits iterator; that is, the `Natural`'s
    /// significant base-2<sup>`log_base`</sup>-digit count. The format is
    /// (lower bound, Option<upper bound>), but in this case it's trivial to always have an exact
    /// bound.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2).size_hint(),
    ///     (0, Some(0))
    /// );
    ///
    /// let n = Natural::from(105u32);
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<u32>::power_of_2_digits(&n, 2).size_hint(),
    ///     (4, Some(4))
    /// );
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        match *self {
            NaturalPowerOf2DigitPrimitiveIterator::Small(ref xs) => xs.size_hint(),
            NaturalPowerOf2DigitPrimitiveIterator::FitsInLimb(ref xs) => xs.0.size_hint(),
            NaturalPowerOf2DigitPrimitiveIterator::SizeOfLimb(ref xs) => xs.0.size_hint(),
            NaturalPowerOf2DigitPrimitiveIterator::MultipleOfLimb(ref xs) => xs.0.size_hint(),
            NaturalPowerOf2DigitPrimitiveIterator::Irregular(ref xs) => xs.0.size_hint(),
        }
    }
}

impl<'a, T: PrimitiveUnsigned> DoubleEndedIterator
    for NaturalPowerOf2DigitPrimitiveIterator<'a, T>
{
    /// A function to iterate through the digits of a `Natural` in descending order (most-
    /// significant first), where the base is a power of 2.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2).next(), None);
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2);
    /// assert_eq!(digits.next_back(), Some(1));
    /// assert_eq!(digits.next_back(), Some(2));
    /// assert_eq!(digits.next_back(), Some(2));
    /// assert_eq!(digits.next_back(), Some(3));
    /// assert_eq!(digits.next_back(), None);
    /// ```
    fn next_back(&mut self) -> Option<T> {
        match *self {
            NaturalPowerOf2DigitPrimitiveIterator::Small(ref mut xs) => xs.next_back(),
            NaturalPowerOf2DigitPrimitiveIterator::FitsInLimb(ref mut xs) => xs.0.next_back(),
            NaturalPowerOf2DigitPrimitiveIterator::SizeOfLimb(ref mut xs) => xs.0.next_back(),
            NaturalPowerOf2DigitPrimitiveIterator::MultipleOfLimb(ref mut xs) => xs.0.next_back(),
            NaturalPowerOf2DigitPrimitiveIterator::Irregular(ref mut xs) => xs.0.next_back(),
        }
    }
}

impl<'a, T: PrimitiveUnsigned> ExactSizeIterator for NaturalPowerOf2DigitPrimitiveIterator<'a, T> {}

impl<'a, T: PrimitiveUnsigned> PowerOf2DigitIterator<T>
    for NaturalPowerOf2DigitPrimitiveIterator<'a, T>
{
    /// A function to retrieve digits by index, where the base is a power of 2. The base-2
    /// logarithm of the base is specified. The type of each digit is `T`, and `log_base` must be no
    /// larger than the width of `T`. Indexing at or above the significant digit count returns
    /// zeros.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::{
    ///     PowerOf2DigitIterable,
    ///     PowerOf2DigitIterator
    /// };
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2).get(0), 0);
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// let digits = PowerOf2DigitIterable::<u32>::power_of_2_digits(&n, 2);
    /// assert_eq!(digits.get(0), 3);
    /// assert_eq!(digits.get(1), 2);
    /// assert_eq!(digits.get(2), 2);
    /// assert_eq!(digits.get(3), 1);
    /// assert_eq!(digits.get(4), 0);
    /// assert_eq!(digits.get(100), 0);
    /// ```
    fn get(&self, index: u64) -> T {
        match *self {
            NaturalPowerOf2DigitPrimitiveIterator::Small(ref xs) => xs.get(index),
            NaturalPowerOf2DigitPrimitiveIterator::FitsInLimb(ref xs) => xs.0.get(index),
            NaturalPowerOf2DigitPrimitiveIterator::SizeOfLimb(ref xs) => xs.0.get(index),
            NaturalPowerOf2DigitPrimitiveIterator::MultipleOfLimb(ref xs) => xs.0.get(index),
            NaturalPowerOf2DigitPrimitiveIterator::Irregular(ref xs) => xs.0.get(index),
        }
    }
}

fn fits_in_limb_iterator<T: PrimitiveUnsigned>(
    xs: &[Limb],
    log_base: u64,
) -> FitsInLimbIterator<'_, T> {
    let significant_bits = limbs_significant_bits(xs);
    let log_log_base = log_base.floor_log_base_2();
    let significant_digits = significant_bits.shr_round(log_log_base, RoundingMode::Ceiling);
    FitsInLimbIterator(FILIterator {
        significant_digits: usize::exact_from(significant_digits),
        limbs: xs,
        log_base,
        some_remaining: true,
        limb_i: 0,
        limb_j: xs.len() - 1,
        i: 0,
        j: (significant_digits - 1).mod_power_of_2(Limb::LOG_WIDTH - log_log_base) << log_log_base,
        mask: Limb::low_mask(log_base),
        phantom: PhantomData,
    })
}

fn size_of_limb_iterator<T: PrimitiveUnsigned>(xs: &[Limb]) -> SizeOfLimbIterator<'_, T> {
    SizeOfLimbIterator(SOLIterator {
        limbs: xs,
        some_remaining: true,
        i: 0,
        j: xs.len() - 1,
        phantom: PhantomData,
    })
}

fn multiple_of_limb_iterator<T: PrimitiveUnsigned>(
    xs: &[Limb],
    log_base: u64,
) -> MultipleOfLimbIterator<'_, T> {
    let log_log_base = log_base.floor_log_base_2();
    let log_ratio = log_log_base - Limb::LOG_WIDTH;
    let significant_digits = xs.len().shr_round(log_ratio, RoundingMode::Ceiling);
    MultipleOfLimbIterator(MOLIterator {
        significant_digits,
        log_ratio,
        limbs: xs,
        chunks: xs.chunks(usize::power_of_2(log_ratio)),
        phantom: PhantomData,
    })
}

fn irregular_iterator<T: PrimitiveUnsigned>(
    xs: &[Limb],
    log_base: u64,
) -> IrregularIterator<'_, T> {
    let significant_digits = limbs_significant_bits(xs).div_round(log_base, RoundingMode::Ceiling);
    IrregularIterator(IIterator {
        significant_digits: usize::exact_from(significant_digits),
        limbs: xs,
        log_base,
        some_remaining: true,
        i: 0,
        j: significant_digits - 1,
        phantom: PhantomData,
    })
}

fn _power_of_2_digits<T: PrimitiveUnsigned>(
    x: &Natural,
    log_base: u64,
) -> NaturalPowerOf2DigitPrimitiveIterator<T>
where
    Limb: PowerOf2DigitIterable<T, PowerOf2DigitIterator = PrimitivePowerOf2DigitIterator<Limb, T>>,
{
    assert_ne!(log_base, 0);
    if log_base > T::WIDTH {
        panic!(
            "type {:?} is too small for a digit of width {}",
            T::NAME,
            log_base
        );
    }
    match x {
        Natural(Small(small)) => NaturalPowerOf2DigitPrimitiveIterator::Small(
            PowerOf2DigitIterable::<T>::power_of_2_digits(*small, log_base),
        ),
        Natural(Large(ref limbs)) => {
            if let Some(log_log_base) = log_base.checked_log_base_2() {
                match log_log_base.cmp(&Limb::LOG_WIDTH) {
                    Ordering::Equal => NaturalPowerOf2DigitPrimitiveIterator::SizeOfLimb(
                        size_of_limb_iterator(limbs),
                    ),
                    Ordering::Less => NaturalPowerOf2DigitPrimitiveIterator::FitsInLimb(
                        fits_in_limb_iterator(limbs, log_base),
                    ),
                    Ordering::Greater => NaturalPowerOf2DigitPrimitiveIterator::MultipleOfLimb(
                        multiple_of_limb_iterator(limbs, log_base),
                    ),
                }
            } else {
                NaturalPowerOf2DigitPrimitiveIterator::Irregular(irregular_iterator(
                    limbs, log_base,
                ))
            }
        }
    }
}

macro_rules! iterables {
    (
        $t: ident
    ) => {
        impl<'a> PowerOf2DigitIterable<$t> for &'a Natural {
            type PowerOf2DigitIterator = NaturalPowerOf2DigitPrimitiveIterator<'a, $t>;

            /// Returns a double-ended iterator over the digits of a `Natural`, where the base is a
            /// power of 2. The base-2 logarithm of the base is specified. The type of each digit
            /// is `T`, and `log_base` must be no larger than the width of `T`. The forward order is
            /// ascending, so that less significant digits appear first. There are no trailing zero
            /// digits going forward, or leading zero digits going backward.
            ///
            /// If it's necessary to get a `Vec` of all the digits, consider using
            /// `to_power_of_2_digits_asc` or `to_power_of_2_digits_desc` instead.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// extern crate itertools;
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use itertools::Itertools;
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
            /// use malachite_nz::natural::Natural;
            ///
            /// let n = Natural::ZERO;
            /// assert!(PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2).next().is_none());
            ///
            /// // 107 = 1223_4
            /// let n = Natural::from(107u32);
            /// assert_eq!(
            ///     PowerOf2DigitIterable::<u32>::power_of_2_digits(&n, 2).collect_vec(),
            ///     vec![3, 2, 2, 1]
            /// );
            ///
            /// let n = Natural::ZERO;
            /// assert!(
            ///     PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2).next_back().is_none()
            /// );
            ///
            /// // 107 = 1223_4
            /// let n = Natural::from(107u32);
            /// assert_eq!(
            ///     PowerOf2DigitIterable::<u32>::power_of_2_digits(&n, 2).rev().collect_vec(),
            ///     vec![1, 2, 2, 3]
            /// );
            /// ```
            #[inline]
            fn power_of_2_digits(
                self,
                log_base: u64,
            ) -> NaturalPowerOf2DigitPrimitiveIterator<'a, $t> {
                _power_of_2_digits(self, log_base)
            }
        }
    };
}
apply_to_unsigneds!(iterables);

#[derive(Clone, Debug)]
pub struct NaturalMultipleOfLimbIterator<'a>(NMOLIterator<'a>);

#[derive(Clone, Debug)]
struct NMOLIterator<'a> {
    significant_digits: usize,
    log_ratio: u64,
    limbs: &'a [Limb],
    chunks: Chunks<'a, Limb>,
}

impl<'a> Iterator for NMOLIterator<'a> {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        self.chunks.next().map(Natural::from_limbs_asc)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_digits = self.significant_digits;
        (significant_digits, Some(significant_digits))
    }
}

impl<'a> DoubleEndedIterator for NMOLIterator<'a> {
    fn next_back(&mut self) -> Option<Natural> {
        self.chunks.next_back().map(Natural::from_limbs_asc)
    }
}

impl<'a> ExactSizeIterator for NMOLIterator<'a> {}

impl<'a> PowerOf2DigitIterator<Natural> for NMOLIterator<'a> {
    fn get(&self, index: u64) -> Natural {
        let start_index = usize::exact_from(index << self.log_ratio);
        if start_index >= self.limbs.len() {
            Natural::ZERO
        } else {
            let end_index = min(
                self.limbs.len(),
                start_index + usize::power_of_2(self.log_ratio),
            );
            Natural::from_limbs_asc(&self.limbs[start_index..end_index])
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NaturalIrregularIterator<'a>(NIIterator<'a>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct NIIterator<'a> {
    significant_digits: usize,
    limbs: &'a [Limb],
    log_base: u64,
    some_remaining: bool,
    // This index initially points to the least-significant digit, and is incremented by next().
    i: u64,
    // This index initially points to the most-significant nonzero digit, and is decremented by
    // next_back().
    j: u64,
}

impl<'a> Iterator for NIIterator<'a> {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.some_remaining {
            let digit = self.get(self.i);
            if self.i == self.j {
                self.some_remaining = false;
            }
            self.i += 1;
            Some(digit)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let significant_digits = self.significant_digits;
        (significant_digits, Some(significant_digits))
    }
}

impl<'a> DoubleEndedIterator for NIIterator<'a> {
    fn next_back(&mut self) -> Option<Natural> {
        if self.some_remaining {
            let digit = self.get(self.j);
            if self.i == self.j {
                self.some_remaining = false;
            }
            self.j.saturating_sub_assign(1);
            Some(digit)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for NIIterator<'a> {}

impl<'a> NIIterator<'a> {
    fn get(&self, index: u64) -> Natural {
        let start_index = index.checked_mul(self.log_base).unwrap();
        Natural::from_owned_limbs_asc(limbs_slice_get_bits(
            self.limbs,
            start_index,
            start_index + self.log_base,
        ))
    }
}

/// A double-ended iterator over the digits of a `Natural`, where the base is a power of 2. The
/// base-2 logarithm of the base is specified. The type of each digit is `Natural`. The forward
/// order is ascending (least-significant first). The iterator does not iterate over the implicit
/// leading zero digits.
///
/// This struct also supports retrieving digits by index. This functionality is completely
/// independent of the iterator's state. Indexing the implicit leading zero digits is allowed.
#[derive(Clone, Debug)]
pub enum NaturalPowerOf2DigitIterator<'a> {
    Small(PrimitivePowerOf2DigitIterator<Limb, Limb>),
    SmallerThanLimb(NaturalPowerOf2DigitPrimitiveIterator<'a, Limb>),
    MultipleOfLimb(NaturalMultipleOfLimbIterator<'a>),
    Irregular(NaturalIrregularIterator<'a>),
}

impl<'a> Iterator for NaturalPowerOf2DigitIterator<'a> {
    type Item = Natural;

    /// A function to iterate through the digits of a `Natural` in ascending order (least-
    /// significant first), where the base is a power of 2.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `log_base`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).next(), None);
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2);
    /// assert_eq!(digits.next(), Some(Natural::from(3u32)));
    /// assert_eq!(digits.next(), Some(Natural::from(2u32)));
    /// assert_eq!(digits.next(), Some(Natural::from(2u32)));
    /// assert_eq!(digits.next(), Some(Natural::from(1u32)));
    /// assert_eq!(digits.next(), None);
    /// ```
    fn next(&mut self) -> Option<Natural> {
        match *self {
            NaturalPowerOf2DigitIterator::Small(ref mut xs) => xs.next().map(Natural::from),
            NaturalPowerOf2DigitIterator::SmallerThanLimb(ref mut xs) => {
                xs.next().map(Natural::from)
            }
            NaturalPowerOf2DigitIterator::MultipleOfLimb(ref mut xs) => xs.0.next(),
            NaturalPowerOf2DigitIterator::Irregular(ref mut xs) => xs.0.next(),
        }
    }

    /// A function that returns the length of the digits iterator; that is, the `Natural`'s
    /// significant base-2<sup>`log_base`</sup>-digit count. The format is
    /// (lower bound, Option<upper bound>), but in this case it's trivial to always have an exact
    /// bound.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).size_hint(),
    ///     (0, Some(0))
    /// );
    ///
    /// let n = Natural::from(105u32);
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).size_hint(),
    ///     (4, Some(4))
    /// );
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        match *self {
            NaturalPowerOf2DigitIterator::Small(ref xs) => xs.size_hint(),
            NaturalPowerOf2DigitIterator::SmallerThanLimb(ref xs) => xs.size_hint(),
            NaturalPowerOf2DigitIterator::MultipleOfLimb(ref xs) => xs.0.size_hint(),
            NaturalPowerOf2DigitIterator::Irregular(ref xs) => xs.0.size_hint(),
        }
    }
}

impl<'a> DoubleEndedIterator for NaturalPowerOf2DigitIterator<'a> {
    /// A function to iterate through the digits of a `Natural` in descending order (most-
    /// significant first), where the base is a power of 2.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `log_base`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).next(), None);
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2);
    /// assert_eq!(digits.next_back(), Some(Natural::from(1u32)));
    /// assert_eq!(digits.next_back(), Some(Natural::from(2u32)));
    /// assert_eq!(digits.next_back(), Some(Natural::from(2u32)));
    /// assert_eq!(digits.next_back(), Some(Natural::from(3u32)));
    /// assert_eq!(digits.next_back(), None);
    /// ```
    fn next_back(&mut self) -> Option<Natural> {
        match *self {
            NaturalPowerOf2DigitIterator::Small(ref mut xs) => xs.next_back().map(Natural::from),
            NaturalPowerOf2DigitIterator::SmallerThanLimb(ref mut xs) => {
                xs.next_back().map(Natural::from)
            }
            NaturalPowerOf2DigitIterator::MultipleOfLimb(ref mut xs) => xs.0.next_back(),
            NaturalPowerOf2DigitIterator::Irregular(ref mut xs) => xs.0.next_back(),
        }
    }
}

impl<'a> ExactSizeIterator for NaturalPowerOf2DigitIterator<'a> {}

impl<'a> PowerOf2DigitIterator<Natural> for NaturalPowerOf2DigitIterator<'a> {
    /// A function to retrieve digits by index, where the base is a power of 2. The base-2
    /// logarithm of the base is specified. The type of each digit is `Natural`. Indexing at or
    /// above the significant digit count returns zeros.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `log_base`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::{
    ///     PowerOf2DigitIterable,
    ///     PowerOf2DigitIterator
    /// };
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).get(0), 0);
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// let digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2);
    /// assert_eq!(digits.get(0), Natural::from(3u32));
    /// assert_eq!(digits.get(1), Natural::from(2u32));
    /// assert_eq!(digits.get(2), Natural::from(2u32));
    /// assert_eq!(digits.get(3), Natural::from(1u32));
    /// assert_eq!(digits.get(4), Natural::from(0u32));
    /// assert_eq!(digits.get(100), Natural::from(0u32));
    /// ```
    fn get(&self, index: u64) -> Natural {
        match *self {
            NaturalPowerOf2DigitIterator::Small(ref xs) => Natural::from(xs.get(index)),
            NaturalPowerOf2DigitIterator::SmallerThanLimb(ref xs) => Natural::from(xs.get(index)),
            NaturalPowerOf2DigitIterator::MultipleOfLimb(ref xs) => xs.0.get(index),
            NaturalPowerOf2DigitIterator::Irregular(ref xs) => xs.0.get(index),
        }
    }
}

fn multiple_of_limb_fn(xs: &[Limb], log_base: u64) -> NaturalMultipleOfLimbIterator<'_> {
    let log_log_base = log_base.floor_log_base_2();
    let log_ratio = log_log_base - Limb::LOG_WIDTH;
    let significant_digits = xs.len().shr_round(log_ratio, RoundingMode::Ceiling);
    NaturalMultipleOfLimbIterator(NMOLIterator {
        significant_digits,
        log_ratio,
        limbs: xs,
        chunks: xs.chunks(usize::power_of_2(log_ratio)),
    })
}

fn irregular_fn(xs: &[Limb], log_base: u64) -> NaturalIrregularIterator<'_> {
    let significant_digits = limbs_significant_bits(xs).div_round(log_base, RoundingMode::Ceiling);
    NaturalIrregularIterator(NIIterator {
        significant_digits: usize::exact_from(significant_digits),
        limbs: xs,
        log_base,
        some_remaining: true,
        i: 0,
        j: significant_digits - 1,
    })
}

impl<'a> PowerOf2DigitIterable<Natural> for &'a Natural {
    type PowerOf2DigitIterator = NaturalPowerOf2DigitIterator<'a>;

    /// Returns a double-ended iterator over the digits of a `Natural`, where the base is a power of
    /// two. The base-2 logarithm of the base is specified. The type of each digit is `Natural`. The
    /// forward order is ascending, so that less significant digits appear first. There are no
    /// trailing zero digits going forward, or leading zero digits going backward.
    ///
    /// If it's necessary to get a `Vec` of all the digits, consider using
    /// `to_power_of_2_digits_asc` or `to_power_of_2_digits_desc` instead.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate itertools;
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use itertools::Itertools;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert!(PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).next().is_none());
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).collect_vec(),
    ///     vec![Natural::from(3u32), Natural::from(2u32), Natural::from(2u32), Natural::from(1u32)]
    /// );
    ///
    /// let n = Natural::ZERO;
    /// assert!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).next_back().is_none()
    /// );
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).rev().collect_vec(),
    ///     vec![Natural::from(1u32), Natural::from(2u32), Natural::from(2u32), Natural::from(3u32)]
    /// );
    /// ```
    fn power_of_2_digits(self, log_base: u64) -> NaturalPowerOf2DigitIterator<'a> {
        assert_ne!(log_base, 0);
        match self {
            Natural(Small(small)) => NaturalPowerOf2DigitIterator::Small(PowerOf2DigitIterable::<
                Limb,
            >::power_of_2_digits(
                *small,
                min(log_base, Limb::WIDTH),
            )),
            Natural(Large(ref limbs)) => {
                if let Some(log_log_base) = log_base.checked_log_base_2() {
                    if log_log_base <= Limb::LOG_WIDTH {
                        NaturalPowerOf2DigitIterator::SmallerThanLimb(
                            PowerOf2DigitIterable::<Limb>::power_of_2_digits(self, log_base),
                        )
                    } else {
                        NaturalPowerOf2DigitIterator::MultipleOfLimb(multiple_of_limb_fn(
                            limbs, log_base,
                        ))
                    }
                } else {
                    NaturalPowerOf2DigitIterator::Irregular(irregular_fn(limbs, log_base))
                }
            }
        }
    }
}
