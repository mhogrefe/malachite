// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::Debug;
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul, ModSolve};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_solve::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

fn verify_solve<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> Option<UnsignedMatrix<T>> {
    let x = a.mod_solve(b, m);
    assert_eq!(a.clone().mod_solve(b.clone(), m), x);
    assert_eq!(a.clone().mod_solve(b, m), x);
    assert_eq!(a.mod_solve(b.clone(), m), x);
    assert_eq!(unsigned_matrix_mod_solve_naive(a, b, m), x);
    if let Some(x) = &x {
        assert!(x.is_valid());
        assert!(x.mod_is_reduced(&m));
        assert_eq!(x.height(), a.width());
        assert_eq!(x.width(), b.width());
        assert_eq!(a.mod_mul(x, m), *b);
    }
    // Each column of B, solved as a vector, gives the corresponding column of X.
    for j in 0..b.width() {
        let column = b.column(j);
        let y = a.mod_solve(&column, m);
        assert_eq!(a.clone().mod_solve(column.clone(), m), y);
        assert_eq!(a.clone().mod_solve(&column, m), y);
        assert_eq!(a.mod_solve(column.clone(), m), y);
        match (&x, y) {
            (Some(x), Some(y)) => assert_eq!(x.column(j), y),
            (Some(_), None) => panic!("a column has no solution, but the whole system does"),
            _ => {}
        }
    }
    x
}

fn test<T: PrimitiveUnsigned>(a: &str, b: &str, m: T, out: Option<&str>)
where
    <UnsignedMatrix<T> as FromStr>::Err: Debug,
{
    let a = UnsignedMatrix::<T>::from_str(a).unwrap();
    let b = UnsignedMatrix::<T>::from_str(b).unwrap();
    let x = verify_solve(&a, &b, m);
    assert_eq!(x.map(|x| x.to_string()).as_deref(), out);
}

#[test]
fn test_mod_solve() {
    test::<u64>("[]_{0,0}", "[]_{0,2}", 5, Some("[]_{0,2}"));
    test::<u64>("[]_{0,3}", "[]_{0,2}", 5, Some("[0, 0; 0, 0; 0, 0]"));
    test::<u64>("[]_{2,0}", "[0; 0]", 5, Some("[]_{0,1}"));
    test::<u64>("[]_{2,0}", "[0; 1]", 5, None);
    test::<u64>("[3]", "[1, 2]", 7, Some("[5, 3]"));
    test::<u64>("[3]", "[]_{1,0}", 7, Some("[]_{1,0}"));
    test::<u64>("[0]", "[1]", 7, None);
    test::<u64>("[0]", "[0]", 7, Some("[0]"));
    test::<u64>("[1, 2; 3, 4]", "[1; 1]", 7, Some("[6; 1]"));
    test::<u64>("[1, 2; 2, 4]", "[1; 2]", 7, Some("[1; 0]"));
    test::<u64>("[1, 2; 2, 4]", "[1; 1]", 7, None);
    test::<u64>(
        "[1, 2, 3; 4, 5, 6]",
        "[1, 0; 0, 1]",
        11,
        Some("[2, 8; 5, 7; 0, 0]"),
    );
    test::<u64>("[1, 2; 3, 4; 5, 6]", "[1; 2; 3]", 11, Some("[0; 6]"));
    test::<u64>("[1, 2; 3, 4; 5, 6]", "[1; 2; 4]", 11, None);
    test::<u64>("[0, 1, 2; 0, 2, 4]", "[3; 6]", 13, Some("[0; 3; 0]"));
    test::<u8>("[200, 100; 50, 26]", "[1; 2]", 251, Some("[92; 190]"));
    test::<u64>("[0, 0]", "[0]", 1, Some("[0; 0]"));
    test::<u64>(
        "[18446744073709551556, 5; 7, 18446744073709551000]",
        "[1; 2]",
        18446744073709551557,
        Some("[10495561283317503471; 16856507515631141940]"),
    );
}

// The next output of the SplitMix64 generator with state `seed`.
const fn splitmix64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

// A matrix with pseudorandom elements less than `m`, from the SplitMix64 generator.
fn pseudorandom_matrix(height: u64, width: u64, m: u64, seed: &mut u64) -> UnsignedMatrix<u64> {
    let mut elements = Vec::new();
    for _ in 0..height * width {
        let x = splitmix64(seed);
        elements.push(x % m);
    }
    UnsignedMatrix::from_owned_elements(height, width, elements)
}

#[test]
fn test_mod_solve_large() {
    // - square, nonsingular
    // - square, singular, falling back to the general method
    // - more rows than columns
    // - more columns than rows
    let mut seed = 0;
    for (h, w, r, k, m) in [
        (70, 70, 70, 3, 1099511627791),
        (70, 70, 40, 70, 2),
        (130, 70, 50, 5, 18446744073709551557),
        (60, 100, 60, 64, 2305843009213693951),
    ] {
        let a = pseudorandom_matrix(h, r, m, &mut seed)
            .mod_mul(pseudorandom_matrix(r, w, m, &mut seed), m);
        // A solvable system, and one that is almost certainly not solvable unless A has full row
        // rank.
        let b = (&a).mod_mul(pseudorandom_matrix(w, k, m, &mut seed), m);
        assert!(verify_solve(&a, &b, m).is_some());
        let b = pseudorandom_matrix(h, k, m, &mut seed);
        verify_solve(&a, &b, m);
    }
}

#[test]
#[should_panic]
fn mod_solve_fail_1() {
    let a = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve(b, 0);
}

#[test]
#[should_panic]
fn mod_solve_fail_2() {
    let a = UnsignedMatrix::<u64>::from_str("[1; 2]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve(b, 5);
}

#[test]
#[should_panic]
fn mod_solve_fail_3() {
    let a = UnsignedMatrix::<u64>::from_str("[5]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve(b, 5);
}

#[test]
#[should_panic]
fn mod_solve_fail_4() {
    let a = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[5]").unwrap();
    a.mod_solve(b, 5);
}

#[test]
#[should_panic]
fn mod_solve_fail_5() {
    let a = UnsignedMatrix::<u64>::from_str("[2, 1; 1, 1]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1; 1]").unwrap();
    a.mod_solve(b, 4);
}

#[test]
fn mod_solve_properties() {
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_8().test_properties(|(a, b, m)| {
        verify_solve(&a, &b, m);
    });

    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_9().test_properties(|(a, b, m)| {
        assert!(verify_solve(&a, &b, m).is_some());
    });
}
