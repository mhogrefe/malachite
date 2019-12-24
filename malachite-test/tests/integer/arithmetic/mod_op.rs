use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, DivRem, Mod, ModAssign,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use num::{BigInt, Integer as NumInteger};
use rug;
use rug::ops::RemRounding;

use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer,
    pairs_of_integer_and_nonzero_integer_var_1,
};

#[test]
fn test_mod() {
    let test = |u, v, remainder| {
        let mut x = Integer::from_str(u).unwrap();
        x.mod_assign(Integer::from_str(v).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = Integer::from_str(u).unwrap();
        x.mod_assign(&Integer::from_str(v).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = Integer::from_str(u)
            .unwrap()
            .mod_op(Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = Integer::from_str(u)
            .unwrap()
            .mod_op(&Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&Integer::from_str(u).unwrap()).mod_op(Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&Integer::from_str(u).unwrap()).mod_op(&Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = BigInt::from_str(u)
            .unwrap()
            .mod_floor(&BigInt::from_str(v).unwrap());
        assert_eq!(r.to_string(), remainder);

        let r = rug::Integer::from_str(u)
            .unwrap()
            .rem_floor(rug::Integer::from_str(v).unwrap());
        assert_eq!(r.to_string(), remainder);

        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .div_mod(Integer::from_str(v).unwrap())
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

    test("0", "-1", "0");
    test("0", "-123", "0");
    test("1", "-1", "0");
    test("123", "-1", "0");
    test("123", "-123", "0");
    test("123", "-456", "-333");
    test("456", "-123", "-36");
    test("4294967295", "-1", "0");
    test("4294967295", "-4294967295", "0");
    test("1000000000000", "-1", "0");
    test("1000000000000", "-3", "-2");
    test("1000000000000", "-123", "-23");
    test("1000000000000", "-4294967295", "-727379735");
    test("1000000000000000000000000", "-1", "0");
    test("1000000000000000000000000", "-3", "-2");
    test("1000000000000000000000000", "-123", "-86");
    test("1000000000000000000000000", "-4294967295", "-1127243600");
    test(
        "1000000000000000000000000",
        "-1234567890987",
        "-704498996588",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         0",
        "-1234567890987654321234567890987654321",
        "-454912836989613466895606299668358255",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         0",
        "-316049380092839506236049380092839506176",
        "-278232688309211835744673381771890580480",
    );
    test(
        "253640751230376270397812803167",
        "-2669936877441",
        "-1149635115107",
    );
    test(
        "3768477692975601",
        "-11447376614057827956",
        "-11443608136364852355",
    );
    test("3356605361737854", "-3081095617839357", "-2805585873940860");
    test(
        "1098730198198174614195",
        "-953382298040157850476",
        "-808034397882141086757",
    );
    test(
        "69738658860594537152875081748",
        "-69738658860594537152875081748",
        "0",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000000000000000",
        "0",
    );
    test("0", "-1000000000000000000000000", "0");
    test(
        "123",
        "-1000000000000000000000000",
        "-999999999999999999999877",
    );

    test("-1", "1", "0");
    test("-123", "1", "0");
    test("-123", "123", "0");
    test("-123", "456", "333");
    test("-456", "123", "36");
    test("-4294967295", "-1", "0");
    test("-4294967295", "4294967295", "0");
    test("-1000000000000", "1", "0");
    test("-1000000000000", "3", "2");
    test("-1000000000000", "123", "23");
    test("-1000000000000", "4294967295", "727379735");
    test("-1000000000000000000000000", "1", "0");
    test("-1000000000000000000000000", "3", "2");
    test("-1000000000000000000000000", "123", "86");
    test("-1000000000000000000000000", "4294967295", "1127243600");
    test(
        "-1000000000000000000000000",
        "1234567890987",
        "704498996588",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "1234567890987654321234567890987654321",
        "454912836989613466895606299668358255",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "316049380092839506236049380092839506176",
        "278232688309211835744673381771890580480",
    );
    test(
        "-253640751230376270397812803167",
        "2669936877441",
        "1149635115107",
    );
    test(
        "-3768477692975601",
        "11447376614057827956",
        "11443608136364852355",
    );
    test("-3356605361737854", "3081095617839357", "2805585873940860");
    test(
        "-1098730198198174614195",
        "953382298040157850476",
        "808034397882141086757",
    );
    test(
        "-69738658860594537152875081748",
        "69738658860594537152875081748",
        "0",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000000000000000",
        "0",
    );
    test(
        "-123",
        "1000000000000000000000000",
        "999999999999999999999877",
    );

    test("-1", "-1", "0");
    test("-123", "-1", "0");
    test("-123", "-123", "0");
    test("-123", "-456", "-123");
    test("-456", "-123", "-87");
    test("-4294967295", "-1", "0");
    test("-4294967295", "-4294967295", "0");
    test("-1000000000000", "-1", "0");
    test("-1000000000000", "-3", "-1");
    test("-1000000000000", "-123", "-100");
    test("-1000000000000", "-4294967295", "-3567587560");
    test("-1000000000000000000000000", "-1", "0");
    test("-1000000000000000000000000", "-3", "-1");
    test("-1000000000000000000000000", "-123", "-37");
    test("-1000000000000000000000000", "-4294967295", "-3167723695");
    test(
        "-1000000000000000000000000",
        "-1234567890987",
        "-530068894399",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "-1234567890987654321234567890987654321",
        "-779655053998040854338961591319296066",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "-316049380092839506236049380092839506176",
        "-37816691783627670491375998320948925696",
    );
    test(
        "-253640751230376270397812803167",
        "-2669936877441",
        "-1520301762334",
    );
    test(
        "-3768477692975601",
        "-11447376614057827956",
        "-3768477692975601",
    );
    test("-3356605361737854", "-3081095617839357", "-275509743898497");
    test(
        "-1098730198198174614195",
        "-953382298040157850476",
        "-145347900158016763719",
    );
    test(
        "-69738658860594537152875081748",
        "-69738658860594537152875081748",
        "0",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000000000000000",
        "0",
    );
    test("-123", "-1000000000000000000000000", "-123");
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    Integer::from(10).mod_assign(Integer::ZERO);
}

#[test]
#[should_panic]
fn mod_assign_ref_fail() {
    Integer::from(10).mod_assign(&Integer::ZERO);
}

#[test]
#[should_panic]
fn mod_fail() {
    Integer::from(10).mod_op(Integer::ZERO);
}

#[test]
#[should_panic]
fn mod_val_ref_fail() {
    Integer::from(10).mod_op(&Integer::ZERO);
}

#[test]
#[should_panic]
fn mod_ref_val_fail() {
    (&Integer::from(10)).mod_op(Integer::ZERO);
}

#[test]
#[should_panic]
fn mod_ref_ref_fail() {
    (&Integer::from(10)).mod_op(&Integer::ZERO);
}

#[test]
fn test_rem() {
    let test = |u, v, remainder| {
        let mut x = Integer::from_str(u).unwrap();
        x %= Integer::from_str(v).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = Integer::from_str(u).unwrap();
        x %= &Integer::from_str(v).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = Integer::from_str(u).unwrap() % Integer::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = Integer::from_str(u).unwrap() % &Integer::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = &Integer::from_str(u).unwrap() % Integer::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = &Integer::from_str(u).unwrap() % &Integer::from_str(v).unwrap();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = BigInt::from_str(u).unwrap() % &BigInt::from_str(v).unwrap();
        assert_eq!(r.to_string(), remainder);

        let r = rug::Integer::from_str(u).unwrap() % rug::Integer::from_str(v).unwrap();
        assert_eq!(r.to_string(), remainder);

        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .div_rem(Integer::from_str(v).unwrap())
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
    test("4294967295", "1", "0");
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

    test("0", "-1", "0");
    test("0", "-123", "0");
    test("1", "-1", "0");
    test("123", "-1", "0");
    test("123", "-123", "0");
    test("123", "-456", "123");
    test("456", "-123", "87");
    test("4294967295", "-1", "0");
    test("4294967295", "-4294967295", "0");
    test("1000000000000", "-1", "0");
    test("1000000000000", "-3", "1");
    test("1000000000000", "-123", "100");
    test("1000000000000", "-4294967295", "3567587560");
    test("1000000000000000000000000", "-1", "0");
    test("1000000000000000000000000", "-3", "1");
    test("1000000000000000000000000", "-123", "37");
    test("1000000000000000000000000", "-4294967295", "3167723695");
    test(
        "1000000000000000000000000",
        "-1234567890987",
        "530068894399",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "-1234567890987654321234567890987654321",
        "779655053998040854338961591319296066",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "-316049380092839506236049380092839506176",
        "37816691783627670491375998320948925696",
    );
    test(
        "253640751230376270397812803167",
        "-2669936877441",
        "1520301762334",
    );
    test(
        "3768477692975601",
        "-11447376614057827956",
        "3768477692975601",
    );
    test("3356605361737854", "-3081095617839357", "275509743898497");
    test(
        "1098730198198174614195",
        "-953382298040157850476",
        "145347900158016763719",
    );
    test(
        "69738658860594537152875081748",
        "-69738658860594537152875081748",
        "0",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000000000000000",
        "0",
    );
    test("0", "-1000000000000000000000000", "0");
    test("123", "-1000000000000000000000000", "123");

    test("-1", "1", "0");
    test("-123", "1", "0");
    test("-123", "123", "0");
    test("-123", "456", "-123");
    test("-456", "123", "-87");
    test("-4294967295", "1", "0");
    test("-4294967295", "4294967295", "0");
    test("-1000000000000", "1", "0");
    test("-1000000000000", "3", "-1");
    test("-1000000000000", "123", "-100");
    test("-1000000000000", "4294967295", "-3567587560");
    test("-1000000000000000000000000", "1", "0");
    test("-1000000000000000000000000", "3", "-1");
    test("-1000000000000000000000000", "123", "-37");
    test("-1000000000000000000000000", "4294967295", "-3167723695");
    test(
        "-1000000000000000000000000",
        "1234567890987",
        "-530068894399",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "1234567890987654321234567890987654321",
        "-779655053998040854338961591319296066",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "316049380092839506236049380092839506176",
        "-37816691783627670491375998320948925696",
    );
    test(
        "-253640751230376270397812803167",
        "2669936877441",
        "-1520301762334",
    );
    test(
        "-3768477692975601",
        "11447376614057827956",
        "-3768477692975601",
    );
    test("-3356605361737854", "3081095617839357", "-275509743898497");
    test(
        "-1098730198198174614195",
        "953382298040157850476",
        "-145347900158016763719",
    );
    test(
        "-69738658860594537152875081748",
        "69738658860594537152875081748",
        "0",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000000000000000",
        "0",
    );
    test("-123", "1000000000000000000000000", "-123");

    test("-1", "-1", "0");
    test("-123", "-1", "0");
    test("-123", "-123", "0");
    test("-123", "-456", "-123");
    test("-456", "-123", "-87");
    test("-4294967295", "-1", "0");
    test("-4294967295", "-4294967295", "0");
    test("-1000000000000", "-1", "0");
    test("-1000000000000", "-3", "-1");
    test("-1000000000000", "-123", "-100");
    test("-1000000000000", "-4294967295", "-3567587560");
    test("-1000000000000000000000000", "-1", "0");
    test("-1000000000000000000000000", "-3", "-1");
    test("-1000000000000000000000000", "-123", "-37");
    test("-1000000000000000000000000", "-4294967295", "-3167723695");
    test(
        "-1000000000000000000000000",
        "-1234567890987",
        "-530068894399",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "-1234567890987654321234567890987654321",
        "-779655053998040854338961591319296066",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "-316049380092839506236049380092839506176",
        "-37816691783627670491375998320948925696",
    );
    test(
        "-253640751230376270397812803167",
        "-2669936877441",
        "-1520301762334",
    );
    test(
        "-3768477692975601",
        "-11447376614057827956",
        "-3768477692975601",
    );
    test("-3356605361737854", "-3081095617839357", "-275509743898497");
    test(
        "-1098730198198174614195",
        "-953382298040157850476",
        "-145347900158016763719",
    );
    test(
        "-69738658860594537152875081748",
        "-69738658860594537152875081748",
        "0",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000000000000000",
        "0",
    );
    test("-123", "-1000000000000000000000000", "-123");
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut x = Integer::from(10);
    x %= Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_ref_fail() {
    let mut x = Integer::from(10);
    x %= &Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn rem_fail() {
    Integer::from(10) % Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn rem_val_ref_fail() {
    Integer::from(10) % &Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn rem_ref_val_fail() {
    &Integer::from(10) % Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn rem_ref_ref_fail() {
    &Integer::from(10) % &Integer::ZERO;
}

#[test]
fn test_ceiling_mod() {
    let test = |u, v, remainder| {
        let mut x = Integer::from_str(u).unwrap();
        x.ceiling_mod_assign(Integer::from_str(v).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = Integer::from_str(u).unwrap();
        x.ceiling_mod_assign(&Integer::from_str(v).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = Integer::from_str(u)
            .unwrap()
            .ceiling_mod(Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = Integer::from_str(u)
            .unwrap()
            .ceiling_mod(&Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&Integer::from_str(u).unwrap()).ceiling_mod(Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&Integer::from_str(u).unwrap()).ceiling_mod(&Integer::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = rug::Integer::from_str(u)
            .unwrap()
            .rem_ceil(rug::Integer::from_str(v).unwrap());
        assert_eq!(r.to_string(), remainder);

        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .ceiling_div_mod(Integer::from_str(v).unwrap())
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
    test("123", "456", "-333");
    test("456", "123", "-36");
    test("4294967295", "1", "0");
    test("4294967295", "4294967295", "0");
    test("1000000000000", "1", "0");
    test("1000000000000", "3", "-2");
    test("1000000000000", "123", "-23");
    test("1000000000000", "4294967295", "-727379735");
    test("1000000000000000000000000", "1", "0");
    test("1000000000000000000000000", "3", "-2");
    test("1000000000000000000000000", "123", "-86");
    test("1000000000000000000000000", "4294967295", "-1127243600");
    test(
        "1000000000000000000000000",
        "1234567890987",
        "-704498996588",
    );
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "1234567890987654321234567890987654321",
        "-454912836989613466895606299668358255",
    );
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "316049380092839506236049380092839506176",
        "-278232688309211835744673381771890580480",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "-1149635115107",
    );
    test(
        "3768477692975601",
        "11447376614057827956",
        "-11443608136364852355",
    );
    test("3356605361737854", "3081095617839357", "-2805585873940860");
    test(
        "1098730198198174614195",
        "953382298040157850476",
        "-808034397882141086757",
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
        "-999999999999999999999877",
    );

    test("0", "-1", "0");
    test("0", "-123", "0");
    test("1", "-1", "0");
    test("123", "-1", "0");
    test("123", "-123", "0");
    test("123", "-456", "123");
    test("456", "-123", "87");
    test("4294967295", "-1", "0");
    test("4294967295", "-4294967295", "0");
    test("1000000000000", "-1", "0");
    test("1000000000000", "-3", "1");
    test("1000000000000", "-123", "100");
    test("1000000000000", "-4294967295", "3567587560");
    test("1000000000000000000000000", "-1", "0");
    test("1000000000000000000000000", "-3", "1");
    test("1000000000000000000000000", "-123", "37");
    test("1000000000000000000000000", "-4294967295", "3167723695");
    test(
        "1000000000000000000000000",
        "-1234567890987",
        "530068894399",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "-1234567890987654321234567890987654321",
        "779655053998040854338961591319296066",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "-316049380092839506236049380092839506176",
        "37816691783627670491375998320948925696",
    );
    test(
        "253640751230376270397812803167",
        "-2669936877441",
        "1520301762334",
    );
    test(
        "3768477692975601",
        "-11447376614057827956",
        "3768477692975601",
    );
    test("3356605361737854", "-3081095617839357", "275509743898497");
    test(
        "1098730198198174614195",
        "-953382298040157850476",
        "145347900158016763719",
    );
    test(
        "69738658860594537152875081748",
        "-69738658860594537152875081748",
        "0",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000000000000000",
        "0",
    );
    test("0", "-1000000000000000000000000", "0");
    test("123", "-1000000000000000000000000", "123");

    test("-1", "1", "0");
    test("-123", "1", "0");
    test("-123", "123", "0");
    test("-123", "456", "-123");
    test("-456", "123", "-87");
    test("-4294967295", "1", "0");
    test("-4294967295", "4294967295", "0");
    test("-1000000000000", "1", "0");
    test("-1000000000000", "3", "-1");
    test("-1000000000000", "123", "-100");
    test("-1000000000000", "4294967295", "-3567587560");
    test("-1000000000000000000000000", "1", "0");
    test("-1000000000000000000000000", "3", "-1");
    test("-1000000000000000000000000", "123", "-37");
    test("-1000000000000000000000000", "4294967295", "-3167723695");
    test(
        "-1000000000000000000000000",
        "1234567890987",
        "-530068894399",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "1234567890987654321234567890987654321",
        "-779655053998040854338961591319296066",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "316049380092839506236049380092839506176",
        "-37816691783627670491375998320948925696",
    );
    test(
        "-253640751230376270397812803167",
        "2669936877441",
        "-1520301762334",
    );
    test(
        "-3768477692975601",
        "11447376614057827956",
        "-3768477692975601",
    );
    test("-3356605361737854", "3081095617839357", "-275509743898497");
    test(
        "-1098730198198174614195",
        "953382298040157850476",
        "-145347900158016763719",
    );
    test(
        "-69738658860594537152875081748",
        "69738658860594537152875081748",
        "0",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000000000000000",
        "0",
    );
    test("0", "1000000000000000000000000", "0");
    test("-123", "1000000000000000000000000", "-123");

    test("-1", "-1", "0");
    test("-123", "-1", "0");
    test("-123", "-123", "0");
    test("-123", "-456", "333");
    test("-456", "-123", "36");
    test("-4294967295", "-1", "0");
    test("-4294967295", "-4294967295", "0");
    test("-1000000000000", "-1", "0");
    test("-1000000000000", "-3", "2");
    test("-1000000000000", "-123", "23");
    test("-1000000000000", "-4294967295", "727379735");
    test("-1000000000000000000000000", "-1", "0");
    test("-1000000000000000000000000", "-3", "2");
    test("-1000000000000000000000000", "-123", "86");
    test("-1000000000000000000000000", "-4294967295", "1127243600");
    test(
        "-1000000000000000000000000",
        "-1234567890987",
        "704498996588",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "-1234567890987654321234567890987654321",
        "454912836989613466895606299668358255",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "-316049380092839506236049380092839506176",
        "278232688309211835744673381771890580480",
    );
    test(
        "-253640751230376270397812803167",
        "-2669936877441",
        "1149635115107",
    );
    test(
        "-3768477692975601",
        "-11447376614057827956",
        "11443608136364852355",
    );
    test("-3356605361737854", "-3081095617839357", "2805585873940860");
    test(
        "-1098730198198174614195",
        "-953382298040157850476",
        "808034397882141086757",
    );
    test(
        "-69738658860594537152875081748",
        "-69738658860594537152875081748",
        "0",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000000000000000",
        "0",
    );
    test("0", "-1000000000000000000000000", "0");
    test(
        "-123",
        "-1000000000000000000000000",
        "999999999999999999999877",
    );
}

#[test]
#[should_panic]
fn ceiling_mod_assign_fail() {
    Integer::from(10).ceiling_mod_assign(Integer::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_assign_ref_fail() {
    Integer::from(10).ceiling_mod_assign(&Integer::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_fail() {
    Integer::from(10).ceiling_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_val_ref_fail() {
    Integer::from(10).ceiling_mod(&Integer::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_ref_val_fail() {
    (&Integer::from(10)).ceiling_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_ref_ref_fail() {
    (&Integer::from(10)).ceiling_mod(&Integer::ZERO);
}

fn mod_properties_helper(x: &Integer, y: &Integer) {
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

    assert_eq!(x.div_mod(y).1, remainder);

    let num_remainder = integer_to_bigint(x).mod_floor(&integer_to_bigint(y));
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let rug_remainder = integer_to_rug_integer(x).rem_floor(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == Integer::ZERO || (remainder > Integer::ZERO) == (*y > Integer::ZERO));

    assert_eq!((-x).mod_op(y), -x.ceiling_mod(y));
    assert_eq!(x.mod_op(-y), x.ceiling_mod(y));
}

#[test]
fn mod_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer,
        |&(ref x, ref y)| {
            mod_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            mod_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x.mod_op(Integer::ONE), Integer::ZERO);
        assert_eq!(x.mod_op(Integer::NEGATIVE_ONE), Integer::ZERO);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x.mod_op(Integer::ONE), Integer::ZERO);
        assert_eq!(x.mod_op(Integer::NEGATIVE_ONE), Integer::ZERO);
        assert_eq!(x.mod_op(x), Integer::ZERO);
        assert_eq!(x.mod_op(-x), Integer::ZERO);
        assert_eq!(Integer::ZERO.mod_op(x), Integer::ZERO);
        if *x > Integer::ONE {
            assert_eq!(Integer::ONE.mod_op(x), Integer::ONE);
            assert_eq!(Integer::NEGATIVE_ONE.mod_op(x), x - Integer::ONE);
        }
    });
}

fn rem_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    mut_x %= y;
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x %= y.clone();
    assert!(mut_x.is_valid());
    assert_eq!(mut_x, remainder);

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

    assert_eq!(x.div_rem(y).1, remainder);

    let num_remainder = integer_to_bigint(x) % &integer_to_bigint(y);
    assert_eq!(bigint_to_integer(&num_remainder), remainder);

    let rug_remainder = integer_to_rug_integer(x) % integer_to_rug_integer(y);
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == Integer::ZERO || (remainder > Integer::ZERO) == (*x > Integer::ZERO));

    assert_eq!((-x) % y, -&remainder);
    assert_eq!(x % (-y), remainder);
}

#[test]
fn rem_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer,
        |&(ref x, ref y)| {
            rem_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            rem_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x % Integer::ONE, Integer::ZERO);
        assert_eq!(x % Integer::NEGATIVE_ONE, Integer::ZERO);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x % Integer::ONE, Integer::ZERO);
        assert_eq!(x % Integer::NEGATIVE_ONE, Integer::ZERO);
        assert_eq!(x % x, Integer::ZERO);
        assert_eq!(x % -x, Integer::ZERO);
        assert_eq!(Integer::ZERO % x, Integer::ZERO);
        if *x > Integer::ONE {
            assert_eq!(Integer::ONE % x, Integer::ONE);
            assert_eq!(Integer::NEGATIVE_ONE % x, Integer::NEGATIVE_ONE);
        }
    });
}

fn ceiling_mod_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    mut_x.ceiling_mod_assign(y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.ceiling_mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.ceiling_mod(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.ceiling_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().ceiling_mod(y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().ceiling_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!(x.ceiling_div_mod(y).1, remainder);

    let rug_remainder = integer_to_rug_integer(x).rem_ceil(integer_to_rug_integer(y));
    assert_eq!(rug_integer_to_integer(&rug_remainder), remainder);

    assert!(remainder.lt_abs(y));
    assert!(remainder == Integer::ZERO || (remainder >= Integer::ZERO) != (*y > Integer::ZERO));

    assert_eq!((-x).ceiling_mod(y), -x.mod_op(y));
    assert_eq!(x.ceiling_mod(-y), x.mod_op(y));
}

#[test]
fn ceiling_mod_properties() {
    test_properties(pairs_of_integer_and_nonzero_integer, |&(ref x, ref y)| {
        ceiling_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            ceiling_mod_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x.ceiling_mod(Integer::ONE), Integer::ZERO);
        assert_eq!(x.ceiling_mod(Integer::NEGATIVE_ONE), Integer::ZERO);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(x.ceiling_mod(Integer::ONE), Integer::ZERO);
        assert_eq!(x.ceiling_mod(Integer::NEGATIVE_ONE), Integer::ZERO);
        assert_eq!(x.ceiling_mod(x), Integer::ZERO);
        assert_eq!(x.ceiling_mod(-x), Integer::ZERO);
        assert_eq!(Integer::ZERO.ceiling_mod(x), Integer::ZERO);
    });
}
