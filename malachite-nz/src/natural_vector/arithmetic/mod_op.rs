// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use core::ops::{Rem, RemAssign};
use malachite_base::num::arithmetic::traits::{Mod, ModAssign};

impl Rem<Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], keeping the remainders,
    /// taking the vector by value and the modulus by value.
    ///
    /// $v \\% m$ is the vector whose $i$th element is $v_i \\% m$. The dimension is unchanged:
    /// unlike a polynomial, a vector keeps the elements that reduce to zero, so $(4, 3)$ modulo 4
    /// is $(0, 3)$.
    ///
    /// The result is reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](malachite_base::num::arithmetic::traits::ModIsReduced::mod_is_reduced)
    /// returns `true` for it.
    ///
    /// $$
    /// f(v, m) = w, \\quad \text{where} \\quad w_i = v_i - m \left \lfloor \frac{v_i}{m}
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let m = Natural::from_str("1000000000000").unwrap();
    /// let v = NaturalVector::from_str("(1000000000001, 2000000000003, 5)").unwrap();
    /// assert_eq!((v % m).to_string(), "(1, 3, 5)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = NaturalVector::from_str("(4, 3)").unwrap();
    /// assert_eq!((v % Natural::from(4u32)).to_string(), "(0, 3)");
    /// ```
    #[inline]
    fn rem(mut self, m: Natural) -> Self {
        self %= m;
        self
    }
}

impl<'a> Rem<&'a Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], keeping the remainders,
    /// taking the vector by value and the modulus by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalVector`] that takes both
    /// arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let m = Natural::from_str("1000000000000").unwrap();
    /// let v = NaturalVector::from_str("(1000000000001, 2000000000003, 5)").unwrap();
    /// assert_eq!((v % &m).to_string(), "(1, 3, 5)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = NaturalVector::from_str("(4, 3)").unwrap();
    /// assert_eq!((v % &Natural::from(4u32)).to_string(), "(0, 3)");
    /// ```
    #[inline]
    fn rem(mut self, m: &'a Natural) -> Self {
        self %= m;
        self
    }
}

impl Rem<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], keeping the remainders,
    /// taking the vector by reference and the modulus by value.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalVector`] that takes both
    /// arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let m = Natural::from_str("1000000000000").unwrap();
    /// let v = NaturalVector::from_str("(1000000000001, 2000000000003, 5)").unwrap();
    /// assert_eq!((&v % m).to_string(), "(1, 3, 5)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = NaturalVector::from_str("(4, 3)").unwrap();
    /// assert_eq!((&v % Natural::from(4u32)).to_string(), "(0, 3)");
    /// // The vector is left alone.
    /// assert_eq!(v.to_string(), "(4, 3)");
    /// ```
    #[inline]
    fn rem(self, m: Natural) -> NaturalVector {
        self % &m
    }
}

impl<'a> Rem<&'a Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], keeping the remainders,
    /// taking the vector by reference and the modulus by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalVector`] that takes both
    /// arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let m = Natural::from_str("1000000000000").unwrap();
    /// let v = NaturalVector::from_str("(1000000000001, 2000000000003, 5)").unwrap();
    /// assert_eq!((&v % &m).to_string(), "(1, 3, 5)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = NaturalVector::from_str("(4, 3)").unwrap();
    /// assert_eq!((&v % &Natural::from(4u32)).to_string(), "(0, 3)");
    /// // The vector is left alone.
    /// assert_eq!(v.to_string(), "(4, 3)");
    /// ```
    fn rem(self, m: &'a Natural) -> NaturalVector {
        assert_ne!(*m, 0u32, "division by zero");
        NaturalVector {
            elements: self.elements.iter().map(|x| x % m).collect(),
        }
    }
}

impl RemAssign<Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], replacing each element by the
    /// remainder, taking the modulus by value.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalVector`] that takes both
    /// arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1000000000001, 2000000000003, 5)").unwrap();
    /// v %= Natural::from_str("1000000000000").unwrap();
    /// assert_eq!(v.to_string(), "(1, 3, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(4, 3)").unwrap();
    /// v %= Natural::from(4u32);
    /// assert_eq!(v.to_string(), "(0, 3)");
    /// ```
    #[inline]
    fn rem_assign(&mut self, m: Natural) {
        *self %= &m;
    }
}

