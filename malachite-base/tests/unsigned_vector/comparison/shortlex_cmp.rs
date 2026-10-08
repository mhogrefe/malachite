// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::common::test_custom_cmp_helper;
use malachite_base::test_util::generators::{
    unsigned_vec_gen, unsigned_vec_pair_gen, unsigned_vec_triple_gen, unsigned_vector_pair_gen,
    unsigned_vector_triple_gen,
};
use malachite_base::test_util::unsigned_vector::comparison::shortlex_cmp::*;
use malachite_base::unsigned_vector::{
    ShortlexUnsignedVector, ShortlexUnsignedVectorRef, UnsignedVector,
};

fn shortlex<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, w: &UnsignedVector<T>) -> Ordering {
    ShortlexUnsignedVectorRef(v).cmp(&ShortlexUnsignedVectorRef(w))
}

#[test]
fn test_shortlex_cmp() {
    test_custom_cmp_helper::<UnsignedVector<u32>, _>(
        &[
            "()",
            "(0)",
            "(1)",
            "(5)",
            "(100)",
            "(0, 0)",
            "(0, 1)",
            "(1, 0)",
            "(5, 5)",
            "(0, 0, 0)",
            "(0, 0, 1)",
            "(0, 0, 0, 0)",
        ],
        shortlex,
    );
}

#[test]
fn shortlex_cmp_properties() {
    unsigned_vector_pair_gen().test_properties(|(v, w)| {
        let c = shortlex(&v, &w);
        assert_eq!(shortlex(&w, &v), c.reverse());
        assert_eq!(v == w, c == Equal);
        assert_eq!(unsigned_vector_shortlex_cmp_naive(&v, &w), c);
        match v.dimension().cmp(&w.dimension()) {
            Equal => assert_eq!(v.elements.cmp(&w.elements), c),
            d => assert_eq!(c, d),
        }
    });

    unsigned_vector_triple_gen().test_properties(|(u, v, w)| {
        if shortlex(&u, &v) == Less && shortlex(&v, &w) == Less {
            assert_eq!(shortlex(&u, &w), Less);
        } else if shortlex(&u, &v) == Greater && shortlex(&v, &w) == Greater {
            assert_eq!(shortlex(&u, &w), Greater);
        }
    });

    unsigned_vec_pair_gen::<u8>().test_properties(|(xs, ys)| {
        let v = UnsignedVector { elements: xs };
        let w = UnsignedVector { elements: ys };
        let c = shortlex(&v, &w);
        assert_eq!(shortlex(&w, &v), c.reverse());
        assert_eq!(v == w, c == Equal);
        assert_eq!(unsigned_vector_shortlex_cmp_naive(&v, &w), c);
        assert_eq!(
            (v.dimension(), &v.elements).cmp(&(w.dimension(), &w.elements)),
            c
        );
        let owned = ShortlexUnsignedVector(v.clone());
        let other_owned = ShortlexUnsignedVector(w.clone());
        assert_eq!(owned.cmp(&other_owned), c);
        assert_eq!(owned.as_ref().cmp(&other_owned.as_ref()), c);
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = UnsignedVector { elements: xs };
        assert_eq!(shortlex(&v, &v), Equal);
        assert!(
            ShortlexUnsignedVector(v)
                >= ShortlexUnsignedVector(UnsignedVector {
                    elements: Vec::new()
                })
        );
    });

    unsigned_vec_triple_gen::<u8>().test_properties(|(xs, ys, zs)| {
        let (u, v, w) = (
            UnsignedVector { elements: xs },
            UnsignedVector { elements: ys },
            UnsignedVector { elements: zs },
        );
        if shortlex(&u, &v) == Less && shortlex(&v, &w) == Less {
            assert_eq!(shortlex(&u, &w), Less);
        } else if shortlex(&u, &v) == Greater && shortlex(&v, &w) == Greater {
            assert_eq!(shortlex(&u, &w), Greater);
        }
    });
}
