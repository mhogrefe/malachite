// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural::{Natural, TWICE_WIDTH};
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    AddMul, Mod, ModAddAssign, ModMulPrecomputed, ModMulPrecomputedAssign, ModPermanent, ModPow,
    ModPowerOf2, ModPowerOf2AddAssign, ModPowerOf2Inverse, ModPowerOf2MulAssign,
    ModPowerOf2Permanent, ModPowerOf2SubAssign, ModSubAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::{
    PermanentElement, mod_permanent_by_parts,
};

impl PermanentElement for Natural {
    type ModMulData = ModMulData;

    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0u32
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == 1u32
    }

    #[inline]
    fn power_of_2_fits(_pow: u64) -> bool {
        true
    }

    #[inline]
    fn trailing_zeros(&self) -> u64 {
        Self::trailing_zeros(self).unwrap()
    }

    #[inline]
    fn shr(&self, bits: u64) -> Self {
        self >> bits
    }

    #[inline]
    fn add_mul(self, y: Self, z: Self) -> Self {
        AddMul::add_mul(self, y, z)
    }

    #[inline]
    fn mod_power_of_2(&self, pow: u64) -> Self {
        ModPowerOf2::mod_power_of_2(self, pow)
    }

    #[inline]
    fn mod_power_of_2_add_assign(&mut self, other: &Self, pow: u64) {
        ModPowerOf2AddAssign::mod_power_of_2_add_assign(self, other, pow);
    }

    #[inline]
    fn mod_power_of_2_sub_assign(&mut self, other: &Self, pow: u64) {
        ModPowerOf2SubAssign::mod_power_of_2_sub_assign(self, other, pow);
    }

    #[inline]
    fn mod_power_of_2_mul_assign(&mut self, other: &Self, pow: u64) {
        ModPowerOf2MulAssign::mod_power_of_2_mul_assign(self, other, pow);
    }

    #[inline]
    fn mod_power_of_2_inverse(&self, pow: u64) -> Option<Self> {
        ModPowerOf2Inverse::mod_power_of_2_inverse(self, pow)
    }

    #[inline]
    fn mod_op(&self, m: &Self) -> Self {
        Mod::mod_op(self, m)
    }

    #[inline]
    fn precompute_mod_mul_data(m: &Self) -> ModMulData {
        <Self as ModMulPrecomputed<Self, Self>>::precompute_mod_mul_data(m)
    }

    #[inline]
    fn mod_add_assign(&mut self, other: &Self, m: &Self) {
        ModAddAssign::mod_add_assign(self, other, m);
    }

    #[inline]
    fn mod_sub_assign(&mut self, other: &Self, m: &Self) {
        ModSubAssign::mod_sub_assign(self, other, m);
    }

    #[inline]
    fn mod_mul_precomputed_assign(&mut self, other: &Self, m: &Self, data: &ModMulData) {
        ModMulPrecomputedAssign::mod_mul_precomputed_assign(self, other, m, data);
    }

    #[inline]
    fn mod_pow(&self, exp: u64, m: &Self) -> Self {
        ModPow::mod_pow(self, Self::from(exp), m)
    }
}

// The permanent modulo m of the n by n matrix with elements `xs`, reduced modulo m: on single limbs
// when m fits in one, on double limbs when it fits in two, and otherwise on `Natural`s, by
// `mod_permanent_by_parts`, whose odd part is found by this function in turn.
fn mod_permanent_elements(xs: Vec<Natural>, n: usize, m: &Natural) -> Natural {
    let n_u64 = u64::exact_from(n);
    if let Natural(Small(small_m)) = *m {
        let elements: Vec<Limb> = xs.iter().map(Limb::exact_from).collect();
        return Natural::from(
            UnsignedMatrix::from_owned_elements(n_u64, n_u64, elements).mod_permanent(small_m),
        );
    }
    if m.significant_bits() <= TWICE_WIDTH {
        let elements: Vec<DoubleLimb> = xs.iter().map(DoubleLimb::exact_from).collect();
        return Natural::from(
            UnsignedMatrix::from_owned_elements(n_u64, n_u64, elements)
                .mod_permanent(DoubleLimb::exact_from(m)),
        );
    }
    mod_permanent_by_parts(
        xs,
        n,
        m,
        |xs, pow| {
            NaturalMatrix::from_owned_elements(n_u64, n_u64, xs).mod_power_of_2_permanent(pow)
        },
        |xs, q| mod_permanent_elements(xs, n, q),
    )
}

// Returns the permanent modulo m of the square matrix `a`, whose elements are reduced modulo m.
fn mod_permanent_helper(a: &NaturalMatrix, m: &Natural) -> Natural {
    assert_reduced(a, m);
    assert_eq!(a.height, a.width, "the matrix must be square");
    mod_permanent_elements(a.elements.clone(), a.height, m)
}

impl ModPermanent<Natural> for NaturalMatrix {
    type Output = Natural;

