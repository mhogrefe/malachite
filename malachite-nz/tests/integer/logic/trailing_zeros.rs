use malachite_nz::integer::Integer;
use malachite_nz_test_util::integer::logic::trailing_zeros::integer_trailing_zeros_alt;
use std::str::FromStr;

#[test]
fn test_trailing_zeros() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.trailing_zeros(), out);
        assert_eq!(integer_trailing_zeros_alt(&n), out);
    };
    test("0", None);
    test("123", Some(0));
    test("-123", Some(0));
    test("1000000000000", Some(12));
    test("-1000000000000", Some(12));
    test("4294967295", Some(0));
    test("-4294967295", Some(0));
    test("4294967296", Some(32));
    test("-4294967296", Some(32));
    test("18446744073709551615", Some(0));
    test("-18446744073709551615", Some(0));
    test("18446744073709551616", Some(64));
    test("-18446744073709551616", Some(64));
}
