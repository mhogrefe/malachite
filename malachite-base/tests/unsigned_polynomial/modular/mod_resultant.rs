// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::unsigned_polynomial::modular::hgcd::generated_pair;
use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::num::linear_algebra::traits::ModDeterminant;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_resultant::mod_resultant_sylvester;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::modular::mod_resultant::*;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::Modulus;

// The resultant modulo `m` from the dispatcher, Euclid's algorithm, and the half-GCD algorithm with
// the given cutoffs, against the determinant of the Sylvester matrix.
#[test]
fn test_mod_resultant() {
    let test = |p, q, m: u64, inner, outer, out: u64| {
        let p = UnsignedPolynomial::<u64>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<u64>::from_str(q).unwrap();
        let (xs, ys) = (p.coefficients_asc(), q.coefficients_asc());
        let modulus = Modulus(m);
        assert_eq!(mod_resultant_sylvester(xs, ys, m), out);
        assert_eq!(checked_resultant(xs, ys, &modulus), Some(out));
        assert_eq!(resultant_euclidean(xs, ys, &modulus), Some(out));
        assert_eq!(resultant_hgcd(xs, ys, inner, outer, &modulus), Some(out));
    };
    // - match arm (1, 1)
    // - ys.len() == 1, the first
    // - r.is_empty(), the first
    // - ys.len() == 1, the second
    // - let Some(res) = resultant_small(xs, ys, m)
    test("1", "1", 2, 1, 1, 1);
    // - e != 0, the first
    // - e.odd()
    // - e != 0, the second
    // - match arm (len1, 1)
    test("x^2", "1", 2, 1, 1, 1);
    // - match arm (3, 3)
    // - l2 == 0
    // - !(l1 == 1)
    // - !(ys.len() == 1)
    test("x^2", "x^2", 2, 1, 1, 0);
    // - match arm (2, 1)
    test("x", "1", 2, 1, 1, 1);
    // - match arm (3, 2)
    test("x^2", "x", 2, 1, 1, 0);
    // - match arm _
    // - ys.len() < cutoffs.gcd_hgcd
    test("x^3", "x^2", 2, 1, 1, 0);
    // - match arm (2, 2)
    test("x", "x", 2, 1, 1, 0);
    // - match arm (len1, 2)
    // - i < len1 - 1
    test("x^3", "x", 2, 1, 1, 0);
    // - l1 == 1
    // - !j.is_empty()
    // - r.is_empty(), the second
    // - j.len() == 1
    test("x^2", "x+1", 2, 1, 1, 1);
    // - both_even(l0, l1)
    // - both_even(xs.len(), ys.len())
    test("x+1", "x", 2, 1, 1, 1);
    // - let Some(res) = res, the third
    // - let Some(res) = res, the fourth
    // - let Some(res) = res, the fifth
    // - let Some(res) = res, the sixth
    // - result.b.len() < b.len()
    // - result.b.is_empty()
    // - !(res.len1 == 1)
    test("x^2", "x^2+x", 2, 1, 1, 0);
    // - !(result.b.is_empty())
    test("x^3", "x^2+1", 2, 1, 1, 1);
}

// The resultant modulo 7 of longer polynomials, which track the resultant through the recursive
// half-GCD steps, against the determinant of the Sylvester matrix.
#[test]
fn test_mod_resultant_generated() {
    let test = |p_len, q_len, shared, inner, outer| {
        let (p, q) = generated_pair(p_len, q_len, shared);
        let (xs, ys) = (p.coefficients_asc(), q.coefficients_asc());
        let r = mod_resultant_sylvester(xs, ys, 7);
        assert_eq!(resultant_hgcd(xs, ys, inner, outer, &Modulus(7)), Some(r));
    };
    // - b2.len() < b.len()
    test(60, 50, true, 1, 1);
    // - let Some(res) = res, the first
    // - let Some(res) = res, the second
    // - !(r.len() > half)
    test(60, 50, true, 4, 8);
    // - r.len() > half
    test(60, 50, true, 8, 16);
    // - ((len_a + self.off) | (len_b + self.off)).even()
    test(60, 50, false, 1, 1);
    // - j.len() < cutoff
    test(60, 50, false, 4, 8);
    // - !(j.len() == 1)
    test(120, 100, true, 1, 1);
    // - both_even(g.len(), j.len())
    test(120, 100, false, 1, 1);
}

// Checks the resultant modulo the prime `m` of nonzero polynomials, the first at least as long,
// from the dispatcher, Euclid's algorithm, and the half-GCD algorithm with several cutoffs, against
// the determinant of the Sylvester matrix.
fn mod_resultant_properties_helper<T: PrimitiveUnsigned + IsPrime>()
where
    for<'a> &'a UnsignedMatrix<T>: ModDeterminant<T, Output = T>,
{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>().test_properties(
        |(p, q, m)| {
            let (xs, ys) = if p.len() >= q.len() {
                (p.coefficients_asc(), q.coefficients_asc())
            } else {
                (q.coefficients_asc(), p.coefficients_asc())
            };
            if ys.is_empty() {
                return;
            }
            let modulus = Modulus(m);
            let r = mod_resultant_sylvester(xs, ys, m);
            assert_eq!(checked_resultant(xs, ys, &modulus), Some(r));
            assert_eq!(resultant_euclidean(xs, ys, &modulus), Some(r));
            for (inner, outer) in [(1, 1), (2, 3), (4, 8)] {
                assert_eq!(resultant_hgcd(xs, ys, inner, outer, &modulus), Some(r));
            }
        },
    );
}

#[test]
fn mod_resultant_properties() {
    mod_resultant_properties_helper::<u8>();
    mod_resultant_properties_helper::<u16>();
    mod_resultant_properties_helper::<u32>();
    mod_resultant_properties_helper::<u64>();
    mod_resultant_properties_helper::<usize>();
}
