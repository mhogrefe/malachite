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
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_gen, natural_vector_natural_pair_gen_var_1, natural_vector_unsigned_pair_gen,
};

#[test]
fn test_rem() {
    let test = |s, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let w = &v % &m;
        assert_eq!(w.to_string(), out);
        assert_eq!(&v % m.clone(), w);
        assert_eq!(v.clone() % &m, w);
        assert_eq!(v.clone() % m.clone(), w);
        let mut x = v.clone();
        x %= &m;
        assert_eq!(x, w);
        let mut x = v;
        x %= m;
        assert_eq!(x, w);
    };
    test("()", "1", "()");
    test("()", "7", "()");
    test("(0)", "7", "(0)");
    // Every element is taken modulo the divisor.
    test("(5, 4, 1)", "3", "(2, 1, 1)");
    test("(5, 4, 1)", "7", "(5, 4, 1)");
    // Modulo 1 every element is zero, but the dimension is unchanged.
    test("(5, 4, 1)", "1", "(0, 0, 0)");
    // Elements that reduce to zero stay, wherever they are.
    test("(4, 3)", "4", "(0, 3)");
    test("(3, 4)", "4", "(3, 0)");
    test("(6, 3, 9)", "3", "(0, 0, 0)");
    // Elements and divisor of many limbs.
    test(
        "(1000000000001, 2000000000003, 5)",
        "1000000000000",
        "(1, 3, 5)",
    );
    test(
        "(1000000000000000000000000, 1)",
        "1234567890987",
        "(530068894399, 1)",
    );
    // A divisor larger than every element leaves the vector alone.
    test("(5, 4, 1)", "1000000000000000000000000", "(5, 4, 1)");
}

