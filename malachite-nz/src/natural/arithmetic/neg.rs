// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993-1997, 1999-2016, 2020 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::natural::logic::not::limbs_not_in_place;
use crate::platform::Limb;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use malachite_base::slices::slice_leading_zeros;

// This is equivalent to `mpn_neg` from `gmp.h`, GMP 6.2.1, where rp == up.
pub(crate) fn limbs_neg_in_place(xs: &mut [Limb]) -> bool {
    let n = xs.len();
    let zeros = slice_leading_zeros(xs);
    if zeros == n {
        return false;
    }
    xs[zeros].wrapping_neg_assign();
    let offset = zeros + 1;
    if offset != n {
        limbs_not_in_place(&mut xs[offset..]);
    }
    true
}

impl Neg for Natural {
    type Output = Integer;

    /// Negates a [`Natural`], taking it by value and returning an [`Integer`].
    ///
    /// $$
    /// f(x) = -x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(-Natural::ZERO, 0);
    /// assert_eq!(-Natural::from(123u32), -123);
    /// ```
    fn neg(self) -> Integer {
        Integer::from_sign_and_abs(self == 0, self)
    }
}

impl Neg for &Natural {
    type Output = Integer;

    /// Negates a [`Natural`], taking it by reference and returning an [`Integer`].
    ///
    /// $$
    /// f(x) = -x.
    /// $$
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(-&Natural::ZERO, 0);
    /// assert_eq!(-&Natural::from(123u32), -123);
    /// ```
    fn neg(self) -> Integer {
        Integer::from_sign_and_abs_ref(*self == 0, self)
    }
}
