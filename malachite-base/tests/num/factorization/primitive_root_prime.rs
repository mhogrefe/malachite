// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::traits::{IsPrime, PrimitiveRootPrime};
use malachite_base::test_util::generators::unsigned_gen_var_29;
use std::fmt::Debug;
use std::panic::catch_unwind;

fn primitive_root_prime_helper<T: PrimitiveRootPrime<Output = T> + PrimitiveUnsigned>() {
    let test = |n: u64, out| {
        if let (Ok(n), Ok(out)) = (T::try_from(n), T::try_from(out)) {
            assert_eq!(n.primitive_root_prime(), out);
        }
    };
    test(2, 1);
    test(3, 2);
    test(5, 2);
    test(7, 3);
    test(11, 2);
    test(191, 19);

    test(9223372036854775807, 2);
    test(8760810010780182161, 3);
}

#[test]
fn test_primitive_root_prime() {
    primitive_root_prime_helper::<u8>();
    primitive_root_prime_helper::<u16>();
    primitive_root_prime_helper::<u32>();
    primitive_root_prime_helper::<u64>();
    primitive_root_prime_helper::<usize>();
}

fn primitive_root_prime_fail_helper<T: PrimitiveRootPrime + PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.primitive_root_prime());
}

#[test]
pub fn primitive_root_prime_fail() {
    primitive_root_prime_fail_helper::<u8>();
    primitive_root_prime_fail_helper::<u16>();
    primitive_root_prime_fail_helper::<u32>();
    primitive_root_prime_fail_helper::<u64>();
    primitive_root_prime_fail_helper::<usize>();
}

fn primitive_root_prime_properties_helper<T: IsPrime + PrimitiveRootPrime + PrimitiveUnsigned>()
where
    <T as PrimitiveRootPrime>::Output: Copy + Debug + PartialOrd<T>,
{
    unsigned_gen_var_29::<T>().test_properties(|n| {
        let primitive_root = n.primitive_root_prime();
        assert_ne!(primitive_root, T::ZERO);
        if n > T::TWO {
            assert_ne!(primitive_root, T::ONE);
        }
        assert!(primitive_root < n);
    });
}

#[test]
fn primitive_root_prime_properties() {
    primitive_root_prime_properties_helper::<u8>();
    primitive_root_prime_properties_helper::<u16>();
    primitive_root_prime_properties_helper::<u32>();
    primitive_root_prime_properties_helper::<u64>();
    primitive_root_prime_properties_helper::<usize>();
}
