// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::Mod;
use malachite_base::num::number_theory::traits::Gcd;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::factorization::ecpp::point::{EcppPoint, naf_digits, point_mul};
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::factorization::ecpp::{
    ecpp_fixture_steps, ecpp_point_mul_naive, jacobian_to_affine, naf_digits_naive,
};
use std::str::FromStr;

#[test]
fn test_point_mul() {
    // Coordinates of the point at infinity (Z = 0) are not specified, so only Z is compared then.
    #[allow(clippy::too_many_arguments)]
    let test = |x, y, z, k, a, acc, n, out: (&str, &str, &str), ok: bool, acc_out| {
        let p = EcppPoint {
            x: Natural::from_str(x).unwrap(),
            y: Natural::from_str(y).unwrap(),
            z: Natural::from_str(z).unwrap(),
        };
        let mut acc = Natural::from_str(acc).unwrap();
        let (r, r_ok) = point_mul(
            &p,
            &Natural::from_str(k).unwrap(),
            &Natural::from_str(a).unwrap(),
            &mut acc,
            &Natural::from_str(n).unwrap(),
        );
        assert_eq!(r.z.to_string(), out.2);
        if r.z != 0u32 {
            assert_eq!(r.x.to_string(), out.0);
            assert_eq!(r.y.to_string(), out.1);
        }
        assert_eq!(r_ok, ok);
        assert_eq!(acc.to_string(), acc_out);
    };
    // - general case: NAF digits of both signs, P + Q = 2P, P + Q = O
    test(
        "3",
        "15",
        "1",
        "521621592227",
        "8",
        "1",
        "31",
        ("22", "1", "11"),
        true,
        "30",
    );
    // - k = 0 in point_mul_reduced
    test(
        "3",
        "15",
        "1",
        "0",
        "8",
        "1",
        "31",
        ("0", "0", "0"),
        true,
        "1",
    );
    // - P = O in point_mul_reduced
    test(
        "3",
        "15",
        "0",
        "5",
        "8",
        "1",
        "31",
        ("0", "0", "0"),
        true,
        "1",
    );
    // - k = 1: binary loop does nothing
    test(
        "3",
        "15",
        "1",
        "1",
        "8",
        "1",
        "31",
        ("3", "15", "1"),
        true,
        "1",
    );
    // - k < 2^31: binary loop with bits 0 and 1, P + Q = O
    test(
        "14636",
        "13693",
        "1",
        "837108039",
        "464",
        "1",
        "14711",
        ("3244", "11336", "6608"),
        true,
        "1508",
    );
    // - n composite: acc becomes 0 in the binary loop, for k < 2^31
    test(
        "1768",
        "3119",
        "1",
        "533491832",
        "1296",
        "1",
        "3263",
        ("0", "0", "0"),
        false,
        "0",
    );
    // - n prime, P of order 2: 2P = O, so the first normalisation fails and the binary method runs
    test(
        "0",
        "0",
        "1",
        "1099511640121",
        "5",
        "1",
        "1009",
        ("0", "0", "1"),
        true,
        "1",
    );
    // - n prime, P of order 3: 3P = O, so the second normalisation fails
    test(
        "42",
        "357",
        "1",
        "1099511640121",
        "994",
        "1",
        "1009",
        ("42", "357", "1"),
        true,
        "834",
    );
    // - n prime, P of order 5
    test(
        "400",
        "333",
        "1",
        "1099511640121",
        "670",
        "1",
        "1009",
        ("400", "333", "1"),
        true,
        "812",
    );
    // - n prime, P of order 7
    test(
        "543",
        "43",
        "1",
        "1099511640121",
        "802",
        "1",
        "1009",
        ("451", "450", "481"),
        true,
        "521",
    );
    // - n composite: a Z is not invertible, so a normalisation fails
    test(
        "11637",
        "34648",
        "1",
        "985698858246",
        "490",
        "1",
        "41003",
        ("23162", "7825", "31613"),
        true,
        "1565",
    );
    // - k = 0 with n composite
    test(
        "5969",
        "1",
        "1",
        "0",
        "1",
        "1",
        "5969",
        ("0", "0", "0"),
        true,
        "1",
    );
    // - n composite: acc is 0 before the fallback binary method
    test(
        "2991",
        "1834",
        "1",
        "208584648688",
        "2233",
        "1",
        "5969",
        ("0", "0", "0"),
        false,
        "0",
    );
    // - n composite: acc becomes 0 in the fallback binary method
    test(
        "27391",
        "30180",
        "1",
        "775513838373",
        "50097",
        "1",
        "52001",
        ("0", "0", "0"),
        false,
        "0",
    );
    // - n composite: acc becomes 0 in the NAF loop
    test(
        "27386",
        "74782",
        "1",
        "229279340339",
        "15964",
        "1",
        "82319",
        ("0", "0", "0"),
        false,
        "0",
    );
    // - n composite: the first normalisation fails on a Z that is not invertible
    test(
        "452",
        "735",
        "1",
        "340806568723",
        "126",
        "1",
        "965",
        ("815", "920", "20"),
        true,
        "945",
    );
}

#[test]
fn point_mul_properties() {
    // On the curves of the fixture steps, whose n are prime, k P agrees with affine double-and-add.
    let steps = ecpp_fixture_steps();
    let len = Natural::from(steps.len());
    natural_gen().test_properties(|k| {
        let s = &steps[usize::exact_from(&(&k).mod_op(&len))];
        let p = EcppPoint::from_affine(s.x.clone(), s.y.clone());
        let mut acc = Natural::ONE;
        let (r, ok) = point_mul(&p, &k, &s.a, &mut acc, &s.n);
        assert!(ok);
        assert_eq!(acc.gcd(&s.n), 1u32);
        assert_eq!(
            jacobian_to_affine(&r, &s.n),
            ecpp_point_mul_naive(&s.x, &s.y, &k, &s.a, &s.n)
        );
    });
}

#[test]
fn test_naf_digits() {
    let test = |k, out| {
        assert_eq!(
            format!("{:?}", naf_digits(&Natural::from_str(k).unwrap())),
            out
        );
    };
    test("0", "[]");
    test("1", "[1]");
    test("7", "[7]");
    test("8", "[0, 0, 0, 1]");
    // - a negative digit carries into the next window
    test("15", "[-1, 0, 0, 0, 1]");
    test("16", "[0, 0, 0, 0, 1]");
    // - the carry runs through a block of ones
    test("255", "[-1, 0, 0, 0, 0, 0, 0, 0, 1]");
    test("1000", "[0, 0, 0, -3, 0, 0, 0, 0, 0, 0, 1]");
}

#[test]
fn naf_digits_properties() {
    natural_gen().test_properties(|k| {
        let naf = naf_digits(&k);
        assert_eq!(naf, naf_digits_naive(&k));
        // the digits evaluate to k
        let mut sum = Integer::ZERO;
        for &d in naf.iter().rev() {
            sum <<= 1u32;
            sum += Integer::from(d);
        }
        assert_eq!(sum, k);
        // nonzero digits are odd, in (-8, 8), and followed by at least three zeros
        for (i, &d) in naf.iter().enumerate() {
            if d != 0 {
                assert!(d.odd() && d > -8 && d < 8);
                assert!(naf[i + 1..].iter().take(3).all(|&e| e == 0));
            }
        }
        assert_ne!(naf.last(), Some(&0));
    });
}
