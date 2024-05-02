// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::test_util::generators::{
    signed_gen_var_3, signed_unsigned_pair_gen_var_1, unsigned_gen, unsigned_pair_gen_var_2,
};

#[test]
pub fn test_index_of_next_false_bit() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, start: u64, out: Option<u64>) {
        assert_eq!(x.index_of_next_false_bit(start), out);
    }
    test_unsigned(0xb00000000u64, 0, Some(0));
    test_unsigned(0xb00000000u64, 20, Some(20));
    test_unsigned(0xb00000000u64, 31, Some(31));
    test_unsigned(0xb00000000u64, 32, Some(34));
    test_unsigned(0xb00000000u64, 33, Some(34));
    test_unsigned(0xb00000000u64, 34, Some(34));
    test_unsigned(0xb00000000u64, 35, Some(36));
    test_unsigned(0xb00000000u64, 100, Some(100));

    test_unsigned(0xb00000000u128, 0, Some(0));
    test_unsigned(0xb00000000u128, 20, Some(20));
    test_unsigned(0xb00000000u128, 31, Some(31));
    test_unsigned(0xb00000000u128, 32, Some(34));
    test_unsigned(0xb00000000u128, 33, Some(34));
    test_unsigned(0xb00000000u128, 34, Some(34));
    test_unsigned(0xb00000000u128, 35, Some(36));
    test_unsigned(0xb00000000u128, 100, Some(100));

    fn test_signed<T: PrimitiveSigned>(x: T, start: u64, out: Option<u64>) {
        assert_eq!(x.index_of_next_false_bit(start), out);
    }
    test_signed(-0x500000000i64, 0, Some(0));
    test_signed(-0x500000000i64, 20, Some(20));
    test_signed(-0x500000000i64, 31, Some(31));
    test_signed(-0x500000000i64, 32, Some(34));
    test_signed(-0x500000000i64, 33, Some(34));
    test_signed(-0x500000000i64, 34, Some(34));
    test_signed(-0x500000000i64, 35, None);
    test_signed(-0x500000000i64, 100, None);

    test_signed(-0x500000000i128, 0, Some(0));
    test_signed(-0x500000000i128, 20, Some(20));
    test_signed(-0x500000000i128, 31, Some(31));
    test_signed(-0x500000000i128, 32, Some(34));
    test_signed(-0x500000000i128, 33, Some(34));
    test_signed(-0x500000000i128, 34, Some(34));
    test_signed(-0x500000000i128, 35, None);
    test_signed(-0x500000000i128, 100, None);
}

fn properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, u)| {
        let result = n.index_of_next_false_bit(u).unwrap();
        assert!(result >= u);
        assert!(!n.get_bit(result));
        assert_eq!(result == u, !n.get_bit(u));
        assert_eq!(
            (!n).index_of_next_true_bit(u),
            if result < T::WIDTH {
                Some(result)
            } else {
                None
            }
        );
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(
            n.index_of_next_false_bit(0),
            Some(TrailingZeros::trailing_zeros(!n))
        );
    });

    unsigned_gen::<u64>().test_properties(|u| {
        assert_eq!(T::ZERO.index_of_next_false_bit(u), Some(u));
    });
}

fn properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(n, u)| {
        let result = n.index_of_next_false_bit(u);
        assert_eq!(
            result.is_some(),
            if u >= T::WIDTH {
                n >= T::ZERO
            } else {
                n | T::low_mask(u) != T::NEGATIVE_ONE
            }
        );
        if let Some(result) = result {
            assert!(result >= u);
            assert!(!n.get_bit(result));
            assert_eq!(result == u, !n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_true_bit(u), result);
    });

    signed_gen_var_3::<T>().test_properties(|n| {
        assert_eq!(
            n.index_of_next_false_bit(0),
            Some(TrailingZeros::trailing_zeros(!n))
        );
    });

    unsigned_gen::<u64>().test_properties(|u| {
        assert_eq!(T::ZERO.index_of_next_false_bit(u), Some(u));
        assert_eq!(T::NEGATIVE_ONE.index_of_next_false_bit(u), None);
    });
}

#[test]
fn index_of_next_false_bit_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_signeds!(properties_helper_signed);
}
