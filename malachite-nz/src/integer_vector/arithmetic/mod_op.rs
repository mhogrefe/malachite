// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Mod, NegMod};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_vector::UnsignedVector;

// The remainder of `x` modulo `m`, in $[0, m)$.
fn integer_mod_natural(x: &Integer, m: &Natural) -> Natural {
    if x.sign {
        &x.abs % m
    } else {
        (&x.abs).neg_mod(m)
    }
}

impl Mod<Natural> for IntegerVector {
    type Output = NaturalVector;

    /// Divides every element of an [`IntegerVector`] by a [`Natural`], keeping the remainders as a
    /// [`NaturalVector`], taking the vector by value and the modulus by value.
    ///
    /// Each remainder is taken in $[0, m)$, the way [`Mod`] does for [`Integer`]s, so a negative
    /// element $x$ that is not a multiple of $m$ becomes $m - (-x \bmod m)$. The result is
    /// therefore reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](malachite_base::num::arithmetic::traits::ModIsReduced::mod_is_reduced)
    /// returns `true` for it, and every element of the result is congruent to the corresponding
    /// element of the input. The dimension is unchanged: unlike a polynomial, a vector keeps the
    /// elements that reduce to zero.
    ///
    /// $$
    /// f(v, m) = w, \quad \text{where} \quad w_i = v_i - m \left \lfloor \frac{v_i}{m}
    /// \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// vector's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Every element is taken modulo 3, and negative ones become non-negative.
    /// let v = IntegerVector::from_str("(1, -4, -5)").unwrap();
    /// assert_eq!(
    ///     v.clone().mod_op(Natural::from(3u32)).to_string(),
    ///     "(1, 2, 1)"
    /// );
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = IntegerVector::from_str("(-6, 3)").unwrap();
    /// assert_eq!(v.clone().mod_op(Natural::from(3u32)).to_string(), "(0, 0)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mod_fmpz` from `fmpz_vec/scalar_mod.c`, FLINT 3.6.0,
    /// for a positive modulus.
    #[inline]
    fn mod_op(self, m: Natural) -> NaturalVector {
        self.mod_op(&m)
    }
}

impl<'a> Mod<&'a Natural> for IntegerVector {
    type Output = NaturalVector;

    /// Divides every element of an [`IntegerVector`] by a [`Natural`], keeping the remainders as a
    /// [`NaturalVector`], taking the vector by value and the modulus by reference.
    ///
    /// See the documentation for the [`Mod`] implementation on [`IntegerVector`] that takes both
    /// arguments by value for details, including how negative elements are handled.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// vector's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Every element is taken modulo 3, and negative ones become non-negative.
    /// let v = IntegerVector::from_str("(1, -4, -5)").unwrap();
    /// assert_eq!(
    ///     v.clone().mod_op(&Natural::from(3u32)).to_string(),
    ///     "(1, 2, 1)"
    /// );
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = IntegerVector::from_str("(-6, 3)").unwrap();
    /// assert_eq!(v.clone().mod_op(&Natural::from(3u32)).to_string(), "(0, 0)");
    /// ```
    fn mod_op(self, m: &'a Natural) -> NaturalVector {
        assert_ne!(*m, 0u32, "division by zero");
        NaturalVector {
            elements: self
                .elements
                .into_iter()
                .map(|Integer { sign, abs }| if sign { abs % m } else { abs.neg_mod(m) })
                .collect(),
        }
    }
}

impl Mod<Natural> for &IntegerVector {
    type Output = NaturalVector;

    /// Divides every element of an [`IntegerVector`] by a [`Natural`], keeping the remainders as a
    /// [`NaturalVector`], taking the vector by reference and the modulus by value.
    ///
    /// See the documentation for the [`Mod`] implementation on [`IntegerVector`] that takes both
    /// arguments by value for details, including how negative elements are handled.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// vector's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Every element is taken modulo 3, and negative ones become non-negative.
    /// let v = IntegerVector::from_str("(1, -4, -5)").unwrap();
    /// assert_eq!((&v).mod_op(Natural::from(3u32)).to_string(), "(1, 2, 1)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = IntegerVector::from_str("(-6, 3)").unwrap();
    /// assert_eq!((&v).mod_op(Natural::from(3u32)).to_string(), "(0, 0)");
    /// ```
    #[inline]
    fn mod_op(self, m: Natural) -> NaturalVector {
        self.mod_op(&m)
    }
}

