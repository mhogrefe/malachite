use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_gmp,
                             native_natural_to_num_biguint, native_natural_to_rugint_integer,
                             num_biguint_to_native_natural, rugint_integer_to_native_natural};
use malachite_test::natural::arithmetic::sub::{num_sub, rugint_sub};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::str::FromStr;

#[test]
fn test_sub_assign_natural() {
    let test = |u, v, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n -= &native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n -= &gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
}

#[test]
fn test_sub_natural() {
    let test = |u, v, out| {
        let on = native::Natural::from_str(u).unwrap() - &native::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = gmp::Natural::from_str(u).unwrap() - &gmp::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = &native::Natural::from_str(u).unwrap() - &native::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = &gmp::Natural::from_str(u).unwrap() - &gmp::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = num_sub(num::BigUint::from_str(u).unwrap(),
                         num::BigUint::from_str(v).unwrap())
                .map(|x| num_biguint_to_native_natural(&x));
        assert_eq!(format!("{:?}", on), out);

        let on = rugint_sub(rugint::Integer::from_str(u).unwrap(),
                            rugint::Integer::from_str(v).unwrap());
        assert_eq!(format!("{:?}", on), out);
    };
    test("0", "0", "Some(0)");
    test("0", "123", "None");
    test("123", "0", "Some(123)");
    test("456", "123", "Some(333)");
    test("1000000000000", "123", "Some(999999999877)");
    test("123", "1000000000000", "None");
    test("12345678987654321",
         "314159265358979",
         "Some(12031519722295342)");
    test("4294967296", "1", "Some(4294967295)");
    test("4294967295", "4294967295", "Some(0)");
    test("4294967296", "4294967295", "Some(1)");
    test("4294967296", "4294967296", "Some(0)");
    test("4294967295", "4294967296", "None");
    test("18446744073709551616", "1", "Some(18446744073709551615)");
    test("18446744073709551615", "18446744073709551615", "Some(0)");
    test("18446744073709551616", "18446744073709551615", "Some(1)");
    test("18446744073709551615", "18446744073709551616", "None");
    test("70734740290631708",
         "282942734368",
         "Some(70734457347897340)");
    test("282942734368", "70734740290631708", "None");
}

#[test]
fn sub_properties() {
    // x -= y is equivalent for malachite-gmp, malachite-native, and rugint.
    // x - &y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x -= y; n is valid.
    // x - &y is valid.
    // &x - &y is valid.
    // x -= y, x - &y, and &x - &y give the same result.
    // if x >= y, x - y <= x
    // if x >= y, (x - y).unwrap() + y == x
    let two_naturals = |mut gmp_x: gmp::Natural, gmp_y: gmp::Natural| {
        let mut x = gmp_natural_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let old_x = x.clone();

        if x >= y {
            gmp_x -= &gmp_y;
            assert!(gmp_x.is_valid());

            x -= &y;
            assert!(x.is_valid());
            assert_eq!(gmp_natural_to_native(&gmp_x), x);

            let mut rugint_x = native_natural_to_rugint_integer(&old_x);
            rugint_x -= native_natural_to_rugint_integer(&y);
            assert_eq!(rugint_integer_to_native_natural(&rugint_x), x);
        }
        let ox = if old_x >= y { Some(x) } else { None };

        let x2 = old_x.clone();
        let result = &x2 - &y;
        assert_eq!(result, ox);
        assert!(result.map_or(true, |x| x.is_valid()));
        let result = x2 - &y;
        assert_eq!(result, ox);
        assert!(result.map_or(true, |x| x.is_valid()));

        let x2 = old_x.clone();
        let result = &y - &x2;
        assert_eq!(result.is_some(), y == x2 || ox.is_none());
        assert!(result.map_or(true, |x| x.is_valid()));

        let gmp_x2 = native_natural_to_gmp(&old_x);
        let result = &gmp_x2 - &gmp_y;
        assert_eq!(result.clone().map(|x| gmp_natural_to_native(&x)), ox);
        assert!(result.map_or(true, |x| x.is_valid()));
        let result = gmp_x2 - &gmp_y;
        assert_eq!(result.clone().map(|x| gmp_natural_to_native(&x)), ox);
        assert!(result.map_or(true, |x| x.is_valid()));

        let gmp_x2 = native_natural_to_gmp(&old_x);
        let result = &gmp_y - &gmp_x2;
        assert_eq!(result.is_some(), gmp_y == gmp_x2 || ox.is_none());
        assert!(result.map_or(true, |x| x.is_valid()));

        let num_x2 = native_natural_to_num_biguint(&old_x);
        let num_y = native_natural_to_num_biguint(&y);
        assert_eq!(num_sub(num_x2, num_y).map(|x| num_biguint_to_native_natural(&x)),
                   ox);

        let rugint_x2 = native_natural_to_rugint_integer(&old_x);
        let rugint_y = native_natural_to_rugint_integer(&y);
        assert_eq!(rugint_sub(rugint_x2, rugint_y).map(|x| rugint_integer_to_native_natural(&x)),
                   ox);

        if ox.is_some() {
            assert!(ox.clone().unwrap() <= old_x);
            assert_eq!(ox.unwrap() + &y, old_x);
        }
    };

    // x - 0 == x
    // x - x == 0
    // if x != 0, 0 - x == None
    #[allow(identity_op, eq_op)]
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        assert_eq!((&x - 0).unwrap(), x);
        assert_eq!((&x - &x).unwrap(), native::Natural::from(0u32));
        if x != 0 {
            assert!((0 - &x).is_none());
        }
    };

    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
