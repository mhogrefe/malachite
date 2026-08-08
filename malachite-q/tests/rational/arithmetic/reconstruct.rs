// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivisibleBy, FloorSqrt, Gcd, ModInverse, Parity};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    natural_pair_gen_var_1, natural_quadruple_gen_var_1, rational_gen,
};
use std::str::FromStr;

#[test]
fn test_reconstruct() {
    let test = |a, m, out| {
        let a = Natural::from_str(a).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(format!("{:?}", Rational::reconstruct_ref(&a, &m)), out);
        assert_eq!(format!("{:?}", Rational::reconstruct(a, m)), out);
    };
    // - a <= N (zero)
    test("0", "3", "Some(0)");
    // - a <= N (positive integer)
    test("1", "4", "Some(1)");
    // - m - a <= N (negative integer)
    test("2", "3", "Some(-1)");
    // - denominator bound exceeded
    test("2", "4", "None");
    // - genuine fraction
    test("33", "97", "Some(2/3)");
    // - 1/25 is a solution for looser bounds, but not for the balanced ones
    test("444", "1009", "None");
    // - multi-limb success
    test(
        "8818342134038800723104056361",
        "12345678987654321012345678901",
        "Some(22/7)",
    );
    test(
        "3527336853615520289241622543",
        "12345678987654321012345678901",
        "Some(-1/7)",
    );
    // - multi-limb negative integer fast path
    test(
        "12345678987654321012345678884",
        "12345678987654321012345678901",
        "Some(-17)",
    );
    // - multi-limb failure
    test(
        "4052555153018976267",
        "12345678987654321012345678901",
        "None",
    );
    // - gcd(n, d) != 1 with a nonzero remainder
    test("4", "10", "None");
}

#[test]
fn test_reconstruct_with_bounds() {
    let test = |a, m, n_bound, d_bound, out| {
        let a = Natural::from_str(a).unwrap();
        let m = Natural::from_str(m).unwrap();
        let n_bound = Natural::from_str(n_bound).unwrap();
        let d_bound = Natural::from_str(d_bound).unwrap();
        assert_eq!(
            format!(
                "{:?}",
                Rational::reconstruct_with_bounds_ref(&a, &m, &n_bound, &d_bound)
            ),
            out
        );
        assert_eq!(
            format!(
                "{:?}",
                Rational::reconstruct_with_bounds(a, m, &n_bound, &d_bound)
            ),
            out
        );
    };
    // - a <= N (zero)
    test("0", "5", "1", "1", "Some(0)");
    // - m - a <= N
    test("4", "5", "1", "1", "Some(-1)");
    // - balanced bounds
    test("33", "97", "6", "6", "Some(2/3)");
    // - asymmetric bounds
    test("33", "97", "2", "30", "Some(2/3)");
    // - asymmetric bounds succeed where the balanced ones fail
    test("444", "1009", "1", "30", "Some(1/25)");
    // - flipped asymmetric bounds fail
    test("444", "1009", "30", "1", "None");
    // - the loop ends with a zero remainder, so the gcd check requires d = 1 and fails
    test("2", "10", "1", "5", "None");
    // - non-unique regime (2 * N * D >= m): pinned outputs, which agree with FLINT's reference
    //   implementation
    test("5", "11", "5", "5", "Some(5)");
    test("7", "11", "5", "5", "Some(-4)");
    // - non-unique regime with a two-limb m and a three-limb d_bound: FLINT 3.6.0's two-limb kernel
    //   misreads the bound here (fmpz_get_uiui drops limbs beyond the second) and spuriously fails,
    //   disagreeing with its own reference implementation, which returns this
    test(
        "778029533528",
        "39510926782646445715540418031384",
        "32098388",
        "157980302531428379809519276806140673520080351",
        "Some(21455872/8785515252104296477493)",
    );
    // - multi-limb, tightest bounds that still succeed
    test(
        "8818342134038800723104056361",
        "12345678987654321012345678901",
        "22",
        "7",
        "Some(22/7)",
    );
    // - either bound one tighter fails
    test(
        "8818342134038800723104056361",
        "12345678987654321012345678901",
        "21",
        "7",
        "None",
    );
    test(
        "8818342134038800723104056361",
        "12345678987654321012345678901",
        "22",
        "6",
        "None",
    );
}

#[test]
#[should_panic]
fn reconstruct_fail_small_m() {
    Rational::reconstruct(Natural::ONE, Natural::TWO);
}

#[test]
#[should_panic]
fn reconstruct_fail_unreduced_a() {
    Rational::reconstruct(Natural::from(5u32), Natural::from(4u32));
}

