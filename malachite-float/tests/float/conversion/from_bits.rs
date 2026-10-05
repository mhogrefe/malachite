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
use malachite_base::test_util::generators::bool_vec_gen;
use malachite_float::test_util::common::{assert_rounding_ordering_consistent, to_hex_string};
use malachite_float::test_util::float::conversion::from_digits::{
    fraction_bits, non_dyadic_fraction, non_dyadic_from_bits_prec_round_naive, sparse_bits,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_unsigned_pair_gen_var_3;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn check_against_naive<I: Clone + Iterator<Item = bool>>(
    bits: &I,
    prec: u64,
    rm: RoundingMode,
    x: &Float,
    o: Ordering,
) {
    let (x_alt, o_alt) = non_dyadic_from_bits_prec_round_naive(bits.clone(), prec, rm);
    assert_eq!(ComparableFloat(x_alt), ComparableFloat(x.clone()));
    assert_eq!(o_alt, o);
}

#[test]
fn test_non_dyadic_from_bits_prec_round() {
    let test = |prec: u64, rm: RoundingMode, out: &str, out_hex: &str, out_o: Ordering| {
        let (x, o) = Float::non_dyadic_from_bits_prec_round(sparse_bits(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);
        check_against_naive(&sparse_bits(), prec, rm, &x, o);
    };
    test(1, Floor, "0.50", "0x0.8#1", Less);
    test(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test(1, Down, "0.50", "0x0.8#1", Less);
    test(1, Up, "1.0", "0x1.0#1", Greater);
    test(1, Nearest, "0.50", "0x0.8#1", Less);

    test(2, Floor, "0.50", "0x0.8#2", Less);
    test(2, Ceiling, "0.75", "0x0.c#2", Greater);
    test(2, Nearest, "0.75", "0x0.c#2", Greater);

    test(10, Floor, "0.64160", "0x0.a44#10", Less);
    test(10, Ceiling, "0.64258", "0x0.a48#10", Greater);
    test(10, Nearest, "0.64160", "0x0.a44#10", Less);

    test(
        100,
        Floor,
        "0.64163256065515386629384277022540",
        "0x0.a442081010080200400400200#100",
        Less,
    );
    test(
        100,
        Ceiling,
        "0.64163256065515386629384277022619",
        "0x0.a442081010080200400400201#100",
        Greater,
    );
    test(
        100,
        Down,
        "0.64163256065515386629384277022540",
        "0x0.a442081010080200400400200#100",
        Less,
    );
    test(
        100,
        Up,
        "0.64163256065515386629384277022619",
        "0x0.a442081010080200400400201#100",
        Greater,
    );
    test(
        100,
        Nearest,
        "0.64163256065515386629384277022540",
        "0x0.a442081010080200400400200#100",
        Less,
    );
}

#[test]
fn test_non_dyadic_from_bits_prec_round_rational() {
    let test =
        |n: u32, d: u32, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, out_o: Ordering| {
            let q = Rational::from_unsigneds(n, d);
            let (x, o) = Float::non_dyadic_from_bits_prec_round(fraction_bits(&q), prec, rm);
            assert!(x.is_valid());
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(o, out_o);
            let (x_alt, o_alt) = Float::from_rational_prec_round(q, prec, rm);
            assert_eq!(ComparableFloat(x_alt), ComparableFloat(x));
            assert_eq!(o_alt, o);
        };
    // 1/3 = 0.010101... in binary: the first bit is 0, so the exponent drops by one.
    test(1, 3, 1, Floor, "0.25", "0x0.4#1", Less);
    test(1, 3, 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test(1, 3, 10, Nearest, "0.33350", "0x0.556#10", Greater);
    test(
        1,
        3,
        64,
        Floor,
        "0.333333333333333333315",
        "0x0.55555555555555550#64",
        Less,
    );
    test(
        1,
        3,
        64,
        Ceiling,
        "0.333333333333333333342",
        "0x0.55555555555555558#64",
        Greater,
    );
    // 2/3 = 0.101010... begins with a 1.
    test(2, 3, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        2,
        3,
        63,
        Up,
        "0.66666666666666666674",
        "0x0.aaaaaaaaaaaaaaac#63",
        Greater,
    );
    test(
        2,
        3,
        65,
        Down,
        "0.666666666666666666658",
        "0x0.aaaaaaaaaaaaaaaa8#65",
        Less,
    );
    // 1/10 begins with three 0 bits.
    test(1, 10, 20, Nearest, "0.10000002", "0x0.19999a#20", Greater);
    // 2047/2049 lies just above 1 - 2^-10, so rounding it up to 10 bits carries to 1.
    test(2047, 2049, 10, Up, "1.0000", "0x1.000#10", Greater);
}

#[test]
fn test_non_dyadic_from_bits_prec() {
    let test = |prec: u64, out: &str, out_hex: &str, out_o: Ordering| {
        let (x, o) = Float::non_dyadic_from_bits_prec(sparse_bits(), prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);
        check_against_naive(&sparse_bits(), prec, Nearest, &x, o);
    };
    test(1, "0.50", "0x0.8#1", Less);
    test(10, "0.64160", "0x0.a44#10", Less);
    test(
        100,
        "0.64163256065515386629384277022540",
        "0x0.a442081010080200400400200#100",
        Less,
    );
}

#[test]
fn non_dyadic_from_bits_prec_round_fail() {
    assert_panic!(Float::non_dyadic_from_bits_prec_round(
        sparse_bits(),
        0,
        Floor
    ));
    assert_panic!(Float::non_dyadic_from_bits_prec_round(
        sparse_bits(),
        10,
        Exact
    ));
}

#[test]
fn non_dyadic_from_bits_prec_fail() {
    assert_panic!(Float::non_dyadic_from_bits_prec(sparse_bits(), 0));
}

fn non_dyadic_from_bits_prec_round_properties_helper<I: Clone + Iterator<Item = bool>>(
    bits: &I,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let (x, o) = Float::non_dyadic_from_bits_prec_round(bits.clone(), prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.get_prec(), Some(prec));
    assert_ne!(o, Equal);
    assert_rounding_ordering_consistent(&x, rm, o);
    check_against_naive(bits, prec, rm, &x, o);
    (x, o)
}

#[test]
fn non_dyadic_from_bits_prec_round_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(q, prec)| {
        let x = non_dyadic_fraction(&q);
        let bits = fraction_bits(&x);
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let (f, o) = non_dyadic_from_bits_prec_round_properties_helper(&bits, prec, rm);
            let (f_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloat(f_alt), ComparableFloat(f));
            assert_eq!(o_alt, o);
        }
    });

    // An arbitrary prefix followed by an irrational tail.
    bool_vec_gen().test_properties(|prefix| {
        let bits = prefix.clone().into_iter().chain(sparse_bits());
        for prec in [1, 2, 3, 10, 63, 64, 65, 100] {
            for rm in exhaustive_rounding_modes() {
                if rm != Exact {
                    non_dyadic_from_bits_prec_round_properties_helper(&bits, prec, rm);
                }
            }
        }
    });
}

#[test]
fn non_dyadic_from_bits_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(q, prec)| {
        let x = non_dyadic_fraction(&q);
        let (f, o) = Float::non_dyadic_from_bits_prec(fraction_bits(&x), prec);
        let (f_alt, o_alt) =
            Float::non_dyadic_from_bits_prec_round(fraction_bits(&x), prec, Nearest);
        assert_eq!(ComparableFloat(f_alt), ComparableFloat(f.clone()));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = Float::from_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloat(f_alt), ComparableFloat(f));
        assert_eq!(o_alt, o);
    });
}
