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
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_gen, natural_matrix_natural_pair_gen_var_1,
    natural_matrix_unsigned_pair_gen_var_1,
};

#[test]
fn test_rem() {
    let test = |s, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let b = &a % &m;
        assert_eq!(b.to_string(), out);
        assert_eq!(&a % m.clone(), b);
        assert_eq!(a.clone() % &m, b);
        assert_eq!(a.clone() % m.clone(), b);
        let mut x = a.clone();
        x %= &m;
        assert_eq!(x, b);
        let mut x = a;
        x %= m;
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "1", "[]_{0,0}");
    test("[]_{0,0}", "7", "[]_{0,0}");
    test("[]_{3,0}", "7", "[]_{3,0}");
    test("[]_{0,4}", "7", "[]_{0,4}");
    test("[0]", "7", "[0]");
    // Every element is taken modulo the divisor.
    test("[5, 4, 1]", "3", "[2, 1, 1]");
    test("[5, 4, 1]", "7", "[5, 4, 1]");
    // Modulo 1 every element is zero, but the shape is unchanged.
    test("[5, 4, 1]", "1", "[0, 0, 0]");
    // Elements that reduce to zero stay, wherever they are.
    test("[4, 3]", "4", "[0, 3]");
    test("[3, 4]", "4", "[3, 0]");
    test("[6, 3, 9]", "3", "[0, 0, 0]");
    // A matrix with several rows is reduced element by element, keeping its shape.
    test("[5, 4; 1, 9]", "3", "[2, 1; 1, 0]");
    test("[4; 3]", "4", "[0; 3]");
    // Elements and divisor of many limbs.
    test(
        "[1000000000001, 2000000000003, 5]",
        "1000000000000",
        "[1, 3, 5]",
    );
    test(
        "[1000000000000000000000000, 1]",
        "1234567890987",
        "[530068894399, 1]",
    );
    // A divisor larger than every element leaves the matrix alone.
    test("[5, 4, 1]", "1000000000000000000000000", "[5, 4, 1]");
}