#[test]
#[should_panic]
fn reconstruct_with_bounds_fail_unreduced_a() {
    Rational::reconstruct_with_bounds(
        Natural::from(5u32),
        Natural::from(4u32),
        &Natural::ONE,
        &Natural::ONE,
    );
}

#[test]
#[should_panic]
fn reconstruct_with_bounds_fail_zero_n_bound() {
    Rational::reconstruct_with_bounds(
        Natural::ONE,
        Natural::from(10u32),
        &Natural::ZERO,
        &Natural::ONE,
    );
}

#[test]
#[should_panic]
fn reconstruct_with_bounds_fail_zero_d_bound() {
    Rational::reconstruct_with_bounds(
        Natural::ONE,
        Natural::from(10u32),
        &Natural::ONE,
        &Natural::ZERO,
    );
}

fn balanced_bound(m: &Natural) -> Natural {
    let mut b = m >> 1u32;
    if m.even() {
        b -= Natural::ONE;
    }
    b.floor_sqrt()
}

// Verifies that a returned rational actually satisfies the reconstruction constraints.
fn check_solution(x: &Rational, a: &Natural, m: &Natural, n_bound: &Natural, d_bound: &Natural) {
    assert!(x.is_valid());
    assert!(x.to_numerator() <= *n_bound);
    assert!(x.to_denominator() <= *d_bound);
    let signed_num = Integer::from_sign_and_abs(*x >= 0u32, x.to_numerator());
    let diff = signed_num - Integer::from(x.to_denominator()) * Integer::from(a);
    assert!(diff.divisible_by(Integer::from(m)));
}

// Exhaustively verifies that no solution exists. Only valid in the unique regime 2 * N * D < m,
// which also guarantees the loop is short.
fn assert_no_solution(a: u64, m: u64, n_bound: u64, d_bound: u64) {
    for d in 1..=d_bound {
        let n0 = a * d % m;
        assert!(!(n0 <= n_bound && n0.gcd(d) == 1));
        assert!(!(m - n0 <= n_bound && (m - n0).gcd(d) == 1));
    }
}

#[test]
fn reconstruct_properties() {
    natural_pair_gen_var_1().test_properties(|(a, m)| {
        let ox = Rational::reconstruct_ref(&a, &m);
        assert_eq!(Rational::reconstruct(a.clone(), m.clone()), ox);
        let b = balanced_bound(&m);
        assert_eq!(
            Rational::reconstruct_with_bounds_ref(&a, &m, &b, &b),
            ox,
            "balanced bounds disagree"
        );
        if let Some(x) = ox {
            check_solution(&x, &a, &m, &b, &b);
        } else if m <= 1_000u32 {
            // The balanced bounds always satisfy 2 * N * D < m.
            assert_no_solution(
                u64::exact_from(&a),
                u64::exact_from(&m),
                u64::exact_from(&b),
                u64::exact_from(&b),
            );
        }
    });

    natural_quadruple_gen_var_1().test_properties(|(a, m, n_bound, d_bound)| {
        let ox = Rational::reconstruct_with_bounds_ref(&a, &m, &n_bound, &d_bound);
        assert_eq!(
            Rational::reconstruct_with_bounds(a.clone(), m.clone(), &n_bound, &d_bound),
            ox
        );
        if let Some(x) = ox {
            check_solution(&x, &a, &m, &n_bound, &d_bound);
        } else if m <= 1_000u32 && (&n_bound * &d_bound) << 1u32 < m {
            assert_no_solution(
                u64::exact_from(&a),
                u64::exact_from(&m),
                u64::exact_from(&n_bound),
                u64::exact_from(&d_bound),
            );
        }
    });

    rational_gen().test_properties(|x| {
        // Round trip: reduce x modulo a large enough coprime modulus, then reconstruct it.
        let n = x.to_numerator();
        let d = x.to_denominator();
        let k = x.to_height();
        let mut m = ((&k * &k) << 1u32) + Natural::ONE;
        while (&d).gcd(&m) != 1u32 {
            m += Natural::ONE;
        }
        let inv = (&d).mod_inverse(&m).unwrap();
        let mut a = &n * inv % &m;
        if x < 0u32 {
            a = &m - a;
        }
        assert_eq!(Rational::reconstruct_ref(&a, &m), Some(x.clone()));
        let n_bound = if n == 0u32 { Natural::ONE } else { n };
        assert_eq!(
            Rational::reconstruct_with_bounds(a, m, &n_bound, &d),
            Some(x)
        );
    });
}
