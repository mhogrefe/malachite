// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::ops::Index;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

/// A double-ended iterator over the [limbs](crate#limbs) of a [`Natural`].
///
/// The forward order is ascending (least-significant first). The iterator does not iterate over any
/// implicit leading zero limbs.
///
/// This struct also supports retrieving limbs by index. This functionality is completely
/// independent of the iterator's state. Indexing the implicit leading zero limbs is allowed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LimbIterator<'a> {
    pub(crate) n: &'a Natural,
    pub(crate) limb_count: usize,
    pub(crate) remaining: usize,
    // If `n` is nonzero, this index initially points to the least-significant limb, and is
    // incremented by next().
    pub(crate) i: u64,
    // If `n` is nonzero, this index initially points to the most-significant limb, and is
    // decremented by next_back().
    pub(crate) j: u64,
}

impl<'a> Iterator for LimbIterator<'a> {
    type Item = Limb;

    /// A function to iterate through the [limbs](crate#limbs) of a [`Natural`] in ascending order
    /// (least-significant first).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::ZERO.limbs().next(), None);
    ///
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     let trillion = Natural::from(10u32).pow(12);
    ///     let mut limbs = trillion.limbs();
    ///     assert_eq!(limbs.next(), Some(3567587328));
    ///     assert_eq!(limbs.next(), Some(232));
    ///     assert_eq!(limbs.next(), None);
    /// }
    /// ```
    fn next(&mut self) -> Option<Limb> {
        if self.remaining != 0 {
            let limb = match *self.n {
                Natural(Small(small)) => small,
                Natural(Large(ref limbs)) => limbs[usize::exact_from(self.i)],
            };
            if self.i != self.j {
                self.i += 1;
            }
            self.remaining -= 1;
            Some(limb)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> DoubleEndedIterator for LimbIterator<'a> {
    /// A function to iterate through the [limbs](crate#limbs) of a [`Natural`] in descending order
    /// (most-significant first).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::ZERO.limbs().next_back(), None);
    ///
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     let trillion = Natural::from(10u32).pow(12);
    ///     let mut limbs = trillion.limbs();
    ///     assert_eq!(limbs.next_back(), Some(232));
    ///     assert_eq!(limbs.next_back(), Some(3567587328));
    ///     assert_eq!(limbs.next_back(), None);
    /// }
    /// ```
    fn next_back(&mut self) -> Option<Limb> {
        if self.remaining != 0 {
            let limb = match *self.n {
                Natural(Small(small)) => small,
                Natural(Large(ref limbs)) => limbs[usize::exact_from(self.j)],
            };
            if self.j != self.i {
                self.j -= 1;
            }
            self.remaining -= 1;
            Some(limb)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for LimbIterator<'a> {}

impl<'a> Index<usize> for LimbIterator<'a> {
    type Output = Limb;

    /// A function to retrieve a [`Natural`]'s [limbs](crate#limbs) by index.
    ///
    /// The index is the power of $2^W$ of which the limbs is a coefficient, where $W$ is the width
    /// of a limb. Indexing at or above the limb count returns zeros.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::ZERO.limbs()[0], 0);
    ///
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     let trillion = Natural::from(10u32).pow(12);
    ///     let limbs = trillion.limbs();
    ///     assert_eq!(limbs[0], 3567587328);
    ///     assert_eq!(limbs[1], 232);
    ///     assert_eq!(limbs[2], 0);
    ///     assert_eq!(limbs[100], 0);
    /// }
    /// ```
    fn index(&self, index: usize) -> &Limb {
        if index >= self.limb_count {
            &0
        } else {
            match *self.n {
                Natural(Small(ref small)) => small,
                Natural(Large(ref limbs)) => limbs.index(index),
            }
        }
    }
}

impl Natural {
    /// Returns the [limbs](crate#limbs) of a [`Natural`], in ascending order, so that
    /// less-significant limbs have lower indices in the output vector.
    ///
    /// There are no trailing zero limbs.
    ///
    /// This function borrows the [`Natural`]. If taking ownership is possible instead,
    /// [`into_limbs_asc`](Self::into_limbs_asc) is more efficient.
    ///
    /// This function is more efficient than [`to_limbs_desc`](Self::to_limbs_desc).
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Natural::ZERO.to_limbs_asc().is_empty());
    ///     assert_eq!(Natural::from(123u32).to_limbs_asc(), &[123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from(10u32).pow(12).to_limbs_asc(),
    ///         &[3567587328, 232]
    ///     );
    /// }
    /// ```
    pub fn to_limbs_asc(&self) -> Vec<Limb> {
        match *self {
            Natural::ZERO => Vec::new(),
            Natural(Small(small)) => vec![small],
            Natural(Large(ref limbs)) => limbs.clone(),
        }
    }

    /// Returns the [limbs](crate#limbs) of a [`Natural`] in descending order, so that
    /// less-significant limbs have higher indices in the output vector.
    ///
    /// There are no leading zero limbs.
    ///
    /// This function borrows the [`Natural`]. If taking ownership is possible instead,
    /// [`into_limbs_desc`](Self::into_limbs_desc) is more efficient.
    ///
    /// This function is less efficient than [`to_limbs_asc`](Self::to_limbs_asc).
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Natural::ZERO.to_limbs_desc().is_empty());
    ///     assert_eq!(Natural::from(123u32).to_limbs_desc(), &[123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from(10u32).pow(12).to_limbs_desc(),
    ///         &[232, 3567587328]
    ///     );
    /// }
    /// ```
    pub fn to_limbs_desc(&self) -> Vec<Limb> {
        match *self {
            Natural::ZERO => Vec::new(),
            Natural(Small(small)) => vec![small],
            Natural(Large(ref limbs)) => limbs.iter().copied().rev().collect(),
        }
    }

    /// Returns the [limbs](crate#limbs) of a [`Natural`], in ascending order, so that
    /// less-significant limbs have lower indices in the output vector.
    ///
    /// There are no trailing zero limbs.
    ///
    /// This function takes ownership of the [`Natural`]. If it's necessary to borrow instead, use
    /// [`to_limbs_asc`](Self::to_limbs_asc).
    ///
    /// This function is more efficient than [`into_limbs_desc`](Self::into_limbs_desc).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Natural::ZERO.into_limbs_asc().is_empty());
    ///     assert_eq!(Natural::from(123u32).into_limbs_asc(), &[123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from(10u32).pow(12).into_limbs_asc(),
    ///         &[3567587328, 232]
    ///     );
    /// }
    /// ```
    pub fn into_limbs_asc(self) -> Vec<Limb> {
        match self {
            Natural::ZERO => Vec::new(),
            Natural(Small(small)) => vec![small],
            Natural(Large(limbs)) => limbs,
        }
    }

    /// Returns the [limbs](crate#limbs) of a [`Natural`], in descending order, so that
    /// less-significant limbs have higher indices in the output vector.
    ///
    /// There are no leading zero limbs.
    ///
    /// This function takes ownership of the [`Natural`]. If it's necessary to borrow instead, use
    /// [`to_limbs_desc`](Self::to_limbs_desc).
    ///
    /// This function is less efficient than [`into_limbs_asc`](Self::into_limbs_asc).
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Natural::ZERO.into_limbs_desc().is_empty());
    ///     assert_eq!(Natural::from(123u32).into_limbs_desc(), &[123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from(10u32).pow(12).into_limbs_desc(),
    ///         &[232, 3567587328]
    ///     );
    /// }
    /// ```
    pub fn into_limbs_desc(self) -> Vec<Limb> {
        match self {
            Natural::ZERO => Vec::new(),
            Natural(Small(small)) => vec![small],
            Natural(Large(mut limbs)) => {
                limbs.reverse();
                limbs
            }
        }
    }

    /// Returns a double-ended iterator over the [limbs](crate#limbs) of a [`Natural`].
    ///
    /// The forward order is ascending, so that less-significant limbs appear first. There are no
    /// trailing zero limbs going forward, or leading zeros going backward.
    ///
    /// If it's necessary to get a [`Vec`] of all the limbs, consider using
    /// [`to_limbs_asc`](Self::to_limbs_asc), [`to_limbs_desc`](Self::to_limbs_desc),
    /// [`into_limbs_asc`](Self::into_limbs_asc), or [`into_limbs_desc`](Self::into_limbs_desc)
    /// instead.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert!(Natural::ZERO.limbs().next().is_none());
    ///     assert_eq!(Natural::from(123u32).limbs().collect_vec(), &[123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from(10u32).pow(12).limbs().collect_vec(),
    ///         &[3567587328, 232]
    ///     );
    ///
    ///     assert!(Natural::ZERO.limbs().next_back().is_none());
    ///     assert_eq!(Natural::from(123u32).limbs().rev().collect_vec(), &[123]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(
    ///         Natural::from(10u32).pow(12).limbs().rev().collect_vec(),
    ///         &[232, 3567587328]
    ///     );
    /// }
    /// ```
    pub fn limbs(&self) -> LimbIterator {
        let limb_count = self.limb_count();
        let limb_count_usize = usize::exact_from(limb_count);
        LimbIterator {
            n: self,
            limb_count: limb_count_usize,
            remaining: limb_count_usize,
            i: 0,
            j: limb_count.saturating_sub(1),
        }
    }
}
