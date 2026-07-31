// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Average, AverageAssign, AverageRound, AverageRoundAssign, DivRound, Parity,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_natural_rounding_mode_triple_gen_var_3, natural_pair_gen,
};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_average() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u.clone().average(v.clone()).to_string(), out);
        assert_eq!(u.clone().average(&v).to_string(), out);
        assert_eq!((&u).average(v.clone()).to_string(), out);
        assert_eq!((&u).average(&v).to_string(), out);

        let mut mut_u = u.clone();
        mut_u.average_assign(v.clone());
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u;
        mut_u.average_assign(&v);
        assert_eq!(mut_u.to_string(), out);
    };
    test("0", "0", "0");
    test("4", "6", "5");
    // 4.5 rounds to the even neighbor, 4
    test("4", "5", "4");
    // 5.5 rounds to the even neighbor, 6
    test("5", "6", "6");
    test("123", "456", "290");
    test("1000000000000", "1000000000002", "1000000000001");
    test("1000000000000000000000000", "2", "500000000000000000000001");
}

#[test]
fn test_average_round() {
    let test = |s, t, rm, out, o: Ordering| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let (avg, actual_o) = u.clone().average_round(v.clone(), rm);
        assert_eq!(avg.to_string(), out);
        assert_eq!(actual_o, o);
        let (avg, actual_o) = u.clone().average_round(&v, rm);
        assert_eq!(avg.to_string(), out);
        assert_eq!(actual_o, o);
        let (avg, actual_o) = (&u).average_round(v.clone(), rm);
        assert_eq!(avg.to_string(), out);
        assert_eq!(actual_o, o);
        let (avg, actual_o) = (&u).average_round(&v, rm);
        assert_eq!(avg.to_string(), out);
        assert_eq!(actual_o, o);

        let mut mut_u = u.clone();
        assert_eq!(mut_u.average_round_assign(v.clone(), rm), o);
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u;
        assert_eq!(mut_u.average_round_assign(&v, rm), o);
        assert_eq!(mut_u.to_string(), out);
    };
    // - exact averages are unaffected by the rounding mode
    test("4", "6", Floor, "5", Equal);
    test("4", "6", Exact, "5", Equal);
    // - the exact average is 5.5
    test("4", "7", Floor, "5", Less);
    test("4", "7", Ceiling, "6", Greater);
    test("4", "7", Down, "5", Less);
    test("4", "7", Up, "6", Greater);
    test("4", "7", Nearest, "6", Greater);
    test("4", "9", Nearest, "6", Less);
    test(
        "1000000000000",
        "1000000000001",
        Floor,
        "1000000000000",
        Less,
    );
    test(
        "1000000000000",
        "1000000000001",
        Ceiling,
        "1000000000001",
        Greater,
    );
}

#[test]
#[should_panic]
fn average_round_exact_fail() {
    Natural::from_str("4")
        .unwrap()
        .average_round(Natural::from_str("7").unwrap(), Exact);
}

#[test]
fn average_properties() {
    natural_natural_rounding_mode_triple_gen_var_3().test_properties(|(x, y, rm)| {
        let (avg, o) = (&x).average_round(&y, rm);
        assert_eq!((&y).average_round(&x, rm), (avg.clone(), o));
        assert_eq!(x.clone().average_round(y.clone(), rm), (avg.clone(), o));
        assert_eq!(x.clone().average_round(&y, rm), (avg.clone(), o));
        assert_eq!((&x).average_round(y.clone(), rm), (avg.clone(), o));
        let mut mut_x = x.clone();
        assert_eq!(mut_x.average_round_assign(y.clone(), rm), o);
        assert_eq!(mut_x, avg);
        let mut mut_x = x.clone();
        assert_eq!(mut_x.average_round_assign(&y, rm), o);
        assert_eq!(mut_x, avg);

        // an independent computation: divide the sum by 2
        assert_eq!(
            (&x + &y).div_round(Natural::from(2u32), rm),
            (avg.clone(), o)
        );

        assert!(avg >= core::cmp::min(&x, &y).clone());
        assert!(avg <= core::cmp::max(&x, &y).clone());
        let (floor, floor_o) = (&x).average_round(&y, Floor);
        let (ceiling, ceiling_o) = (&x).average_round(&y, Ceiling);
        assert_eq!(&floor + &ceiling, &x + &y);
        if (&x + &y).even() {
            assert_eq!(o, Equal);
            assert_eq!(floor, ceiling);
            assert_eq!(floor_o, Equal);
        } else {
            assert_eq!(ceiling, &floor + Natural::from(1u32));
            assert_eq!(floor_o, Less);
            assert_eq!(ceiling_o, Greater);
            match o {
                Less => assert_eq!(avg, floor),
                Greater => assert_eq!(avg, ceiling),
                Equal => panic!("inexact average reported as exact"),
            }
        }

        // agreement with the primitive implementation on small values
        if let (Ok(sx), Ok(sy)) = (u64::try_from(&x), u64::try_from(&y)) {
            let (savg, so) = sx.average_round(sy, rm);
            assert_eq!(avg, Natural::exact_from(savg));
            assert_eq!(o, so);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        let avg = (&x).average(&y);
        assert_eq!((&y).average(&x), avg);
        assert_eq!(x.clone().average(y.clone()), avg);
        assert_eq!(x.clone().average(&y), avg);
        assert_eq!((&x).average(y.clone()), avg);
        let mut mut_x = x.clone();
        mut_x.average_assign(y.clone());
        assert_eq!(mut_x, avg);
        let mut mut_x = x.clone();
        mut_x.average_assign(&y);
        assert_eq!(mut_x, avg);

        assert_eq!((&x).average_round(&y, Nearest).0, avg);
        // a two-way tie rounds to the even neighbor
        if (&x + &y).odd() {
            assert!(avg.even());
        }
        assert_eq!((&x).average(&x), x);
    });
}
