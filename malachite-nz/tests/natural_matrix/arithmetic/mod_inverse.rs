// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    CoprimeWith, ModDeterminant, ModInverse, ModMul, ModSolve,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_6;

fn verify_inverse(a: &NaturalMatrix, m: &Natural) -> Option<NaturalMatrix> {
    let x = a.mod_inverse(m);
    assert_eq!(a.clone().mod_inverse(m.clone()), x);
    assert_eq!(a.clone().mod_inverse(m), x);
    assert_eq!(a.mod_inverse(m.clone()), x);
    let n = a.height();
    if *m != 1u32 {
        // A matrix is invertible exactly when its determinant is a unit.
        assert_eq!(x.is_some(), a.mod_determinant(m).coprime_with(m));
    }
    if let Some(x) = &x {
        assert!(x.is_valid());
        let identity = if *m == 1u32 {
            a.clone()
        } else {
            NaturalMatrix::identity(n)
        };
        assert_eq!(a.mod_mul(x, m), identity);
        assert_eq!(x.mod_mul(a, m), identity);
        if *m != 1u32 {
            assert_eq!(x.mod_inverse(m).as_ref(), Some(a));
            assert_eq!(a.mod_solve(NaturalMatrix::identity(n), m).as_ref(), Some(x));
        }
    }
    x
}

fn test(a: &str, m: &str, out: Option<&str>) {
    let a = NaturalMatrix::from_str(a).unwrap();
    let m = Natural::from_str(m).unwrap();
    assert_eq!(
        verify_inverse(&a, &m).map(|x| x.to_string()).as_deref(),
        out
    );
}

#[test]
fn test_mod_inverse() {
    test(
        "[]_{0,0}",
        "170141183460469231731687303715884105727",
        Some("[]_{0,0}"),
    );
    test(
        "[3]",
        "170141183460469231731687303715884105727",
        Some("[113427455640312821154458202477256070485]"),
    );
    test("[0]", "170141183460469231731687303715884105727", None);
    test(
        "[1, 2; 3, 4]",
        "170141183460469231731687303715884105727",
        Some(
            "[170141183460469231731687303715884105725, 1; 85070591730234615865843651857942052865, \
            85070591730234615865843651857942052863]",
        ),
    );
    test("[1, 2; 2, 4]", "36893488147419103363", None);
    test(
        "[2, 0, 1; 1, 1, 0; 0, 3, 1]",
        "170141183460469231731687303715884105727",
        Some(
            "[68056473384187692692674921486353642291, 34028236692093846346337460743176821146, \
            102084710076281539039012382229530463436; 102084710076281539039012382229530463436, \
            136112946768375385385349842972707284582, 68056473384187692692674921486353642291; \
            34028236692093846346337460743176821146, 102084710076281539039012382229530463435, \
            136112946768375385385349842972707284582]",
        ),
    );
    test(
        "[170141183460469231731687303715884105726, 5; 7, 170141183460469231731687303715884105725]",
        "170141183460469231731687303715884105727",
        Some(
            "[36090554067372261276418518970036022427, 5155793438196037325202645567148003204; \
            41246347505568298601621164537184025631, 103115868763920746504052911342960064077]",
        ),
    );
    test("[1, 2; 3, 4]", "7", Some("[5, 1; 5, 3]"));
    test("[0, 0; 0, 0]", "1", Some("[0, 0; 0, 0]"));
    // - a 1 by 1 matrix whose element is not a unit, which is not invertible even though m is
    //   composite
    test("[2]", "1267650600228229401496703205376", None);
}

// The next output of the SplitMix64 generator with state `seed`.
const fn splitmix64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

// A pseudorandom element less than `m`, made of several outputs of the SplitMix64 generator.
fn pseudorandom_natural(m: &Natural, seed: &mut u64) -> Natural {
    (0..3).fold(Natural::ZERO, |x, _| {
        (x << 64u32) + Natural::from(splitmix64(seed))
    }) % m
}

// A matrix with pseudorandom elements less than `m`.
fn pseudorandom_matrix(height: u64, width: u64, m: &Natural, seed: &mut u64) -> NaturalMatrix {
    let elements = (0..height * width)
        .map(|_| pseudorandom_natural(m, seed))
        .collect();
    NaturalMatrix::from_owned_elements(height, width, elements)
}

#[test]
fn test_mod_inverse_large() {
    let mut seed = 0;
    for (n, m) in [
        (20, "170141183460469231731687303715884105727"),
        (
            9,
            "1606938044258990275541962092341162602522202993782792835301611",
        ),
    ] {
        let m = Natural::from_str(m).unwrap();
        let a = pseudorandom_matrix(n, n, &m, &mut seed);
        verify_inverse(&a, &m);
    }
}

#[test]
#[should_panic]
fn mod_inverse_fail_1() {
    NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_inverse(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_inverse_fail_2() {
    NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .mod_inverse(Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_inverse_fail_3() {
    NaturalMatrix::from_str("[5]")
        .unwrap()
        .mod_inverse(Natural::from(5u32));
}

#[test]
fn mod_inverse_properties() {
    natural_matrix_natural_pair_gen_var_6().test_properties(|(a, m)| {
        verify_inverse(&a, &m);
    });
}
