// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::oracle_row;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational::number_theory::bernoulli_number::{
    bernoulli_numbers_prefix, even_bernoulli_numbers_multi_mod, even_bernoulli_numbers_prefix,
    even_bernoulli_numbers_recursive,
};
use malachite_q::test_util::rational::number_theory::bernoulli_number::*;

#[test]
fn test_bernoulli_number() {
    let test = |n, out| {
        oracle_row!(
            "azurite",
            "az_rat_bernoulli_number",
            "bernoulli_number({n}) = {out}"
        );
        oracle_row!(
            "flint",
            "arith_bernoulli_number",
            "bernoulli_number({n}) = {out}"
        );

        assert_eq!(Rational::bernoulli_number(n).to_string(), out);
    };
    // - n == 0
    test(0, "1");
    // - n == 1
    test(1, "-1/2");
    // - n < BERNOULLI_SMALL_NUMERATOR_LIMIT
    test(2, "1/6");
    // - n.odd()
    test(3, "0");
    test(4, "-1/30");
    test(6, "1/42");
    test(12, "-691/2730");
    test(32, "-7709321041217/510");
    test(34, "2577687858367/6");
    test(35, "0");
    // - n >= BERNOULLI_SMALL_NUMERATOR_LIMIT
    // - in bernoulli_number_size: n > 14
    test(36, "-26315271553053477373/1919190");
    test(38, "2929993913841559/6");
    test(
        100,
        "-94598037819122125295227433069493721872702841533066936133385696204311395415197247711/3333\
        0",
    );
    test(
        1000,
        "-1824310473866188725457264025685778887933833686704290605219715815764112625726249111586574\
        725773210697096154899246274955229080874882995394551887918567582241551668492697244184914012\
        242579830955617098629924652251740979191563722636142834278054897100228104546530844116137235\
        069692022011624417917606802626020196202602557900584165392713328528060009666284676390683434\
        226380702951226108116666172815817157023611889303668166839919156379768387784569011484312275\
        342742688059179988378025533827866457866021850458959626704420114436303214602594867646743124\
        369948560543017655574251371150213401051058408679874766352952749178734973676859834707623881\
        634625147148994251287819057432353129907040693030947738925173870541768065311836481894518927\
        257264459495897596007053347675853897699248576309729639976364832442643512622073858780110731\
        539833099817555775136008111170797625059732295130888490067011333916764195379399451237761030\
        619842931093312146321416835426077466412320898548150646291295965369973806082564288019784909\
        897301658268809203555030692846151917069465607257641149187197651090551596684031241184554365\
        059302140284922169134185281979123358930199410122917734417940274935746518810594322744943540\
        922319548942807420684727146192942133436054611475404867886313250114399681532753236429290625\
        909341100039136833631213891562170153595481408420879424166549229427077334760558784157659275\
        820142147265848222364436913143660975700854733545840009985915190584047337934331297339403392\
        719579093995842312746836871169674978646091341187252716699004712622210934593384735892423095\
        171837988374325634654876041703160774187542427100652698181905912716906954466338361203745255\
        515267088218383996330164203403732365333352120338272021319718003599422045899487646001835027\
        03856341178077687451616229338340631455056219106004731529642292049578901/342999030",
    );
}

#[test]
fn test_bernoulli_numbers_prefix() {
    // - in even_bernoulli_numbers: len < BERNOULLI_RECURSIVE_LIMIT
    // - in ramanujan_even_common_denominator: start >= len
    assert!(bernoulli_numbers_prefix(0).is_empty());
    assert_eq!(
        bernoulli_numbers_prefix(10)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["1", "-1/2", "1/6", "0", "-1/30", "0", "1/42", "0", "-1/30", "0"]
    );
    // in even_bernoulli_numbers: len >= BERNOULLI_RECURSIVE_LIMIT
    assert_eq!(
        bernoulli_numbers_prefix(760),
        bernoulli_numbers_prefix_tangent(760)
    );
    #[cfg(feature = "oracle")]
    for n in [0, 10, 760] {
        malachite_base::oracle_row!(
            "azurite",
            "az_rat_bernoulli_numbers_prefix",
            "bernoulli_numbers_prefix({n}) = {:?}",
            bernoulli_numbers_prefix(n)
        );
        malachite_base::oracle_row!(
            "flint",
            "arith_bernoulli_number_vec",
            "bernoulli_numbers_prefix({n}) = {:?}",
            bernoulli_numbers_prefix(n)
        );
    }
}

