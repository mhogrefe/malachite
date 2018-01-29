use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rugint_integer,
                             rugint_integer_to_natural, GenerationMode};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};
use num::BigUint;
use rugint;
use rugint::Assign as rugint_assign;
use std::str::FromStr;

#[test]
fn test_clone() {
    let test = |u| {
        let x = Natural::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigUint::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rugint::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_clone_from_and_assign() {
    let test = |u, v| {
        // clone_from
        let mut x = Natural::from_str(u).unwrap();
        x.clone_from(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigUint::from_str(u).unwrap();
        x.clone_from(&BigUint::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.clone_from(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        // assign Natural by value
        let mut x = Natural::from_str(u).unwrap();
        x.assign(Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        // assign Natural by reference
        let mut x = Natural::from_str(u).unwrap();
        x.assign(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("123", "456");
    test("123", "1000000000000");
    test("1000000000000", "123");
    test("1000000000000", "2000000000000");
}

#[test]
fn clone_clone_from_and_assign_properties() {
    // x.clone() is equivalent for malachite, num, and rugint.
    // x.clone() is valid.
    // x.clone() == x
    let one_natural = |x: Natural| {
        let x_cloned = x.clone();
        assert!(x_cloned.is_valid());
        assert_eq!(biguint_to_natural(&natural_to_biguint(&x).clone()), x);
        assert_eq!(
            rugint_integer_to_natural(&natural_to_rugint_integer(&x).clone()),
            x
        );
        assert_eq!(x_cloned, x);
    };

    // x.clone_from(y) is equivalent for malachite, num, and rugint.
    // x.clone_from(y) is valid.
    // x.clone_from(y); x == y
    // x.assign(y) is equivalent for malachite and rugint.
    // x.assign(y) is valid.
    // x.assign(y); x == y
    // x.assign(&y) is equivalent for malachite and rugint.
    // x.assign(&y) is valid.
    // x.assign(&y); x == y
    let two_naturals = |mut x: Natural, y: Natural| {
        let old_x = x.clone();
        x.clone_from(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut num_x = natural_to_biguint(&old_x);
        let num_y = natural_to_biguint(&y);
        num_x.clone_from(&num_y);
        assert_eq!(biguint_to_natural(&num_x), y);
        let mut rugint_x = natural_to_rugint_integer(&old_x);
        let rugint_y = natural_to_rugint_integer(&y);
        rugint_x.clone_from(&rugint_y);
        assert_eq!(rugint_integer_to_natural(&rugint_x), y);

        x = old_x.clone();
        x.assign(y.clone());
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = natural_to_rugint_integer(&old_x);
        let rugint_y = natural_to_rugint_integer(&y);
        rugint_x.assign(rugint_y);
        assert_eq!(rugint_integer_to_natural(&rugint_x), y);

        x = old_x.clone();
        x.assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = natural_to_rugint_integer(&old_x);
        let rugint_y = natural_to_rugint_integer(&y);
        rugint_x.assign(&rugint_y);
        assert_eq!(rugint_integer_to_natural(&rugint_x), y);
    };

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for (x, y) in pairs_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in pairs_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }
}
