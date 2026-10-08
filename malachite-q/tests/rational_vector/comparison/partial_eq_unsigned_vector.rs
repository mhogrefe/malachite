// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_unsigned_vector_pair_gen;

#[test]
fn test_partial_eq_unsigned_vector() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, out: bool)
    where
        Rational: PartialEq<T>,
    {
        let v = RationalVector::from_str(s).unwrap();
        let w = UnsignedVector::<T>::from_str(t).unwrap();
        assert_eq!(v == w, out);
        assert_eq!(w == v, out);
    }
    test::<u8>("()", "()", true);
    test::<u64>("()", "(0)", false);
    test::<u32>("(0)", "()", false);
    test::<u16>("(1)", "(1)", true);
    test::<u8>("(1, 2)", "(1, 2)", true);
    test::<u8>("(1, 2)", "(2, 1)", false);
    test::<u128>("(1, 2, 3)", "(1, 2)", false);
    test::<u8>("(3, 255)", "(3, 255)", true);
    test::<u8>("(3, 254)", "(3, 255)", false);
    // An element too large for the element type matches nothing, including the value it would wrap
    // to.
    test::<u8>("(3, 256)", "(3, 0)", false);
    test::<u8>("(256)", "(0)", false);
    test::<u64>(
        "(18446744073709551615, 18446744073709551615)",
        "(18446744073709551615, 18446744073709551615)",
        true,
    );
    test::<u64>("(18446744073709551616)", "(0)", false);
    // An element written as a fraction matches if it is an integer.
    test::<u8>("(510/2)", "(255)", true);
    // A non-integer element matches nothing, including its floor.
    test::<u8>("(1/2)", "(0)", false);
    test::<u32>("(7/2, 1)", "(3, 1)", false);
    // A negative element matches nothing, including the value it would wrap to.
    test::<u8>("(-1)", "(255)", false);
    test::<u64>("(1, -1)", "(1, 18446744073709551615)", false);
    // A trailing zero matters: the dimensions differ.
    test::<usize>("(1, 0)", "(1)", false);
}

// Comparing with a converted vector is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_unsigned_vector_properties_helper<T: PrimitiveUnsigned>()
where
    Rational: From<T> + PartialEq<T>,
{
    rational_vector_unsigned_vector_pair_gen::<T>().test_properties(|(v, w)| {
        let eq = v == w;
        assert_eq!(w == v, eq);
        // Extra refs for type inference: with `RationalVector: PartialEq<UnsignedVector<T>>` in
        // scope, `v == w` would look for that impl.
        assert_eq!(&v == &RationalVector::from(w.clone()), eq);
        if eq {
            assert_eq!(v.dimension(), w.dimension());
        }

        let w_q = RationalVector::from(w.clone());
        assert!(w_q == w);
        assert!(w == w_q);
    });
}

#[test]
fn partial_eq_unsigned_vector_properties() {
    apply_fn_to_unsigneds!(partial_eq_unsigned_vector_properties_helper);
}
