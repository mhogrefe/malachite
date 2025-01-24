// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, DivRem, Mod, ModAssign, NegMod,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::signed_pair_gen_var_4;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_var_8, integer_pair_gen_var_1, integer_pair_gen_var_2,
    natural_pair_gen_var_5,
};
use num::{BigInt, Integer as NumInteger};
use rug::ops::RemRounding;
use std::str::FromStr;

#[test]
fn test_mod() {
    let test = |s, t, remainder| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut x = u.clone();
        x.mod_assign(v.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = u.clone();
        x.mod_assign(&v);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = u.clone().mod_op(v.clone());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = u.clone().mod_op(&v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&u).mod_op(v.clone());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&u).mod_op(&v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = BigInt::from_str(s)
            .unwrap()
            .mod_floor(&BigInt::from_str(t).unwrap());
        assert_eq!(r.to_string(), remainder);

        let r = rug::Integer::from_str(s)
            .unwrap()
            .rem_floor(rug::Integer::from_str(t).unwrap());
        assert_eq!(r.to_string(), remainder);

        assert_eq!(u.div_mod(v).1.to_string(), remainder);
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
    let test = |s, t, remainder| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut x = u.clone();
        x %= v.clone();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = u.clone();
        x %= &v;
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = u.clone() % v.clone();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = u.clone() % &v;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = &u % v.clone();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = &u % &v;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = BigInt::from_str(s).unwrap() % &BigInt::from_str(t).unwrap();
        assert_eq!(r.to_string(), remainder);

        let r = rug::Integer::from_str(s).unwrap() % rug::Integer::from_str(t).unwrap();
        assert_eq!(r.to_string(), remainder);

        assert_eq!(u.div_rem(v).1.to_string(), remainder);
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
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn rem_fail() {
    Integer::from(10) % Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn rem_val_ref_fail() {
    Integer::from(10) % &Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn rem_ref_val_fail() {
    &Integer::from(10) % Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn rem_ref_ref_fail() {
    &Integer::from(10) % &Integer::ZERO;
}

#[test]
fn test_ceiling_mod() {
    let test = |s, t, remainder| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut x = u.clone();
        x.ceiling_mod_assign(v.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = u.clone();
        x.ceiling_mod_assign(&v);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = u.clone().ceiling_mod(v.clone());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = u.clone().ceiling_mod(&v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&u).ceiling_mod(v.clone());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&u).ceiling_mod(&v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = rug::Integer::from_str(s)
            .unwrap()
            .rem_ceil(rug::Integer::from_str(t).unwrap());
        assert_eq!(r.to_string(), remainder);

        assert_eq!(u.ceiling_div_mod(v).1.to_string(), remainder);
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

fn mod_properties_helper(x: Integer, y: Integer) {
    let mut mut_x = x.clone();
    mut_x.mod_assign(&y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = (&x).mod_op(&y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = (&x).mod_op(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().mod_op(&y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().mod_op(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!((&x).div_mod(&y).1, remainder);

    let num_remainder = BigInt::from(&x).mod_floor(&BigInt::from(&y));
    assert_eq!(Integer::from(&num_remainder), remainder);

    let rug_remainder = rug::Integer::from(&x).rem_floor(rug::Integer::from(&y));
    assert_eq!(Integer::from(&rug_remainder), remainder);

    assert!(remainder.lt_abs(&y));
    assert!(remainder == 0 || (remainder > 0) == (y > 0));

    assert_eq!((-&x).mod_op(&y), -(&x).ceiling_mod(&y));
    assert_eq!((&x).mod_op(-&y), x.ceiling_mod(y));
}

#[test]
fn mod_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    integer_pair_gen_var_1()
        .test_properties_with_config(&config, |(x, y)| mod_properties_helper(x, y));

    integer_pair_gen_var_2()
        .test_properties_with_config(&config, |(x, y)| mod_properties_helper(x, y));

    integer_gen().test_properties(|x| {
        assert_eq!((&x).mod_op(Integer::ONE), 0);
        assert_eq!(x.mod_op(Integer::NEGATIVE_ONE), 0);
    });

    integer_gen_var_8().test_properties(|ref x| {
        assert_eq!(x.mod_op(Integer::ONE), 0);
        assert_eq!(x.mod_op(Integer::NEGATIVE_ONE), 0);
        assert_eq!(x.mod_op(x), 0);
        assert_eq!(x.mod_op(-x), 0);
        assert_eq!(Integer::ZERO.mod_op(x), 0);
        if *x > 1 {
            assert_eq!(Integer::ONE.mod_op(x), 1);
            assert_eq!(Integer::NEGATIVE_ONE.mod_op(x), x - Integer::ONE);
        }
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x).mod_op(Integer::from(&y)), x.mod_op(y));
    });

    signed_pair_gen_var_4::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x).mod_op(Integer::from(y)), x.mod_op(y));
    });
}

fn rem_properties_helper(x: Integer, y: Integer) {
    let mut mut_x = x.clone();
    mut_x %= &y;
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x %= y.clone();
    assert!(mut_x.is_valid());
    assert_eq!(mut_x, remainder);

    let remainder_alt = &x % &y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = &x % y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone() % &y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone() % y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!((&x).div_rem(&y).1, remainder);

    let num_remainder = BigInt::from(&x) % &BigInt::from(&y);
    assert_eq!(Integer::from(&num_remainder), remainder);

    let rug_remainder = rug::Integer::from(&x) % rug::Integer::from(&y);
    assert_eq!(Integer::from(&rug_remainder), remainder);

    assert!(remainder.lt_abs(&y));
    assert!(remainder == 0 || (remainder > 0) == (x > 0));

    assert_eq!((-&x) % &y, -&remainder);
    assert_eq!(x % (-y), remainder);
}

#[test]
fn rem_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    integer_pair_gen_var_1()
        .test_properties_with_config(&config, |(x, y)| rem_properties_helper(x, y));

    integer_pair_gen_var_2()
        .test_properties_with_config(&config, |(x, y)| rem_properties_helper(x, y));

    integer_gen().test_properties(|x| {
        assert_eq!(&x % Integer::ONE, 0);
        assert_eq!(x % Integer::NEGATIVE_ONE, 0);
    });

    integer_gen_var_8().test_properties(|ref x| {
        assert_eq!(x % Integer::ONE, 0);
        assert_eq!(x % Integer::NEGATIVE_ONE, 0);
        assert_eq!(x % x, 0);
        assert_eq!(x % -x, 0);
        assert_eq!(Integer::ZERO % x, 0);
        if *x > 1 {
            assert_eq!(Integer::ONE % x, 1);
            assert_eq!(Integer::NEGATIVE_ONE % x, -1);
        }
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x) % Integer::from(&y), x % y);
    });

    signed_pair_gen_var_4::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x) % Integer::from(y), x % y);
    });
}

