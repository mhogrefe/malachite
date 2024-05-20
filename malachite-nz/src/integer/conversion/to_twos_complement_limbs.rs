// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::conversion::to_limbs::LimbIterator;
use crate::natural::logic::not::limbs_not_in_place;
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{IsPowerOf2, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::slices::slice_leading_zeros;

// Given the limbs of the absolute value of an `Integer`, in ascending order, returns the two's
// complement limbs. The input limbs should not be all zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_crate_test! {limbs_twos_complement(xs: &[Limb]) -> Vec<Limb> {
    let i = slice_leading_zeros(xs);
    let mut result = vec![0; i];
    if i != xs.len() {
        result.push(xs[i].wrapping_neg());
        for x in &xs[i + 1..] {
            result.push(!x);
        }
    }
    result
}}

// Given the limbs of a non-negative `Integer`, in ascending order, checks whether the most
// significant bit is `false`; if it isn't, appends an extra zero bit. This way the `Integer`'s
// non-negativity is preserved in its limbs.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {limbs_maybe_sign_extend_non_negative_in_place(xs: &mut Vec<Limb>) {
    if let Some(last) = xs.last() {
        if last.get_highest_bit() {
            // Sign-extend with an extra 0 limb to indicate a positive Integer
            xs.push(0);
        }
    }
}}

// Given the limbs of the absolute value of an `Integer`, in ascending order, converts the limbs to
// two's complement. Returns whether there is a carry left over from the two's complement conversion
// process.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_crate_test! {limbs_twos_complement_in_place(xs: &mut [Limb]) -> bool {
    limbs_not_in_place(xs);
    limbs_slice_add_limb_in_place(xs, 1)
}}

// Given the limbs of the absolute value of a negative `Integer`, in ascending order, converts the
// limbs to two's complement and checks whether the most significant bit is `true`; if it isn't,
// appends an extra `Limb::MAX` bit. This way the `Integer`'s negativity is preserved in its limbs.
// The limbs cannot be empty or contain only zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` contains only zeros.
pub_test! {limbs_twos_complement_and_maybe_sign_extend_negative_in_place(xs: &mut Vec<Limb>) {
    assert!(!limbs_twos_complement_in_place(xs));
    if let Some(last) = xs.last() {
        if !last.get_highest_bit() {
            // Sign-extend with an extra !0 limb to indicate a negative Integer
            xs.push(Limb::MAX);
        }
    }
}}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NegativeLimbIterator<'a>(NLIterator<'a>);

// A double-ended iterator over the two's complement [limbs](crate#limbs) of the negative of an
// [`Integer`].
//
// The forward order is ascending (least-significant first). There may be at most one
// most-significant `Limb::MAX` limb.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct NLIterator<'a> {
    pub(crate) limbs: LimbIterator<'a>,
    first_nonzero_index: Option<usize>,
}

impl<'a> NLIterator<'a> {
    fn get(&self, index: u64) -> Limb {
        let index = usize::exact_from(index);
        if index >= self.limbs.len() {
            // We're indexing into the infinite suffix of Limb::MAXs
            Limb::MAX
        } else {
            for i in 0..index {
                if self.limbs[i] != 0 {
                    return !self.limbs[index];
                }
            }
            self.limbs[index].wrapping_neg()
        }
    }
}

impl<'a> Iterator for NLIterator<'a> {
    type Item = Limb;

    // A function to iterate through the two's complement limbs of the negative of a `Natural` in
    // ascending order (least-significant first).
    //
    // # Worst-case complexity
    // Constant time and additional memory.
    fn next(&mut self) -> Option<Limb> {
        let previous_i = self.limbs.i;
        self.limbs.next().map(|limb| {
            if let Some(first_nonzero_index) = self.first_nonzero_index {
                if previous_i <= u64::wrapping_from(first_nonzero_index) {
                    limb.wrapping_neg()
                } else {
                    !limb
                }
            } else {
                if limb != 0 {
                    self.first_nonzero_index = Some(usize::exact_from(previous_i));
                }
                limb.wrapping_neg()
            }
        })
    }

