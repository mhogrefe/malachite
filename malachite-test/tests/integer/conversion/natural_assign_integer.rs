use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rugint_integer, natural_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::integer::conversion::natural_assign_integer::select_inputs;
use rugint;
use rugint::Assign as rugint_assign;
use std::str::FromStr;

#[test]
fn test_natural_assign_integer() {
    let test = |u, v, out| {
        // assign Integer by value
        let mut x = Natural::from_str(u).unwrap();
        x.assign(Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        // assign Integer by reference
        let mut x = Natural::from_str(u).unwrap();
        x.assign(&Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
    };
    test("123", "456", "456");
    test("123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
#[should_panic(expected = "Cannot assign from a negative Integer. Invalid other: -456")]
fn natural_assign_integer_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x.assign(&Integer::from_str("-456").unwrap());
}

#[test]
fn natural_assign_integer_properties() {
    // x.assign(y) is equivalent for malachite and rugint.
    // x.assign(y) is valid.
    // x.assign(y); x == y
    // x.assign(&y) is equivalent for malachite and rugint.
    // x.assign(&y) is valid.
    // x.assign(&y); x == y
    let natural_and_integer = |mut x: Natural, y: Integer| {
        let old_x = x.clone();
        x.assign(y.clone());
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = natural_to_rugint_integer(&old_x);
        let rugint_y = integer_to_rugint_integer(&y);
        rugint_x.assign(rugint_y);
        assert_eq!(rugint_integer_to_integer(&rugint_x), y);

        x = old_x.clone();
        x.assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = natural_to_rugint_integer(&old_x);
        let rugint_y = integer_to_rugint_integer(&y);
        rugint_x.assign(&rugint_y);
        assert_eq!(rugint_integer_to_integer(&rugint_x), y);
    };

    for (x, y) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }

    for (x, y) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }
}
