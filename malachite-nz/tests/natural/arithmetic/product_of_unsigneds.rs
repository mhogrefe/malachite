// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Product;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vec_pair_gen};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::arithmetic::mul::natural_product_of_unsigneds_naive;

#[test]
fn test_product_of_unsigneds() {
    fn test<T: PrimitiveUnsigned>(xs: &[T], out: &str)
    where
        Natural: From<T> + Product<T> + for<'a> Product<&'a T>,
    {
        let product = Natural::product(xs.iter());
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);
        assert_eq!(Natural::product(xs.iter().copied()), product);
        assert_eq!(natural_product_of_unsigneds_naive(xs), product);
    }
    test::<u8>(&[], "1");
    test::<u8>(&[5], "5");
    test::<u8>(&[2, 3, 7], "42");
    test::<u8>(&[2, 0, 7], "0");
    // - a single packed limb
    test::<u32>(&[65536, 65536], "4294967296");
    // - several packed limbs, through `limbs_product`
    test::<u8>(&[255; 9], "4558916353692287109375");
    test::<u64>(
        &[18446744073709551615, 18446744073709551615],
        "340282366920938463426481119284349108225",
    );
    test::<u64>(
        &[18446744073709551615, 2, 18446744073709551615, 3, 18446744073709551615],
        "37662610412320084576889653934669106154605431126339151200250",
    );
    // - factors wider than a limb, alone and mixed with narrow ones
    test::<u128>(
        &[340282366920938463463374607431768211455],
        "340282366920938463463374607431768211455",
    );
    test::<u128>(
        &[340282366920938463463374607431768211455, 2, 340282366920938463463374607431768211455, 3],
        "694752535423897172541425910052127447115536519590792122675185008758297559302150",
    );
    // - a zero after factors wider than a limb
    test::<u128>(&[340282366920938463463374607431768211455, 0], "0");
    test::<usize>(&[3, 5], "15");
}

fn product_of_unsigneds_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T> + Product<T> + for<'a> Product<&'a T>,
{
    unsigned_vec_gen::<T>().test_properties(|xs| {
        let product = Natural::product(xs.iter());
        assert!(product.is_valid());
        assert_eq!(Natural::product(xs.iter().copied()), product);
        assert_eq!(natural_product_of_unsigneds_naive(&xs), product);
        assert_eq!(
            Natural::product(xs.iter().map(|&x| Natural::from(x))),
            product
        );
        // The product is zero exactly when a factor is.
        assert_eq!(product == 0u32, xs.contains(&T::ZERO));
    });

    unsigned_vec_pair_gen::<T>().test_properties(|(xs, ys)| {
        // The product of a concatenation is the product of the two products.
        let mut zs = xs.clone();
        zs.extend_from_slice(&ys);
        assert_eq!(
            Natural::product(zs.iter()),
            Natural::product(xs.iter()) * Natural::product(ys.iter())
        );
    });
}

#[test]
fn product_of_unsigneds_properties() {
    apply_fn_to_unsigneds!(product_of_unsigneds_properties_helper);
}