impl<'a> Mod<&'a Natural> for &IntegerVector {
    type Output = NaturalVector;

    /// Divides every element of an [`IntegerVector`] by a [`Natural`], keeping the remainders as a
    /// [`NaturalVector`], taking the vector by reference and the modulus by reference.
    ///
    /// See the documentation for the [`Mod`] implementation on [`IntegerVector`] that takes both
    /// arguments by value for details, including how negative elements are handled.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// vector's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Every element is taken modulo 3, and negative ones become non-negative.
    /// let v = IntegerVector::from_str("(1, -4, -5)").unwrap();
    /// assert_eq!((&v).mod_op(&Natural::from(3u32)).to_string(), "(1, 2, 1)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = IntegerVector::from_str("(-6, 3)").unwrap();
    /// assert_eq!((&v).mod_op(&Natural::from(3u32)).to_string(), "(0, 0)");
    /// ```
    fn mod_op(self, m: &'a Natural) -> NaturalVector {
        assert_ne!(*m, 0u32, "division by zero");
        NaturalVector {
            elements: self
                .elements
                .iter()
                .map(|x| integer_mod_natural(x, m))
                .collect(),
        }
    }
}

impl<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>> Mod<T> for &IntegerVector
where
    Natural: From<T>,
{
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`IntegerVector`] by a value of an unsigned primitive integer
    /// type, keeping the remainders as an [`UnsignedVector`] with that element type, taking the
    /// vector by reference.
    ///
    /// Each remainder is taken in $[0, m)$, so negative elements become non-negative, and every
    /// remainder fits in `m`'s type. Apart from the result's type, this is the same operation as
    /// reducing modulo `Natural::from(m)`; see the documentation for the [`Mod`] implementation on
    /// [`IntegerVector`] that takes both arguments by value for details.
    ///
    /// The result is reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](malachite_base::num::arithmetic::traits::ModIsReduced::mod_is_reduced)
    /// returns `true` for it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits in the vector's
    /// elements, and $m$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-1000000000001, 2000000000003, -5)").unwrap();
    /// let w: UnsignedVector<u8> = (&v).mod_op(7u8);
    /// assert_eq!(w.to_string(), "(5, 5, 2)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = IntegerVector::from_str("(-1024, -3)").unwrap();
    /// assert_eq!((&v).mod_op(4u64).to_string(), "(0, 1)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_get_nmod_vec` from `fmpz_vec/get_nmod_vec.c`, FLINT 3.6.0.
    fn mod_op(self, m: T) -> UnsignedVector<T> {
        assert_ne!(m, T::ZERO, "division by zero");
        let m = Natural::from(m);
        UnsignedVector {
            elements: self
                .elements
                .iter()
                .map(|x| T::exact_from(&integer_mod_natural(x, &m)))
                .collect::<Vec<_>>(),
        }
    }
}

impl<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>> Mod<T> for IntegerVector
where
    Natural: From<T>,
{
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`IntegerVector`] by a value of an unsigned primitive integer
    /// type, keeping the remainders as an [`UnsignedVector`] with that element type, taking the
    /// vector by value.
    ///
    /// Taking the vector by value saves nothing, since the remainders go into new storage either
    /// way. See the documentation for the [`Mod`] implementation that takes the vector by reference
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits in the vector's
    /// elements, and $m$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -4, -5)").unwrap();
    /// assert_eq!(v.mod_op(3u32).to_string(), "(1, 2, 1)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_get_nmod_vec` from `fmpz_vec/get_nmod_vec.c`, FLINT 3.6.0.
    #[inline]
    fn mod_op(self, m: T) -> UnsignedVector<T> {
        (&self).mod_op(m)
    }
}
