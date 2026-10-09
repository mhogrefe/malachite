// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModMul, ModMulAssign, ModMulPrecomputed, ModMulPrecomputedAssign,
};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_reduced(v: &NaturalVector, c: &Natural, m: &Natural) {
    assert!(
        v.mod_is_reduced(m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

// Multiplies every element of `v` by `c` modulo `m`, in place. The data for multiplying modulo `m`
// is computed once and shared by all the elements.
fn mod_mul_assign_helper(v: &mut NaturalVector, c: &Natural, m: &Natural) {
    assert_reduced(v, c, m);
    match *c {
        Natural::ZERO => v.elements.fill(Natural::ZERO),
        Natural::ONE => {}
        _ => {
            let data =
                <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
            for x in &mut v.elements {
                x.mod_mul_precomputed_assign(c, m, &data);
            }
        }
    }
}

fn mod_mul_ref_helper(v: &NaturalVector, c: &Natural, m: &Natural) -> NaturalVector {
    assert_reduced(v, c, m);
    NaturalVector {
        elements: match *c {
            Natural::ZERO => vec![Natural::ZERO; v.elements.len()],
            Natural::ONE => v.elements.clone(),
            _ => {
                let data =
                    <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
                v.elements
                    .iter()
                    .map(|x| x.mod_mul_precomputed(c, m, &data))
                    .collect()
            }
        },
    }
}

impl ModMul<Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the vector by value, the scalar by value, and the modulus by value. The elements
    /// and the scalar must already be reduced modulo $m$.
    ///
    /// Each product is reduced modulo $m$, and the result has the same dimension as the vector. The
    /// data for multiplying modulo $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, c, m) = cv \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_mul(Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(mut self, c: Natural, m: Natural) -> Self {
        mod_mul_assign_helper(&mut self, &c, &m);
        self
    }
}

impl ModMul<Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the vector by value, the scalar by value, and the modulus by reference. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_mul(Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(mut self, c: Natural, m: &Natural) -> Self {
        mod_mul_assign_helper(&mut self, &c, m);
        self
    }
}

impl ModMul<&Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the vector by value, the scalar by reference, and the modulus by value. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_mul(&Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(mut self, c: &Natural, m: Natural) -> Self {
        mod_mul_assign_helper(&mut self, c, &m);
        self
    }
}

impl ModMul<&Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the vector by value, the scalar by reference, and the modulus by reference. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_mul(&Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(mut self, c: &Natural, m: &Natural) -> Self {
        mod_mul_assign_helper(&mut self, c, m);
        self
    }
}

impl ModMul<Natural, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the vector by reference, the scalar by value, and the modulus by value. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_mul(Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, c: Natural, m: Natural) -> NaturalVector {
        mod_mul_ref_helper(self, &c, &m)
    }
}

impl ModMul<Natural, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the vector by reference, the scalar by value, and the modulus by reference. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_mul(Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, c: Natural, m: &Natural) -> NaturalVector {
        mod_mul_ref_helper(self, &c, m)
    }
}

impl ModMul<&Natural, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the vector by reference, the scalar by reference, and the modulus by value. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_mul(&Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, c: &Natural, m: Natural) -> NaturalVector {
        mod_mul_ref_helper(self, c, &m)
    }
}

impl ModMul<&Natural, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the vector by reference, the scalar by reference, and the modulus by reference.
    /// The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_mul(&Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "(1, 3, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, c: &Natural, m: &Natural) -> NaturalVector {
        mod_mul_ref_helper(self, c, m)
    }
}

impl ModMulAssign<Natural, Natural> for NaturalVector {
    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, in place, taking the scalar by value and the modulus by value. The elements and the
    /// scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// v.mod_mul_assign(Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, c: Natural, m: Natural) {
        mod_mul_assign_helper(self, &c, &m);
    }
}

impl ModMulAssign<Natural, &Natural> for NaturalVector {
    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, in place, taking the scalar by value and the modulus by reference. The elements and the
    /// scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// v.mod_mul_assign(Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, c: Natural, m: &Natural) {
        mod_mul_assign_helper(self, &c, m);
    }
}

impl ModMulAssign<&Natural, Natural> for NaturalVector {
    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, in place, taking the scalar by reference and the modulus by value. The elements and the
    /// scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// v.mod_mul_assign(&Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, c: &Natural, m: Natural) {
        mod_mul_assign_helper(self, c, &m);
    }
}

impl ModMulAssign<&Natural, &Natural> for NaturalVector {
    /// Multiplies every element of a [`NaturalVector`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, in place, taking the scalar by reference and the modulus by reference. The elements and
    /// the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// v.mod_mul_assign(&Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(1, 3, 2)");
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, c: &Natural, m: &Natural) {
        mod_mul_assign_helper(self, c, m);
    }
}
