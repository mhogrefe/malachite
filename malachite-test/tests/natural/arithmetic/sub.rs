use common::LARGE_LIMIT;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rugint_integer,
                             rugint_integer_to_natural, GenerationMode};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};
use malachite_test::natural::arithmetic::sub::checked_sub;
use num::BigUint;
use rugint;
use std::str::FromStr;

#[test]
fn test_sub_assign_natural() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n -= &Natural::from_str(v).unwrap();
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
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural")]
fn sub_assign_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= &Natural::from_str("456").unwrap();
}

#[test]
fn test_sub_natural() {
    let test = |u, v, out| {
        let on = Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = &Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = checked_sub(BigUint::from_str(u).unwrap(), BigUint::from_str(v).unwrap())
            .map(|x| biguint_to_natural(&x));
        assert_eq!(format!("{:?}", on), out);

        let on = checked_sub(
            rugint::Integer::from_str(u).unwrap(),
            rugint::Integer::from_str(v).unwrap(),
        );
        assert_eq!(format!("{:?}", on), out);
    };
    test("0", "0", "Some(0)");
    test("0", "123", "None");
    test("123", "0", "Some(123)");
    test("456", "123", "Some(333)");
    test("1000000000000", "123", "Some(999999999877)");
    test("123", "1000000000000", "None");
    test(
        "12345678987654321",
        "314159265358979",
        "Some(12031519722295342)",
    );
    test("4294967296", "1", "Some(4294967295)");
    test("4294967295", "4294967295", "Some(0)");
    test("4294967296", "4294967295", "Some(1)");
    test("4294967296", "4294967296", "Some(0)");
    test("4294967295", "4294967296", "None");
    test("18446744073709551616", "1", "Some(18446744073709551615)");
    test("18446744073709551615", "18446744073709551615", "Some(0)");
    test("18446744073709551616", "18446744073709551615", "Some(1)");
    test("18446744073709551615", "18446744073709551616", "None");
    test(
        "70734740290631708",
        "282942734368",
        "Some(70734457347897340)",
    );
    test("282942734368", "70734740290631708", "None");
}

#[test]
fn sub_properties() {
    // x -= y is equivalent for malachite and rugint.
    // x - &y is equivalent for malachite and rugint.
    // x -= y; n is valid.
    // x - &y is valid.
    // &x - &y is valid.
    // x -= y, x - &y, and &x - &y give the same result.
    // if x >= y, x - y <= x
    // if x >= y, (x - y).unwrap() + y == x
    let two_naturals = |mut x: Natural, y: Natural| {
        let old_x = x.clone();

        if x >= y {
            x -= &y;
            assert!(x.is_valid());

            let mut rugint_x = natural_to_rugint_integer(&old_x);
            rugint_x -= natural_to_rugint_integer(&y);
            assert_eq!(rugint_integer_to_natural(&rugint_x), x);
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

        let num_x2 = natural_to_biguint(&old_x);
        let num_y = natural_to_biguint(&y);
        assert_eq!(
            checked_sub(num_x2, num_y).map(|x| biguint_to_natural(&x)),
            ox
        );

        let rugint_x2 = natural_to_rugint_integer(&old_x);
        let rugint_y = natural_to_rugint_integer(&y);
        assert_eq!(
            checked_sub(rugint_x2, rugint_y).map(|x| rugint_integer_to_natural(&x)),
            ox
        );

        if ox.is_some() {
            assert!(ox.clone().unwrap() <= old_x);
            assert_eq!(ox.unwrap() + &y, old_x);
        }
    };

    // x - 0 == x
    // x - x == 0
    // if x != 0, 0 - x == None
    #[allow(unknown_lints, identity_op, eq_op)]
    let one_natural = |x: Natural| {
        assert_eq!((&x - 0).unwrap(), x);
        assert_eq!((&x - &x).unwrap(), Natural::ZERO);
        if x != 0 {
            assert!((0 - &x).is_none());
        }
    };

    for (x, y) in pairs_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in pairs_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
