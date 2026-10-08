// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::{
    unsigned_vec_gen, unsigned_vector_unsigned_pair_gen_var_1,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_index() {
    let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);
    let mut v = v;
    v[1] = 10;
    v[2] += 4;
    assert_eq!(v.to_string(), "(1, 10, 7)");
}

#[test]
#[should_panic]
fn index_fail_1() {
    let v = UnsignedVector::<u32>::from_str("()").unwrap();
    let _ = v[0];
}

#[test]
#[should_panic]
fn index_fail_2() {
    let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    let _ = v[3];
}

#[test]
#[should_panic]
fn index_mut_fail() {
    let mut v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    v[3] = 0;
}

#[test]
fn index_properties() {
    unsigned_vector_unsigned_pair_gen_var_1().test_properties(|(v, i)| {
        assert_eq!(v[i], v.elements[i]);
        assert_eq!(v[i], v.elements_ref()[i]);
        let mut w = v.clone();
        w[i] = w[i].wrapping_add(1);
        assert_eq!(w.dimension(), v.dimension());
        for j in 0..v.elements.len() {
            if j != i {
                assert_eq!(w[j], v[j]);
            }
        }
        w[i] = w[i].wrapping_sub(1);
        assert_eq!(w, v);
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = UnsignedVector { elements: xs };
        for i in 0..v.elements.len() {
            assert_eq!(v[i], v.elements[i]);
            // Changing one element through `IndexMut` changes only that element.
            let mut w = v.clone();
            w[i] = w[i].wrapping_add(1);
            assert_eq!(w.dimension(), v.dimension());
            for j in 0..v.elements.len() {
                if j == i {
                    assert_ne!(w[j], v[j]);
                } else {
                    assert_eq!(w[j], v[j]);
                }
            }
        }
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = UnsignedVector {
            elements: xs.clone(),
        };
        let ys: Vec<u8> = (0..xs.len()).map(|i| v[i]).collect();
        assert_eq!(ys, xs);
    });
}
