// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::gcd::{gcd_binary, gcd_euclidean, gcd_fast_a, gcd_fast_b};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_pair_gen_var_27, unsigned_triple_gen_var_19,
};

#[test]
fn test_gcd() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, out: T) {
        assert_eq!(x.gcd(y), out);

        let mut x = x;
        x.gcd_assign(y);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, 0);
    test::<u16>(0, 6, 6);
    test::<u32>(6, 0, 6);
    test::<u64>(1, 6, 1);
    test::<u128>(6, 1, 1);
    test::<usize>(8, 12, 4);
    test::<u8>(54, 24, 6);
    test::<u16>(42, 56, 14);
    test::<u32>(48, 18, 6);
    test::<u64>(3, 5, 1);
    test::<u128>(12, 60, 12);
    test::<usize>(12, 90, 6);
}

fn gcd_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let gcd = x.gcd(y);
        assert_eq!(gcd_euclidean(x, y), gcd);
        assert_eq!(gcd_binary(x, y), gcd);
        assert_eq!(gcd_fast_a(x, y), gcd);
        assert_eq!(gcd_fast_b(x, y), gcd);

        let mut x_mut = x;
        x_mut.gcd_assign(y);
        assert_eq!(x_mut, gcd);

        assert_eq!(y.gcd(x), gcd);
        assert!(x.divisible_by(gcd));
        assert!(y.divisible_by(gcd));
        if gcd != T::ZERO {
            assert!((x.div_exact(gcd)).coprime_with(y.div_exact(gcd)));
        }
        assert_eq!(gcd == T::ZERO, x == T::ZERO && y == T::ZERO);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.gcd(x), x);
        assert_eq!(x.gcd(T::ONE), T::ONE);
        assert_eq!(x.gcd(T::ZERO), x);
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        assert_eq!(x.gcd(y).gcd(z), x.gcd(y.gcd(z)));
    });
}

#[test]
fn gcd_properties() {
    apply_fn_to_unsigneds!(gcd_properties_helper);
}
