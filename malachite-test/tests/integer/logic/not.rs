use std::str::FromStr;

use malachite_base::num::logic::traits::NotAssign;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;
use malachite_test::inputs::natural::naturals;

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
    test_properties(integers, |x| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rug_not = !integer_to_rug_integer(x);
        assert_eq!(rug_integer_to_integer(&rug_not), not);

        let not_alt = !x;
        assert!(not_alt.is_valid());

        assert_eq!(not_alt, not);

        assert_ne!(not, *x);
        assert_eq!(!&not, *x);
        assert_eq!(*x >= 0, not < 0);
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(!Integer::from(i), !i);
    });

    test_properties(naturals, |x| {
        assert_eq!(!Integer::from(x), !x);
    });
}
