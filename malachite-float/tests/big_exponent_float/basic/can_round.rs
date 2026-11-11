// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_to_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::big_exponent_float_gen_var_1;

const RMS: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

#[test]
fn test_can_round() {
    let test = |s, err, rnd1, rnd2, prec, out| {
        let x = parse_big_exponent_float_hex(s);
        assert_eq!(x.can_round(err, rnd1, rnd2, prec), out);
        assert_eq!(
            big_exponent_float_to_float(&x)
                .unwrap()
                .can_round(err, rnd1, rnd2, prec),
            out
        );
    };
    test("0x1.0#1", 10, Nearest, Nearest, 5, true);
    test("0x1.0#1", 5, Floor, Nearest, 5, false);
    test(
        "0x1.6a09e667f3bcc908b2fb13660#100",
        90,
        Nearest,
        Nearest,
        53,
        true,
    );
    test(
        "0x1.6a09e667f3bcc908b2fb13660#100",
        53,
        Nearest,
        Nearest,
        53,
        false,
    );
    for x in [BigExponentFloat::NAN, BigExponentFloat::INFINITY, BigExponentFloat::ZERO] {
        assert!(!x.can_round(100, Nearest, Nearest, 1));
    }
}

#[test]
#[should_panic]
fn can_round_fail_1() {
    BigExponentFloat::ONE.can_round(10, Nearest, Nearest, 0);
}

#[test]
#[should_panic]
fn can_round_fail_2() {
    BigExponentFloat::ONE.can_round(10, Exact, Nearest, 1);
}

#[test]
#[should_panic]
fn can_round_fail_3() {
    BigExponentFloat::ONE.can_round(10, Nearest, Exact, 1);
}

#[test]
fn can_round_properties() {
    // - agreement with `Float` at the same precision, which has the same significand
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        let p = x.get_prec().unwrap_or(1);
        for prec in [1, p, p + 1] {
            for err in [p - 1, p, p + 1, p + 10] {
                let err = i64::try_from(err).unwrap();
                for rnd1 in RMS {
                    for rnd2 in RMS {
                        assert_eq!(
                            x.can_round(err, rnd1, rnd2, prec),
                            f.can_round(err, rnd1, rnd2, prec)
                        );
                    }
                }
            }
        }
    });
}
