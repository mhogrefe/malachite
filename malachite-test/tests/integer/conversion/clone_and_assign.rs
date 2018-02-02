use common::LARGE_LIMIT;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rug_integer,
                             rug_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::{integers, pairs_of_integers};
use num::BigInt;
use rug;
use rug::Assign as rug_assign;
use std::str::FromStr;

#[test]
fn test_clone() {
    let test = |u| {
        let x = Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigInt::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rug::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_clone_from_assign() {
    let test = |u, v| {
        // clone_from
        let mut x = Integer::from_str(u).unwrap();
        x.clone_from(&Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigInt::from_str(u).unwrap();
        x.clone_from(&BigInt::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rug::Integer::from_str(u).unwrap();
        x.clone_from(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        // assign Integer by value
        let mut x = Integer::from_str(u).unwrap();
        x.assign(Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        // assign Integer by reference
        let mut x = Integer::from_str(u).unwrap();
        x.assign(&Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("-123", "456");
    test("-123", "1000000000000");
    test("1000000000000", "-123");
    test("1000000000000", "2000000000000");
}

#[test]
fn clone_clone_from_and_assign_properties() {
    // x.clone() is equivalent for malachite, num, and rug.
    // x.clone() is valid.
    // x.clone() == x
    let one_integer = |x: Integer| {
        let x_cloned = x.clone();
        assert!(x_cloned.is_valid());
        assert_eq!(bigint_to_integer(&integer_to_bigint(&x).clone()), x);
        assert_eq!(
            rug_integer_to_integer(&integer_to_rug_integer(&x).clone()),
            x
        );
        assert_eq!(x_cloned, x);
    };

    // x.clone_from(y) is equivalent for malachite, num, and rug.
    // x.clone_from(y) is valid.
    // x.clone_from(y); x == y
    // x.assign(y) is equivalent for malachite and rug.
    // x.assign(y) is valid.
    // x.assign(y); x == y
    // x.assign(&y) is equivalent for malachite and rug.
    // x.assign(&y) is valid.
    // x.assign(&y); x == y
    let two_integers = |mut x: Integer, y: Integer| {
        let old_x = x.clone();
        x.clone_from(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut num_x = integer_to_bigint(&old_x);
        let num_y = integer_to_bigint(&y);
        num_x.clone_from(&num_y);
        assert_eq!(bigint_to_integer(&num_x), y);
        let mut rug_x = integer_to_rug_integer(&old_x);
        let rug_y = integer_to_rug_integer(&y);
        rug_x.clone_from(&rug_y);
        assert_eq!(rug_integer_to_integer(&rug_x), y);

        x = old_x.clone();
        x.assign(y.clone());
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rug_x = integer_to_rug_integer(&old_x);
        let rug_y = integer_to_rug_integer(&y);
        rug_x.assign(rug_y);
        assert_eq!(rug_integer_to_integer(&rug_x), y);

        x = old_x.clone();
        x.assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rug_x = integer_to_rug_integer(&old_x);
        let rug_y = integer_to_rug_integer(&y);
        rug_x.assign(&rug_y);
        assert_eq!(rug_integer_to_integer(&rug_x), y);
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for (x, y) in pairs_of_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in pairs_of_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }
}