    // A function that returns the length of the negative limbs iterator; that is, the `Natural`'s
    // negative limb count (this is the same as its limb count). The format is (lower bound,
    // Option<upper bound>), but in this case it's trivial to always have an exact bound.
    //
    // # Worst-case complexity
    // Constant time and additional memory.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.limbs.size_hint()
    }
}

impl<'a> DoubleEndedIterator for NLIterator<'a> {
    // A function to iterate through the two's complement limbs of the negative of a `Natural` in
    // descending order (most-significant first). This is worst-case linear since the first
    // `next_back` call needs to determine the index of the least-significant nonzero limb.
    //
    // # Worst-case complexity
    // $T(n) = O(n)$
    //
    // $M(n) = O(1)$
    //
    // where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    fn next_back(&mut self) -> Option<Limb> {
        let previous_j = self.limbs.j;
        self.limbs.next_back().map(|limb| {
            if self.first_nonzero_index.is_none() {
                let mut i = 0;
                while self.limbs[i] == 0 {
                    i += 1;
                }
                self.first_nonzero_index = Some(i);
            }
            let first_nonzero_index = self.first_nonzero_index.unwrap();
            if previous_j <= u64::wrapping_from(first_nonzero_index) {
                limb.wrapping_neg()
            } else {
                !limb
            }
        })
    }
}

trait SignExtendedLimbIterator: DoubleEndedIterator<Item = Limb> {
    const EXTENSION: Limb;

    fn needs_sign_extension(&self) -> bool;

    fn iterate_forward(&mut self, extension_checked: &mut bool) -> Option<Limb> {
        let next = self.next();
        if next.is_none() {
            if *extension_checked {
                None
            } else {
                *extension_checked = true;
                if self.needs_sign_extension() {
                    Some(Self::EXTENSION)
                } else {
                    None
                }
            }
        } else {
            next
        }
    }

    fn iterate_backward(&mut self, extension_checked: &mut bool) -> Option<Limb> {
        if !*extension_checked {
            *extension_checked = true;
            if self.needs_sign_extension() {
                return Some(Self::EXTENSION);
            }
        }
        self.next_back()
    }
}

impl<'a> SignExtendedLimbIterator for LimbIterator<'a> {
    const EXTENSION: Limb = 0;

    fn needs_sign_extension(&self) -> bool {
        self[self.limb_count - 1].get_highest_bit()
    }
}

impl<'a> SignExtendedLimbIterator for NLIterator<'a> {
    const EXTENSION: Limb = Limb::MAX;

    fn needs_sign_extension(&self) -> bool {
        let mut i = 0;
        while self.limbs[i] == 0 {
            i += 1;
        }
        let last_limb_index = self.limbs.limb_count - 1;
        let last_limb = self.limbs[last_limb_index];
        let twos_complement_limb = if i == last_limb_index {
            last_limb.wrapping_neg()
        } else {
            !last_limb
        };
        !twos_complement_limb.get_highest_bit()
    }
}