fn ceiling_mod_properties_helper(x: Integer, y: Integer) {
    let mut mut_x = x.clone();
    mut_x.ceiling_mod_assign(&y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.ceiling_mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = (&x).ceiling_mod(&y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = (&x).ceiling_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().ceiling_mod(&y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().ceiling_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert_eq!((&x).ceiling_div_mod(&y).1, remainder);

    let rug_remainder = rug::Integer::from(&x).rem_ceil(rug::Integer::from(&y));
    assert_eq!(Integer::from(&rug_remainder), remainder);

    assert!(remainder.lt_abs(&y));
    assert!(remainder == 0 || (remainder >= 0) != (y > 0));

    assert_eq!((-&x).ceiling_mod(&y), -(&x).mod_op(&y));
    assert_eq!((&x).ceiling_mod(-&y), x.mod_op(y));
}

#[test]
fn ceiling_mod_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    integer_pair_gen_var_1()
        .test_properties_with_config(&config, |(x, y)| ceiling_mod_properties_helper(x, y));

    integer_pair_gen_var_2()
        .test_properties_with_config(&config, |(x, y)| ceiling_mod_properties_helper(x, y));

    integer_gen().test_properties(|x| {
        assert_eq!((&x).ceiling_mod(Integer::ONE), 0);
        assert_eq!(x.ceiling_mod(Integer::NEGATIVE_ONE), 0);
    });

    integer_gen_var_8().test_properties(|ref x| {
        assert_eq!(x.ceiling_mod(Integer::ONE), 0);
        assert_eq!(x.ceiling_mod(Integer::NEGATIVE_ONE), 0);
        assert_eq!(x.ceiling_mod(x), 0);
        assert_eq!(x.ceiling_mod(-x), 0);
        assert_eq!(Integer::ZERO.ceiling_mod(x), 0);
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        assert_eq!(
            Integer::from(&x).ceiling_mod(Integer::from(&y)),
            -x.neg_mod(y)
        );
    });

    signed_pair_gen_var_4::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(
            Integer::from(x).ceiling_mod(Integer::from(y)),
            x.ceiling_mod(y)
        );
    });
}
