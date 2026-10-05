// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::test_util::common::{assert_rounding_ordering_consistent, to_hex_string};
use malachite_float::test_util::float::conversion::from_digits::{
    fraction_digits, fraction_power_of_2_digits, non_dyadic_fraction,
    non_dyadic_from_digits_prec_round_naive, sparse_bits,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_unsigned_pair_gen_var_3;
use std::cmp::Ordering::{self, *};
use std::iter::repeat;
use std::panic::catch_unwind;

// Bases that are powers of 2 take the bit-copying path; the others take the bracketing loop.
const BASES: [u64; 10] = [2, 3, 5, 6, 8, 10, 16, 36, 1000, u64::MAX];
const LOG_BASES: [u64; 9] = [1, 2, 3, 4, 5, 8, 32, 63, 64];

// The digits 1, 0, 1, 0, 0, 1, ... in any base: irrational for every base.
fn sparse_digits() -> impl Iterator<Item = u64> + Clone {
    sparse_bits().map(u64::from)
}

fn check_against_naive<I: Clone + Iterator<Item = u64>>(
    digits: &I,
    base: u64,
    prec: u64,
    rm: RoundingMode,
    x: &Float,
    o: Ordering,
) {
    let (x_alt, o_alt) = non_dyadic_from_digits_prec_round_naive(digits.clone(), base, prec, rm);
    assert_eq!(ComparableFloat(x_alt), ComparableFloat(x.clone()));
    assert_eq!(o_alt, o);
}

#[test]
fn test_non_dyadic_from_digits_prec_round() {
    let test =
        |base: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, out_o: Ordering| {
            let (x, o) = Float::non_dyadic_from_digits_prec_round(sparse_digits(), base, prec, rm);
            assert!(x.is_valid());
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(o, out_o);
            check_against_naive(&sparse_digits(), base, prec, rm, &x, o);
        };
    test(3, 1, Floor, "0.25", "0x0.4#1", Less);
    test(3, 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test(3, 10, Nearest, "0.37158", "0x0.5f2#10", Less);
    test(
        3,
        100,
        Down,
        "0.37175911735802381558129569787145",
        "0x0.5f2b9b030ae3c8c18d1d781918#100",
        Less,
    );
    test(
        3,
        100,
        Up,
        "0.37175911735802381558129569787185",
        "0x0.5f2b9b030ae3c8c18d1d781920#100",
        Greater,
    );
    test(10, 1, Nearest, "0.12", "0x0.2#1", Greater);
    test(10, 20, Floor, "0.10100091", "0x0.19db32#20", Less);
    test(10, 20, Ceiling, "0.10100102", "0x0.19db34#20", Greater);
    test(
        10,
        100,
        Nearest,
        "0.10100100010000100000100000010000",
        "0x0.19db33984af4beece6adf0b204#100",
        Greater,
    );
    test(16, 10, Floor, "0.062744", "0x0.1010#10", Less);
    test(16, 10, Ceiling, "0.062866", "0x0.1018#10", Greater);
    test(
        1000,
        64,
        Nearest,
        "0.00100000100000000099996",
        "0x0.0041893b9749a20bc58#64",
        Less,
    );
}

#[test]
fn test_non_dyadic_from_digits_prec_round_rational() {
    let test = |n: u32,
                d: u32,
                base: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                out_o: Ordering| {
        let q = Rational::from_unsigneds(n, d);
        let (x, o) =
            Float::non_dyadic_from_digits_prec_round(fraction_digits(&q, base), base, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);
        let (x_alt, o_alt) = Float::from_rational_prec_round(q, prec, rm);
        assert_eq!(ComparableFloat(x_alt), ComparableFloat(x));
        assert_eq!(o_alt, o);
    };
    // 0.333... in base 10.
    test(1, 3, 10, 20, Floor, "0.33333302", "0x0.555550#20", Less);
    test(
        1,
        3,
        10,
        20,
        Ceiling,
        "0.33333349",
        "0x0.555558#20",
        Greater,
    );
    // 1/3 is 0.1 in base 3: a terminating expansion, padded with 0s.
    test(1, 3, 3, 10, Nearest, "0.33350", "0x0.556#10", Greater);
    test(
        1,
        3,
        3,
        64,
        Up,
        "0.333333333333333333342",
        "0x0.55555555555555558#64",
        Greater,
    );
    // 1/10 is 0.1 in base 10.
    test(
        1,
        10,
        10,
        30,
        Down,
        "0.099999999977",
        "0x0.199999998#30",
        Less,
    );
    // 1/7 has a repeating expansion in base 10.
    test(
        1,
        7,
        10,
        50,
        Nearest,
        "0.14285714285714279",
        "0x0.2492492492492#50",
        Less,
    );
    test(
        5,
        7,
        36,
        65,
        Floor,
        "0.714285714285714285691",
        "0x0.b6db6db6db6db6db0#65",
        Less,
    );
}

#[test]
fn test_non_dyadic_from_digits_prec() {
    let test = |base: u64, prec: u64, out: &str, out_hex: &str, out_o: Ordering| {
        let (x, o) = Float::non_dyadic_from_digits_prec(sparse_digits(), base, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);
        check_against_naive(&sparse_digits(), base, prec, Nearest, &x, o);
    };
    test(3, 10, "0.37158", "0x0.5f2#10", Less);
    test(
        10,
        100,
        "0.10100100010000100000100000010000",
        "0x0.19db33984af4beece6adf0b204#100",
        Greater,
    );
    // The documentation's example: 1/3 in base 10, rounded down.
    let (x, o) = Float::non_dyadic_from_digits_prec_round(repeat(3), 10, 20, Floor);
    assert_eq!(x.to_string(), "0.33333302");
    assert_eq!(o, Less);
}

#[test]
fn test_non_dyadic_from_power_of_2_digits_prec_round() {
    let test =
        |log_base: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, out_o: Ordering| {
            let (x, o) = Float::non_dyadic_from_power_of_2_digits_prec_round(
                sparse_digits(),
                log_base,
                prec,
                rm,
            );
            assert!(x.is_valid());
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(o, out_o);
            check_against_naive(&sparse_digits(), 1 << log_base, prec, rm, &x, o);
        };
    test(1, 10, Floor, "0.64160", "0x0.a44#10", Less);
    test(1, 10, Ceiling, "0.64258", "0x0.a48#10", Greater);
    test(3, 1, Nearest, "0.12", "0x0.2#1", Less);
    test(3, 20, Down, "0.12695694", "0x0.208040#20", Less);
    test(
        4,
        64,
        Up,
        "0.0627442002305542709665",
        "0x0.10100100010000102#64",
        Greater,
    );
    test(
        5,
        100,
        Nearest,
        "0.031280518509448462793924685529883",
        "0x0.08020004000040000020000001#100",
        Greater,
    );
}

#[test]
fn test_non_dyadic_from_power_of_2_digits_prec() {
    let test = |log_base: u64, prec: u64, out: &str, out_hex: &str, out_o: Ordering| {
        let (x, o) = Float::non_dyadic_from_power_of_2_digits_prec(sparse_digits(), log_base, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);
        check_against_naive(&sparse_digits(), 1 << log_base, prec, Nearest, &x, o);
    };
    test(1, 10, "0.64160", "0x0.a44#10", Less);
    test(
        4,
        100,
        "0.062744200230554270959759717925107",
        "0x0.10100100010000100000100000#100",
        Less,
    );
}

#[test]
fn non_dyadic_from_digits_prec_round_fail() {
    assert_panic!(Float::non_dyadic_from_digits_prec_round(
        sparse_digits(),
        1,
        10,
        Floor
    ));
    assert_panic!(Float::non_dyadic_from_digits_prec_round(
        sparse_digits(),
        10,
        0,
        Floor
    ));
    assert_panic!(Float::non_dyadic_from_digits_prec_round(
        sparse_digits(),
        10,
        10,
        Exact
    ));
    assert_panic!(Float::non_dyadic_from_digits_prec_round(
        repeat(10),
        10,
        10,
        Floor
    ));
}

#[test]
fn non_dyadic_from_digits_prec_fail() {
    assert_panic!(Float::non_dyadic_from_digits_prec(sparse_digits(), 1, 10));
    assert_panic!(Float::non_dyadic_from_digits_prec(sparse_digits(), 10, 0));
}

#[test]
fn non_dyadic_from_power_of_2_digits_prec_round_fail() {
    assert_panic!(Float::non_dyadic_from_power_of_2_digits_prec_round(
        sparse_digits(),
        0,
        10,
        Floor
    ));
    assert_panic!(Float::non_dyadic_from_power_of_2_digits_prec_round(
        sparse_digits(),
        65,
        10,
        Floor
    ));
    assert_panic!(Float::non_dyadic_from_power_of_2_digits_prec_round(
        sparse_digits(),
        3,
        0,
        Floor
    ));
    assert_panic!(Float::non_dyadic_from_power_of_2_digits_prec_round(
        sparse_digits(),
        3,
        10,
        Exact
    ));
    assert_panic!(Float::non_dyadic_from_power_of_2_digits_prec_round(
        repeat(8),
        3,
        10,
        Floor
    ));
}

#[test]
fn non_dyadic_from_power_of_2_digits_prec_fail() {
    assert_panic!(Float::non_dyadic_from_power_of_2_digits_prec(
        sparse_digits(),
        0,
        10
    ));
    assert_panic!(Float::non_dyadic_from_power_of_2_digits_prec(
        sparse_digits(),
        3,
        0
    ));
}

fn check_output(x: &Float, o: Ordering, prec: u64, rm: RoundingMode) {
    assert!(x.is_valid());
    assert_eq!(x.get_prec(), Some(prec));
    assert_ne!(o, Equal);
    assert_rounding_ordering_consistent(x, rm, o);
}

#[test]
fn non_dyadic_from_digits_prec_round_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(q, prec)| {
        let x = non_dyadic_fraction(&q);
        for base in BASES {
            let digits = fraction_digits(&x, base);
            for rm in exhaustive_rounding_modes() {
                if rm == Exact {
                    continue;
                }
                let (f, o) =
                    Float::non_dyadic_from_digits_prec_round(digits.clone(), base, prec, rm);
                check_output(&f, o, prec, rm);
                let (f_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
                assert_eq!(ComparableFloat(f_alt), ComparableFloat(f.clone()));
                assert_eq!(o_alt, o);
                check_against_naive(&digits, base, prec, rm, &f, o);
            }
        }
    });
}

#[test]
fn non_dyadic_from_digits_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(q, prec)| {
        let x = non_dyadic_fraction(&q);
        for base in BASES {
            let (f, o) = Float::non_dyadic_from_digits_prec(fraction_digits(&x, base), base, prec);
            let (f_alt, o_alt) = Float::non_dyadic_from_digits_prec_round(
                fraction_digits(&x, base),
                base,
                prec,
                Nearest,
            );
            assert_eq!(ComparableFloat(f_alt), ComparableFloat(f));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn non_dyadic_from_power_of_2_digits_prec_round_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(q, prec)| {
        let x = non_dyadic_fraction(&q);
        for log_base in LOG_BASES {
            let digits = fraction_power_of_2_digits(&x, log_base);
            for rm in exhaustive_rounding_modes() {
                if rm == Exact {
                    continue;
                }
                let (f, o) = Float::non_dyadic_from_power_of_2_digits_prec_round(
                    digits.clone(),
                    log_base,
                    prec,
                    rm,
                );
                check_output(&f, o, prec, rm);
                let (f_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
                assert_eq!(ComparableFloat(f_alt), ComparableFloat(f.clone()));
                assert_eq!(o_alt, o);
                if log_base < 64 {
                    // Through the general function, which defers to this one for power-of-2
                    // bases.
                    let (f_alt, o_alt) = Float::non_dyadic_from_digits_prec_round(
                        digits.clone(),
                        1 << log_base,
                        prec,
                        rm,
                    );
                    assert_eq!(ComparableFloat(f_alt), ComparableFloat(f));
                    assert_eq!(o_alt, o);
                }
            }
        }
    });
}

#[test]
fn non_dyadic_from_power_of_2_digits_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(q, prec)| {
        let x = non_dyadic_fraction(&q);
        for log_base in LOG_BASES {
            let (f, o) = Float::non_dyadic_from_power_of_2_digits_prec(
                fraction_power_of_2_digits(&x, log_base),
                log_base,
                prec,
            );
            let (f_alt, o_alt) = Float::non_dyadic_from_power_of_2_digits_prec_round(
                fraction_power_of_2_digits(&x, log_base),
                log_base,
                prec,
                Nearest,
            );
            assert_eq!(ComparableFloat(f_alt), ComparableFloat(f));
            assert_eq!(o_alt, o);
        }
    });
}