/// A double-ended iterator over the twos-complement [limbs](crate#limbs) of an [`Integer`].
///
/// The forward order is ascending (least-significant first). The most significant bit of the most
/// significant limb corresponds to the sign of the [`Integer`]; `false` for non-negative and `true`
/// for negative. This means that there may be a single most-significant sign-extension limb that is
/// 0 or `Limb::MAX`.
///
/// This struct also supports retrieving limbs by index. This functionality is completely
/// independent of the iterator's state. Indexing the implicit leading limbs is allowed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TwosComplementLimbIterator<'a> {
    Zero,
    Positive(LimbIterator<'a>, bool),
    Negative(NegativeLimbIterator<'a>, bool),
}

impl<'a> TwosComplementLimbIterator<'a> {
    /// A function to retrieve twos-complement [limbs](crate#limbs) by index. Indexing at or above
    /// the limb count returns zero or `Limb::MAX` limbs, depending on the sign of the `[Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Integer::ZERO.twos_complement_limbs().get(0), 0);
    ///
    ///     // 2^64 - 10^12 = 4294967063 * 2^32 + 727379968
    ///     let negative_trillion = -Integer::from(10u32).pow(12);
    ///     let limbs = negative_trillion.twos_complement_limbs();
    ///     assert_eq!(limbs.get(0), 727379968);
    ///     assert_eq!(limbs.get(1), 4294967063);
    ///     assert_eq!(limbs.get(2), 4294967295);
    ///     assert_eq!(limbs.get(100), 4294967295);
    /// }
    /// ```
    pub fn get(&self, index: u64) -> Limb {
        match *self {
            TwosComplementLimbIterator::Zero => 0,
            TwosComplementLimbIterator::Positive(ref limbs, _) => limbs[usize::exact_from(index)],
            TwosComplementLimbIterator::Negative(ref limbs, _) => limbs.0.get(index),
        }
    }
}

impl<'a> Iterator for TwosComplementLimbIterator<'a> {
    type Item = Limb;

    /// A function to iterate through the twos-complement [limbs](crate#limbs) of an [`Integer`] in
    /// ascending order (least-significant first). The last limb may be a sign-extension limb.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Integer::ZERO.twos_complement_limbs().next(), None);
    ///
    ///     // 2^64 - 10^12 = 4294967063 * 2^32 + 727379968
    ///     let negative_trillion = -Integer::from(10u32).pow(12);
    ///     let mut limbs = negative_trillion.twos_complement_limbs();
    ///     assert_eq!(limbs.next(), Some(727379968));
    ///     assert_eq!(limbs.next(), Some(4294967063));
    ///     assert_eq!(limbs.next(), None);
    /// }
    /// ```
    fn next(&mut self) -> Option<Limb> {
        match *self {
            TwosComplementLimbIterator::Zero => None,
            TwosComplementLimbIterator::Positive(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_forward(extension_checked)
            }
            TwosComplementLimbIterator::Negative(ref mut limbs, ref mut extension_checked) => {
                limbs.0.iterate_forward(extension_checked)
            }
        }
    }
}

impl<'a> DoubleEndedIterator for TwosComplementLimbIterator<'a> {
    /// A function to iterate through the twos-complement [limbs](crate#limbs) of an [`Integer`] in
    /// descending order (most-significant first). The first limb may be a sign-extension limb.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Integer::ZERO.twos_complement_limbs().next_back(), None);
    ///
    ///     // 2^64 - 10^12 = 4294967063 * 2^32 + 727379968
    ///     let negative_trillion = -Integer::from(10u32).pow(12);
    ///     let mut limbs = negative_trillion.twos_complement_limbs();
    ///     assert_eq!(limbs.next_back(), Some(4294967063));
    ///     assert_eq!(limbs.next_back(), Some(727379968));
    ///     assert_eq!(limbs.next_back(), None);
    /// }
    /// ```
    fn next_back(&mut self) -> Option<Limb> {
        match *self {
            TwosComplementLimbIterator::Zero => None,
            TwosComplementLimbIterator::Positive(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_backward(extension_checked)
            }
            TwosComplementLimbIterator::Negative(ref mut limbs, ref mut extension_checked) => {
                limbs.0.iterate_backward(extension_checked)
            }
        }
    }
}

impl Natural {
    /// Returns a double-ended iterator over the two's complement limbs of the negative of a
    /// [`Natural`]. The forward order is ascending, so that less significant limbs appear first.
    /// There may be at most one trailing `Limb::MAX` limb going forward, or leading `Limb::MAX`
    /// limb going backward. The [`Natural`] cannot be zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn negative_limbs(&self) -> NegativeLimbIterator {
        assert_ne!(*self, 0, "Cannot get negative limbs of 0.");
        NegativeLimbIterator(NLIterator {
            limbs: self.limbs(),
            first_nonzero_index: None,
        })
    }
}

