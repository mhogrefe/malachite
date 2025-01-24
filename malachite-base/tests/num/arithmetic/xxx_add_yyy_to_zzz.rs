// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_nonuple_gen_var_1, unsigned_sextuple_gen_var_1, unsigned_triple_gen_var_19,
};

#[test]
#[allow(clippy::too_many_arguments)]
fn test_xxx_add_yyy_to_zzz() {
    fn test<T: PrimitiveUnsigned>(
        x_2: T,
        x_1: T,
        x_0: T,
        y_2: T,
        y_1: T,
        y_0: T,
        z_2: T,
        z_1: T,
        z_0: T,
    ) {
        assert_eq!(
            T::xxx_add_yyy_to_zzz(x_2, x_1, x_0, y_2, y_1, y_0),
            (z_2, z_1, z_0)
        );
    }
    test::<u32>(0, 0, 0, 0, 0, 0, 0, 0, 0);
    test::<u64>(0x12, 0x34, 0x56, 0x33, 0x33, 0x33, 0x45, 0x67, 0x89);
    test::<u8>(0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc, 0x57, 0x99, 0x98);
    test::<u8>(u8::MAX, u8::MAX, u8::MAX, 0, 0, 1, 0, 0, 0);
    test(
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX - 1,
    );
}

fn xxx_add_yyy_to_zzz_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_sextuple_gen_var_1::<T>().test_properties(|(x_2, x_1, x_0, y_2, y_1, y_0)| {
        let (z_2, z_1, z_0) = T::xxx_add_yyy_to_zzz(x_2, x_1, x_0, y_2, y_1, y_0);

        assert_eq!(
            T::xxx_sub_yyy_to_zzz(z_2, z_1, z_0, y_2, y_1, y_0),
            (x_2, x_1, x_0)
        );
        assert_eq!(
            T::xxx_sub_yyy_to_zzz(z_2, z_1, z_0, x_2, x_1, x_0),
            (y_2, y_1, y_0)
        );
        assert_eq!(
            T::xxx_add_yyy_to_zzz(y_2, y_1, y_0, x_2, x_1, x_0),
            (z_2, z_1, z_0)
        );

        let (neg_y_2, neg_y_1, neg_y_0) =
            T::xxx_sub_yyy_to_zzz(T::ZERO, T::ZERO, T::ZERO, y_2, y_1, y_0);
        assert_eq!(
            T::xxx_sub_yyy_to_zzz(x_2, x_1, x_0, neg_y_2, neg_y_1, neg_y_0),
            (z_2, z_1, z_0)
        );
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x_2, x_1, x_0)| {
        assert_eq!(
            T::xxx_add_yyy_to_zzz(x_2, x_1, x_0, T::ZERO, T::ZERO, T::ZERO),
            (x_2, x_1, x_0)
        );
        assert_eq!(
            T::xxx_add_yyy_to_zzz(T::ZERO, T::ZERO, T::ZERO, x_2, x_1, x_0),
            (x_2, x_1, x_0)
        );

        let (neg_x_2, neg_x_1, neg_x_0) =
            T::xxx_sub_yyy_to_zzz(T::ZERO, T::ZERO, T::ZERO, x_2, x_1, x_0);
        assert_eq!(
            T::xxx_add_yyy_to_zzz(x_2, x_1, x_0, neg_x_2, neg_x_1, neg_x_0),
            (T::ZERO, T::ZERO, T::ZERO)
        );
    });

    unsigned_nonuple_gen_var_1::<T>().test_properties(
        |(x_2, x_1, x_0, y_2, y_1, y_0, z_2, z_1, z_0)| {
            let (sum_1_2, sum_1_1, sum_1_0) = T::xxx_add_yyy_to_zzz(x_2, x_1, x_0, y_2, y_1, y_0);
            let (sum_2_2, sum_2_1, sum_2_0) = T::xxx_add_yyy_to_zzz(y_2, y_1, y_0, z_2, z_1, z_0);
            assert_eq!(
                T::xxx_add_yyy_to_zzz(sum_1_2, sum_1_1, sum_1_0, z_2, z_1, z_0),
                T::xxx_add_yyy_to_zzz(x_2, x_1, x_0, sum_2_2, sum_2_1, sum_2_0)
            );
        },
    );
}

#[test]
fn xxx_add_yyy_to_zzz_properties() {
    apply_fn_to_unsigneds!(xxx_add_yyy_to_zzz_properties_helper);
}
