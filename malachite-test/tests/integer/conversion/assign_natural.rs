use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rugint_integer, natural_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::pairs_of_integer_and_natural;
use rugint;
use rugint::Assign as rugint_assign;
use std::str::FromStr;

#[test]
fn test_assign_natural() {
    let test = |u, v, out| {
        // assign Integer by value
        let mut x = Integer::from_str(u).unwrap();
        x.assign(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        // assign Integer by reference
        let mut x = Integer::from_str(u).unwrap();
        x.assign(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
    };
    test("-123", "456", "456");
    test("-123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
fn assign_natural_properties() {
    // x.assign(y) is equivalent for malachite and rugint.
    // x.assign(y) is valid.
    // x.assign(y); x == y
    // x.assign(&y) is equivalent for malachite and rugint.
    // x.assign(&y) is valid.
    // x.assign(&y); x == y
    let integer_and_natural = |mut x: Integer, y: Natural| {
        let old_x = x.clone();
        x.assign(y.clone());
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = integer_to_rugint_integer(&old_x);
        let rugint_y = natural_to_rugint_integer(&y);
        rugint_x.assign(rugint_y);
        assert_eq!(rugint_integer_to_integer(&rugint_x), y);

        x = old_x.clone();
        x.assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = integer_to_rugint_integer(&old_x);
        let rugint_y = natural_to_rugint_integer(&y);
        rugint_x.assign(&rugint_y);
        assert_eq!(rugint_integer_to_integer(&rugint_x), y);
    };

    for (x, y) in pairs_of_integer_and_natural(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_natural(x, y);
    }

    for (x, y) in pairs_of_integer_and_natural(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_natural(x, y);
    }
}
