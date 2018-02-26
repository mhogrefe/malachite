use common::test_properties;
use malachite_base::num::BitAccess;
use malachite_base::num::NotAssign;
use malachite_nz::integer::Integer;
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
    test("-4294967295", 0, "-4294967296");
}

#[test]
fn clear_bit_properties() {
    test_properties(pairs_of_integer_and_small_u64, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.clear_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, false);
        assert_eq!(mut_n, result);

        assert!(result <= *n);
        if n.get_bit(index) {
            assert_ne!(result, *n);
            let mut mut_result = result.clone();
            mut_result.set_bit(index);
            assert_eq!(mut_result, *n);
        } else {
            assert_eq!(result, *n);
        }

        let mut mut_not_n = !n;
        mut_not_n.set_bit(index);
        mut_not_n.not_assign();
        assert_eq!(mut_not_n, result);
    });
}
