// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{MulShrRound, MulShrRoundAssign, ShrRound};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_integer_unsigned_rounding_mode_quadruple_gen_var_1;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mul_shr_round() {
    let test = |s, t, bits, rm, out, o: Ordering| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();
        let expected = Integer::from_str(out).unwrap();

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
    test("0", "0", 0, Down, "0", Equal);
    test("0", "-123", 100, Exact, "0", Equal);
    test("-96", "8", 8, Exact, "-3", Equal);
    // - Floor and Down split on a negative product; Up and Ceiling likewise
    test("-100", "200", 8, Floor, "-79", Less);
    test("-100", "200", 8, Down, "-78", Greater);
    test("-100", "200", 8, Up, "-79", Less);
    test("-100", "200", 8, Ceiling, "-78", Greater);
    test("-100", "200", 8, Nearest, "-78", Greater);
    test("100", "-200", 8, Floor, "-79", Less);
    test("-100", "-200", 8, Floor, "78", Less);
    // - a negative value between -1 and 0
    test("-100", "200", 1000, Floor, "-1", Less);
    test("-100", "200", 1000, Down, "0", Greater);
    test("-100", "200", 1000, Ceiling, "0", Greater);
    test("-100", "200", 1000, Nearest, "0", Greater);
}

#[test]
fn mul_shr_round_fail() {
    assert_panic!(Integer::from(-100).mul_shr_round(Integer::from(200), 8, Exact));
}

#[test]
fn mul_shr_round_properties() {
    integer_integer_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, y, bits, rm)| {
            let (r, o) = (&x).mul_shr_round(&y, bits, rm);
            assert!(r.is_valid());

            // the definitional oracle
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

            // negating one factor negates the result, with the mode and Ordering mirrored
            assert_eq!((&x).mul_shr_round(&(-&y), bits, -rm), (-&r, o.reverse()));

            // sign sanity
            if x == 0 || y == 0 {
                assert_eq!((r, o), (Integer::ZERO, Equal));
            } else {
                let negative = (x < 0) != (y < 0);
                if r != 0 {
                    assert_eq!(r < 0, negative);
                }
            }
        },
    );
}
