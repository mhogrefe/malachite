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
use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModMul, ModSolveLowerTriangular, ModSolveUpperTriangular,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_solve_lower_triangular::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// The lower triangular part of `a`, with ones on the diagonal if `unit_diagonal`.
fn triangular_part<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    unit_diagonal: bool,
) -> UnsignedMatrix<T> {
    let n = a.height();
    let mut elements = Vec::new();
    for i in 0..n {
        for j in 0..n {
            elements.push(if i == j && unit_diagonal {
                T::ONE
            } else if j <= i {
                *a.get(i, j).unwrap()
            } else {
                T::ZERO
            });
        }
    }
    UnsignedMatrix::from_owned_elements(n, n, elements)
}

// `a` with the order of its rows and the order of its columns reversed.
fn reversed<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>) -> UnsignedMatrix<T> {
    let (h, w) = (a.height(), a.width());
    let mut elements = Vec::new();
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            elements.push(*a.get(i, j).unwrap());
        }
    }
    UnsignedMatrix::from_owned_elements(h, w, elements)
}

fn test_helper<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
    unit_diagonal: bool,
) -> UnsignedMatrix<T> {
    let x = a.mod_solve_lower_triangular(b, m, unit_diagonal);
    assert!(x.is_valid());
    assert_eq!(
        a.clone()
            .mod_solve_lower_triangular(b.clone(), m, unit_diagonal),
        x
    );
    assert_eq!(a.clone().mod_solve_lower_triangular(b, m, unit_diagonal), x);
    assert_eq!(a.mod_solve_lower_triangular(b.clone(), m, unit_diagonal), x);
    assert_eq!(
        unsigned_matrix_mod_solve_lower_triangular_naive(a, b, m, unit_diagonal),
        x
    );
    x
}

fn test<T: PrimitiveUnsigned>(a: &str, b: &str, m: T, unit_diagonal: bool, out: &str)
where
    <UnsignedMatrix<T> as FromStr>::Err: Debug,
{
    let a = UnsignedMatrix::<T>::from_str(a).unwrap();
    let b = UnsignedMatrix::<T>::from_str(b).unwrap();
    assert_eq!(test_helper(&a, &b, m, unit_diagonal).to_string(), out);
}

#[test]
fn test_mod_solve_lower_triangular() {
    test::<u64>("[]_{0,0}", "[]_{0,3}", 7, false, "[]_{0,3}");
    test::<u64>("[3]", "[1, 2]", 7, false, "[5, 3]");
    test::<u64>("[3]", "[1, 2]", 7, true, "[1, 2]");
    test::<u64>("[2, 9; 3, 4]", "[1; 2]", 11, false, "[6; 7]");
    test::<u64>("[2, 9; 3, 4]", "[1; 2]", 11, true, "[1; 10]");
    test::<u64>(
        "[1, 3, 2; 2, 1, 4; 3, 4, 1]",
        "[1, 2; 3, 4; 0, 1]",
        5,
        true,
        "[1, 2; 1, 0; 3, 0]",
    );
    test::<u64>(
        "[2, 4, 4; 2, 3, 4; 3, 4, 4]",
        "[1, 2; 3, 4; 0, 1]",
        5,
        false,
        "[3, 1; 4, 4; 0, 3]",
    );
    test::<u64>("[0]", "[0]", 1, false, "[0]");
    test::<u8>(
        "[200, 0; 100, 7]",
        "[1, 2; 3, 4]",
        251,
        false,
        "[187, 123; 90, 108]",
    );
    test::<u64>(
        "[18446744073709551556, 1; 5, 18446744073709551550]",
        "[1; 18446744073709551000]",
        18446744073709551557,
        false,
        "[18446744073709551556; 15811494920322472842]",
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

// A matrix with pseudorandom elements less than `m`, from the SplitMix64 generator, with nonzero
// diagonal elements if it is square.
fn pseudorandom_matrix(height: u64, width: u64, m: u64, seed: &mut u64) -> UnsignedMatrix<u64> {
    let mut elements = Vec::new();
    for i in 0..height {
        for j in 0..width {
            let x = splitmix64(seed);
            let mut x = x % m;
            if i == j && height == width && x == 0 {
                x = 1;
            }
            elements.push(x);
        }
    }
    UnsignedMatrix::from_owned_elements(height, width, elements)
}

#[test]
fn test_mod_solve_lower_triangular_large() {
    // - recursive, with classical products
    // - recursive, with Strassen products
    let mut seed = 0;
    for (n, k, m) in [(70, 65, (1 << 61) - 1), (64, 64, 2), (400, 200, (1 << 61) - 1)] {
        let a = pseudorandom_matrix(n, n, m, &mut seed);
        let b = pseudorandom_matrix(n, k, m, &mut seed);
        for unit_diagonal in [false, true] {
            let x = test_helper(&a, &b, m, unit_diagonal);
            assert_eq!(triangular_part(&a, unit_diagonal).mod_mul(&x, m), b);
        }
    }
}

#[test]
#[should_panic]
fn mod_solve_lower_triangular_fail_1() {
    let a = UnsignedMatrix::<u64>::from_str("[1, 2]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve_lower_triangular(b, 5, false);
}

#[test]
#[should_panic]
fn mod_solve_lower_triangular_fail_2() {
    let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve_lower_triangular(b, 5, false);
}

#[test]
#[should_panic]
fn mod_solve_lower_triangular_fail_3() {
    let a = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve_lower_triangular(b, 0, false);
}

#[test]
#[should_panic]
fn mod_solve_lower_triangular_fail_4() {
    let a = UnsignedMatrix::<u64>::from_str("[5]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve_lower_triangular(b, 5, false);
}

#[test]
#[should_panic]
fn mod_solve_lower_triangular_fail_5() {
    let a = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[5]").unwrap();
    a.mod_solve_lower_triangular(b, 5, false);
}

#[test]
#[should_panic]
fn mod_solve_lower_triangular_fail_6() {
    let a = UnsignedMatrix::<u64>::from_str("[0]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve_lower_triangular(b, 5, false);
}

#[test]
#[should_panic]
fn mod_solve_lower_triangular_fail_7() {
    let a = UnsignedMatrix::<u64>::from_str("[2]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_solve_lower_triangular(b, 4, false);
}

#[test]
fn mod_solve_lower_triangular_properties() {
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_7().test_properties(|(a, b, m)| {
        for unit_diagonal in [false, true] {
            let x = test_helper(&a, &b, m, unit_diagonal);
            assert!(x.mod_is_reduced(&m));
            assert_eq!(x.height(), b.height());
            assert_eq!(x.width(), b.width());
            assert_eq!(triangular_part(&a, unit_diagonal).mod_mul(&x, m), b);
            assert_eq!(
                triangular_part(&a, unit_diagonal).mod_solve_lower_triangular(&b, m, false),
                x
            );
            assert_eq!(
                reversed(&a).mod_solve_upper_triangular(reversed(&b), m, unit_diagonal),
                reversed(&x)
            );
        }
    });
}
