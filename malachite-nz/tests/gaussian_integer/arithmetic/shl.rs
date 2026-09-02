// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_unsigned_pair_gen_var_1, integer_unsigned_pair_gen_var_2,
};
use std::ops::{Shl, ShlAssign};
use std::str::FromStr;

fn test_shl_unsigned_helper<T: PrimitiveUnsigned>()
where
    GaussianInteger: Shl<T, Output = GaussianInteger> + ShlAssign<T>,
    for<'a> &'a GaussianInteger: Shl<T, Output = GaussianInteger>,
{
    let test = |s, v: u8, out| {
        let u = GaussianInteger::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = u.clone() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = &u << v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("123", 1, "246");
    test("i", 1, "2i");
    test("-i", 3, "-8i");
    test("1+i", 1, "2+2i");
    test("3-2i", 3, "24-16i");
    test("-123+456i", 2, "-492+1824i");
    test("123+i", 25, "4127195136+33554432i");
    test("1000000000000-i", 32, "4294967296000000000000-4294967296i");
    test(
        "1000000000000+1000000000000i",
        100,
        "1267650600228229401496703205376000000000000+1267650600228229401496703205376000000000000i",
    );
}

#[test]
fn test_shl_unsigned() {
    apply_fn_to_unsigneds!(test_shl_unsigned_helper);
}

fn shl_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    GaussianInteger: Shl<T, Output = GaussianInteger> + ShlAssign<T>,
    for<'a> &'a GaussianInteger: Shl<T, Output = GaussianInteger>,
    Integer: Shl<T, Output = Integer>,
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    u64: TryFrom<T>,
{
    gaussian_integer_unsigned_pair_gen_var_1::<T>().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n <<= u;
        assert!(mut_n.real.is_valid());
        assert!(mut_n.imaginary.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n << u;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone() << u;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!(shifted.real, &n.real << u);
        assert_eq!(shifted.imaginary, &n.imaginary << u);
        assert!(shifted.ge_abs(&n));
        assert_eq!(-&n << u, -(&n << u));
        assert_eq!((&n).conjugate() << u, (&n << u).conjugate());
        assert_eq!(
            &n << u,
            &n * GaussianInteger::power_of_2(u64::exact_from(u))
        );
    });

    gaussian_integer_gen().test_properties(|n| {
        assert_eq!(&n << T::ZERO, n);
    });

    unsigned_gen_var_5::<T>().test_properties(|u| {
        assert_eq!(GaussianInteger::ZERO << u, GaussianInteger::ZERO);
    });

    integer_unsigned_pair_gen_var_2::<T>().test_properties(|(n, u)| {
        assert_eq!(
            GaussianInteger::from(n.clone()) << u,
            GaussianInteger::from(n << u)
        );
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper_unsigned);
}
