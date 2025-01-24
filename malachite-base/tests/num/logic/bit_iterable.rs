// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitIterable, SignificantBits};
use malachite_base::test_util::common::test_double_ended_iterator_size_hint;
use malachite_base::test_util::generators::{
    signed_bool_vec_pair_gen_var_1, signed_gen, signed_unsigned_pair_gen_var_1,
    unsigned_bool_vec_pair_gen_var_1, unsigned_gen, unsigned_gen_var_5, unsigned_pair_gen_var_2,
};
use std::cmp::Ordering::*;
use std::ops::Index;

#[test]
pub fn test_bits() {
    let mut bits = 105u8.bits();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], false);
    assert_eq!(bits[2], false);
    assert_eq!(bits[3], true);
    assert_eq!(bits[4], false);
    assert_eq!(bits[5], true);
    assert_eq!(bits[6], true);
    assert_eq!(bits[7], false);
    assert_eq!(bits[8], false);

    let mut bits = 105u32.bits();
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    let mut bits = (-105i8).bits();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], true);
    assert_eq!(bits[2], true);
    assert_eq!(bits[3], false);
    assert_eq!(bits[4], true);
    assert_eq!(bits[5], false);
    assert_eq!(bits[6], false);
    assert_eq!(bits[7], true);
    assert_eq!(bits[8], true);

    let mut bits = (-105i32).bits();
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);
}

fn bits_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    T::BitIterator: Clone + Index<u64, Output = bool>,
{
    unsigned_gen::<T>().test_properties(|n| {
        test_double_ended_iterator_size_hint(n.bits(), usize::exact_from(n.significant_bits()));
    });

    unsigned_bool_vec_pair_gen_var_1::<T>().test_properties(|(n, bs)| {
        let mut bits = n.bits();
        let mut bit_vec = Vec::new();
        let mut i = 0;
        for b in bs {
            if b {
                bit_vec.insert(i, bits.next().unwrap());
                i += 1;
            } else {
                bit_vec.insert(i, bits.next_back().unwrap());
            }
        }
        assert!(bits.next().is_none());
        assert!(bits.next_back().is_none());
        assert_eq!(n.to_bits_asc(), bit_vec);
    });

    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[usize::exact_from(u)]);
        } else {
            assert_eq!(n.bits()[u], false);
        }
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(T::ZERO.bits()[u], false);
    });
}

fn bits_properties_helper_signed<T: PrimitiveSigned>()
where
    T::BitIterator: Clone + Index<u64, Output = bool>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned + WrappingFrom<T>,
{
    signed_gen::<T>().test_properties(|i| {
        let unsigned = <T as UnsignedAbs>::Output::wrapping_from(i);
        let significant_bits = match i.sign() {
            Equal => 0,
            Greater => unsigned.significant_bits() + 1,
            Less => (!unsigned).significant_bits() + 1,
        };
        test_double_ended_iterator_size_hint(i.bits(), usize::exact_from(significant_bits));
    });

    signed_bool_vec_pair_gen_var_1::<T>().test_properties(|(n, bs)| {
        let mut bits = n.bits();
        let mut bit_vec = Vec::new();
        let mut i = 0;
        for b in bs {
            if b {
                bit_vec.insert(i, bits.next().unwrap());
                i += 1;
            } else {
                bit_vec.insert(i, bits.next_back().unwrap());
            }
        }
        assert!(bits.next().is_none());
        assert!(bits.next_back().is_none());
        assert_eq!(n.to_bits_asc(), bit_vec);
    });

    signed_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[usize::exact_from(u)]);
        } else {
            assert_eq!(n.bits()[u], n < T::ZERO);
        }
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(T::ZERO.bits()[u], false);
    });
}

#[test]
fn bits_properties() {
    apply_fn_to_unsigneds!(bits_properties_helper_unsigned);
    apply_fn_to_signeds!(bits_properties_helper_signed);
}
