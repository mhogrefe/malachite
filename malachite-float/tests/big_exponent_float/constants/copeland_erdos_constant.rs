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
fn test_copeland_erdos_constant_base_prec_round() {
    let test = |b: u64, p: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let (x, o_alt) = BigExponentFloat::copeland_erdos_constant_base_prec_round(b, p, rm);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out_hex, "{:?}", (b, p, rm));
        assert_eq!(x.to_string(), out);
        assert_eq!(o_alt, o);
    };
    test(2, 1, Floor, "0.50", "0x0.8", Less);
    test(2, 1, Ceiling, "1.0", "0x1.0", Greater);
    test(2, 1, Down, "0.50", "0x0.8", Less);
    test(2, 1, Up, "1.0", "0x1.0", Greater);
    test(2, 1, Nearest, "0.50", "0x0.8", Less);
    test(2, 2, Floor, "0.50", "0x0.8", Less);
    test(2, 2, Ceiling, "0.75", "0x0.c", Greater);
    test(2, 2, Down, "0.50", "0x0.8", Less);
    test(2, 2, Up, "0.75", "0x0.c", Greater);
    test(2, 2, Nearest, "0.75", "0x0.c", Greater);
    test(2, 10, Floor, "0.73340", "0x0.bbc", Less);
    test(2, 10, Ceiling, "0.734", "0x0.bc", Greater);
    test(2, 10, Down, "0.73340", "0x0.bbc", Less);
    test(2, 10, Up, "0.734", "0x0.bc", Greater);
    test(2, 10, Nearest, "0.734", "0x0.bc", Greater);
    test(
        2,
        100,
        Floor,
        "0.73412151540828612060627828845711",
        "0x0.bbef633bf7f2d35dfaf7ec38f",
        Less,
    );
    test(
        2,
        100,
        Ceiling,
        "0.734121515408286120606278288458",
        "0x0.bbef633bf7f2d35dfaf7ec39",
        Greater,
    );
    test(
        2,
        100,
        Down,
        "0.73412151540828612060627828845711",
        "0x0.bbef633bf7f2d35dfaf7ec38f",
        Less,
    );
    test(
        2,
        100,
        Up,
        "0.734121515408286120606278288458",
        "0x0.bbef633bf7f2d35dfaf7ec39",
        Greater,
    );
    test(
        2,
        100,
        Nearest,
        "0.73412151540828612060627828845711",
        "0x0.bbef633bf7f2d35dfaf7ec38f",
        Less,
    );
    test(3, 1, Floor, "0.50", "0x0.8", Less);
    test(3, 1, Ceiling, "1.0", "0x1.0", Greater);
    test(3, 1, Down, "0.50", "0x0.8", Less);
    test(3, 1, Up, "1.0", "0x1.0", Greater);
    test(3, 1, Nearest, "1.0", "0x1.0", Greater);
    test(3, 2, Floor, "0.75", "0x0.c", Less);
    test(3, 2, Ceiling, "1.0", "0x1.0", Greater);
    test(3, 2, Down, "0.75", "0x0.c", Less);
    test(3, 2, Up, "1.0", "0x1.0", Greater);
    test(3, 2, Nearest, "0.75", "0x0.c", Less);
    test(3, 10, Floor, "0.8008", "0x0.cd", Less);
    test(3, 10, Ceiling, "0.80176", "0x0.cd4", Greater);
    test(3, 10, Down, "0.8008", "0x0.cd", Less);
    test(3, 10, Up, "0.80176", "0x0.cd4", Greater);
    test(3, 10, Nearest, "0.80176", "0x0.cd4", Greater);
    test(
        3,
        100,
        Floor,
        "0.80174949296954506695856974572014",
        "0x0.cd3f746be386f441c9c14f7e5",
        Less,
    );
    test(
        3,
        100,
        Ceiling,
        "0.8017494929695450669585697457209",
        "0x0.cd3f746be386f441c9c14f7e6",
        Greater,
    );
    test(
        3,
        100,
        Down,
        "0.80174949296954506695856974572014",
        "0x0.cd3f746be386f441c9c14f7e5",
        Less,
    );
    test(
        3,
        100,
        Up,
        "0.8017494929695450669585697457209",
        "0x0.cd3f746be386f441c9c14f7e6",
        Greater,
    );
    test(
        3,
        100,
        Nearest,
        "0.8017494929695450669585697457209",
        "0x0.cd3f746be386f441c9c14f7e6",
        Greater,
    );
    test(10, 1, Floor, "0.12", "0x0.2", Less);
    test(10, 1, Ceiling, "0.25", "0x0.4", Greater);
    test(10, 1, Down, "0.12", "0x0.2", Less);
    test(10, 1, Up, "0.25", "0x0.4", Greater);
    test(10, 1, Nearest, "0.25", "0x0.4", Greater);
    test(10, 2, Floor, "0.19", "0x0.3", Less);
    test(10, 2, Ceiling, "0.25", "0x0.4", Greater);
    test(10, 2, Down, "0.19", "0x0.3", Less);
    test(10, 2, Up, "0.25", "0x0.4", Greater);
    test(10, 2, Nearest, "0.25", "0x0.4", Greater);
    test(10, 10, Floor, "0.23560", "0x0.3c5", Less);
    test(10, 10, Ceiling, "0.2358", "0x0.3c6", Greater);
    test(10, 10, Down, "0.23560", "0x0.3c5", Less);
    test(10, 10, Up, "0.2358", "0x0.3c6", Greater);
    test(10, 10, Nearest, "0.23560", "0x0.3c5", Less);
    test(
        10,
        100,
        Floor,
        "0.23571113171923293137414347535946",
        "0x0.3c579092098975475a5c13b984",
        Less,
    );
    test(
        10,
        100,
        Ceiling,
        "0.2357111317192329313741434753597",
        "0x0.3c579092098975475a5c13b988",
        Greater,
    );
    test(
        10,
        100,
        Down,
        "0.23571113171923293137414347535946",
        "0x0.3c579092098975475a5c13b984",
        Less,
    );
    test(
        10,
        100,
        Up,
        "0.2357111317192329313741434753597",
        "0x0.3c579092098975475a5c13b988",
        Greater,
    );
    test(
        10,
        100,
        Nearest,
        "0.2357111317192329313741434753597",
        "0x0.3c579092098975475a5c13b988",
        Greater,
    );
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_round_fail_1() {
    BigExponentFloat::copeland_erdos_constant_base_prec_round(10, 0, Floor);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_round_fail_2() {
    BigExponentFloat::copeland_erdos_constant_base_prec_round(10, 10, Exact);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_round_fail_3() {
    BigExponentFloat::copeland_erdos_constant_base_prec_round(1, 10, Floor);
}

#[test]
fn copeland_erdos_constant_base_prec_round_properties() {
    unsigned_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(b, p, rm)| {
        let (x, o) = BigExponentFloat::copeland_erdos_constant_base_prec_round(b, p, rm);
        assert!(x.is_valid());
        let (y, o_alt) = Float::copeland_erdos_constant_base_prec_round(b, p, rm);
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
            let (x_alt, o_alt) = BigExponentFloat::copeland_erdos_constant_base_prec(b, p);
            assert!(same(&x_alt, &x));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn test_copeland_erdos_constant_base_prec() {
    let test = |b: u64, p: u64, out: &str, out_hex: &str, o: Ordering| {
        let (x, o_alt) = BigExponentFloat::copeland_erdos_constant_base_prec(b, p);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out_hex, "{:?}", (b, p));
        assert_eq!(x.to_string(), out);
        assert_eq!(o_alt, o);
    };
    test(2, 1, "0.50", "0x0.8", Less);
    test(2, 2, "0.75", "0x0.c", Greater);
    test(2, 10, "0.734", "0x0.bc", Greater);
    test(
        2,
        100,
        "0.73412151540828612060627828845711",
        "0x0.bbef633bf7f2d35dfaf7ec38f",
        Less,
    );
    test(3, 1, "1.0", "0x1.0", Greater);
    test(3, 2, "0.75", "0x0.c", Less);
    test(3, 10, "0.80176", "0x0.cd4", Greater);
    test(
        3,
        100,
        "0.8017494929695450669585697457209",
        "0x0.cd3f746be386f441c9c14f7e6",
        Greater,
    );
    test(10, 1, "0.25", "0x0.4", Greater);
    test(10, 2, "0.25", "0x0.4", Greater);
    test(10, 10, "0.23560", "0x0.3c5", Less);
    test(
        10,
        100,
        "0.2357111317192329313741434753597",
        "0x0.3c579092098975475a5c13b988",
        Greater,
    );
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_fail_1() {
    BigExponentFloat::copeland_erdos_constant_base_prec(10, 0);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_fail_2() {
    BigExponentFloat::copeland_erdos_constant_base_prec(1, 10);
}

#[test]
fn copeland_erdos_constant_base_prec_properties() {
    unsigned_pair_gen_var_51().test_properties(|(b, p)| {
        let (x, o) = BigExponentFloat::copeland_erdos_constant_base_prec(b, p);
        assert!(x.is_valid());
        let (y, o_alt) = Float::copeland_erdos_constant_base_prec(b, p);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
    });
}

#[test]
fn test_copeland_erdos_constant_prec_round() {
    let test = |p: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let (x, o_alt) = BigExponentFloat::copeland_erdos_constant_prec_round(p, rm);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out_hex, "{:?}", (p, rm));
        assert_eq!(x.to_string(), out);
        assert_eq!(o_alt, o);
    };
    test(1, Floor, "0.12", "0x0.2", Less);
    test(1, Ceiling, "0.25", "0x0.4", Greater);
    test(1, Down, "0.12", "0x0.2", Less);
    test(1, Up, "0.25", "0x0.4", Greater);
    test(1, Nearest, "0.25", "0x0.4", Greater);
    test(2, Floor, "0.19", "0x0.3", Less);
    test(2, Ceiling, "0.25", "0x0.4", Greater);
    test(2, Down, "0.19", "0x0.3", Less);
    test(2, Up, "0.25", "0x0.4", Greater);
    test(2, Nearest, "0.25", "0x0.4", Greater);
    test(10, Floor, "0.23560", "0x0.3c5", Less);
    test(10, Ceiling, "0.2358", "0x0.3c6", Greater);
    test(10, Down, "0.23560", "0x0.3c5", Less);
    test(10, Up, "0.2358", "0x0.3c6", Greater);
    test(10, Nearest, "0.23560", "0x0.3c5", Less);
    test(
        100,
        Floor,
        "0.23571113171923293137414347535946",
        "0x0.3c579092098975475a5c13b984",
        Less,
    );
    test(
        100,
        Ceiling,
        "0.2357111317192329313741434753597",
        "0x0.3c579092098975475a5c13b988",
        Greater,
    );
    test(
        100,
        Down,
        "0.23571113171923293137414347535946",
        "0x0.3c579092098975475a5c13b984",
        Less,
    );
    test(
        100,
        Up,
        "0.2357111317192329313741434753597",
        "0x0.3c579092098975475a5c13b988",
        Greater,
    );
    test(
        100,
        Nearest,
        "0.2357111317192329313741434753597",
        "0x0.3c579092098975475a5c13b988",
        Greater,
    );
}

#[test]
#[should_panic]
fn copeland_erdos_constant_prec_round_fail_1() {
    BigExponentFloat::copeland_erdos_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_prec_round_fail_2() {
    BigExponentFloat::copeland_erdos_constant_prec_round(10, Exact);
}

#[test]
fn copeland_erdos_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(p, rm)| {
        let (x, o) = BigExponentFloat::copeland_erdos_constant_prec_round(p, rm);
        assert!(x.is_valid());
        let (y, o_alt) = Float::copeland_erdos_constant_prec_round(p, rm);
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
            let (x_alt, o_alt) = BigExponentFloat::copeland_erdos_constant_prec(p);
            assert!(same(&x_alt, &x));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn test_copeland_erdos_constant_prec() {
    let test = |p: u64, out: &str, out_hex: &str, o: Ordering| {
        let (x, o_alt) = BigExponentFloat::copeland_erdos_constant_prec(p);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out_hex, "{:?}", (p));
        assert_eq!(x.to_string(), out);
        assert_eq!(o_alt, o);
    };
    test(1, "0.25", "0x0.4", Greater);
    test(2, "0.25", "0x0.4", Greater);
    test(10, "0.23560", "0x0.3c5", Less);
    test(
        100,
        "0.2357111317192329313741434753597",
        "0x0.3c579092098975475a5c13b988",
        Greater,
    );
}

#[test]
#[should_panic]
fn copeland_erdos_constant_prec_fail_1() {
    BigExponentFloat::copeland_erdos_constant_prec(0);
}

#[test]
fn copeland_erdos_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|p| {
        let (x, o) = BigExponentFloat::copeland_erdos_constant_prec(p);
        assert!(x.is_valid());
        let (y, o_alt) = Float::copeland_erdos_constant_prec(p);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
    });
}