impl<'a> RemAssign<&'a Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], replacing each element by the
    /// remainder, taking the modulus by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`NaturalVector`] that takes both
    /// arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1000000000001, 2000000000003, 5)").unwrap();
    /// v %= &Natural::from_str("1000000000000").unwrap();
    /// assert_eq!(v.to_string(), "(1, 3, 5)");
    ///
    /// let mut v = NaturalVector::from_str("(4, 3)").unwrap();
    /// v %= &Natural::from(4u32);
    /// assert_eq!(v.to_string(), "(0, 3)");
    /// ```
    fn rem_assign(&mut self, m: &'a Natural) {
        assert_ne!(*m, 0u32, "division by zero");
        for x in &mut self.elements {
            *x %= m;
        }
    }
}

impl Mod<Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], keeping the remainders,
    /// taking the vector by value and the modulus by value.
    ///
    /// A [`NaturalVector`]'s elements are never negative, so this agrees with `%` everywhere; it is
    /// the same operation under the name the mod-family traits use. See the documentation for the
    /// [`Rem`] implementation on [`NaturalVector`] that takes both arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(4, 4, 5)").unwrap();
    /// assert_eq!(v.mod_op(Natural::from(3u32)).to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_op(self, m: Natural) -> Self {
        self % m
    }
}

impl<'a> Mod<&'a Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], keeping the remainders,
    /// taking the vector by value and the modulus by reference.
    ///
    /// A [`NaturalVector`]'s elements are never negative, so this agrees with `%` everywhere; it is
    /// the same operation under the name the mod-family traits use. See the documentation for the
    /// [`Rem`] implementation on [`NaturalVector`] that takes both arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(4, 4, 5)").unwrap();
    /// assert_eq!(v.mod_op(&Natural::from(3u32)).to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_op(self, m: &'a Natural) -> Self {
        self % m
    }
}

impl Mod<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], keeping the remainders,
    /// taking the vector by reference and the modulus by value.
    ///
    /// A [`NaturalVector`]'s elements are never negative, so this agrees with `%` everywhere; it is
    /// the same operation under the name the mod-family traits use. See the documentation for the
    /// [`Rem`] implementation on [`NaturalVector`] that takes both arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(4, 4, 5)").unwrap();
    /// assert_eq!((&v).mod_op(Natural::from(3u32)).to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_op(self, m: Natural) -> NaturalVector {
        self % m
    }
}

impl<'a> Mod<&'a Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`], keeping the remainders,
    /// taking the vector by reference and the modulus by reference.
    ///
    /// A [`NaturalVector`]'s elements are never negative, so this agrees with `%` everywhere; it is
    /// the same operation under the name the mod-family traits use. See the documentation for the
    /// [`Rem`] implementation on [`NaturalVector`] that takes both arguments by value for details.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(4, 4, 5)").unwrap();
    /// assert_eq!((&v).mod_op(&Natural::from(3u32)).to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_op(self, m: &'a Natural) -> NaturalVector {
        self % m
    }
}

impl ModAssign<Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], replacing each element by the
    /// remainder, taking the modulus by value.
    ///
    /// A [`NaturalVector`]'s elements are never negative, so this agrees with `%=` everywhere; it
    /// is the same operation under the name the mod-family traits use. See the documentation for
    /// the [`Rem`] implementation on [`NaturalVector`] that takes both arguments by value for
    /// details.
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(4, 4, 5)").unwrap();
    /// v.mod_assign(Natural::from(3u32));
    /// assert_eq!(v.to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_assign(&mut self, m: Natural) {
        *self %= m;
    }
}

impl<'a> ModAssign<&'a Natural> for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by a [`Natural`], replacing each element by the
    /// remainder, taking the modulus by reference.
    ///
    /// A [`NaturalVector`]'s elements are never negative, so this agrees with `%=` everywhere; it
    /// is the same operation under the name the mod-family traits use. See the documentation for
    /// the [`Rem`] implementation on [`NaturalVector`] that takes both arguments by value for
    /// details.
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(4, 4, 5)").unwrap();
    /// v.mod_assign(&Natural::from(3u32));
    /// assert_eq!(v.to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_assign(&mut self, m: &'a Natural) {
        *self %= m;
    }
}
