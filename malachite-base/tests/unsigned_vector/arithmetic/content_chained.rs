// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Content, ContentChained};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_content_chained() {
    fn test<T: PrimitiveUnsigned>(s: &str, x: T, out: T) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let g = (&v).content_chained(x);
        assert_eq!(g, out);
        assert_eq!(v.content_chained(x), g);
    }
    // - all elements zero, or none
    test::<u8>("()", 6, 6);
    test::<u8>("()", 0, 0);
    test::<u8>("(0, 0)", 6, 6);
    // - one element
    test::<u8>("(4)", 6, 2);
    // - zeros at both ends are skipped
    test::<u8>("(0, 4, 0)", 6, 2);
    // - an element of 1 at either end
    test::<u8>("(1, 6)", 4, 1);
    test::<u8>("(6, 1)", 4, 1);
    // - the GCD is 1 from the start
    test::<u8>("(4, 6)", 1, 1);
    // - pairs from both ends, then one element left
    test::<u8>("(6, 4, 10)", 4, 2);
    test::<u8>("(12, 24, 36, 48, 60)", 12, 12);
    // - the GCD reaches 1 with elements left
    test::<u8>("(2, 6, 6, 3)", 6, 1);
    test::<u64>("(6000000000000, 4000000000000, 0)", 0, 2000000000000);
}

fn content_chained_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vec_unsigned_pair_gen::<T, T>().test_properties(|(xs, x)| {
        let v = UnsignedVector::from_owned_elements(xs);
        let g = (&v).content_chained(x);
        assert_eq!(v.clone().content_chained(x), g);
        // It is the GCD of `x` and the elements, taken one at a time.
        assert_eq!(v.elements.iter().fold(x, |g, &c| g.gcd(c)), g);
        assert_eq!(x.gcd((&v).content()), g);
        assert!(x.divisible_by(g));
        assert!(v.elements.iter().all(|&c| c.divisible_by(g)));
    });
}

#[test]
fn content_chained_properties() {
    apply_fn_to_unsigneds!(content_chained_properties_helper);
}
