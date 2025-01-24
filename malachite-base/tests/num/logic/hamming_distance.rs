// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::{CountOnes, CountZeros};
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_2, signed_gen_var_4, signed_pair_gen, signed_pair_gen_var_1,
    signed_triple_gen_var_3, unsigned_gen, unsigned_pair_gen_var_27, unsigned_triple_gen_var_19,
};

#[test]
pub fn test_hamming_distance() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, out: u64) {
        assert_eq!(x.hamming_distance(y), out);
    }
    test(123u16, 456u16, 6);
    test(0xffffu32, 0xffff0000u32, 32);
    test(0xffffu32, u32::MAX, 16);
    test(0xffff0000u32, u32::MAX, 16);
}

#[test]
pub fn test_checked_hamming_distance() {
    fn test<T: PrimitiveSigned>(x: T, y: T, out: Option<u64>) {
        assert_eq!(x.checked_hamming_distance(y), out);
    }
    test(123i32, 456i32, Some(6));
    test(-123i32, -456i32, Some(7));
    test(0i8, 127i8, Some(7));
    test(0i8, -1i8, None);
    test(-1i8, -128i8, Some(7));
    test(0i128, i128::MAX, Some(127));
    test(0i128, -1i128, None);
    test(-1i128, i128::MIN, Some(127));
    test(0xffffi32, 0x7fff0000i32, Some(31));
    test(0xffffi32, i32::MAX, Some(15));
    test(0x7fff0000i32, i32::MAX, Some(16));
}

fn hamming_distance_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let distance = x.hamming_distance(y);
        assert_eq!(y.hamming_distance(x), distance);
        assert_eq!(distance == 0, x == y);
        assert_eq!(CountOnes::count_ones(x ^ y), distance);
        assert_eq!((!x).hamming_distance(!y), distance);
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        assert!(x.hamming_distance(z) <= x.hamming_distance(y) + y.hamming_distance(z));
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.hamming_distance(x), 0);
        assert_eq!(x.hamming_distance(!x), T::WIDTH);
        assert_eq!(x.hamming_distance(T::ZERO), CountOnes::count_ones(x));
        assert_eq!(T::ZERO.hamming_distance(x), CountOnes::count_ones(x));
    });
}

#[test]
fn hamming_distance_properties() {
    apply_fn_to_unsigneds!(hamming_distance_properties_helper);
}

fn checked_hamming_distance_properties_helper<T: PrimitiveSigned>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let distance = x.checked_hamming_distance(y);
        assert_eq!(y.checked_hamming_distance(x), distance);
        assert_eq!(distance == Some(0), x == y);
        assert_eq!((!x).checked_hamming_distance(!y), distance);
    });

    signed_pair_gen_var_1::<T>().test_properties(|(x, y)| {
        let distance = x.checked_hamming_distance(y).unwrap();
        assert_eq!(y.checked_hamming_distance(x).unwrap(), distance);
        assert_eq!(distance == 0, x == y);
        assert_eq!(CountOnes::count_ones(x ^ y), distance);
        assert_eq!((!x).checked_hamming_distance(!y).unwrap(), distance);
    });

    signed_triple_gen_var_3::<T>().test_properties(|(x, y, z)| {
        assert!(
            x.checked_hamming_distance(z).unwrap()
                <= x.checked_hamming_distance(y).unwrap() + y.checked_hamming_distance(z).unwrap()
        );
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.checked_hamming_distance(x), Some(0));
        assert_eq!(x.checked_hamming_distance(!x), None);
    });

    signed_gen_var_2::<T>().test_properties(|x| {
        assert_eq!(
            x.checked_hamming_distance(T::ZERO),
            Some(CountOnes::count_ones(x))
        );
        assert_eq!(
            T::ZERO.checked_hamming_distance(x),
            Some(CountOnes::count_ones(x))
        );
    });

    signed_gen_var_4::<T>().test_properties(|x| {
        assert_eq!(
            x.checked_hamming_distance(T::NEGATIVE_ONE),
            Some(CountZeros::count_zeros(x))
        );
        assert_eq!(
            T::NEGATIVE_ONE.checked_hamming_distance(x),
            Some(CountZeros::count_zeros(x))
        );
    });
}

#[test]
fn checked_hamming_distance_properties() {
    apply_fn_to_signeds!(checked_hamming_distance_properties_helper);
}
