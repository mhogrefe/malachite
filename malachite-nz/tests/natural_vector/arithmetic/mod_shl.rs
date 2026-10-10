// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul, ModShl, ModShlAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_4;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_unsigned_natural_triple_gen_var_1;
use std::panic::catch_unwind;

fn test_mod_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalVector: ModShl<T, Natural, Output = NaturalVector>
        + for<'a> ModShl<T, &'a Natural, Output = NaturalVector>
        + ModShlAssign<T, Natural>
        + for<'a> ModShlAssign<T, &'a Natural>,
    for<'a> &'a NaturalVector: ModShl<T, Natural, Output = NaturalVector>,
    for<'a, 'b> &'a NaturalVector: ModShl<T, &'b Natural, Output = NaturalVector>,
{
    let test = |s, bits: u8, m, out| {
        let bits = T::from(bits);
        let v = NaturalVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let w = (&v).mod_shl(bits, &m);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).mod_shl(bits, m.clone()), w);
        assert_eq!(v.clone().mod_shl(bits, &m), w);
        assert_eq!(v.clone().mod_shl(bits, m.clone()), w);
        let mut x = v.clone();
        x.mod_shl_assign(bits, &m);
        assert_eq!(x, w);
        let mut x = v;
        x.mod_shl_assign(bits, m);
        assert_eq!(x, w);
    };
    test("()", 1, "7", "()");
    // Modulo 1 every element is 0.
    test("(0, 0)", 5, "1", "(0, 0)");
    test("(5, 1, 3)", 0, "7", "(5, 1, 3)");
    test("(5, 1, 3)", 1, "7", "(3, 2, 6)");
    // The modulus need not be odd, so elements can become zero, keeping the dimension.
    test("(5, 1, 3)", 3, "8", "(0, 0, 0)");
    test("(1, 2)", 8, "255", "(1, 2)");
    test(
        "(1, 1)",
        100,
        "1000000000000000000000000000000",
        "(267650600228229401496703205376, 267650600228229401496703205376)",
    );
}

#[test]
fn test_mod_shl() {
    apply_fn_to_unsigneds!(test_mod_shl_helper);
}

#[test]
fn mod_shl_fail() {
    // The modulus is zero, even with no element to reduce.
    assert_panic!(
        NaturalVector::from_str("()")
            .unwrap()
            .mod_shl(1u8, Natural::ZERO)
    );
    // An element is not reduced.
    let v = NaturalVector::from_str("(7)").unwrap();
    assert_panic!((&v).mod_shl(1u8, Natural::from(7u32)));
    assert_panic!({
        let mut w = v.clone();
        w.mod_shl_assign(1u8, &Natural::from(7u32));
    });
}

fn mod_shl_properties_helper<T: PrimitiveUnsigned>()
where
    NaturalVector: for<'a> ModShl<T, &'a Natural, Output = NaturalVector>
        + ModShl<T, Natural, Output = NaturalVector>
        + for<'a> ModShlAssign<T, &'a Natural>,
    for<'a, 'b> &'a NaturalVector: ModShl<T, &'b Natural, Output = NaturalVector>,
    for<'a> &'a NaturalVector: Shl<T, Output = NaturalVector>,
    for<'a> &'a Natural: ModShl<T, &'a Natural, Output = Natural>,
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
    u64: ExactFrom<T>,
{
    natural_vector_unsigned_natural_triple_gen_var_1::<T>().test_properties(|(v, bits, m)| {
        let w = (&v).mod_shl(bits, &m);
        assert_eq!(v.clone().mod_shl(bits, &m), w);
        assert_eq!(v.clone().mod_shl(bits, m.clone()), w);
        let mut x = v.clone();
        x.mod_shl_assign(bits, &m);
        assert_eq!(x, w);

        // The result is reduced, it is the ordinary shift reduced, and element by element it is the
        // scalar operation.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), v.dimension());
        assert_eq!((&v << bits) % &m, w);
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.mod_shl(bits, &m));
        }
        // It is multiplication by 2^bits mod m, when 1 is reduced modulo m.
        if m != 1u32 {
            assert_eq!((&v).mod_mul(Natural::ONE.mod_shl(bits, &m), &m), w);
        }
        // Shifting by the same amount of another type gives the same result, and shifting by 0
        // changes nothing.
        assert_eq!(
            <&NaturalVector as ModShl<u64, &Natural>>::mod_shl(&v, u64::exact_from(bits), &m),
            w
        );
        assert_eq!((&v).mod_shl(T::ZERO, &m), v);
    });
}

#[test]
fn mod_shl_properties() {
    apply_fn_to_unsigneds!(mod_shl_properties_helper);

    unsigned_vector_unsigned_unsigned_triple_gen_var_4::<u64, u64>().test_properties(
        |(v, bits, m)| {
            // The `u64` elements shift as their `Natural` counterparts do.
            assert_eq!(
                NaturalVector::from(v.clone()).mod_shl(bits, Natural::from(m)),
                NaturalVector::from(v.mod_shl(bits, m))
            );
        },
    );
}
