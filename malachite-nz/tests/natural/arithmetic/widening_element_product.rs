// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Product;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ElementProduct, WideningElementProduct};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;

#[test]
fn test_widening_element_product() {
    fn test<T: PrimitiveUnsigned>(s: &str, out: &str)
    where
        UnsignedVector<T>: WideningElementProduct<Natural>,
    {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let product: Natural = v.to_widening_element_product();
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);
        let product_alt: Natural = v.into_widening_element_product();
        assert_eq!(product_alt, product);
    }
    test::<u8>("()", "1");
    test::<u8>("(255, 255)", "65025");
    test::<u8>("(2, 0, 7)", "0");
    test::<u64>("(4294967296, 4294967296, 3)", "55340232221128654848");
    test::<u64>(
        "(18446744073709551615, 18446744073709551615)",
        "340282366920938463426481119284349108225",
    );
    test::<u128>(
        "(340282366920938463463374607431768211455, 2)",
        "680564733841876926926749214863536422910",
    );
}

fn widening_element_product_properties_helper<T: PrimitiveUnsigned>()
where
    UnsignedVector<T>: WideningElementProduct<Natural>,
    NaturalVector: From<UnsignedVector<T>>,
    Natural: for<'a> Product<&'a T>,
{
    unsigned_vec_gen::<T>().test_properties(|xs| {
        let product = Natural::product(xs.iter());
        let v = UnsignedVector::from_owned_elements(xs);
        let product_alt: Natural = v.to_widening_element_product();
        assert_eq!(product_alt, product);
        // It agrees with the NaturalVector implementation of `ElementProduct`.
        assert_eq!(
            NaturalVector::from(v.clone()).into_element_product(),
            product
        );
        let product_alt: Natural = v.into_widening_element_product();
        assert_eq!(product_alt, product);
    });
}

#[test]
fn widening_element_product_properties() {
    apply_fn_to_unsigneds!(widening_element_product_properties_helper);
}
