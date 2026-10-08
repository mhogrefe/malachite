// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Mod, ModAssign, ModIsReduced, ModPowerOf2, PowerOf2,
};
use malachite_base::test_util::generators::{
    unsigned_vector_gen, unsigned_vector_unsigned_pair_gen_var_3,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_rem() {
    let test = |s, m: u64, out| {
        let v = UnsignedVector::<u64>::from_str(s).unwrap();
        // by reference
        let w = &v % m;
        assert_eq!(w.to_string(), out);
        // by value
        assert_eq!(v.clone() % m, w);
        // in place
        let mut x = v;
        x %= m;
        assert_eq!(x, w);
    };
    test("()", 1, "()");
    test("()", 7, "()");
    test("(0)", 7, "(0)");
    // Every element is taken modulo the divisor.
    test("(5, 4, 1)", 3, "(2, 1, 1)");
    test("(5, 4, 1)", 7, "(5, 4, 1)");
    // Modulo 1 every element is zero, but the dimension is unchanged.
    test("(5, 4, 1)", 1, "(0, 0, 0)");
    // Elements that reduce to zero stay, wherever they are.
    test("(4, 3)", 4, "(0, 3)");
    test("(3, 4)", 4, "(3, 0)");
    test("(6, 3, 9)", 3, "(0, 0, 0)");
    // The largest modulus.
    test(
        "(18446744073709551615, 18446744073709551614)",
        u64::MAX,
        "(0, 18446744073709551614)",
    );
}

#[test]
fn test_rem_u8() {
    let test = |s, m: u8, out| {
        assert_eq!(
            (UnsignedVector::<u8>::from_str(s).unwrap() % m).to_string(),
            out
        );
    };
    test("(255, 128)", 7, "(3, 2)");
    test("(255, 128)", 255, "(0, 128)");
    test("(255, 128)", 128, "(127, 0)");
}

#[test]
fn test_mod_op() {
    // `mod_op` is `%` under the name the mod-family traits use; an `UnsignedVector`'s elements are
    // never negative, so there is no case where the two could differ.
    let test = |s, m: u64, out| {
        let v = UnsignedVector::<u64>::from_str(s).unwrap();
        let w = (&v).mod_op(m);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_op(m), w);
        let mut x = v;
        x.mod_assign(m);
        assert_eq!(x, w);
    };
    test("()", 1, "()");
    test("(5, 4, 1)", 3, "(2, 1, 1)");
    test("(5, 4, 1)", 1, "(0, 0, 0)");
    test("(4, 3)", 4, "(0, 3)");
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = UnsignedVector::<u64>::from_str("(1)").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_ref_fail() {
    let _ = &UnsignedVector::<u64>::from_str("(1)").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut v = UnsignedVector::<u64>::from_str("(1)").unwrap();
    v %= 0;
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = UnsignedVector::<u64>::from_str("(1)").unwrap().mod_op(0);
}

#[test]
#[should_panic]
fn mod_op_ref_fail() {
    let _ = (&UnsignedVector::<u64>::from_str("(1)").unwrap()).mod_op(0);
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    let mut v = UnsignedVector::<u64>::from_str("(1)").unwrap();
    v.mod_assign(0);
}

// The 0-dimensional vector has no elements, so a zero divisor is never reached by the elementwise
// loop; only the explicit check makes these panic rather than quietly returning the vector.
#[test]
#[should_panic]
fn rem_empty_vector_fail() {
    let _ = UnsignedVector::<u64>::from_str("()").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_ref_empty_vector_fail() {
    let _ = &UnsignedVector::<u64>::from_str("()").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_assign_empty_vector_fail() {
    let mut v = UnsignedVector::<u64>::from_str("()").unwrap();
    v %= 0;
}

// Reducing modulo 1 is the degenerate case the lint warns about, and asserting that it gives all
// zeros is the point.
#[test]
#[allow(clippy::modulo_one)]
fn rem_properties() {
    unsigned_vector_unsigned_pair_gen_var_3().test_properties(|(v, m)| {
        let w = &v % m;
        // The forms agree.
        assert_eq!(v.clone() % m, w);
        let mut x = v.clone();
        x %= m;
        assert_eq!(x, w);

        // The result is reduced, which is the whole point, and reducing it again changes nothing.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(&w % m, w);

        // The dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());

        // A vector that is already reduced is left alone.
        assert_eq!(v.mod_is_reduced(&m), v == w);

        // `mod_op` is the same operation, in all three forms.
        assert_eq!((&v).mod_op(m), w);
        assert_eq!(v.clone().mod_op(m), w);
        let mut x = v.clone();
        x.mod_assign(m);
        assert_eq!(x, w);

        // This really is the remainder of a division: with the elementwise quotient, `v` is
        // recovered exactly, element by element.
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(x, m * (x / m) + y);
            assert_eq!(y, x % m);
        }
    });

    unsigned_vector_gen().test_properties(|v| {
        // Modulo 1 every element vanishes, but the dimension is unchanged.
        assert!((&v % 1).elements.iter().all(|&x| x == 0));
        // A divisor above the largest element leaves the vector alone.
        if let Some(above) = v.elements.iter().max().copied().unwrap_or(0).checked_add(1) {
            assert_eq!(&v % above, v);
        }
        // For a power-of-2 divisor this is `mod_power_of_2`, which reaches the same answer by
        // masking rather than dividing.
        for pow in 1..64 {
            assert_eq!(&v % u64::power_of_2(pow), (&v).mod_power_of_2(pow));
        }
    });
}
