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
use malachite_base::num::arithmetic::traits::{ModDeterminant, ModInverse, ModMul, ModSolve};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_12;
use malachite_base::unsigned_matrix::UnsignedMatrix;

fn test<T: PrimitiveUnsigned>(a: &str, m: T, out: Option<&str>)
where
    <UnsignedMatrix<T> as FromStr>::Err: Debug,
{
    let a = UnsignedMatrix::<T>::from_str(a).unwrap();
    let x = (&a).mod_inverse(m);
    assert_eq!(a.clone().mod_inverse(m), x);
    if let Some(x) = &x {
        assert!(x.is_valid());
        let identity = if m == T::ONE {
            a.clone()
        } else {
            UnsignedMatrix::identity(a.height())
        };
        assert_eq!((&a).mod_mul(x, m), identity);
        assert_eq!(x.mod_mul(&a, m), identity);
    }
    assert_eq!(x.map(|x| x.to_string()).as_deref(), out);
}

#[test]
fn test_mod_inverse() {
    test::<u64>("[]_{0,0}", 5, Some("[]_{0,0}"));
    test::<u64>("[3]", 7, Some("[5]"));
    test::<u64>("[0]", 7, None);
    // - a 1 by 1 matrix whose element is not a unit, which is not invertible even though m is
    //   composite
    test::<u64>("[2]", 4, None);
    test::<u64>("[1, 2; 3, 4]", 7, Some("[5, 1; 5, 3]"));
    test::<u64>("[1, 2; 2, 4]", 7, None);
    test::<u64>("[2, 0, 1; 1, 1, 0; 0, 3, 1]", 5, None);
    test::<u8>("[200, 100; 50, 26]", 251, Some("[93, 125; 188, 1]"));
    test::<u64>("[0, 0; 0, 0]", 1, Some("[0, 0; 0, 0]"));
    test::<u64>(
        "[18446744073709551556, 5; 7, 18446744073709551000]",
        18446744073709551557,
        Some(
            "[16361767253117858947, 15513641088809373819; 6961702265365482101, \
            14170774661987605698]",
        ),
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
fn test_mod_inverse_large() {
    let mut seed = 0;
    for (n, m) in [(70, 1099511627791), (130, 18446744073709551557), (100, 2)] {
        let a = pseudorandom_matrix(n, n, m, &mut seed);
        if let Some(x) = (&a).mod_inverse(m) {
            assert_eq!((&a).mod_mul(&x, m), UnsignedMatrix::identity(n));
        } else {
            assert_eq!((&a).mod_determinant(m), 0);
        }
    }
}

#[test]
#[should_panic]
fn mod_inverse_fail_1() {
    UnsignedMatrix::<u64>::from_str("[1]")
        .unwrap()
        .mod_inverse(0);
}

#[test]
#[should_panic]
fn mod_inverse_fail_2() {
    UnsignedMatrix::<u64>::from_str("[1, 2]")
        .unwrap()
        .mod_inverse(5);
}

#[test]
#[should_panic]
fn mod_inverse_fail_3() {
    UnsignedMatrix::<u64>::from_str("[5]")
        .unwrap()
        .mod_inverse(5);
}

#[test]
fn mod_inverse_properties() {
    unsigned_matrix_unsigned_pair_gen_var_12().test_properties(|(a, m)| {
        let x = (&a).mod_inverse(m);
        assert_eq!(a.clone().mod_inverse(m), x);
        let n = a.height();
        assert_eq!(x.is_some(), (&a).mod_determinant(m) != 0);
        if let Some(x) = &x {
            assert!(x.is_valid());
            assert_eq!((&a).mod_mul(x, m), UnsignedMatrix::identity(n));
            assert_eq!(x.mod_mul(&a, m), UnsignedMatrix::identity(n));
            assert_eq!(x.mod_inverse(m).as_ref(), Some(&a));
            assert_eq!(
                (&a).mod_solve(UnsignedMatrix::identity(n), m).as_ref(),
                Some(x)
            );
        }
    });
}