    /// Computes the permanent of a square [`NaturalMatrix`] modulo $m$, taking the matrix by value
    /// and the modulus by value. The elements must already be reduced modulo $m$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. It is computed as for an
    /// [`UnsignedMatrix`] (see [`mod_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPermanent)), on single limbs when $m$ fits in
    /// one, on double limbs when it fits in two, and otherwise on [`Natural`]s: writing $m = 2^k q$
    /// with $q$ odd, by Glynn's formula modulo $q$ and as by [`mod_power_of_2_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2Permanent) modulo $2^k$, combined by the
    /// Chinese remainder theorem.
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(2^n n(n + k) \log (n + k) \log\log (n + k))$ for a dense matrix of dimension at
    /// most 63
    ///
    /// $M(n, k) = O(n^2 (n + k))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPermanent;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_permanent(Natural::from(7u32)).to_string(),
    ///     "3"
    /// );
    ///
    /// let a = NaturalMatrix::from_str(
    ///     "[1267650600228229401496703205373, 1, 2; 3, 1267650600228229401496703205373, 4; \
    ///     5, 6, 1267650600228229401496703205373]",
    /// )
    /// .unwrap();
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_permanent(m).to_string(),
    ///     "81129638329604085074504135475118"
    /// );
    /// ```
    #[inline]
    fn mod_permanent(self, m: Natural) -> Natural {
        mod_permanent_helper(&self, &m)
    }
}

impl ModPermanent<&Natural> for NaturalMatrix {
    type Output = Natural;

    /// Computes the permanent of a square [`NaturalMatrix`] modulo $m$, taking the matrix by value
    /// and the modulus by reference. The elements must already be reduced modulo $m$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. It is computed as for an
    /// [`UnsignedMatrix`] (see [`mod_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPermanent)), on single limbs when $m$ fits in
    /// one, on double limbs when it fits in two, and otherwise on [`Natural`]s: writing $m = 2^k q$
    /// with $q$ odd, by Glynn's formula modulo $q$ and as by [`mod_power_of_2_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2Permanent) modulo $2^k$, combined by the
    /// Chinese remainder theorem.
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(2^n n(n + k) \log (n + k) \log\log (n + k))$ for a dense matrix of dimension at
    /// most 63
    ///
    /// $M(n, k) = O(n^2 (n + k))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPermanent;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_permanent(&Natural::from(7u32)).to_string(),
    ///     "3"
    /// );
    ///
    /// let a = NaturalMatrix::from_str(
    ///     "[1267650600228229401496703205373, 1, 2; 3, 1267650600228229401496703205373, 4; \
    ///     5, 6, 1267650600228229401496703205373]",
    /// )
    /// .unwrap();
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_permanent(&m).to_string(),
    ///     "81129638329604085074504135475118"
    /// );
    /// ```
    #[inline]
    fn mod_permanent(self, m: &Natural) -> Natural {
        mod_permanent_helper(&self, m)
    }
}

impl ModPermanent<Natural> for &NaturalMatrix {
    type Output = Natural;

    /// Computes the permanent of a square [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// reference and the modulus by value. The elements must already be reduced modulo $m$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. It is computed as for an
    /// [`UnsignedMatrix`] (see [`mod_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPermanent)), on single limbs when $m$ fits in
    /// one, on double limbs when it fits in two, and otherwise on [`Natural`]s: writing $m = 2^k q$
    /// with $q$ odd, by Glynn's formula modulo $q$ and as by [`mod_power_of_2_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2Permanent) modulo $2^k$, combined by the
    /// Chinese remainder theorem.
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(2^n n(n + k) \log (n + k) \log\log (n + k))$ for a dense matrix of dimension at
    /// most 63
    ///
    /// $M(n, k) = O(n^2 (n + k))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPermanent;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_permanent(Natural::from(7u32)).to_string(), "3");
    ///
    /// let a = NaturalMatrix::from_str(
    ///     "[1267650600228229401496703205373, 1, 2; 3, 1267650600228229401496703205373, 4; \
    ///     5, 6, 1267650600228229401496703205373]",
    /// )
    /// .unwrap();
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// assert_eq!(
    ///     (&a).mod_permanent(m).to_string(),
    ///     "81129638329604085074504135475118"
    /// );
    /// ```
    #[inline]
    fn mod_permanent(self, m: Natural) -> Natural {
        mod_permanent_helper(self, &m)
    }
}

impl ModPermanent<&Natural> for &NaturalMatrix {
    type Output = Natural;

    /// Computes the permanent of a square [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// reference and the modulus by reference. The elements must already be reduced modulo $m$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. It is computed as for an
    /// [`UnsignedMatrix`] (see [`mod_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPermanent)), on single limbs when $m$ fits in
    /// one, on double limbs when it fits in two, and otherwise on [`Natural`]s: writing $m = 2^k q$
    /// with $q$ odd, by Glynn's formula modulo $q$ and as by [`mod_power_of_2_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2Permanent) modulo $2^k$, combined by the
    /// Chinese remainder theorem.
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(2^n n(n + k) \log (n + k) \log\log (n + k))$ for a dense matrix of dimension at
    /// most 63
    ///
    /// $M(n, k) = O(n^2 (n + k))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPermanent;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_permanent(&Natural::from(7u32)).to_string(), "3");
    ///
    /// let a = NaturalMatrix::from_str(
    ///     "[1267650600228229401496703205373, 1, 2; 3, 1267650600228229401496703205373, 4; \
    ///     5, 6, 1267650600228229401496703205373]",
    /// )
    /// .unwrap();
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// assert_eq!(
    ///     (&a).mod_permanent(&m).to_string(),
    ///     "81129638329604085074504135475118"
    /// );
    /// ```
    #[inline]
    fn mod_permanent(self, m: &Natural) -> Natural {
        mod_permanent_helper(self, m)
    }
}
