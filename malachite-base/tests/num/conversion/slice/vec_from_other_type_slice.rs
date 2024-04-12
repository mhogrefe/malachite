// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegModPowerOf2;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use malachite_base::slices::slice_test_zero;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_gen;
use std::fmt::Debug;

#[test]
pub fn test_vec_from_other_type_slice() {
    fn test<T: Debug + Eq, U: Debug + Eq + VecFromOtherTypeSlice<T>>(slice: &[T], vec: &[U]) {
        assert_eq!(U::vec_from_other_type_slice(slice), vec);
    }
    test::<u32, u32>(&[123, 456], &[123, 456]);
    test::<u8, u16>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xff],
        &[0xcdab, 0x01ef, 0x4523, 0x8967, 0xff],
    );
    test::<u8, u16>(&[0xab], &[0xab]);
    test::<u16, u8>(
        &[0xcdab, 0x01ef, 0x4523, 0x8967],
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89],
    );
}

fn vec_from_other_type_slice_helper<
    T: PrimitiveUnsigned + VecFromOtherTypeSlice<U>,
    U: PrimitiveUnsigned + VecFromOtherTypeSlice<T>,
>() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << U::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_gen::<U>().test_properties_with_config(&config, |xs| {
        let ys = T::vec_from_other_type_slice(&xs);
        let xs_alt = U::vec_from_other_type_slice(&ys);
        if U::WIDTH >= T::WIDTH {
            assert_eq!(xs_alt, xs);
        } else {
            let number_of_extra_zeros = xs.len().neg_mod_power_of_2(T::LOG_WIDTH - U::LOG_WIDTH);
            let (xs_alt_lo, xs_alt_hi) = xs_alt.split_at(xs.len());
            assert_eq!(xs_alt_hi.len(), number_of_extra_zeros);
            assert_eq!(xs_alt_lo, xs);
            assert!(slice_test_zero(xs_alt_hi));
        }
    });
}

#[test]
fn vec_from_other_type_slice_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(vec_from_other_type_slice_helper);
}
