// Copyright © 2024 Mikhail Hogrefe
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
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::unsigned_unsigned_rounding_mode_triple_gen_var_2;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_rounding_mode_triple_gen_var_2, natural_pair_gen_var_5, natural_pair_gen_var_7,
    natural_rounding_mode_pair_gen,
};
use std::cmp::Ordering;
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
    test("0", "1", RoundingMode::Down, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Floor, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Up, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", "1", RoundingMode::Exact, "0", Ordering::Equal);

    test("0", "123", RoundingMode::Down, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Floor, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Up, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", "123", RoundingMode::Exact, "0", Ordering::Equal);

    test("1", "1", RoundingMode::Down, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Floor, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Up, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Ceiling, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Nearest, "1", Ordering::Equal);
    test("1", "1", RoundingMode::Exact, "1", Ordering::Equal);

    test("123", "1", RoundingMode::Down, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Floor, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Up, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Ceiling, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Nearest, "123", Ordering::Equal);
    test("123", "1", RoundingMode::Exact, "123", Ordering::Equal);

    test("123", "2", RoundingMode::Down, "122", Ordering::Less);
    test("123", "2", RoundingMode::Floor, "122", Ordering::Less);
    test("123", "2", RoundingMode::Up, "124", Ordering::Greater);
    test("123", "2", RoundingMode::Ceiling, "124", Ordering::Greater);
    test("123", "2", RoundingMode::Nearest, "124", Ordering::Greater);

    test("125", "2", RoundingMode::Down, "124", Ordering::Less);
    test("125", "2", RoundingMode::Floor, "124", Ordering::Less);
    test("125", "2", RoundingMode::Up, "126", Ordering::Greater);
    test("125", "2", RoundingMode::Ceiling, "126", Ordering::Greater);
    test("125", "2", RoundingMode::Nearest, "124", Ordering::Less);

    test("123", "123", RoundingMode::Down, "123", Ordering::Equal);
    test("123", "123", RoundingMode::Floor, "123", Ordering::Equal);
    test("123", "123", RoundingMode::Up, "123", Ordering::Equal);
    test("123", "123", RoundingMode::Ceiling, "123", Ordering::Equal);
    test("123", "123", RoundingMode::Nearest, "123", Ordering::Equal);
    test("123", "123", RoundingMode::Exact, "123", Ordering::Equal);

    test("123", "456", RoundingMode::Down, "0", Ordering::Less);
    test("123", "456", RoundingMode::Floor, "0", Ordering::Less);
    test("123", "456", RoundingMode::Up, "456", Ordering::Greater);
    test(
        "123",
        "456",
        RoundingMode::Ceiling,
        "456",
        Ordering::Greater,
    );
    test("123", "456", RoundingMode::Nearest, "0", Ordering::Less);

    test(
        "1000000000000",
        "1",
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        "1",
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "1000000000000",
        "3",
        RoundingMode::Down,
        "999999999999",
        Ordering::Less,
    );
    test(
        "1000000000000",
        "3",
        RoundingMode::Floor,
        "999999999999",
        Ordering::Less,
    );
    test(
        "1000000000000",
        "3",
        RoundingMode::Up,
        "1000000000002",
        Ordering::Greater,
    );
    test(
        "1000000000000",
        "3",
        RoundingMode::Ceiling,
        "1000000000002",
        Ordering::Greater,
    );
    test(
        "1000000000000",
        "3",
        RoundingMode::Nearest,
        "999999999999",
        Ordering::Less,
    );

    test(
        "999999999999",
        "2",
        RoundingMode::Down,
        "999999999998",
        Ordering::Less,
    );
    test(
        "999999999999",
        "2",
        RoundingMode::Floor,
        "999999999998",
        Ordering::Less,
    );
    test(
        "999999999999",
        "2",
        RoundingMode::Up,
        "1000000000000",
        Ordering::Greater,
    );
    test(
        "999999999999",
        "2",
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Greater,
    );
    test(
        "999999999999",
        "2",
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Greater,
    );

    test(
        "1000000000001",
        "2",
        RoundingMode::Down,
        "1000000000000",
        Ordering::Less,
    );
    test(
        "1000000000001",
        "2",
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Less,
    );
    test(
        "1000000000001",
        "2",
        RoundingMode::Up,
        "1000000000002",
        Ordering::Greater,
    );
    test(
        "1000000000001",
        "2",
        RoundingMode::Ceiling,
        "1000000000002",
        Ordering::Greater,
    );
    test(
        "1000000000001",
        "2",
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Less,
    );

    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Down,
        "999999999999996832276305",
        Ordering::Less,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Floor,
        "999999999999996832276305",
        Ordering::Less,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Up,
        "1000000000000001127243600",
        Ordering::Greater,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Ceiling,
        "1000000000000001127243600",
        Ordering::Greater,
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Nearest,
        "1000000000000001127243600",
        Ordering::Greater,
    );

    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Down,
        "1000000000000000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Floor,
        "1000000000000000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Up,
        "1000000000000000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Ceiling,
        "1000000000000000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Nearest,
        "1000000000000000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Exact,
        "1000000000000000000000000",
        Ordering::Equal,
    );

    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Down,
        "999999999999999999999999",
        Ordering::Less,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Floor,
        "999999999999999999999999",
        Ordering::Less,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Up,
        "1000000000001000000000000",
        Ordering::Greater,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Ceiling,
        "1000000000001000000000000",
        Ordering::Greater,
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Nearest,
        "999999999999999999999999",
        Ordering::Less,
    );

    test(
        "2999999999999999999999999",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "2000000000000000000000000",
        Ordering::Less,
    );
    test(
        "3000000000000000000000000",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "4000000000000000000000000",
        Ordering::Greater,
    );
    test(
        "3000000000000000000000001",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "4000000000000000000000000",
        Ordering::Greater,
    );

    test("0", "0", RoundingMode::Floor, "0", Ordering::Equal);
    test("0", "0", RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", "0", RoundingMode::Down, "0", Ordering::Equal);
    test("0", "0", RoundingMode::Up, "0", Ordering::Equal);
    test("0", "0", RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", "0", RoundingMode::Exact, "0", Ordering::Equal);

    test("2", "0", RoundingMode::Floor, "0", Ordering::Less);
    test("2", "0", RoundingMode::Down, "0", Ordering::Less);
    test("2", "0", RoundingMode::Nearest, "0", Ordering::Less);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_1() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_2() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(Natural::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_3() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(Natural::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_4() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(Natural::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_1() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(&Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_2() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(&Natural::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_3() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(&Natural::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_4() {
    let mut n = Natural::from(10u32);
    n.round_to_multiple_assign(&Natural::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_1() {
    Natural::from(10u32).round_to_multiple(Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_2() {
    Natural::from(10u32).round_to_multiple(Natural::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_3() {
    Natural::from(10u32).round_to_multiple(Natural::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_4() {
    Natural::from(10u32).round_to_multiple(Natural::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_1() {
    Natural::from(10u32).round_to_multiple(&Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_2() {
    Natural::from(10u32).round_to_multiple(&Natural::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_3() {
    Natural::from(10u32).round_to_multiple(&Natural::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_4() {
    Natural::from(10u32).round_to_multiple(&Natural::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_1() {
    (&Natural::from(10u32)).round_to_multiple(Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_2() {
    (&Natural::from(10u32)).round_to_multiple(Natural::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_3() {
    (&Natural::from(10u32)).round_to_multiple(Natural::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_4() {
    (&Natural::from(10u32)).round_to_multiple(Natural::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_1() {
    (&Natural::from(10u32)).round_to_multiple(&Natural::from(3u32), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_2() {
    (&Natural::from(10u32)).round_to_multiple(&Natural::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_3() {
    (&Natural::from(10u32)).round_to_multiple(&Natural::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_4() {
    (&Natural::from(10u32)).round_to_multiple(&Natural::ZERO, RoundingMode::Exact);
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
            RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
            RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
        if y == 0 {
            assert_eq!(r, 0);
        } else {
            assert!((Integer::from(&r) - Integer::from(&x)).abs() <= y);
            match rm {
                RoundingMode::Floor | RoundingMode::Down => assert!(r <= x),
                RoundingMode::Ceiling | RoundingMode::Up => assert!(r >= x),
                RoundingMode::Exact => assert_eq!(r, x),
                RoundingMode::Nearest => {
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
                        assert!((&r).div_round(&y, RoundingMode::Exact).0.even());
                    }
                }
            }
        }
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        let product = &x * &y;
        let po = (product.clone(), Ordering::Equal);
        assert_eq!((&product).round_to_multiple(&y, RoundingMode::Down), po);
        assert_eq!((&product).round_to_multiple(&y, RoundingMode::Up), po);
        assert_eq!((&product).round_to_multiple(&y, RoundingMode::Floor), po);
        assert_eq!((&product).round_to_multiple(&y, RoundingMode::Ceiling), po);
        assert_eq!((&product).round_to_multiple(&y, RoundingMode::Nearest), po);
        assert_eq!((&product).round_to_multiple(y, RoundingMode::Exact), po);
    });

    natural_pair_gen_var_7().test_properties(|(x, y)| {
        let down = (&x).round_to_multiple(&y, RoundingMode::Down);
        assert_eq!(down.1, Ordering::Less);
        let up = (&down.0 + &y, Ordering::Greater);
        assert_eq!((&x).round_to_multiple(&y, RoundingMode::Up), up);
        assert_eq!((&x).round_to_multiple(&y, RoundingMode::Floor), down);
        assert_eq!((&x).round_to_multiple(&y, RoundingMode::Ceiling), up);
        let nearest = x.round_to_multiple(y, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    natural_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let xo = (x.clone(), Ordering::Equal);
        assert_eq!((&x).round_to_multiple(Natural::ONE, rm), xo);
        assert_eq!(
            Natural::ZERO.round_to_multiple(&x, rm),
            (Natural::ZERO, Ordering::Equal)
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
