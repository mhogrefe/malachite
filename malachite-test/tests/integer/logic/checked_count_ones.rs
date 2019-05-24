use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_test::inputs::base::natural_signeds;
use malachite_test::inputs::integer::integers;
use malachite_test::integer::logic::checked_count_ones::{
    integer_checked_count_ones_alt_1, integer_checked_count_ones_alt_2,
};
use std::str::FromStr;

#[test]
fn test_checked_count_ones() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().checked_count_ones(), out);
        assert_eq!(
            integer_checked_count_ones_alt_1(&Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            integer_checked_count_ones_alt_2(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", Some(0));
    test("105", Some(4));
    test("-105", None);
    test("1000000000000", Some(13));
    test("-1000000000000", None);
    test("4294967295", Some(32));
    test("-4294967295", None);
    test("4294967296", Some(1));
    test("-4294967296", None);
    test("18446744073709551615", Some(64));
    test("-18446744073709551615", None);
    test("18446744073709551616", Some(1));
    test("-18446744073709551616", None);
}

#[test]
fn checked_count_ones_properties() {
    test_properties(integers, |x| {
        let ones = x.checked_count_ones();
        assert_eq!(integer_checked_count_ones_alt_1(x), ones);
        assert_eq!(integer_checked_count_ones_alt_2(x), ones);
        assert_eq!(ones == Some(0), *x == 0 as Limb);
        assert_eq!((!x).checked_count_zeros(), ones);
    });

    test_properties(natural_signeds::<SignedLimb>, |&i| {
        assert_eq!(
            Integer::from(i).checked_count_ones(),
            Some(u64::from(i.count_ones()))
        );
    });
}
