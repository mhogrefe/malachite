// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_duodecuple_gen_var_1, unsigned_octuple_gen_var_1, unsigned_quadruple_gen_var_10,
};

#[test]
#[allow(clippy::too_many_arguments)]
fn test_xxxx_add_yyyy_to_zzzz() {
    fn test<T: PrimitiveUnsigned>(
        x_3: T,
        x_2: T,
        x_1: T,
        x_0: T,
        y_3: T,
        y_2: T,
        y_1: T,
        y_0: T,
        z_3: T,
        z_2: T,
        z_1: T,
        z_0: T,
    ) {
        assert_eq!(
            T::xxxx_add_yyyy_to_zzzz(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0),
            (z_3, z_2, z_1, z_0)
        );
    }
    test::<u32>(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    test::<u64>(
        0x12, 0x34, 0x56, 0x78, 0x33, 0x33, 0x33, 0x33, 0x45, 0x67, 0x89, 0xab,
    );
    test::<u8>(
        0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc, 0xba, 0x98, 0x77, 0x77, 0x77, 0x76,
    );
    test::<u8>(u8::MAX, u8::MAX, u8::MAX, u8::MAX, 0, 0, 0, 1, 0, 0, 0, 0);
    test(
        u16::MAX,
        u16::MAX,
        u16::MAX,
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

fn xxxx_sub_yyyy_to_zzzz<T: PrimitiveUnsigned>(
    x_3: T,
    x_2: T,
    x_1: T,
    x_0: T,
    y_3: T,
    y_2: T,
    y_1: T,
    y_0: T,
) -> (T, T, T, T) {
    let (z_0, borrow_1) = x_0.overflowing_sub(y_0);
    let (mut z_1, mut borrow_2) = x_1.overflowing_sub(y_1);
    if borrow_1 {
        borrow_2 |= z_1.overflowing_sub_assign(T::ONE);
    }
    let (mut z_2, mut borrow_3) = x_2.overflowing_sub(y_2);
    if borrow_2 {
        borrow_3 |= z_2.overflowing_sub_assign(T::ONE);
    }
    let mut z_3 = x_3.wrapping_sub(y_3);
    if borrow_3 {
        z_3.wrapping_sub_assign(T::ONE);
    }
    (z_3, z_2, z_1, z_0)
}

fn xxxx_add_yyyy_to_zzzz_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_octuple_gen_var_1::<T>().test_properties(
        |(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0)| {
            let (z_3, z_2, z_1, z_0) =
                T::xxxx_add_yyyy_to_zzzz(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0);

            assert_eq!(
                xxxx_sub_yyyy_to_zzzz(z_3, z_2, z_1, z_0, y_3, y_2, y_1, y_0),
                (x_3, x_2, x_1, x_0)
            );
            assert_eq!(
                xxxx_sub_yyyy_to_zzzz(z_3, z_2, z_1, z_0, x_3, x_2, x_1, x_0),
                (y_3, y_2, y_1, y_0)
            );
            assert_eq!(
                T::xxxx_add_yyyy_to_zzzz(y_3, y_2, y_1, y_0, x_3, x_2, x_1, x_0),
                (z_3, z_2, z_1, z_0)
            );

            let (neg_y_3, neg_y_2, neg_y_1, neg_y_0) =
                xxxx_sub_yyyy_to_zzzz(T::ZERO, T::ZERO, T::ZERO, T::ZERO, y_3, y_2, y_1, y_0);
            assert_eq!(
                xxxx_sub_yyyy_to_zzzz(x_3, x_2, x_1, x_0, neg_y_3, neg_y_2, neg_y_1, neg_y_0),
                (z_3, z_2, z_1, z_0)
            );
        },
    );

    unsigned_quadruple_gen_var_10::<T>().test_properties(|(x_3, x_2, x_1, x_0)| {
        assert_eq!(
            T::xxxx_add_yyyy_to_zzzz(x_3, x_2, x_1, x_0, T::ZERO, T::ZERO, T::ZERO, T::ZERO),
            (x_3, x_2, x_1, x_0)
        );
        assert_eq!(
            T::xxxx_add_yyyy_to_zzzz(T::ZERO, T::ZERO, T::ZERO, T::ZERO, x_3, x_2, x_1, x_0),
            (x_3, x_2, x_1, x_0)
        );

        let (neg_x_3, neg_x_2, neg_x_1, neg_x_0) =
            xxxx_sub_yyyy_to_zzzz(T::ZERO, T::ZERO, T::ZERO, T::ZERO, x_3, x_2, x_1, x_0);
        assert_eq!(
            T::xxxx_add_yyyy_to_zzzz(x_3, x_2, x_1, x_0, neg_x_3, neg_x_2, neg_x_1, neg_x_0),
            (T::ZERO, T::ZERO, T::ZERO, T::ZERO)
        );
    });

    unsigned_duodecuple_gen_var_1::<T>().test_properties(
        |(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0, z_3, z_2, z_1, z_0)| {
            let (sum_1_3, sum_1_2, sum_1_1, sum_1_0) =
                T::xxxx_add_yyyy_to_zzzz(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0);
            let (sum_2_3, sum_2_2, sum_2_1, sum_2_0) =
                T::xxxx_add_yyyy_to_zzzz(y_3, y_2, y_1, y_0, z_3, z_2, z_1, z_0);
            assert_eq!(
                T::xxxx_add_yyyy_to_zzzz(sum_1_3, sum_1_2, sum_1_1, sum_1_0, z_3, z_2, z_1, z_0),
                T::xxxx_add_yyyy_to_zzzz(x_3, x_2, x_1, x_0, sum_2_3, sum_2_2, sum_2_1, sum_2_0)
            );
        },
    );
}

#[test]
fn xxxx_add_yyyy_to_zzzz_properties() {
    apply_fn_to_unsigneds!(xxxx_add_yyyy_to_zzzz_properties_helper);
}
