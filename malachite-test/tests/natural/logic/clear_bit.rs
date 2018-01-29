use common::LARGE_LIMIT;
use malachite_base::num::BitAccess;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rugint_integer, rugint_integer_to_natural, GenerationMode};
use malachite_test::inputs::natural::pairs_of_natural_and_small_u64;
use rugint;
use std::str::FromStr;

#[test]
fn test_clear_bit() {
    let test = |u, index, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.clear_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n.set_bit(index as u32, false);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "0");
    test("0", 100, "0");
    test("1024", 10, "0");
    test("101", 0, "100");
    test("1000000001024", 10, "1000000000000");
    test("1000000001024", 100, "1000000001024");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("1267650600228229401496703205381", 100, "5");
}

#[test]
fn clear_bit_properties() {
    // n.clear_bit(index) is equivalent for malachite and rugint.
    // n.clear_bit(index) is equivalent to n.assign_bit(index, false).
    // If n.get_bit(index), clearing and then setting at index won't do anything.
    // Setting a bit does not increase n.
    // If !n.get_bit(index), clearing at index won't do anything.
    let natural_and_u64 = |mut n: Natural, index: u64| {
        let old_n = n.clone();
        n.clear_bit(index);
        assert!(n.is_valid());

        let mut rugint_n = natural_to_rugint_integer(&old_n);
        rugint_n.set_bit(index as u32, false);
        assert_eq!(rugint_integer_to_natural(&rugint_n), n);

        let mut n2 = old_n.clone();
        n2.assign_bit(index, false);
        assert_eq!(n2, n);

        assert!(n <= old_n);
        if old_n.get_bit(index) {
            assert_ne!(n, old_n);
            n.set_bit(index);
            assert_eq!(n, old_n);
        } else {
            assert_eq!(n, old_n);
        }
    };

    for (n, index) in pairs_of_natural_and_small_u64(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u64(n, index);
    }

    for (n, index) in pairs_of_natural_and_small_u64(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u64(n, index);
    }
}
