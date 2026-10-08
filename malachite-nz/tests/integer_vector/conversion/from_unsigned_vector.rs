// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{unsigned_vector_gen, unsigned_vector_pair_gen};
use malachite_base::unsigned_vector::{ShortlexUnsignedVectorRef, UnsignedVector};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::{IntegerVector, ShortlexIntegerVectorRef};
use malachite_nz::natural_vector::NaturalVector;

#[test]
fn test_from_unsigned_vector() {
    fn test<T: PrimitiveUnsigned>(s: &str)
    where
        Integer: From<T>,
    {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        assert_eq!(IntegerVector::from(v).to_string(), s);
    }
    test::<u64>("()");
    test::<u64>("(0)");
    test::<u64>("(1, 2, 3)");
    // The largest element an `UnsignedVector` can hold survives.
    test::<u64>("(18446744073709551615, 0)");
    test::<u8>("(255, 1)");
    test::<u128>("(340282366920938463463374607431768211455)");
    test::<usize>("(5, 6)");
}

#[test]
fn from_unsigned_vector_properties() {
    unsigned_vector_gen().test_properties(|v| {
        let w = IntegerVector::from(v.clone());
        // Nothing is lost: the dimension, every element, and the written form all survive.
        assert_eq!(w.dimension(), v.dimension());
        assert_eq!(w.to_string(), v.to_string());
        assert_eq!(
            w.elements,
            v.elements
                .iter()
                .map(|&x| Integer::from(x))
                .collect::<Vec<_>>()
        );
        // The result is never negative anywhere, since it came from unsigned primitives.
        assert!(w.elements.iter().all(|x| *x >= 0u32));
        // Reading the string back as an `IntegerVector` gives the same thing.
        assert_eq!(IntegerVector::from_str(&v.to_string()).unwrap(), w);
        // Going through a `NaturalVector` gives the same thing.
        assert_eq!(IntegerVector::from(NaturalVector::from(v)), w);
    });

    unsigned_vector_pair_gen().test_properties(|(v, w)| {
        let v_i = IntegerVector::from(v.clone());
        let w_i = IntegerVector::from(w.clone());
        // The two types order their vectors the same way.
        assert_eq!(
            ShortlexUnsignedVectorRef(&v).cmp(&ShortlexUnsignedVectorRef(&w)),
            ShortlexIntegerVectorRef(&v_i).cmp(&ShortlexIntegerVectorRef(&w_i))
        );
        assert_eq!(v == w, v_i == w_i);
    });
}
