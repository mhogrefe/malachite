use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    CeilingDivNegMod, DivMod, Mod, ModAssign, NegMod, NegModAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::{BigUint, Integer};
use rug;
use rug::ops::RemRounding;

use common::{test_properties, test_properties_custom_scale};
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_1,
    positive_naturals,
};
use malachite_test::natural::arithmetic::mod_op::rug_neg_mod;

#[test]
fn test_div_mod() {
    let test = |u, v, remainder| {
        let mut x = Natural::from_str(u).unwrap();
        x.mod_assign(Natural::from_str(v).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = Natural::from_str(u).unwrap();
        x.mod_assign(&Natural::from_str(v).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = Natural::from_str(u)
            .unwrap()
            .mod_op(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = Natural::from_str(u)
            .unwrap()
            .mod_op(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&Natural::from_str(u).unwrap()).mod_op(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&Natural::from_str(u).unwrap()).mod_op(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let mut x = Natural::from_str(u).unwrap();
        x %= Natural::from_str(v).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = Natural::from_str(u).unwrap();
        x %= &Natural::from_str(v).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = Natural::from_str(u).unwrap() % Natural::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = Natural::from_str(u).unwrap() % &Natural::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = &Natural::from_str(u).unwrap() % Natural::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = &Natural::from_str(u).unwrap() % &Natural::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = BigUint::from_str(u)
            .unwrap()
            .mod_floor(&BigUint::from_str(v).unwrap());
        assert_eq!(r.to_string(), remainder);

        let r = BigUint::from_str(u).unwrap() % &BigUint::from_str(v).unwrap();
        assert_eq!(r.to_string(), remainder);

        let r = rug::Integer::from_str(u)
            .unwrap()
            .rem_floor(rug::Integer::from_str(v).unwrap());
        assert_eq!(r.to_string(), remainder);

        let r = rug::Integer::from_str(u).unwrap() % rug::Integer::from_str(v).unwrap();
        assert_eq!(r.to_string(), remainder);

        assert_eq!(
            Natural::from_str(u)
                .unwrap()
                .div_mod(Natural::from_str(v).unwrap())
                .1
                .to_string(),
            remainder
        );
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("1", "1", "0");
    test("123", "1", "0");
    test("123", "123", "0");
    test("123", "456", "123");
    test("456", "123", "87");
    test("4294967295", "4294967295", "0");
    test("4294967295", "4294967295", "0");
    test("1000000000000", "1", "0");
    test("1000000000000", "3", "1");
    test("1000000000000", "123", "100");
    test("1000000000000", "4294967295", "3567587560");
    test("1000000000000000000000000", "1", "0");
    test("1000000000000000000000000", "3", "1");
    test("1000000000000000000000000", "123", "37");
    test("1000000000000000000000000", "4294967295", "3167723695");
    test("1000000000000000000000000", "1234567890987", "530068894399");
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "1234567890987654321234567890987654321",
        "779655053998040854338961591319296066",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "316049380092839506236049380092839506176",
        "37816691783627670491375998320948925696",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "1520301762334",
    );
    test(
        "3768477692975601",
        "11447376614057827956",
        "3768477692975601",
    );
    test("3356605361737854", "3081095617839357", "275509743898497");
    test(
        "1098730198198174614195",
        "953382298040157850476",
        "145347900158016763719",
    );
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "0",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "0",
    );
    test("0", "1000000000000000000000000", "0");
    test("123", "1000000000000000000000000", "123");
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    Natural::from(10u32).mod_assign(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_assign_ref_fail() {
    Natural::from(10u32).mod_assign(&Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_fail() {
    Natural::from(10u32).mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_val_ref_fail() {
    Natural::from(10u32).mod_op(&Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_ref_val_fail() {
    (&Natural::from(10u32)).mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_ref_ref_fail() {
    (&Natural::from(10u32)).mod_op(&Natural::ZERO);
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut n = Natural::from(10u32);
    n %= Natural::ZERO;
}

#[test]
#[should_panic]
fn div_rem_assign_ref_fail() {
    let mut n = Natural::from(10u32);
    n %= &Natural::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn rem_fail() {
    Natural::from(10u32) % Natural::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn rem_val_ref_fail() {
    Natural::from(10u32) % &Natural::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn rem_ref_val_fail() {
    &Natural::from(10u32) % Natural::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn rem_ref_ref_fail() {
    &Natural::from(10u32) % &Natural::ZERO;
}

#[test]
fn test_neg_mod() {
    let test = |u, v, remainder| {
        let mut x = Natural::from_str(u).unwrap();
        x.neg_mod_assign(Natural::from_str(v).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = Natural::from_str(u).unwrap();
        x.neg_mod_assign(&Natural::from_str(v).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = Natural::from_str(u)
            .unwrap()
            .neg_mod(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = Natural::from_str(u)
            .unwrap()
            .neg_mod(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&Natural::from_str(u).unwrap()).neg_mod(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&Natural::from_str(u).unwrap()).neg_mod(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = rug_neg_mod(
            rug::Integer::from_str(u).unwrap(),
            rug::Integer::from_str(v).unwrap(),
        );
        assert_eq!(r.to_string(), remainder);

        assert_eq!(
            Natural::from_str(u)
                .unwrap()
                .ceiling_div_neg_mod(Natural::from_str(v).unwrap())
                .1
                .to_string(),
            remainder
        );
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("1", "1", "0");
    test("123", "1", "0");
    test("123", "123", "0");
    test("123", "456", "333");
    test("456", "123", "36");
    test("4294967295", "1", "0");
    test("4294967295", "4294967295", "0");
    test("1000000000000", "1", "0");
    test("1000000000000", "3", "2");
    test("1000000000000", "123", "23");
    test("1000000000000", "4294967295", "727379735");
    test("1000000000000000000000000", "1", "0");
    test("1000000000000000000000000", "3", "2");
    test("1000000000000000000000000", "123", "86");
    test("1000000000000000000000000", "4294967295", "1127243600");
    test("1000000000000000000000000", "1234567890987", "704498996588");
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "1234567890987654321234567890987654321",
        "454912836989613466895606299668358255",
    );
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "316049380092839506236049380092839506176",
        "278232688309211835744673381771890580480",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "1149635115107",
    );
    test(
        "3768477692975601",
        "11447376614057827956",
        "11443608136364852355",
    );
    test("3356605361737854", "3081095617839357", "2805585873940860");
    test(
        "1098730198198174614195",
        "953382298040157850476",
        "808034397882141086757",
    );
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "0",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "0",
    );
    test("0", "1000000000000000000000000", "0");
    test(
        "123",
        "1000000000000000000000000",
        "999999999999999999999877",
    );
}

#[test]
#[should_panic]
fn neg_mod_assign_fail() {
    Natural::from(10u32).neg_mod_assign(Natural::ZERO);
}

#[test]
#[should_panic]
fn neg_mod_assign_ref_fail() {
    Natural::from(10u32).neg_mod_assign(&Natural::ZERO);
}

#[test]
#[should_panic]
fn neg_mod_fail() {
    Natural::from(10u32).neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn neg_mod_val_ref_fail() {
    Natural::from(10u32).neg_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn neg_mod_ref_val_fail() {
    (&Natural::from(10u32)).neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn neg_mod_ref_ref_fail() {
    (&Natural::from(10u32)).neg_mod(&Natural::ZERO);
}

fn mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    mut_x.mod_assign(y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.mod_op(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.mod_op(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().mod_op(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().mod_op(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let mut remainder_alt = x.clone();
    remainder_alt %= y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let mut remainder_alt = x.clone();
    remainder_alt %= y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x % y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x % y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone() % y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone() % y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.div_mod(y).1;
    assert_eq!(remainder_alt, remainder);

    let num_remainder = natural_to_biguint(x).mod_floor(&natural_to_biguint(y));
    assert_eq!(biguint_to_natural(&num_remainder), remainder);

    let num_remainder = natural_to_biguint(x) % &natural_to_biguint(y);
    assert_eq!(biguint_to_natural(&num_remainder), remainder);

    let rug_remainder = natural_to_rug_integer(x).rem_floor(natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    let rug_remainder = natural_to_rug_integer(x) % natural_to_rug_integer(y);
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    assert!(remainder < *y);
}

#[test]
fn div_mod_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural,
        |&(ref x, ref y)| {
            mod_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            mod_properties_helper(x, y);
        },
    );

    test_properties(positive_naturals, |n| {
        assert_eq!(n % Natural::ONE, 0 as Limb);
        assert_eq!(n % n, 0 as Limb);
        assert_eq!(Natural::ZERO % n, 0 as Limb);
        if *n > 1 as Limb {
            assert_eq!(Natural::ONE % n, 1 as Limb);
        }
    });
}

fn neg_mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    mut_x.neg_mod_assign(y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.neg_mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.neg_mod(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.neg_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().neg_mod(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().neg_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    //TODO
    /*
    let (quotient_alt, remainder_alt) = (x.div_round(y, RoundingMode::Ceiling), x.neg_mod(y));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);
    */

    let rug_remainder = rug_neg_mod(natural_to_rug_integer(x), natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    assert!(remainder < *y);
}

#[test]
fn neg_mod_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        neg_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            neg_mod_properties_helper(x, y);
        },
    );

    test_properties(positive_naturals, |n| {
        assert_eq!(n.neg_mod(Natural::ONE), 0 as Limb);
        assert_eq!(n.neg_mod(n), 0 as Limb);
        assert_eq!(Natural::ZERO.neg_mod(n), 0 as Limb);
        if *n > 1 as Limb {
            assert_eq!(Natural::ONE.neg_mod(n), n - 1 as Limb);
        }
    });
}
