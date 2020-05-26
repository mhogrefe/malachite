use std::str::FromStr;

use malachite_base::num::logic::traits::NotAssign;
use rug;

use malachite_nz::integer::Integer;

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