impl Integer {
    /// Returns the [limbs](crate#limbs) of an [`Integer`], in ascending order, so that less
    /// significant limbs have lower indices in the output vector.
    ///
    /// The limbs are in two's complement, and the most significant bit of the limbs indicates the
    /// sign; if the bit is zero, the [`Integer`] is positive, and if the bit is one it is negative.
    /// There are no trailing zero limbs if the [`Integer`] is positive or trailing `Limb::MAX`
    /// limbs if the [`Integer`] is negative, except as necessary to include the correct sign bit.
    /// Zero is a special case: it contains no limbs.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible,
    /// [`into_twos_complement_limbs_asc`](`Self::into_twos_complement_limbs_asc`) is more
    /// efficient.
    ///
    /// This function is more efficient than
    /// [`to_twos_complement_limbs_desc`](`Self::to_twos_complement_limbs_desc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Integer::ZERO.to_twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).to_twos_complement_limbs_asc(), &[123]);
    ///     assert_eq!(
    ///         Integer::from(-123).to_twos_complement_limbs_asc(),
    ///         &[4294967173]
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from(10u32).pow(12).to_twos_complement_limbs_asc(),
    ///         &[3567587328, 232]
    ///     );
    ///     assert_eq!(
    ///         (-Integer::from(10u32).pow(12)).to_twos_complement_limbs_asc(),
    ///         &[727379968, 4294967063]
    ///     );
    /// }
    /// ```
    pub fn to_twos_complement_limbs_asc(&self) -> Vec<Limb> {
        let mut limbs = self.abs.to_limbs_asc();
        if self.sign {
            limbs_maybe_sign_extend_non_negative_in_place(&mut limbs);
        } else {
            limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut limbs);
        }
        limbs
    }

    /// Returns the [limbs](crate#limbs) of an [`Integer`], in descending order, so that less
    /// significant limbs have higher indices in the output vector.
    ///
    /// The limbs are in two's complement, and the most significant bit of the limbs indicates the
    /// sign; if the bit is zero, the [`Integer`] is positive, and if the bit is one it is negative.
    /// There are no leading zero limbs if the [`Integer`] is non-negative or leading `Limb::MAX`
    /// limbs if the [`Integer`] is negative, except as necessary to include the correct sign bit.
    /// Zero is a special case: it contains no limbs.
    ///
    /// This is similar to how `BigInteger`s in Java are represented.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible,
    /// [`into_twos_complement_limbs_desc`](`Self::into_twos_complement_limbs_desc`) is more
    /// efficient.
    ///
    /// This function is less efficient than
    /// [`to_twos_complement_limbs_asc`](`Self::to_twos_complement_limbs_asc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Integer::ZERO.to_twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).to_twos_complement_limbs_desc(), &[123]);
    ///     assert_eq!(
    ///         Integer::from(-123).to_twos_complement_limbs_desc(),
    ///         &[4294967173]
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from(10u32).pow(12).to_twos_complement_limbs_desc(),
    ///         &[232, 3567587328]
    ///     );
    ///     assert_eq!(
    ///         (-Integer::from(10u32).pow(12)).to_twos_complement_limbs_desc(),
    ///         &[4294967063, 727379968]
    ///     );
    /// }
    /// ```
    pub fn to_twos_complement_limbs_desc(&self) -> Vec<Limb> {
        let mut xs = self.to_twos_complement_limbs_asc();
        xs.reverse();
        xs
    }

    /// Returns the [limbs](crate#limbs) of an [`Integer`], in ascending order, so that less
    /// significant limbs have lower indices in the output vector.
    ///
    /// The limbs are in two's complement, and the most significant bit of the limbs indicates the
    /// sign; if the bit is zero, the [`Integer`] is positive, and if the bit is one it is negative.
    /// There are no trailing zero limbs if the [`Integer`] is positive or trailing `Limb::MAX`
    /// limbs if the [`Integer`] is negative, except as necessary to include the correct sign bit.
    /// Zero is a special case: it contains no limbs.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// [`to_twos_complement_limbs_asc`](`Self::to_twos_complement_limbs_asc`).
    ///
    /// This function is more efficient than
    /// [`into_twos_complement_limbs_desc`](`Self::into_twos_complement_limbs_desc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Integer::ZERO.into_twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).into_twos_complement_limbs_asc(), &[123]);
    ///     assert_eq!(
    ///         Integer::from(-123).into_twos_complement_limbs_asc(),
    ///         &[4294967173]
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from(10u32)
    ///             .pow(12)
    ///             .into_twos_complement_limbs_asc(),
    ///         &[3567587328, 232]
    ///     );
    ///     assert_eq!(
    ///         (-Integer::from(10u32).pow(12)).into_twos_complement_limbs_asc(),
    ///         &[727379968, 4294967063]
    ///     );
    /// }
    /// ```
    pub fn into_twos_complement_limbs_asc(self) -> Vec<Limb> {
        let mut xs = self.abs.into_limbs_asc();
        if self.sign {
            limbs_maybe_sign_extend_non_negative_in_place(&mut xs);
        } else {
            limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut xs);
        }
        xs
    }

    /// Returns the [limbs](crate#limbs) of an [`Integer`], in descending order, so that less
    /// significant limbs have higher indices in the output vector.
    ///
    /// The limbs are in two's complement, and the most significant bit of the limbs indicates the
    /// sign; if the bit is zero, the [`Integer`] is positive, and if the bit is one it is negative.
    /// There are no leading zero limbs if the [`Integer`] is non-negative or leading `Limb::MAX`
    /// limbs if the [`Integer`] is negative, except as necessary to include the correct sign bit.
    /// Zero is a special case: it contains no limbs.
    ///
    /// This is similar to how `BigInteger`s in Java are represented.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// [`to_twos_complement_limbs_desc`](`Self::to_twos_complement_limbs_desc`).
    ///
    /// This function is less efficient than
    /// [`into_twos_complement_limbs_asc`](`Self::into_twos_complement_limbs_asc`).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Integer::ZERO.into_twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).into_twos_complement_limbs_desc(), &[123]);
    ///     assert_eq!(
    ///         Integer::from(-123).into_twos_complement_limbs_desc(),
    ///         &[4294967173]
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from(10u32)
    ///             .pow(12)
    ///             .into_twos_complement_limbs_desc(),
    ///         &[232, 3567587328]
    ///     );
    ///     assert_eq!(
    ///         (-Integer::from(10u32).pow(12)).into_twos_complement_limbs_desc(),
    ///         &[4294967063, 727379968]
    ///     );
    /// }
    /// ```
    pub fn into_twos_complement_limbs_desc(self) -> Vec<Limb> {
        let mut xs = self.into_twos_complement_limbs_asc();
        xs.reverse();
        xs
    }

    /// Returns a double-ended iterator over the twos-complement [limbs](crate#limbs) of an
    /// [`Integer`].
    ///
    /// The forward order is ascending, so that less significant limbs appear first. There may be a
    /// most-significant sign-extension limb.
    ///
    /// If it's necessary to get a [`Vec`] of all the twos_complement limbs, consider using
    /// [`to_twos_complement_limbs_asc`](`Self::to_twos_complement_limbs_asc`),
    /// [`to_twos_complement_limbs_desc`](`Self::to_twos_complement_limbs_desc`),
    /// [`into_twos_complement_limbs_asc`](`Self::into_twos_complement_limbs_asc`), or
    /// [`into_twos_complement_limbs_desc`](`Self::into_twos_complement_limbs_desc`) instead.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Integer::ZERO.twos_complement_limbs().next().is_none());
    ///     assert_eq!(
    ///         Integer::from(123).twos_complement_limbs().collect_vec(),
    ///         &[123]
    ///     );
    ///     assert_eq!(
    ///         Integer::from(-123).twos_complement_limbs().collect_vec(),
    ///         &[4294967173]
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from(10u32)
    ///             .pow(12)
    ///             .twos_complement_limbs()
    ///             .collect_vec(),
    ///         &[3567587328, 232]
    ///     );
    ///     // Sign-extension for a non-negative `Integer`
    ///     assert_eq!(
    ///         Integer::from(4294967295i64)
    ///             .twos_complement_limbs()
    ///             .collect_vec(),
    ///         &[4294967295, 0]
    ///     );
    ///     assert_eq!(
    ///         (-Integer::from(10u32).pow(12))
    ///             .twos_complement_limbs()
    ///             .collect_vec(),
    ///         &[727379968, 4294967063]
    ///     );
    ///     // Sign-extension for a negative `Integer`
    ///     assert_eq!(
    ///         (-Integer::from(4294967295i64))
    ///             .twos_complement_limbs()
    ///             .collect_vec(),
    ///         &[1, 4294967295]
    ///     );
    ///
    ///     assert!(Integer::ZERO.twos_complement_limbs().next_back().is_none());
    ///     assert_eq!(
    ///         Integer::from(123)
    ///             .twos_complement_limbs()
    ///             .rev()
    ///             .collect_vec(),
    ///         &[123]
    ///     );
    ///     assert_eq!(
    ///         Integer::from(-123)
    ///             .twos_complement_limbs()
    ///             .rev()
    ///             .collect_vec(),
    ///         &[4294967173]
    ///     );
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Integer::from(10u32)
    ///             .pow(12)
    ///             .twos_complement_limbs()
    ///             .rev()
    ///             .collect_vec(),
    ///         &[232, 3567587328]
    ///     );
    ///     // Sign-extension for a non-negative `Integer`
    ///     assert_eq!(
    ///         Integer::from(4294967295i64)
    ///             .twos_complement_limbs()
    ///             .rev()
    ///             .collect_vec(),
    ///         &[0, 4294967295]
    ///     );
    ///     assert_eq!(
    ///         (-Integer::from(10u32).pow(12))
    ///             .twos_complement_limbs()
    ///             .rev()
    ///             .collect_vec(),
    ///         &[4294967063, 727379968]
    ///     );
    ///     // Sign-extension for a negative `Integer`
    ///     assert_eq!(
    ///         (-Integer::from(4294967295i64))
    ///             .twos_complement_limbs()
    ///             .rev()
    ///             .collect_vec(),
    ///         &[4294967295, 1]
    ///     );
    /// }
    /// ```
    pub fn twos_complement_limbs(&self) -> TwosComplementLimbIterator {
        if *self == 0 {
            TwosComplementLimbIterator::Zero
        } else if self.sign {
            TwosComplementLimbIterator::Positive(self.abs.limbs(), false)
        } else {
            TwosComplementLimbIterator::Negative(self.abs.negative_limbs(), false)
        }
    }

    /// Returns the number of twos-complement limbs of an [`Integer`]. There may be a
    /// most-significant sign-extension limb, which is included in the count.
    ///
    /// Zero has 0 limbs.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, PowerOf2};
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Integer::ZERO.twos_complement_limb_count(), 0);
    ///     assert_eq!(Integer::from(123u32).twos_complement_limb_count(), 1);
    ///     assert_eq!(Integer::from(10u32).pow(12).twos_complement_limb_count(), 2);
    ///
    ///     let n = Integer::power_of_2(Limb::WIDTH - 1);
    ///     assert_eq!((&n - Integer::ONE).twos_complement_limb_count(), 1);
    ///     assert_eq!(n.twos_complement_limb_count(), 2);
    ///     assert_eq!((&n + Integer::ONE).twos_complement_limb_count(), 2);
    ///     assert_eq!((-(&n - Integer::ONE)).twos_complement_limb_count(), 1);
    ///     assert_eq!((-&n).twos_complement_limb_count(), 1);
    ///     assert_eq!((-(&n + Integer::ONE)).twos_complement_limb_count(), 2);
    /// }
    /// ```
    pub fn twos_complement_limb_count(&self) -> u64 {
        if *self == 0 {
            return 0;
        }
        let abs_limbs_count = self.unsigned_abs_ref().limb_count();
        let highest_bit_of_highest_limb =
            self.unsigned_abs().limbs()[usize::exact_from(abs_limbs_count - 1)].get_highest_bit();
        if highest_bit_of_highest_limb
            && (*self > 0 || (*self < 0 && !self.unsigned_abs_ref().is_power_of_2()))
        {
            abs_limbs_count + 1
        } else {
            abs_limbs_count
        }
    }
}
