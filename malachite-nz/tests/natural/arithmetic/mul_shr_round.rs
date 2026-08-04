// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{MulShrRound, MulShrRoundAssign, Pow, ShrRound};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_rounding_mode_quadruple_gen_var_1,
    natural_natural_unsigned_rounding_mode_quadruple_gen_var_2,
};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mul_shr_round() {
    let test = |s, t, bits, rm, out, o: Ordering| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();
        let expected = Natural::from_str(out).unwrap();

        assert_eq!(
            u.clone().mul_shr_round(v.clone(), bits, rm),
            (expected.clone(), o)
        );
        assert_eq!(u.clone().mul_shr_round(&v, bits, rm), (expected.clone(), o));
        assert_eq!(
            (&u).mul_shr_round(v.clone(), bits, rm),
            (expected.clone(), o)
        );
        assert_eq!((&u).mul_shr_round(&v, bits, rm), (expected.clone(), o));

        let mut mut_u = u.clone();
        assert_eq!(mut_u.mul_shr_round_assign(v.clone(), bits, rm), o);
        assert_eq!(mut_u, expected);

        let mut mut_u = u;
        assert_eq!(mut_u.mul_shr_round_assign(&v, bits, rm), o);
        assert_eq!(mut_u, expected);
    };
    // - zero operands
    test("0", "0", 0, Down, "0", Equal);
    test("0", "123", 100, Exact, "0", Equal);
    // - the exact path, entered via trailing zeros
    test("96", "8", 8, Exact, "3", Equal);
    test(
        "1000000000000",
        "1000000000000",
        12,
        Exact,
        "244140625000000000000",
        Equal,
    );
    // - small operands: the full path
    test("100", "200", 8, Down, "78", Less);
    test("100", "200", 8, Up, "79", Greater);
    test("100", "200", 8, Floor, "78", Less);
    test("100", "200", 8, Ceiling, "79", Greater);
    test("100", "200", 8, Nearest, "78", Less);
    // - a tie, broken toward the even neighbor
    test("5", "102", 2, Nearest, "128", Greater);
    // - everything shifted out
    test("100", "200", 1000, Down, "0", Less);
    test("100", "200", 1000, Up, "1", Greater);
    test("100", "200", 1000, Nearest, "0", Less);
}

#[test]
fn test_mul_shr_round_large() {
    // Constructed operands large enough to reach the short-product path, checked against the
    // definitional oracle rather than hardcoded strings.
    let test = |x: &Natural, y: &Natural, bits, rm| {
        let expected = (x * y).shr_round(bits, rm);
        assert_eq!((x).mul_shr_round(y, bits, rm), expected);
    };
    let x = Natural::from(10u32).pow(300);
    let y = Natural::from(3u32).pow(700);
    let total = 997 + 1110;
    for bits in [1000, 1500, 2000, total - 1, total, total + 1] {
        for rm in [Down, Up, Floor, Ceiling, Nearest] {
            test(&x, &y, bits, rm);
        }
    }
    // The adversarial band: (2^k - 1)^2 = 2^(2k) - 2^(k + 1) + 1 has a long run of ones ending just
    // below bit k + 1, so cuts inside that run force the fallback.
    let m = (Natural::ONE << 2000u32) - Natural::ONE;
    for bits in [1500, 1999, 2000, 2001, 2500, 3998, 3999, 4000] {
        for rm in [Down, Up, Nearest] {
            test(&m, &m, bits, rm);
        }
    }
    // Powers of 2: the exact path with maximal trailing zeros.
    let p = Natural::ONE << 1000u32;
    assert_eq!((&p).mul_shr_round(&p, 2000, Exact), (Natural::ONE, Equal));
    assert_eq!(
        (&p).mul_shr_round(&p, 1500, Exact),
        (Natural::ONE << 500u32, Equal)
    );
}

#[test]
fn mul_shr_round_fail() {
    assert_panic!(Natural::from(100u32).mul_shr_round(Natural::from(200u32), 8, Exact));
    assert_panic!(Natural::from(3u32).mul_shr_round(Natural::from(5u32), 1, Exact));
}

#[test]
fn mul_shr_round_properties() {
    natural_natural_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, y, bits, rm)| {
            mul_shr_round_properties_helper(x, y, bits, rm);
        },
    );
    // Densely exercises the short-product path: large operands, cut near the top.
    natural_natural_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, y, bits, rm)| {
            mul_shr_round_properties_helper(x, y, bits, rm);
        },
    );
}

#[allow(clippy::needless_pass_by_value)]
fn mul_shr_round_properties_helper(x: Natural, y: Natural, bits: u64, rm: RoundingMode) {
    let (r, o) = (&x).mul_shr_round(&y, bits, rm);
    assert!(r.is_valid());

    // the definitional oracle: full product, then shift-round
    assert_eq!((&x * &y).shr_round(bits, rm), (r.clone(), o));

    // every spelling agrees
    assert_eq!(x.clone().mul_shr_round(y.clone(), bits, rm), (r.clone(), o));
    assert_eq!(x.clone().mul_shr_round(&y, bits, rm), (r.clone(), o));
    assert_eq!((&x).mul_shr_round(y.clone(), bits, rm), (r.clone(), o));

    let mut mut_x = x.clone();
    assert_eq!(mut_x.mul_shr_round_assign(y.clone(), bits, rm), o);
    assert_eq!(mut_x, r);
    let mut mut_x = x.clone();
    assert_eq!(mut_x.mul_shr_round_assign(&y, bits, rm), o);
    assert_eq!(mut_x, r);

    // multiplication commutes
    assert_eq!((&y).mul_shr_round(&x, bits, rm), (r.clone(), o));

    // exactness is the trailing-zeros identity
    if x == 0u32 || y == 0u32 {
        assert_eq!((r, o), (Natural::ZERO, Equal));
    } else {
        let exact = x.trailing_zeros().unwrap() + y.trailing_zeros().unwrap() >= bits;
        assert_eq!(o == Equal, exact);
        // for Naturals, Down is Floor and Up is Ceiling
        match rm {
            Down | Floor => assert_ne!(o, Greater),
            Up | Ceiling => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            Nearest => {}
        }
    }
}
