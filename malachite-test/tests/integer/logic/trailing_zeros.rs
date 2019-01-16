use common::test_properties;
use malachite_base::misc::Max;
use malachite_base::num::Parity;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_test::inputs::base::nonzero_signeds;
use malachite_test::inputs::integer::integers;
use malachite_test::integer::logic::trailing_zeros::integer_trailing_zeros_alt;
use std::str::FromStr;

#[test]
fn test_trailing_zeros() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().trailing_zeros(), out);
        assert_eq!(
            integer_trailing_zeros_alt(&Integer::from_str(n).unwrap()),
            out
        );
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

#[test]
fn trailing_zeros_properties() {
    test_properties(integers, |x| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(integer_trailing_zeros_alt(x), trailing_zeros);
        assert_eq!(trailing_zeros.is_none(), *x == 0 as Limb);
        assert_eq!((-x).trailing_zeros(), trailing_zeros);
        if *x != 0 as Limb {
            let trailing_zeros = trailing_zeros.unwrap();
            assert_ne!((!x).trailing_zeros() == Some(0), trailing_zeros == 0);
            if trailing_zeros <= u64::from(Limb::MAX) {
                assert!((x >> trailing_zeros).odd());
                assert_eq!(x >> trailing_zeros << trailing_zeros, *x);
            }
        }
    });

    test_properties(nonzero_signeds::<SignedLimb>, |&i| {
        assert_eq!(
            Integer::from(i).trailing_zeros(),
            Some(u64::from(i.trailing_zeros()))
        );
    });
}
