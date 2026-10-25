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
    Mod, ModAssign, ModIsReduced, ModPowerOf2, PowerOf2,
};
use malachite_base::test_util::generators::{
    unsigned_matrix_gen, unsigned_matrix_unsigned_pair_gen_var_1,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_rem() {
    let test = |s, m: u64, out| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        // by reference
        let b = &a % m;
        assert_eq!(b.to_string(), out);
        // by value
        assert_eq!(a.clone() % m, b);
        // in place
        let mut x = a;
        x %= m;
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 1, "[]_{0,0}");
    test("[]_{0,0}", 7, "[]_{0,0}");
    test("[]_{3,0}", 7, "[]_{3,0}");
    test("[]_{0,4}", 7, "[]_{0,4}");
    test("[0]", 7, "[0]");
    // Every element is taken modulo the divisor.
    test("[5, 4, 1]", 3, "[2, 1, 1]");
    test("[5, 4, 1]", 7, "[5, 4, 1]");
    // Modulo 1 every element is zero, but the shape is unchanged.
    test("[5, 4, 1]", 1, "[0, 0, 0]");
    // Elements that reduce to zero stay, wherever they are.
    test("[4, 3]", 4, "[0, 3]");
    test("[3, 4]", 4, "[3, 0]");
    test("[6, 3, 9]", 3, "[0, 0, 0]");
    // A matrix with several rows is reduced element by element, keeping its shape.
    test("[5, 4; 1, 9]", 3, "[2, 1; 1, 0]");
    test("[4; 3]", 4, "[0; 3]");
    // The largest modulus.
    test(
        "[18446744073709551615, 18446744073709551614]",
        u64::MAX,
        "[0, 18446744073709551614]",
    );
}

#[test]
fn test_rem_u8() {
    let test = |s, m: u8, out| {
        assert_eq!(
            (UnsignedMatrix::<u8>::from_str(s).unwrap() % m).to_string(),
            out
        );
    };
    test("[255, 128]", 7, "[3, 2]");
    test("[255, 128]", 255, "[0, 128]");
    test("[255, 128]", 128, "[127, 0]");
}

#[test]
fn test_mod_op() {
    // `mod_op` is `%` under the name the mod-family traits use; an `UnsignedMatrix`'s elements are
    // never negative, so there is no case where the two could differ.
    let test = |s, m: u64, out| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        let b = (&a).mod_op(m);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_op(m), b);
        let mut x = a;
        x.mod_assign(m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 1, "[]_{0,0}");
    test("[5, 4, 1]", 3, "[2, 1, 1]");
    test("[5, 4, 1]", 1, "[0, 0, 0]");
    test("[4, 3]", 4, "[0, 3]");
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = UnsignedMatrix::<u64>::from_str("[1]").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_ref_fail() {
    let _ = &UnsignedMatrix::<u64>::from_str("[1]").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut a = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a %= 0;
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = UnsignedMatrix::<u64>::from_str("[1]").unwrap().mod_op(0);
}

#[test]
#[should_panic]
fn mod_op_ref_fail() {
    let _ = (&UnsignedMatrix::<u64>::from_str("[1]").unwrap()).mod_op(0);
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    let mut a = UnsignedMatrix::<u64>::from_str("[1]").unwrap();
    a.mod_assign(0);
}

// A matrix with no elements gives the elementwise loop nothing to do, so a zero divisor is never
// reached by the elementwise loop; only the explicit check makes these panic rather than quietly
// returning the matrix.
#[test]
#[should_panic]
fn rem_empty_matrix_fail() {
    let _ = UnsignedMatrix::<u64>::from_str("[]_{0,0}").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_ref_empty_matrix_fail() {
    let _ = &UnsignedMatrix::<u64>::from_str("[]_{0,0}").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_assign_empty_matrix_fail() {
    let mut a = UnsignedMatrix::<u64>::from_str("[]_{0,0}").unwrap();
    a %= 0;
}

// Reducing modulo 1 is the degenerate case the lint warns about, and asserting that it gives all
// zeros is the point.
#[test]
#[allow(clippy::modulo_one)]
fn rem_properties() {
    unsigned_matrix_unsigned_pair_gen_var_1().test_properties(|(a, m)| {
        let b = &a % m;
        // The forms agree.
        assert_eq!(a.clone() % m, b);
        let mut x = a.clone();
        x %= m;
        assert_eq!(x, b);

        // The result is reduced, which is the whole point, and reducing it again changes nothing.
        assert!(b.mod_is_reduced(&m));
        assert_eq!(&b % m, b);

        // The shape is unchanged.
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));

        // A matrix that is already reduced is left alone.
        assert_eq!(a.mod_is_reduced(&m), a == b);

        // `mod_op` is the same operation, in all three forms.
        assert_eq!((&a).mod_op(m), b);
        assert_eq!(a.clone().mod_op(m), b);
        let mut x = a.clone();
        x.mod_assign(m);
        assert_eq!(x, b);

        // This really is the remainder of a division: with the elementwise quotient, `a` is
        // recovered exactly, element by element.
        for (&x, &y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(x, m * (x / m) + y);
            assert_eq!(y, x % m);
        }
    });

    unsigned_matrix_gen().test_properties(|a| {
        // Modulo 1 every element vanishes, but the shape is unchanged.
        assert!((&a % 1).elements_ref().iter().all(|&x| x == 0));
        // A divisor above the largest element leaves the matrix alone.
        if let Some(above) = a
            .elements_ref()
            .iter()
            .max()
            .copied()
            .unwrap_or(0)
            .checked_add(1)
        {
            assert_eq!(&a % above, a);
        }
        // For a power-of-2 divisor this is `mod_power_of_2`, which reaches the same answer by
        // masking rather than dividing.
        for pow in 1..64 {
            assert_eq!(&a % u64::power_of_2(pow), (&a).mod_power_of_2(pow));
        }
    });
}
