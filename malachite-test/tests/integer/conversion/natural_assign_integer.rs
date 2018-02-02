use common::LARGE_LIMIT;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rug_integer, natural_to_rug_integer,
                             rug_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::pairs_of_natural_and_natural_integer;
use rug;
use rug::Assign as rug_assign;
use std::str::FromStr;

#[test]
fn test_natural_assign_integer() {
    let test = |u, v, out| {
        // assign Integer by value
        let mut x = Natural::from_str(u).unwrap();
        x.assign(Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        // assign Integer by reference
        let mut x = Natural::from_str(u).unwrap();
        x.assign(&Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(&rug::Integer::from_str(v).unwrap());
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
    // x.assign(y) is equivalent for malachite and rug.
    // x.assign(y) is valid.
    // x.assign(y); x == y
    // x.assign(&y) is equivalent for malachite and rug.
    // x.assign(&y) is valid.
    // x.assign(&y); x == y
    let natural_and_integer = |mut x: Natural, y: Integer| {
        let old_x = x.clone();
        x.assign(y.clone());
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rug_x = natural_to_rug_integer(&old_x);
        let rug_y = integer_to_rug_integer(&y);
        rug_x.assign(rug_y);
        assert_eq!(rug_integer_to_integer(&rug_x), y);

        x = old_x.clone();
        x.assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rug_x = natural_to_rug_integer(&old_x);
        let rug_y = integer_to_rug_integer(&y);
        rug_x.assign(&rug_y);
        assert_eq!(rug_integer_to_integer(&rug_x), y);
    };

    for (x, y) in pairs_of_natural_and_natural_integer(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_integer(x, y);
    }

    for (x, y) in pairs_of_natural_and_natural_integer(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_integer(x, y);
    }
}
