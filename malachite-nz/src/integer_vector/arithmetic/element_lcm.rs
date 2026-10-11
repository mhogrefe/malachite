// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ElementLcm, LcmAssign};
use malachite_base::num::basic::traits::One;

impl ElementLcm for IntegerVector {
    type Output = Natural;

    /// Returns the LCM of the elements of a [`IntegerVector`], taking the vector by reference.
    ///
    /// The LCM of the elements of the 0-dimensional vector is 1. If an element is 0 the LCM is 0,
    /// and the elements after it are not examined. The LCM is the LCM of the absolute values, so it
    /// is non-negative.
    ///
    /// $$
    /// f(v) = \operatorname{lcm}(v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements, and $m$ is the dimension of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ElementLcm;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(-4, 6, -10)")
    ///         .unwrap()
    ///         .to_element_lcm(),
    ///     60
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(-4, 0, 6)")
    ///         .unwrap()
    ///         .to_element_lcm(),
    ///     0
    /// );
    /// assert_eq!(IntegerVector::from_str("()").unwrap().to_element_lcm(), 1);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_lcm` from `fmpz_vec/lcm.c`, FLINT 3.6.0.
    fn to_element_lcm(&self) -> Natural {
        let mut lcm = Natural::ONE;
        for c in &self.elements {
            lcm.lcm_assign(&c.abs);
            if lcm == 0u32 {
                break;
            }
        }
        lcm
    }

    /// Returns the LCM of the elements of a [`IntegerVector`], taking the vector by value.
    ///
    /// The LCM of the elements of the 0-dimensional vector is 1. If an element is 0 the LCM is 0,
    /// and the elements after it are not examined. The LCM is the LCM of the absolute values, so it
    /// is non-negative.
    ///
    /// $$
    /// f(v) = \operatorname{lcm}(v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements, and $m$ is the dimension of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ElementLcm;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(-4, 6, -10)")
    ///         .unwrap()
    ///         .into_element_lcm(),
    ///     60
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(-4, 0, 6)")
    ///         .unwrap()
    ///         .into_element_lcm(),
    ///     0
    /// );
    /// assert_eq!(IntegerVector::from_str("()").unwrap().into_element_lcm(), 1);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_lcm` from `fmpz_vec/lcm.c`, FLINT 3.6.0.
    fn into_element_lcm(self) -> Natural {
        let mut lcm = Natural::ONE;
        for c in self.elements {
            lcm.lcm_assign(c.abs);
            if lcm == 0u32 {
                break;
            }
        }
        lcm
    }
}