#[test]
fn test_even_bernoulli_numbers_prefix() {
    assert!(even_bernoulli_numbers_prefix(0).is_empty());
    assert_eq!(
        even_bernoulli_numbers_prefix(5)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["1/6", "-1/30", "1/42", "-1/30", "5/66"]
    );
    assert_eq!(
        even_bernoulli_numbers_prefix(400),
        even_bernoulli_numbers_prefix_tangent(400)
    );
    #[cfg(feature = "oracle")]
    for n in [0, 5, 400] {
        malachite_base::oracle_row!(
            "azurite",
            "az_rat_even_bernoulli_numbers_prefix",
            "even_bernoulli_numbers_prefix({n}) = {:?}",
            even_bernoulli_numbers_prefix(n)
        );
        malachite_base::oracle_row!(
            "flint",
            "arith_bernoulli_number_vec",
            "even_bernoulli_numbers_prefix({n}) = {:?}",
            even_bernoulli_numbers_prefix(n)
        );
    }
}

#[test]
fn test_even_bernoulli_numbers_algorithms() {
    // - in even_bernoulli_numbers_multi_mod: len == 0
    // - in even_bernoulli_numbers_multi_mod, bernoulli_number_size: n <= 14 (len = 1)
    // - in ramanujan_even_common_denominator: start < len (len = 19)
    // - in ramanujan_even_common_denominator: m % 6 == 4 (len = 21)
    // - the longest prefix that Ramanujan's recursion computes (len = 722)
    for len in [0, 1, 2, 17, 18, 19, 21, 349, 350, 351, 722] {
        assert_eq!(
            even_bernoulli_numbers_recursive(len),
            even_bernoulli_numbers_multi_mod(len),
            "{len}"
        );
    }
}

#[test]
#[should_panic]
fn even_bernoulli_numbers_recursive_fail() {
    even_bernoulli_numbers_recursive(723);
}

#[test]
fn bernoulli_number_properties() {
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        let b = Rational::bernoulli_number(n);
        assert!(b.is_valid());
        assert_eq!(b, bernoulli_number_tangent(n));
        assert_eq!(
            *b.denominator_ref(),
            Natural::bernoulli_number_denominator(n)
        );
        if n.odd() && n != 1 {
            assert_eq!(b, 0u32);
        } else if n >= 2 {
            // the sign of B_n is (-1)^(n / 2 + 1)
            assert_eq!(b > 0u32, n % 4 == 2);
        }
    });
}

#[test]
fn bernoulli_numbers_prefix_properties() {
    unsigned_gen_var_5::<u8>().test_properties(|len| {
        let len = u64::from(len);
        let prefix = bernoulli_numbers_prefix(len);
        assert_eq!(u64::exact_from(prefix.len()), len);
        assert_eq!(prefix, bernoulli_numbers_prefix_tangent(len));
        if let Some(last) = prefix.last() {
            assert_eq!(*last, Rational::bernoulli_number(len - 1));
        }
        let evens = even_bernoulli_numbers_prefix(len.saturating_sub(1) >> 1);
        assert!(prefix.iter().skip(2).step_by(2).eq(evens.iter()));
        let len = usize::exact_from(len);
        assert_eq!(
            even_bernoulli_numbers_recursive(len),
            even_bernoulli_numbers_multi_mod(len)
        );
    });
}

#[test]
fn even_bernoulli_numbers_prefix_properties() {
    unsigned_gen_var_5::<u8>().test_properties(|len| {
        let len = u64::from(len);
        let evens = even_bernoulli_numbers_prefix(len);
        assert_eq!(u64::exact_from(evens.len()), len);
        assert_eq!(evens, even_bernoulli_numbers_prefix_tangent(len));
        for (b, n) in evens.iter().zip((2u64..).step_by(2)) {
            assert_eq!(*b, Rational::bernoulli_number(n));
        }
    });
}
