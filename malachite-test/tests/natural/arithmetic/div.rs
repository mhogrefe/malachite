use std::str::FromStr;

use malachite_base::num::arithmetic::traits::DivMod;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use common::{test_properties, test_properties_custom_scale};
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_1,
    positive_naturals,
};

#[test]
fn test_div() {
    let test = |u, v, quotient| {
        let mut x = Natural::from_str(u).unwrap();
        x /= Natural::from_str(v).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = Natural::from_str(u).unwrap();
        x /= &Natural::from_str(v).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let q = Natural::from_str(u).unwrap() / Natural::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = Natural::from_str(u).unwrap() / &Natural::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = &Natural::from_str(u).unwrap() / Natural::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = &Natural::from_str(u).unwrap() / &Natural::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = BigUint::from_str(u).unwrap() / &BigUint::from_str(v).unwrap();
        assert_eq!(q.to_string(), quotient);

        let q = rug::Integer::from_str(u).unwrap() / rug::Integer::from_str(v).unwrap();
        assert_eq!(q.to_string(), quotient);

        let q = Natural::from_str(u)
            .unwrap()
            .div_mod(Natural::from_str(v).unwrap())
            .0;
        assert_eq!(q.to_string(), quotient);
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("1", "1", "1");
    test("123", "1", "123");
    test("123", "123", "1");
    test("123", "456", "0");
    test("456", "123", "3");
    test("4294967295", "1", "4294967295");
    test("4294967295", "4294967295", "1");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "3", "333333333333");
    test("1000000000000", "123", "8130081300");
    test("1000000000000", "4294967295", "232");
    test(
        "1000000000000000000000000",
        "1",
        "1000000000000000000000000",
    );
    test("1000000000000000000000000", "3", "333333333333333333333333");
    test("1000000000000000000000000", "123", "8130081300813008130081");
    test("1000000000000000000000000", "4294967295", "232830643708079");
    test("1000000000000000000000000", "1234567890987", "810000006723");
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018654",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253979",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "94998781946290113",
    );
    test("3768477692975601", "11447376614057827956", "0");
    test("3356605361737854", "3081095617839357", "1");
    test("1098730198198174614195", "953382298040157850476", "1");
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "1",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "1",
    );
    test("0", "1000000000000000000000000", "0");
    test("123", "1000000000000000000000000", "0");
}

#[test]
#[should_panic]
fn div_assign_fail() {
    let mut x = Natural::from(10u32);
    x /= Natural::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail() {
    let mut x = Natural::from(10u32);
    x /= &Natural::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn div_fail() {
    Natural::from(10u32) / Natural::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn div_val_ref_fail() {
    Natural::from(10u32) / &Natural::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn div_ref_val_fail() {
    &Natural::from(10u32) / Natural::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn div_ref_ref_fail() {
    &Natural::from(10u32) / &Natural::ZERO;
}

fn div_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    mut_x /= y;
    assert!(mut_x.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    mut_x /= y.clone();
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x / y;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x / y.clone();
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.clone() / y;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.clone() / y.clone();
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.div_mod(y).0;
    assert_eq!(quotient_alt, quotient);

    let num_quotient = natural_to_biguint(x) / &natural_to_biguint(y);
    assert_eq!(biguint_to_natural(&num_quotient), quotient);

    let rug_quotient = natural_to_rug_integer(x) / natural_to_rug_integer(y);
    assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);

    let remainder = x - &quotient * y;
    assert!(remainder < *y);
    assert_eq!(quotient * y + remainder, *x);
}

#[test]
fn div_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural,
        |&(ref x, ref y)| {
            div_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            div_properties_helper(x, y);
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n / Natural::ONE, *n);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(n / n, Natural::ONE);
        assert_eq!(Natural::ZERO / n, Natural::ZERO);
        if *n > 1 as Limb {
            assert_eq!(Natural::ONE / n, Natural::ZERO);
        }
    });
}
