use common::LARGE_LIMIT;
use malachite_base::num::NotAssign;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::integers;
use rug;
use std::str::FromStr;

#[test]
fn test_not() {
    let test = |s, out| {
        let not = !Integer::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&Integer::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rug::Integer::from_str(s).unwrap()).to_string(), out);

        let mut x = Integer::from_str(s).unwrap();
        x.not_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("-123", "122");
    test("1000000000000", "-1000000000001");
    test("-1000000000000", "999999999999");
    test("-2147483648", "2147483647");
    test("2147483647", "-2147483648");
}

#[test]
fn not_properties() {
    // !x is equivalent for malachite and rug.
    // !x is valid.
    //
    // !&x is equivalent for malachite and rug.
    // !&x is valid.
    // !x and -!x are equivalent.
    //
    // !x != x
    // !!x == x
    // (x >= 0) == (!x < 0)
    let one_integer = |x: Integer| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rug_not = !integer_to_rug_integer(&x);
        assert_eq!(rug_integer_to_integer(&rug_not), not);

        let not_2 = !&x;
        assert!(not_2.is_valid());

        assert_eq!(not_2, not);

        assert_ne!(not, x);
        assert_eq!(!&not, x);
        assert_eq!(x >= 0, not < 0);
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
