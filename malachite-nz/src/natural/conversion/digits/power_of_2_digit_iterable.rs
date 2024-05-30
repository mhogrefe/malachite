// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::logic::bit_block_access::limbs_slice_get_bits;
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::{min, Ordering::*};
use core::marker::PhantomData;
use core::slice::Chunks;
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
use malachite_base::rounding_modes::RoundingMode::*;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FitsInLimbIterator<'a, T>(FILIterator<'a, T>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct FILIterator<'a, T> {
    limbs: &'a [Limb],
    log_base: u64,
    remaining: usize,
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
        if self.remaining != 0 {
            let digit = T::wrapping_from((self.limbs[self.limb_i] >> self.i) & self.mask);
            self.i += self.log_base;
            if self.i == Limb::WIDTH {
                self.i = 0;
                self.limb_i += 1;
            }
            self.remaining -= 1;
            Some(digit)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, T: PrimitiveUnsigned> DoubleEndedIterator for FILIterator<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.remaining != 0 {
            let digit = T::wrapping_from((self.limbs[self.limb_j] >> self.j) & self.mask);
            if self.j == 0 {
                self.j = Limb::WIDTH - self.log_base;
                self.limb_j.saturating_sub_assign(1);
            } else {
                self.j -= self.log_base;
            }
            self.remaining -= 1;
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

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SizeOfLimbIterator<'a, T>(SOLIterator<'a, T>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct SOLIterator<'a, T> {
    limbs: &'a [Limb],
    remaining: usize,
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
        if self.remaining != 0 {
            let digit = T::wrapping_from(self.limbs[self.i]);
            self.i += 1;
            self.remaining -= 1;
            Some(digit)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, T: PrimitiveUnsigned> DoubleEndedIterator for SOLIterator<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.remaining != 0 {
            let digit = T::wrapping_from(self.limbs[self.j]);
            self.j.saturating_sub_assign(1);
            self.remaining -= 1;
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

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct MultipleOfLimbIterator<'a, T>(MOLIterator<'a, T>);

#[derive(Clone, Debug)]
struct MOLIterator<'a, T> {
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chunks.size_hint()
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

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IrregularIterator<'a, T>(IIterator<'a, T>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct IIterator<'a, T> {
    limbs: &'a [Limb],
    log_base: u64,
    remaining: usize,
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
        if self.remaining != 0 {
            let digit = self.get(self.i);
            self.i += 1;
            self.remaining -= 1;
            Some(digit)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, T: PrimitiveUnsigned> DoubleEndedIterator for IIterator<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        if self.remaining != 0 {
            let digit = self.get(self.j);
            self.j.saturating_sub_assign(1);
            self.remaining -= 1;
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

/// A double-ended iterator over the base-$2^k$ $digits of a [`Natural`].
///
/// The base-2 logarithm of the base is specified. Each digit has primitive integer type, and
/// `log_base` must be no larger than the width of that type. The forward order is ascending
/// (least-significant first). The iterator does not iterate over the implicit leading zero digits.
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

    /// Iterates through the base-$2^k$ digits of a [`Natural`] in ascending order
    /// (least-significant first).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn next(&mut self) -> Option<T> {
        match *self {
            NaturalPowerOf2DigitPrimitiveIterator::Small(ref mut xs) => xs.next(),
            NaturalPowerOf2DigitPrimitiveIterator::FitsInLimb(ref mut xs) => xs.0.next(),
            NaturalPowerOf2DigitPrimitiveIterator::SizeOfLimb(ref mut xs) => xs.0.next(),
            NaturalPowerOf2DigitPrimitiveIterator::MultipleOfLimb(ref mut xs) => xs.0.next(),
            NaturalPowerOf2DigitPrimitiveIterator::Irregular(ref mut xs) => xs.0.next(),
        }
    }

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
    /// Iterates through the base-$2^k$ digits of a [`Natural`] in descending order
    /// (most-significant first).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
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
    /// Retrieves the base-$2^k$ digits of a [`Natural`] by index.
    ///
    /// $f(x, k, i) = d_i$, where $0 \leq d_i < 2^k$ for all $i$ and
    /// $$
    /// \sum_{i=0}^\infty2^{ki}d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::{
    ///     PowerOf2DigitIterable, PowerOf2DigitIterator,
    /// };
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2).get(0),
    ///     0
    /// );
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
    let significant_digits = significant_bits.shr_round(log_log_base, Ceiling).0;
    FitsInLimbIterator(FILIterator {
        limbs: xs,
        log_base,
        remaining: usize::exact_from(significant_digits),
        limb_i: 0,
        limb_j: xs.len() - 1,
        i: 0,
        j: (significant_digits - 1).mod_power_of_2(Limb::LOG_WIDTH - log_log_base) << log_log_base,
        mask: Limb::low_mask(log_base),
        phantom: PhantomData,
    })
}

const fn size_of_limb_iterator<T: PrimitiveUnsigned>(xs: &[Limb]) -> SizeOfLimbIterator<'_, T> {
    SizeOfLimbIterator(SOLIterator {
        limbs: xs,
        remaining: xs.len(),
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
    MultipleOfLimbIterator(MOLIterator {
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
    let significant_digits = limbs_significant_bits(xs).div_round(log_base, Ceiling).0;
    IrregularIterator(IIterator {
        limbs: xs,
        log_base,
        remaining: usize::exact_from(significant_digits),
        i: 0,
        j: significant_digits - 1,
        phantom: PhantomData,
    })
}

fn power_of_2_digits<T: PrimitiveUnsigned>(
    x: &Natural,
    log_base: u64,
) -> NaturalPowerOf2DigitPrimitiveIterator<T>
where
    Limb: PowerOf2DigitIterable<T, PowerOf2DigitIterator = PrimitivePowerOf2DigitIterator<Limb, T>>,
{
    assert_ne!(log_base, 0);
    assert!(
        log_base <= T::WIDTH,
        "type {:?} is too small for a digit of width {}",
        T::NAME,
        log_base
    );
    match x {
        Natural(Small(small)) => NaturalPowerOf2DigitPrimitiveIterator::Small(
            PowerOf2DigitIterable::<T>::power_of_2_digits(*small, log_base),
        ),
        Natural(Large(ref limbs)) => {
            if let Some(log_log_base) = log_base.checked_log_base_2() {
                match log_log_base.cmp(&Limb::LOG_WIDTH) {
                    Equal => NaturalPowerOf2DigitPrimitiveIterator::SizeOfLimb(
                        size_of_limb_iterator(limbs),
                    ),
                    Less => NaturalPowerOf2DigitPrimitiveIterator::FitsInLimb(
                        fits_in_limb_iterator(limbs, log_base),
                    ),
                    Greater => NaturalPowerOf2DigitPrimitiveIterator::MultipleOfLimb(
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

            /// Returns a double-ended iterator over the base-$2^k$ digits of a [`Natural`].
            ///
            /// The base-2 logarithm of the base is specified. Each digit has primitive integer
            /// type, and `log_base` must be no larger than the width of that type. The forward
            /// order is ascending, so that less significant digits appear first. There are no
            /// trailing zero digits going forward, or leading zero digits going backward.
            ///
            /// If it's necessary to get a [`Vec`] of all the digits, consider using
            /// [`to_power_of_2_digits_asc`](malachite_base::num::conversion::traits::PowerOf2Digits::to_power_of_2_digits_asc)
            /// or
            /// [`to_power_of_2_digits_desc`](malachite_base::num::conversion::traits::PowerOf2Digits::to_power_of_2_digits_desc)
            /// instead.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::power_of_2_digit_iterable#power_of_2_digits).
            #[inline]
            fn power_of_2_digits(
                self,
                log_base: u64,
            ) -> NaturalPowerOf2DigitPrimitiveIterator<'a, $t> {
                power_of_2_digits(self, log_base)
            }
        }
    };
}
apply_to_unsigneds!(iterables);

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct NaturalMultipleOfLimbIterator<'a>(NMOLIterator<'a>);

#[derive(Clone, Debug)]
struct NMOLIterator<'a> {
    log_ratio: u64,
    limbs: &'a [Limb],
    chunks: Chunks<'a, Limb>,
}

impl<'a> Iterator for NMOLIterator<'a> {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        self.chunks.next().map(Natural::from_limbs_asc)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chunks.size_hint()
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

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NaturalIrregularIterator<'a>(NIIterator<'a>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct NIIterator<'a> {
    limbs: &'a [Limb],
    log_base: u64,
    remaining: usize,
    // This index initially points to the least-significant digit, and is incremented by next().
    i: u64,
    // This index initially points to the most-significant nonzero digit, and is decremented by
    // next_back().
    j: u64,
}

impl<'a> Iterator for NIIterator<'a> {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.remaining != 0 {
            let digit = self.get(self.i);
            self.i += 1;
            self.remaining -= 1;
            Some(digit)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> DoubleEndedIterator for NIIterator<'a> {
    fn next_back(&mut self) -> Option<Natural> {
        if self.remaining != 0 {
            let digit = self.get(self.j);
            self.j.saturating_sub_assign(1);
            self.remaining -= 1;
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

/// A double-ended iterator over the base-$2^k$ digits of a [`Natural`].
///
/// The base-2 logarithm of the base is specified. The type of each digit is [`Natural`]. The
/// forward order is ascending (least-significant first). The iterator does not iterate over the
/// implicit leading zero digits.
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

    /// Iterates through the base-$2^k$ digits of a [`Natural`] in ascending order
    /// (least-significant first).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `log_base`.
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
    /// Iterate through the base-$2^k$ digits of a [`Natural`] in descending order (most-significant
    /// first).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `log_base`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).next(),
    ///     None
    /// );
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
    /// Retrieves the base-$2^k$ digits of a [`Natural`] by index.
    ///
    /// $f(x, k, i) = d_i$, where $0 \leq d_i < 2^k$ for all $i$ and
    /// $$
    /// \sum_{i=0}^\infty2^{ki}d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `log_base`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::{
    ///     PowerOf2DigitIterable, PowerOf2DigitIterator,
    /// };
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).get(0),
    ///     0
    /// );
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// let digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2);
    /// assert_eq!(digits.get(0), 3);
    /// assert_eq!(digits.get(1), 2);
    /// assert_eq!(digits.get(2), 2);
    /// assert_eq!(digits.get(3), 1);
    /// assert_eq!(digits.get(4), 0);
    /// assert_eq!(digits.get(100), 0);
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
    let log_ratio = log_base.floor_log_base_2() - Limb::LOG_WIDTH;
    NaturalMultipleOfLimbIterator(NMOLIterator {
        log_ratio,
        limbs: xs,
        chunks: xs.chunks(usize::power_of_2(log_ratio)),
    })
}

fn irregular_fn(xs: &[Limb], log_base: u64) -> NaturalIrregularIterator<'_> {
    let significant_digits = limbs_significant_bits(xs).div_round(log_base, Ceiling).0;
    NaturalIrregularIterator(NIIterator {
        limbs: xs,
        log_base,
        remaining: usize::exact_from(significant_digits),
        i: 0,
        j: significant_digits - 1,
    })
}

impl<'a> PowerOf2DigitIterable<Natural> for &'a Natural {
    type PowerOf2DigitIterator = NaturalPowerOf2DigitIterator<'a>;

    /// Returns a double-ended iterator over the base-$2^k$ digits of a [`Natural`].
    ///
    /// The base-2 logarithm of the base is specified. The type of each digit is [`Natural`]. The
    /// forward order is ascending, so that less significant digits appear first. There are no
    /// trailing zero digits going forward, or leading zero digits going backward.
    ///
    /// If it's necessary to get a [`Vec`] of all the digits, consider using
    /// [`to_power_of_2_digits_asc`](malachite_base::num::conversion::traits::PowerOf2Digits::to_power_of_2_digits_asc)
    /// or
    /// [`to_power_of_2_digits_desc`](malachite_base::num::conversion::traits::PowerOf2Digits::to_power_of_2_digits_desc)
    /// instead.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::PowerOf2DigitIterable;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::ZERO;
    /// assert!(PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2)
    ///     .next()
    ///     .is_none());
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2).collect_vec(),
    ///     vec![
    ///         Natural::from(3u32),
    ///         Natural::from(2u32),
    ///         Natural::from(2u32),
    ///         Natural::from(1u32)
    ///     ]
    /// );
    ///
    /// let n = Natural::ZERO;
    /// assert!(PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2)
    ///     .next_back()
    ///     .is_none());
    ///
    /// // 107 = 1223_4
    /// let n = Natural::from(107u32);
    /// assert_eq!(
    ///     PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2)
    ///         .rev()
    ///         .collect_vec(),
    ///     vec![
    ///         Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         Natural::from(2u32),
    ///         Natural::from(3u32)
    ///     ]
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
