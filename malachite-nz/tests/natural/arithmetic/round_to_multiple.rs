// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, DivRound, DivisibleBy, Parity, RoundToMultiple, RoundToMultipleAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::unsigned_unsigned_rounding_mode_triple_gen_var_2;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_rounding_mode_triple_gen_var_2, natural_pair_gen_var_5, natural_pair_gen_var_7,
    natural_rounding_mode_pair_gen,
};
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_round_to_multiple() {
    let test = |s, t, rm, quotient, o| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let mut x = u.clone();
        assert_eq!(x.round_to_multiple_assign(v.clone(), rm), o);
        assert_eq!(x.to_string(), quotient);
        assert!(x.is_valid());

        let mut x = u.clone();
        assert_eq!(x.round_to_multiple_assign(&v, rm), o);
        assert_eq!(x.to_string(), quotient);
        assert!(x.is_valid());

        let (r, o_alt) = u.clone().round_to_multiple(v.clone(), rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (r, o_alt) = u.clone().round_to_multiple(&v, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (r, o_alt) = (&u).round_to_multiple(v.clone(), rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
        assert_eq!(o_alt, o);

        let (r, o_alt) = (&u).round_to_multiple(&v, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
        assert_eq!(o_alt, o);
    };
    test("0", "1", Down, "0", Equal);
    test("0", "1", Floor, "0", Equal);
    test("0", "1", Up, "0", Equal);
    test("0", "1", Ceiling, "0", Equal);
    test("0", "1", Nearest, "0", Equal);
    test("0", "1", Exact, "0", Equal);

    test("0", "123", Down, "0", Equal);
    test("0", "123", Floor, "0", Equal);
    test("0", "123", Up, "0", Equal);
    test("0", "123", Ceiling, "0", Equal);
    test("0", "123", Nearest, "0", Equal);
    test("0", "123", Exact, "0", Equal);

    test("1", "1", Down, "1", Equal);
    test("1", "1", Floor, "1", Equal);
    test("1", "1", Up, "1", Equal);
    test("1", "1", Ceiling, "1", Equal);
    test("1", "1", Nearest, "1", Equal);
    test("1", "1", Exact, "1", Equal);

    test("123", "1", Down, "123", Equal);
    test("123", "1", Floor, "123", Equal);
    test("123", "1", Up, "123", Equal);
    test("123", "1", Ceiling, "123", Equal);
    test("123", "1", Nearest, "123", Equal);
    test("123", "1", Exact, "123", Equal);

    test("123", "2", Down, "122", Less);
    test("123", "2", Floor, "122", Less);
    test("123", "2", Up, "124", Greater);
    test("123", "2", Ceiling, "124", Greater);
    test("123", "2", Nearest, "124", Greater);

    test("125", "2", Down, "124", Less);
    test("125", "2", Floor, "124", Less);
    test("125", "2", Up, "126", Greater);
    test("125", "2", Ceiling, "126", Greater);
    test("125", "2", Nearest, "124", Less);

    test("123", "123", Down, "123", Equal);
    test("123", "123", Floor, "123", Equal);
    test("123", "123", Up, "123", Equal);
    test("123", "123", Ceiling, "123", Equal);
    test("123", "123", Nearest, "123", Equal);
    test("123", "123", Exact, "123", Equal);

    test("123", "456", Down, "0", Less);
    test("123", "456", Floor, "0", Less);
    test("123", "456", Up, "456", Greater);
    test("123", "456", Ceiling, "456", Greater);
    test("123", "456", Nearest, "0", Less);

    test("1000000000000", "1", Down, "1000000000000", Equal);
    test("1000000000000", "1", Floor, "1000000000000", Equal);
    test("1000000000000", "1", Up, "1000000000000", Equal);
    test("1000000000000", "1", Ceiling, "1000000000000", Equal);
    test("1000000000000", "1", Nearest, "1000000000000", Equal);
    test("1000000000000", "1", Exact, "1000000000000", Equal);

    test("1000000000000", "3", Down, "999999999999", Less);
    test("1000000000000", "3", Floor, "999999999999", Less);
    test("1000000000000", "3", Up, "1000000000002", Greater);
    test("1000000000000", "3", Ceiling, "1000000000002", Greater);
    test("1000000000000", "3", Nearest, "999999999999", Less);

    test("999999999999", "2", Down, "999999999998", Less);
    test("999999999999", "2", Floor, "999999999998", Less);
    test("999999999999", "2", Up, "1000000000000", Greater);
    test("999999999999", "2", Ceiling, "1000000000000", Greater);
    test("999999999999", "2", Nearest, "1000000000000", Greater);

    test("1000000000001", "2", Down, "1000000000000", Less);
    test("1000000000001", "2", Floor, "1000000000000", Less);
    test("1000000000001", "2", Up, "1000000000002", Greater);
    test("1000000000001", "2", Ceiling, "1000000000002", Greater);
    test("1000000000001", "2", Nearest, "1000000000000", Less);

    test(
        "1000000000000000000000000",
        "4294967295",
        Down,
        "999999999999996832276305",
        Less,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        Floor,
        "999999999999996832276305",
        Less,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        Up,
        "1000000000000001127243600",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        Ceiling,
        "1000000000000001127243600",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        Nearest,
        "1000000000000001127243600",
        Greater,
    );

    test(
        "1000000000000000000000000",
        "1000000000000",
        Down,
        "1000000000000000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Floor,
        "1000000000000000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Up,
        "1000000000000000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Ceiling,
        "1000000000000000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Nearest,
        "1000000000000000000000000",
        Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        Exact,
        "1000000000000000000000000",
        Equal,
    );

    test(
        "1000000000000000000000000",
        "1000000000001",
        Down,
        "999999999999999999999999",
        Less,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        Floor,
        "999999999999999999999999",
        Less,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        Up,
        "1000000000001000000000000",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        Ceiling,
        "1000000000001000000000000",
        Greater,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        Nearest,
        "999999999999999999999999",
        Less,
    );

    test(
        "2999999999999999999999999",
        "2000000000000000000000000",
        Nearest,
        "2000000000000000000000000",
        Less,
    );
    test(
        "3000000000000000000000000",
        "2000000000000000000000000",
        Nearest,
        "4000000000000000000000000",
        Greater,
    );
    test(
        "3000000000000000000000001",
        "2000000000000000000000000",
        Nearest,
        "4000000000000000000000000",
        Greater,
    );

    test("0", "0", Floor, "0", Equal);
    test("0", "0", Ceiling, "0", Equal);
    test("0", "0", Down, "0", Equal);
    test("0", "0", Up, "0", Equal);
    test("0", "0", Nearest, "0", Equal);
    test("0", "0", Exact, "0", Equal);

    test("2", "0", Floor, "0", Less);
    test("2", "0", Down, "0", Less);
    test("2", "0", Nearest, "0", Less);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_1() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(Natural::from(3u32), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_2() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(Natural::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_3() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(Natural::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_4() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(Natural::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_1() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(&Natural::from(3u32), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_2() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(&Natural::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_3() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(&Natural::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_4() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(&Natural::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_1() {
    Natural::from(10u32).round_to_multiple(Natural::from(3u32), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_2() {
    Natural::from(10u32).round_to_multiple(Natural::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_3() {
    Natural::from(10u32).round_to_multiple(Natural::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_4() {
    Natural::from(10u32).round_to_multiple(Natural::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_1() {
    Natural::from(10u32).round_to_multiple(&Natural::from(3u32), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_2() {
    Natural::from(10u32).round_to_multiple(&Natural::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_3() {
    Natural::from(10u32).round_to_multiple(&Natural::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_4() {
    Natural::from(10u32).round_to_multiple(&Natural::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_1() {
    (&Natural::from(10u32)).round_to_multiple(Natural::from(3u32), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_2() {
    (&Natural::from(10u32)).round_to_multiple(Natural::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_3() {
    (&Natural::from(10u32)).round_to_multiple(Natural::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_4() {
    (&Natural::from(10u32)).round_to_multiple(Natural::ZERO, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_1() {
    (&Natural::from(10u32)).round_to_multiple(&Natural::from(3u32), Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_2() {
    (&Natural::from(10u32)).round_to_multiple(&Natural::ZERO, Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_3() {
    (&Natural::from(10u32)).round_to_multiple(&Natural::ZERO, Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_4() {
    (&Natural::from(10u32)).round_to_multiple(&Natural::ZERO, Exact);
}

#[test]
fn round_to_multiple_properties() {
    natural_natural_rounding_mode_triple_gen_var_2().test_properties(|(x, y, rm)| {
        let mut mut_x = x.clone();
        let o = mut_x.round_to_multiple_assign(&y, rm);
        assert!(mut_x.is_valid());
        let r = mut_x;

        let mut mut_x = x.clone();
        assert_eq!(mut_x.round_to_multiple_assign(y.clone(), rm), o);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, r);

        let (r_alt, o_alt) = (&x).round_to_multiple(&y, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let (r_alt, o_alt) = (&x).round_to_multiple(y.clone(), rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let (r_alt, o_alt) = x.clone().round_to_multiple(&y, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let (r_alt, o_alt) = x.clone().round_to_multiple(y.clone(), rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        assert!((&r).divisible_by(&y));
        assert_eq!(r.cmp(&x), o);
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
        if y == 0 {
            assert_eq!(r, 0);
        } else {
            assert!((Integer::from(&r) - Integer::from(&x)).abs() <= y);
            match rm {
                Floor | Down => assert!(r <= x),
                Ceiling | Up => assert!(r >= x),
                Exact => assert_eq!(r, x),
                Nearest => {
                    let closest;
                    let second_closest;
                    if r <= x {
                        closest = &x - &r;
                        second_closest = &r + &y - x;
                    } else {
                        closest = &r - &x;
                        second_closest = x + &y - &r;
                    }
                    assert!(closest <= second_closest);
                    if closest == second_closest {
                        assert!((&r).div_round(&y, Exact).0.even());
                    }
                }
            }
        }
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        let product = &x * &y;
        let po = (product.clone(), Equal);
        assert_eq!((&product).round_to_multiple(&y, Down), po);
        assert_eq!((&product).round_to_multiple(&y, Up), po);
        assert_eq!((&product).round_to_multiple(&y, Floor), po);
        assert_eq!((&product).round_to_multiple(&y, Ceiling), po);
        assert_eq!((&product).round_to_multiple(&y, Nearest), po);
        assert_eq!((&product).round_to_multiple(y, Exact), po);
    });

    natural_pair_gen_var_7().test_properties(|(x, y)| {
        let down = (&x).round_to_multiple(&y, Down);
        assert_eq!(down.1, Less);
        let up = (&down.0 + &y, Greater);
        assert_eq!((&x).round_to_multiple(&y, Up), up);
        assert_eq!((&x).round_to_multiple(&y, Floor), down);
        assert_eq!((&x).round_to_multiple(&y, Ceiling), up);
        let nearest = x.round_to_multiple(y, Nearest);
        assert!(nearest == down || nearest == up);
    });

    natural_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let xo = (x.clone(), Equal);
        assert_eq!((&x).round_to_multiple(Natural::ONE, rm), xo);
        assert_eq!(
            Natural::ZERO.round_to_multiple(&x, rm),
            (Natural::ZERO, Equal)
        );
        assert_eq!((&x).round_to_multiple(&x, rm), xo);
    });

    unsigned_unsigned_rounding_mode_triple_gen_var_2::<Limb>().test_properties(|(x, y, rm)| {
        let (n, no) = x.round_to_multiple(y, rm);
        let (i, io) = Natural::from(x).round_to_multiple(Natural::from(y), rm);
        assert_eq!(n, i);
        assert_eq!(no, io);
    });
}
