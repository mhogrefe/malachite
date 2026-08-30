// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Conjugate;
use malachite_base::num::basic::traits::Zero;
use malachite_base::vecs::vec_from_str;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::add::gaussian_integer_sum_alt;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_pair_gen, gaussian_integer_triple_gen,
    gaussian_integer_vec_gen, integer_vec_gen,
};
use std::iter::{Sum, once};
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |s, t, out: &str| {
        let x = GaussianInteger::from_str(s).unwrap();
        let y = GaussianInteger::from_str(t).unwrap();

        let result = x.clone() + y.clone();
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = x.clone() + &y;
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = &x + y.clone();
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = &x + &y;
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let mut result = x.clone();
        result += y.clone();
        assert_eq!(result.to_string(), out);

        let mut result = x;
        result += &y;
        assert_eq!(result.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "2-3i", "2-3i");
    test("2-3i", "0", "2-3i");
    test("1", "i", "1+i");
    test("2-3i", "-1+4i", "1+i");
    test("2-3i", "-2+3i", "0");
    test("1000000000000", "i", "1000000000000+i");
    test("123+456i", "-23-56i", "100+400i");
}

#[test]
fn add_properties() {
    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let sum = x.clone() + y.clone();
        assert!(sum.real.is_valid());
        assert!(sum.imaginary.is_valid());
        assert_eq!(x.clone() + &y, sum);
        assert_eq!(&x + y.clone(), sum);
        assert_eq!(&x + &y, sum);
        let mut sum_alt = x.clone();
        sum_alt += y.clone();
        assert_eq!(sum_alt, sum);
        let mut sum_alt = x.clone();
        sum_alt += &y;
        assert_eq!(sum_alt, sum);

        assert_eq!(sum.real, &x.real + &y.real);
        assert_eq!(sum.imaginary, &x.imaginary + &y.imaginary);
        assert_eq!(&y + &x, sum);
        assert_eq!(&sum - &y, x);
        assert_eq!(&sum - &x, y);
        assert_eq!(-&x + -&y, -&sum);
        assert_eq!((&x).conjugate() + (&y).conjugate(), (&sum).conjugate());
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(&x + GaussianInteger::ZERO, x);
        assert_eq!(GaussianInteger::ZERO + &x, x);
        assert_eq!(&x + -&x, GaussianInteger::ZERO);
    });

    gaussian_integer_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x + &y) + &z, &x + (&y + &z));
    });
}

#[test]
fn test_sum() {
    let test = |xs, out: &str| {
        let xs = vec_from_str::<GaussianInteger>(xs).unwrap();
        let sum = GaussianInteger::sum(xs.iter().cloned());
        assert!(sum.real.is_valid());
        assert!(sum.imaginary.is_valid());
        assert_eq!(sum.to_string(), out);

        let sum_alt = GaussianInteger::sum(xs.iter());
        assert!(sum_alt.real.is_valid());
        assert!(sum_alt.imaginary.is_valid());
        assert_eq!(sum_alt, sum);

        let sum_alt = gaussian_integer_sum_alt(xs.into_iter());
        assert!(sum_alt.real.is_valid());
        assert!(sum_alt.imaginary.is_valid());
        assert_eq!(sum_alt, sum);
    };
    test("[]", "0");
    test("[10]", "10");
    test("[i]", "i");
    test("[2-3i, -1+4i]", "1+i");
    test("[2, -3i, 5+i, 7-2i]", "14-4i");
    test("[1000000000000+i, -i, 234i]", "1000000000000+234i");
    test("[123+456i, -23-56i, 100+400i]", "200+800i");
}

#[test]
fn sum_properties() {
    gaussian_integer_vec_gen().test_properties(|xs| {
        let sum = GaussianInteger::sum(xs.iter().cloned());
        assert!(sum.real.is_valid());
        assert!(sum.imaginary.is_valid());

        let sum_alt = GaussianInteger::sum(xs.iter());
        assert!(sum_alt.real.is_valid());
        assert!(sum_alt.imaginary.is_valid());
        assert_eq!(sum_alt, sum);

        assert_eq!(sum.real, Integer::sum(xs.iter().map(|x| &x.real)));
        assert_eq!(sum.imaginary, Integer::sum(xs.iter().map(|x| &x.imaginary)));

        let sum_alt = gaussian_integer_sum_alt(xs.into_iter());
        assert!(sum_alt.real.is_valid());
        assert!(sum_alt.imaginary.is_valid());
        assert_eq!(sum_alt, sum);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(GaussianInteger::sum(once(&x)), x);
        assert_eq!(GaussianInteger::sum(once(x.clone())), x);
    });

    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let sum = &x + &y;
        assert_eq!(GaussianInteger::sum([&x, &y].into_iter()), sum);
        assert_eq!(GaussianInteger::sum([x, y].into_iter()), sum);
    });

    integer_vec_gen().test_properties(|xs| {
        assert_eq!(
            GaussianInteger::sum(xs.iter().cloned().map(GaussianInteger::from)),
            GaussianInteger::from(Integer::sum(xs.into_iter()))
        );
    });
}
