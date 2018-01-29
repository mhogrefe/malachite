use common::LARGE_LIMIT;
use malachite_base::num::BitAccess;
use malachite_base::traits::NotAssign;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::integer::pairs_of_integer_and_small_u64;
use std::str::FromStr;

#[test]
fn test_clear_bit() {
    let test = |u, index, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.clear_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 10, "0");
    test("0", 100, "0");
    test("1024", 10, "0");
    test("101", 0, "100");
    test("1000000001024", 10, "1000000000000");
    test("1000000001024", 100, "1000000001024");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("1267650600228229401496703205381", 100, "5");
    test("-1", 5, "-33");
    test("-1", 100, "-1267650600228229401496703205377");
    test("-31", 0, "-32");
    test("-999999998976", 10, "-1000000000000");
    test("-1000000000000", 100, "-1267650600228229402496703205376");
    test("-18446744078004518912", 0, "-18446744078004518912");
    test("-18446744078004518912", 32, "-18446744082299486208");
    test("-18446744078004518912", 33, "-18446744086594453504");
    test("-18446744078004518912", 64, "-18446744078004518912");
    test("-18446744078004518912", 65, "-55340232225423622144");
    test("-36893488143124135936", 32, "-36893488147419103232");
}

#[test]
fn clear_bit_properties() {
    // n.clear_bit(index) is equivalent to n.assign_bit(index, false).
    // If n.get_bit(index), clearing and then setting at index won't do anything.
    // Setting a bit does not increase n.
    // If !n.get_bit(index), clearing at index won't do anything.
    // { n.clear_bit(index) } is equivalent to { n := !n; n.set_bit(index); n := !n }
    let integer_and_u64 = |mut n: Integer, index: u64| {
        let old_n = n.clone();
        n.clear_bit(index);
        assert!(n.is_valid());

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

        let mut m = !&old_n;
        m.set_bit(index);
        m.not_assign();
        let mut n = old_n.clone();
        n.clear_bit(index);
        assert_eq!(m, n);
    };

    for (n, index) in pairs_of_integer_and_small_u64(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }

    for (n, index) in pairs_of_integer_and_small_u64(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }
}
