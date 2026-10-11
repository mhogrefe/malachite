// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::arithmetic::mul::product_of_unsigneds::product_of_unsigneds;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::WideningElementProduct;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, WrappingFrom};
use malachite_base::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> WideningElementProduct<Natural> for UnsignedVector<T>
where
    Limb: ConvertibleFrom<T> + WrappingFrom<T>,
    Natural: From<T>,
{
    /// Returns the product of the elements of an [`UnsignedVector`] as a [`Natural`], taking the
    /// vector by reference.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1. Adjacent elements are
    /// multiplied as machine words while their product fits in one, and the resulting words are
    /// multiplied in a balanced tree; multiplication stops as soon as an element is zero.
    ///
    /// $$
    /// f(v) = \prod_i v_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the numbers of
    /// significant bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::WideningElementProduct;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// let v = UnsignedVector::<u64>::from_str("(4294967296, 4294967296, 3)").unwrap();
    /// let p: Natural = v.to_widening_element_product();
    /// assert_eq!(p.to_string(), "55340232221128654848");
    /// let v = UnsignedVector::<u8>::from_str("()").unwrap();
    /// let p: Natural = v.to_widening_element_product();
    /// assert_eq!(p, 1);
    /// ```
    ///
    /// This is equivalent to `_fmpz_ui_vec_prod` from `fmpz_vec/prod.c`, FLINT 3.6.0.
    #[inline]
    fn to_widening_element_product(&self) -> Natural {
        product_of_unsigneds(self.elements.iter().copied())
    }

    /// Returns the product of the elements of an [`UnsignedVector`] as a [`Natural`], taking the
    /// vector by value.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1. Adjacent elements are
    /// multiplied as machine words while their product fits in one, and the resulting words are
    /// multiplied in a balanced tree; multiplication stops as soon as an element is zero.
    ///
    /// $$
    /// f(v) = \prod_i v_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the numbers of
    /// significant bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::WideningElementProduct;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// let v = UnsignedVector::<u64>::from_str("(4294967296, 4294967296, 3)").unwrap();
    /// let p: Natural = v.into_widening_element_product();
    /// assert_eq!(p.to_string(), "55340232221128654848");
    /// let v = UnsignedVector::<u8>::from_str("()").unwrap();
    /// let p: Natural = v.into_widening_element_product();
    /// assert_eq!(p, 1);
    /// ```
    ///
    /// This is equivalent to `_fmpz_ui_vec_prod` from `fmpz_vec/prod.c`, FLINT 3.6.0.
    #[inline]
    fn into_widening_element_product(self) -> Natural {
        product_of_unsigneds(self.elements.into_iter())
    }
}
