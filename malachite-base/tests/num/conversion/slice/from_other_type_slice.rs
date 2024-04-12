// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{FromOtherTypeSlice, VecFromOtherTypeSlice};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_gen;
use std::fmt::Debug;

#[test]
pub fn test_from_other_type_slice() {
    fn test<T: Debug + Eq, U: Copy + Debug + Eq + FromOtherTypeSlice<T>>(slice: &[T], n: U) {
        assert_eq!(U::from_other_type_slice(slice), n);
    }
    test::<u32, u32>(&[], 0);
    test::<u32, u32>(&[123], 123);
    test::<u32, u32>(&[123, 456], 123);

    test::<u8, u16>(&[0xab], 0xab);
    test::<u8, u16>(&[0xab, 0xcd], 0xcdab);
    test::<u8, u16>(&[0xab, 0xcd, 0xef], 0xcdab);
    test::<u8, u64>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67],
        0x67452301efcdab,
    );
    test::<u8, u64>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xff],
        0x8967452301efcdab,
    );

    test::<u64, u32>(&[], 0);
    test::<u16, u8>(&[0xabcd, 0xef01], 0xcd);
    test::<u128, u8>(&[0x1234567890abcdef012345678909bcde], 0xde);
}

fn from_other_type_slice_helper<
    T: PrimitiveUnsigned + FromOtherTypeSlice<U> + VecFromOtherTypeSlice<U>,
    U: PrimitiveUnsigned,
>() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << U::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_gen::<U>().test_properties_with_config(&config, |xs| {
        let value = T::from_other_type_slice(&xs);
        let ys = T::vec_from_other_type_slice(&xs);
        if xs.is_empty() {
            assert_eq!(value, T::ZERO);
        } else {
            assert_eq!(value, ys[0]);
        }
    });
}

#[test]
fn from_other_type_slice_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_other_type_slice_helper);
}
