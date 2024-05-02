// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::coprime_with::{
    coprime_with_check_2, coprime_with_check_2_3, coprime_with_check_2_3_5,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_pair_gen_var_27};

#[test]
fn test_coprime_with() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, out: bool) {
        assert_eq!(x.coprime_with(y), out);
    }
    test::<u8>(0, 0, false);
    test::<u8>(0, 1, true);
    test::<u16>(0, 6, false);
    test::<u32>(6, 0, false);
    test::<u64>(1, 6, true);
    test::<u128>(6, 1, true);
    test::<usize>(8, 12, false);
    test::<u8>(54, 24, false);
    test::<u16>(42, 56, false);
    test::<u32>(48, 18, false);
    test::<u64>(3, 5, true);
    test::<u128>(12, 60, false);
    test::<usize>(12, 90, false);
    test::<usize>(25, 14, true);
}

fn coprime_with_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let c = x.coprime_with(y);
        assert_eq!(x.gcd(y) == T::ONE, c);
        assert_eq!(coprime_with_check_2(x, y), c);
        assert_eq!(coprime_with_check_2_3(x, y), c);
        assert_eq!(coprime_with_check_2_3_5(x, y), c);
        assert_eq!(y.coprime_with(x), c);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.coprime_with(x), x == T::ONE);
        assert!(x.coprime_with(T::ONE));
        assert_eq!(x.coprime_with(T::ZERO), x == T::ONE);
        if x != T::MAX {
            assert!(x.coprime_with(x + T::ONE));
        }
    });
}

#[test]
fn coprime_with_properties() {
    apply_fn_to_unsigneds!(coprime_with_properties_helper);
}