#[test]
fn test_mod_op() {
    // `mod_op` is `%` under the name the mod-family traits use; a `NaturalVector`'s elements are
    // never negative, so there is no case where the two could differ.
    let test = |s, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let w = (&v).mod_op(&m);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).mod_op(m.clone()), w);
        assert_eq!(v.clone().mod_op(&m), w);
        assert_eq!(v.clone().mod_op(m.clone()), w);
        let mut x = v.clone();
        x.mod_assign(&m);
        assert_eq!(x, w);
        let mut x = v;
        x.mod_assign(m);
        assert_eq!(x, w);
    };
    test("()", "1", "()");
    test("(5, 4, 1)", "3", "(2, 1, 1)");
    test("(5, 4, 1)", "1", "(0, 0, 0)");
    test("(4, 3)", "4", "(0, 3)");
    test(
        "(1000000000000000000000000, 1)",
        "1234567890987",
        "(530068894399, 1)",
    );
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = NaturalVector::from_str("(1)").unwrap() % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_val_ref_fail() {
    let _ = NaturalVector::from_str("(1)").unwrap() % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_val_fail() {
    let _ = &NaturalVector::from_str("(1)").unwrap() % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_ref_fail() {
    let _ = &NaturalVector::from_str("(1)").unwrap() % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v %= Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_ref_fail() {
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v %= &Natural::ZERO;
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = NaturalVector::from_str("(1)")
        .unwrap()
        .mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v.mod_assign(&Natural::ZERO);
}

// The 0-dimensional vector has no elements, so a zero divisor is never reached by the elementwise
// loop; only the explicit check makes these panic rather than quietly returning the vector.
#[test]
#[should_panic]
fn rem_empty_vector_fail() {
    let _ = NaturalVector::from_str("()").unwrap() % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_empty_vector_fail() {
    let _ = &NaturalVector::from_str("()").unwrap() % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_empty_vector_fail() {
    let mut v = NaturalVector::from_str("()").unwrap();
    v %= &Natural::ZERO;
}

#[test]
fn rem_properties() {
    natural_vector_natural_pair_gen_var_1().test_properties(|(v, m)| {
        let w = &v % &m;
        // The forms agree.
        assert_eq!(&v % m.clone(), w);
        assert_eq!(v.clone() % &m, w);
        assert_eq!(v.clone() % m.clone(), w);
        let mut x = v.clone();
        x %= &m;
        assert_eq!(x, w);
        let mut x = v.clone();
        x %= m.clone();
        assert_eq!(x, w);

        // The result is reduced, which is the whole point, and reducing it again changes nothing.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(&w % &m, w);

        // The dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());

        // A vector that is already reduced is left alone.
        assert_eq!(v.mod_is_reduced(&m), v == w);

        // `mod_op` is the same operation.
        assert_eq!((&v).mod_op(&m), w);
        assert_eq!(v.clone().mod_op(m.clone()), w);
        let mut x = v.clone();
        x.mod_assign(&m);
        assert_eq!(x, w);

        // This really is the remainder of a division: with the elementwise quotient, `v` is
        // recovered exactly, element by element.
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*x, &m * (x / &m) + y);
            assert_eq!(*y, x % &m);
        }
    });

    natural_vector_gen().test_properties(|v| {
        // Modulo 1 every element vanishes, but the dimension is unchanged.
        assert!((&v % Natural::ONE).elements.iter().all(|x| *x == 0u32));
        // A divisor above the largest element leaves the vector alone.
        let above = v.elements.iter().max().cloned().unwrap_or_default() + Natural::ONE;
        assert_eq!(&v % &above, v);
        // For a power-of-2 divisor this is `mod_power_of_2`, which reaches the same answer by
        // masking rather than dividing.
        for pow in [1, 7, 64, 100] {
            assert_eq!(&v % Natural::power_of_2(pow), (&v).mod_power_of_2(pow));
        }
    });

    unsigned_vector_gen().test_properties(|v| {
        // The `u64` elements reduce as their `Natural` counterparts do.
        let w = NaturalVector::from(v.clone());
        for m in [1u64, 2, 3, 10, 1000, u64::MAX] {
            assert_eq!(
                &w % Natural::from(m),
                NaturalVector {
                    elements: v.elements.iter().map(|&x| Natural::from(x % m)).collect()
                }
            );
        }
    });
}

#[test]
fn test_rem_unsigned() {
    fn test<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(s: &str, m: T, out: &str)
    where
        Natural: From<T>,
    {
        let v = NaturalVector::from_str(s).unwrap();
        // Both forms of `%` and of `mod_op`.
        let w: UnsignedVector<T> = &v % m;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() % m, w);
        assert_eq!((&v).mod_op(m), w);
        assert_eq!(v.mod_op(m), w);
    }
    test("()", 1u8, "()");
    test("()", 5u16, "()");
    test("(1000000000001, 2000000000003, 5)", 1000u32, "(1, 3, 5)");
    test("(1000000000001, 2000000000003, 5)", 1u64, "(0, 0, 0)");
    // The result's element type is the modulus's.
    test("(1000000000001, 2000000000003, 5)", 7u8, "(2, 5, 5)");
    test("(18446744073709551616)", u64::MAX, "(1)");
    test(
        "(18446744073709551616)",
        u128::MAX,
        "(18446744073709551616)",
    );
    // An element that reduces to zero stays, so the dimension is unchanged.
    test("(1024, 3)", 4usize, "(0, 3)");
}

#[test]
#[should_panic]
fn rem_unsigned_fail() {
    let _: UnsignedVector<u8> = NaturalVector::from_str("(1)").unwrap() % 0u8;
}

#[test]
#[should_panic]
fn rem_unsigned_ref_fail() {
    let _: UnsignedVector<u64> = &NaturalVector::from_str("(1)").unwrap() % 0u64;
}

#[test]
#[should_panic]
fn rem_unsigned_empty_vector_fail() {
    let _: UnsignedVector<u32> = &NaturalVector::from_str("()").unwrap() % 0u32;
}

#[test]
#[should_panic]
fn mod_op_unsigned_fail() {
    let _: UnsignedVector<u16> = NaturalVector::from_str("(1)").unwrap().mod_op(0u16);
}

fn rem_unsigned_properties_helper<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>()
where
    Natural: From<T>,
{
    natural_vector_unsigned_pair_gen::<T>().test_properties(|(v, m)| {
        if m == T::ZERO {
            return;
        }
        let w: UnsignedVector<T> = &v % m;
        // The forms agree.
        assert_eq!(v.clone() % m, w);
        assert_eq!((&v).mod_op(m), w);
        assert_eq!(v.clone().mod_op(m), w);

        // The result is reduced, and reducing it again changes nothing.
        assert!(w.mod_is_reduced(&m));
        assert_eq!((&w).mod_op(m), w);

        // Apart from its type, the result is the reduction modulo the Natural with the same value.
        let r = &v % Natural::from(m);
        assert_eq!(UnsignedVector::<T>::try_from(&r), Ok(w.clone()));
        assert_eq!(NaturalVector::from(w), r);
    });
}

#[test]
fn rem_unsigned_properties() {
    apply_fn_to_unsigneds!(rem_unsigned_properties_helper);
}
