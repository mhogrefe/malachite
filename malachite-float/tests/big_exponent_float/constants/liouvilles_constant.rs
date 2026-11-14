// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::generators::{
    unsigned_pair_gen_var_51, unsigned_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{BigExponentFloat, Float};
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_liouvilles_constant_base_prec_round() {
    let cases: &[(u64, u64, RoundingMode, &str, Ordering)] = &[
        (2, 1, Floor, "0x0.8", Less),
        (2, 1, Ceiling, "0x1.0", Greater),
        (2, 1, Down, "0x0.8", Less),
        (2, 1, Up, "0x1.0", Greater),
        (2, 1, Nearest, "0x1.0", Greater),
        (2, 2, Floor, "0x0.c", Less),
        (2, 2, Ceiling, "0x1.0", Greater),
        (2, 2, Down, "0x0.c", Less),
        (2, 2, Up, "0x1.0", Greater),
        (2, 2, Nearest, "0x0.c", Less),
        (2, 10, Floor, "0x0.c4", Less),
        (2, 10, Ceiling, "0x0.c44", Greater),
        (2, 10, Down, "0x0.c4", Less),
        (2, 10, Up, "0x0.c44", Greater),
        (2, 10, Nearest, "0x0.c4", Less),
        (2, 100, Floor, "0x0.c40001", Less),
        (2, 100, Ceiling, "0x0.c400010000000000000000001", Greater),
        (2, 100, Down, "0x0.c40001", Less),
        (2, 100, Up, "0x0.c400010000000000000000001", Greater),
        (2, 100, Nearest, "0x0.c40001", Less),
        (3, 1, Floor, "0x0.4", Less),
        (3, 1, Ceiling, "0x0.8", Greater),
        (3, 1, Down, "0x0.4", Less),
        (3, 1, Up, "0x0.8", Greater),
        (3, 1, Nearest, "0x0.8", Greater),
        (3, 2, Floor, "0x0.6", Less),
        (3, 2, Ceiling, "0x0.8", Greater),
        (3, 2, Down, "0x0.6", Less),
        (3, 2, Up, "0x0.8", Greater),
        (3, 2, Nearest, "0x0.8", Greater),
        (3, 10, Floor, "0x0.722", Less),
        (3, 10, Ceiling, "0x0.724", Greater),
        (3, 10, Down, "0x0.722", Less),
        (3, 10, Up, "0x0.724", Greater),
        (3, 10, Nearest, "0x0.722", Less),
        (3, 100, Floor, "0x0.722102754dfd33a95c713adf5", Less),
        (3, 100, Ceiling, "0x0.722102754dfd33a95c713adf58", Greater),
        (3, 100, Down, "0x0.722102754dfd33a95c713adf5", Less),
        (3, 100, Up, "0x0.722102754dfd33a95c713adf58", Greater),
        (3, 100, Nearest, "0x0.722102754dfd33a95c713adf58", Greater),
        (10, 1, Floor, "0x0.1", Less),
        (10, 1, Ceiling, "0x0.2", Greater),
        (10, 1, Down, "0x0.1", Less),
        (10, 1, Up, "0x0.2", Greater),
        (10, 1, Nearest, "0x0.2", Greater),
        (10, 2, Floor, "0x0.18", Less),
        (10, 2, Ceiling, "0x0.2", Greater),
        (10, 2, Down, "0x0.18", Less),
        (10, 2, Up, "0x0.2", Greater),
        (10, 2, Nearest, "0x0.2", Greater),
        (10, 10, Floor, "0x0.1c28", Less),
        (10, 10, Ceiling, "0x0.1c3", Greater),
        (10, 10, Down, "0x0.1c28", Less),
        (10, 10, Up, "0x0.1c3", Greater),
        (10, 10, Nearest, "0x0.1c28", Less),
        (10, 100, Floor, "0x0.1c29068986fcdee34fc7466d12", Less),
        (10, 100, Ceiling, "0x0.1c29068986fcdee34fc7466d14", Greater),
        (10, 100, Down, "0x0.1c29068986fcdee34fc7466d12", Less),
        (10, 100, Up, "0x0.1c29068986fcdee34fc7466d14", Greater),
        (10, 100, Nearest, "0x0.1c29068986fcdee34fc7466d12", Less),
    ];
    for &(b, p, rm, out, o) in cases {
        let (x, o_alt) = BigExponentFloat::liouvilles_constant_base_prec_round(b, p, rm);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out, "{:?}", (b, p, rm));
        assert_eq!(o_alt, o);
    }
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_round_fail_1() {
    BigExponentFloat::liouvilles_constant_base_prec_round(10, 0, Floor);
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_round_fail_2() {
    BigExponentFloat::liouvilles_constant_base_prec_round(10, 10, Exact);
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_round_fail_3() {
    BigExponentFloat::liouvilles_constant_base_prec_round(1, 10, Floor);
}

#[test]
fn liouvilles_constant_base_prec_round_properties() {
    unsigned_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(b, p, rm)| {
        let (x, o) = BigExponentFloat::liouvilles_constant_base_prec_round(b, p, rm);
        assert!(x.is_valid());
        let (y, o_alt) = Float::liouvilles_constant_base_prec_round(b, p, rm);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
        match rm {
            Floor | Down => assert_eq!(o, Less),
            Ceiling | Up => assert_eq!(o, Greater),
            _ => {}
        }
        if rm == Nearest {
            let (x_alt, o_alt) = BigExponentFloat::liouvilles_constant_base_prec(b, p);
            assert!(same(&x_alt, &x));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn test_liouvilles_constant_base_prec() {
    let cases: &[(u64, u64, &str, Ordering)] = &[
        (2, 1, "0x1.0", Greater),
        (2, 2, "0x0.c", Less),
        (2, 10, "0x0.c4", Less),
        (2, 100, "0x0.c40001", Less),
        (3, 1, "0x0.8", Greater),
        (3, 2, "0x0.8", Greater),
        (3, 10, "0x0.722", Less),
        (3, 100, "0x0.722102754dfd33a95c713adf58", Greater),
        (10, 1, "0x0.2", Greater),
        (10, 2, "0x0.2", Greater),
        (10, 10, "0x0.1c28", Less),
        (10, 100, "0x0.1c29068986fcdee34fc7466d12", Less),
    ];
    for &(b, p, out, o) in cases {
        let (x, o_alt) = BigExponentFloat::liouvilles_constant_base_prec(b, p);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out, "{:?}", (b, p));
        assert_eq!(o_alt, o);
    }
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_fail_1() {
    BigExponentFloat::liouvilles_constant_base_prec(10, 0);
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_fail_2() {
    BigExponentFloat::liouvilles_constant_base_prec(1, 10);
}

#[test]
fn liouvilles_constant_base_prec_properties() {
    unsigned_pair_gen_var_51().test_properties(|(b, p)| {
        let (x, o) = BigExponentFloat::liouvilles_constant_base_prec(b, p);
        assert!(x.is_valid());
        let (y, o_alt) = Float::liouvilles_constant_base_prec(b, p);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
    });
}

#[test]
fn test_liouvilles_constant_prec_round() {
    let cases: &[(u64, RoundingMode, &str, Ordering)] = &[
        (1, Floor, "0x0.1", Less),
        (1, Ceiling, "0x0.2", Greater),
        (1, Down, "0x0.1", Less),
        (1, Up, "0x0.2", Greater),
        (1, Nearest, "0x0.2", Greater),
        (2, Floor, "0x0.18", Less),
        (2, Ceiling, "0x0.2", Greater),
        (2, Down, "0x0.18", Less),
        (2, Up, "0x0.2", Greater),
        (2, Nearest, "0x0.2", Greater),
        (10, Floor, "0x0.1c28", Less),
        (10, Ceiling, "0x0.1c3", Greater),
        (10, Down, "0x0.1c28", Less),
        (10, Up, "0x0.1c3", Greater),
        (10, Nearest, "0x0.1c28", Less),
        (100, Floor, "0x0.1c29068986fcdee34fc7466d12", Less),
        (100, Ceiling, "0x0.1c29068986fcdee34fc7466d14", Greater),
        (100, Down, "0x0.1c29068986fcdee34fc7466d12", Less),
        (100, Up, "0x0.1c29068986fcdee34fc7466d14", Greater),
        (100, Nearest, "0x0.1c29068986fcdee34fc7466d12", Less),
    ];
    for &(p, rm, out, o) in cases {
        let (x, o_alt) = BigExponentFloat::liouvilles_constant_prec_round(p, rm);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out, "{:?}", (p, rm));
        assert_eq!(o_alt, o);
    }
}

#[test]
#[should_panic]
fn liouvilles_constant_prec_round_fail_1() {
    BigExponentFloat::liouvilles_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn liouvilles_constant_prec_round_fail_2() {
    BigExponentFloat::liouvilles_constant_prec_round(10, Exact);
}

#[test]
fn liouvilles_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(p, rm)| {
        let (x, o) = BigExponentFloat::liouvilles_constant_prec_round(p, rm);
        assert!(x.is_valid());
        let (y, o_alt) = Float::liouvilles_constant_prec_round(p, rm);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
        match rm {
            Floor | Down => assert_eq!(o, Less),
            Ceiling | Up => assert_eq!(o, Greater),
            _ => {}
        }
        if rm == Nearest {
            let (x_alt, o_alt) = BigExponentFloat::liouvilles_constant_prec(p);
            assert!(same(&x_alt, &x));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn test_liouvilles_constant_prec() {
    let cases: &[(u64, &str, Ordering)] = &[
        (1, "0x0.2", Greater),
        (2, "0x0.2", Greater),
        (10, "0x0.1c28", Less),
        (100, "0x0.1c29068986fcdee34fc7466d12", Less),
    ];
    for &(p, out, o) in cases {
        let (x, o_alt) = BigExponentFloat::liouvilles_constant_prec(p);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out, "{:?}", (p));
        assert_eq!(o_alt, o);
    }
}

#[test]
#[should_panic]
fn liouvilles_constant_prec_fail_1() {
    BigExponentFloat::liouvilles_constant_prec(0);
}

#[test]
fn liouvilles_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|p| {
        let (x, o) = BigExponentFloat::liouvilles_constant_prec(p);
        assert!(x.is_valid());
        let (y, o_alt) = Float::liouvilles_constant_prec(p);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
    });
}