#[test]
fn test_mod_op() {
    // `mod_op` is `%` under the name the mod-family traits use; a `NaturalMatrix`'s elements are
    // never negative, so there is no case where the two could differ.
    let test = |s, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let b = (&a).mod_op(&m);
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_op(m.clone()), b);
        assert_eq!(a.clone().mod_op(&m), b);
        assert_eq!(a.clone().mod_op(m.clone()), b);
        let mut x = a.clone();
        x.mod_assign(&m);
        assert_eq!(x, b);
        let mut x = a;
        x.mod_assign(m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "1", "[]_{0,0}");
    test("[5, 4, 1]", "3", "[2, 1, 1]");
    test("[5, 4, 1]", "1", "[0, 0, 0]");
    test("[4, 3]", "4", "[0, 3]");
    test(
        "[1000000000000000000000000, 1]",
        "1234567890987",
        "[530068894399, 1]",
    );
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = NaturalMatrix::from_str("[1]").unwrap() % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_val_ref_fail() {
    let _ = NaturalMatrix::from_str("[1]").unwrap() % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_val_fail() {
    let _ = &NaturalMatrix::from_str("[1]").unwrap() % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_ref_fail() {
    let _ = &NaturalMatrix::from_str("[1]").unwrap() % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1]").unwrap();
    a %= Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_ref_fail() {
    let mut a = NaturalMatrix::from_str("[1]").unwrap();
    a %= &Natural::ZERO;
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_assign(&Natural::ZERO);
}

// A matrix with no elements gives the elementwise loop nothing to do, so a zero divisor is never
// reached by the elementwise loop; only the explicit check makes these panic rather than quietly
// returning the matrix.
#[test]
#[should_panic]
fn rem_empty_matrix_fail() {
    let _ = NaturalMatrix::from_str("[]_{0,0}").unwrap() % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_empty_matrix_fail() {
    let _ = &NaturalMatrix::from_str("[]_{0,0}").unwrap() % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_empty_matrix_fail() {
    let mut a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    a %= &Natural::ZERO;
}

#[test]
fn rem_properties() {
    natural_matrix_natural_pair_gen_var_1().test_properties(|(a, m)| {
        let b = &a % &m;
        // The forms agree.
        assert_eq!(&a % m.clone(), b);
        assert_eq!(a.clone() % &m, b);
        assert_eq!(a.clone() % m.clone(), b);
        let mut x = a.clone();
        x %= &m;
        assert_eq!(x, b);
        let mut x = a.clone();
        x %= m.clone();
        assert_eq!(x, b);

        // The result is reduced, which is the whole point, and reducing it again changes nothing.
        assert!(b.mod_is_reduced(&m));
        assert_eq!(&b % &m, b);

        // The shape is unchanged.
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));

        // A matrix that is already reduced is left alone.
        assert_eq!(a.mod_is_reduced(&m), a == b);

        // `mod_op` is the same operation.
        assert_eq!((&a).mod_op(&m), b);
        assert_eq!(a.clone().mod_op(m.clone()), b);
        let mut x = a.clone();
        x.mod_assign(&m);
        assert_eq!(x, b);

        // This really is the remainder of a division: with the elementwise quotient, `a` is
        // recovered exactly, element by element.
        for (x, y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(*x, &m * (x / &m) + y);
            assert_eq!(*y, x % &m);
        }
    });

    natural_matrix_gen().test_properties(|a| {
        // Modulo 1 every element vanishes, but the shape is unchanged.
        assert!(
            (&a % Natural::ONE)
                .elements_ref()
                .iter()
                .all(|x| *x == 0u32)
        );
        // A divisor above the largest element leaves the matrix alone.
        let above = a.elements_ref().iter().max().cloned().unwrap_or_default() + Natural::ONE;
        assert_eq!(&a % &above, a);
        // For a power-of-2 divisor this is `mod_power_of_2`, which reaches the same answer by
        // masking rather than dividing.
        for pow in [1, 7, 64, 100] {
            assert_eq!(&a % Natural::power_of_2(pow), (&a).mod_power_of_2(pow));
        }
    });

    unsigned_matrix_gen().test_properties(|a| {
        // The `u64` elements reduce as their `Natural` counterparts do.
        let b = NaturalMatrix::from(a.clone());
        for m in [1u64, 2, 3, 10, 1000, u64::MAX] {
            assert_eq!(&b % Natural::from(m), NaturalMatrix::from(&a % m));
        }
    });
}

#[test]
fn test_rem_unsigned() {
    fn test<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(s: &str, m: T, out: &str)
    where
        Natural: From<T>,
    {
        let a = NaturalMatrix::from_str(s).unwrap();
        // Both forms of `%` and of `mod_op`.
        let b: UnsignedMatrix<T> = &a % m;
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone() % m, b);
        assert_eq!((&a).mod_op(m), b);
        assert_eq!(a.mod_op(m), b);
    }
    test("[]_{0,0}", 1u8, "[]_{0,0}");
    test("[]_{0,0}", 5u16, "[]_{0,0}");
    test("[1000000000001, 2000000000003, 5]", 1000u32, "[1, 3, 5]");
    test("[1000000000001, 2000000000003, 5]", 1u64, "[0, 0, 0]");
    // The result's element type is the modulus's.
    test("[1000000000001, 2000000000003, 5]", 7u8, "[2, 5, 5]");
    test("[18446744073709551616]", u64::MAX, "[1]");
    test(
        "[18446744073709551616]",
        u128::MAX,
        "[18446744073709551616]",
    );
    // An element that reduces to zero stays, so the shape is unchanged.
    test("[1024, 3]", 4usize, "[0, 3]");
}

#[test]
#[should_panic]
fn rem_unsigned_fail() {
    let _: UnsignedMatrix<u8> = NaturalMatrix::from_str("[1]").unwrap() % 0u8;
}

#[test]
#[should_panic]
fn rem_unsigned_ref_fail() {
    let _: UnsignedMatrix<u64> = &NaturalMatrix::from_str("[1]").unwrap() % 0u64;
}

#[test]
#[should_panic]
fn rem_unsigned_empty_matrix_fail() {
    let _: UnsignedMatrix<u32> = &NaturalMatrix::from_str("[]_{0,0}").unwrap() % 0u32;
}

#[test]
#[should_panic]
fn mod_op_unsigned_fail() {
    let _: UnsignedMatrix<u16> = NaturalMatrix::from_str("[1]").unwrap().mod_op(0u16);
}

fn rem_unsigned_properties_helper<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>()
where
    Natural: From<T>,
{
    natural_matrix_unsigned_pair_gen_var_1::<T>().test_properties(|(a, m)| {
        if m == T::ZERO {
            return;
        }
        let b: UnsignedMatrix<T> = &a % m;
        // The forms agree.
        assert_eq!(a.clone() % m, b);
        assert_eq!((&a).mod_op(m), b);
        assert_eq!(a.clone().mod_op(m), b);

        // The result is reduced, and reducing it again changes nothing.
        assert!(b.mod_is_reduced(&m));
        assert_eq!((&b).mod_op(m), b);

        // Apart from its type, the result is the reduction modulo the Natural with the same value.
        let r = &a % Natural::from(m);
        assert_eq!(UnsignedMatrix::<T>::try_from(&r), Ok(b.clone()));
        assert_eq!(NaturalMatrix::from(b), r);
    });
}

#[test]
fn rem_unsigned_properties() {
    apply_fn_to_unsigneds!(rem_unsigned_properties_helper);
}
