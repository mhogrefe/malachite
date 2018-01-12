use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::integer::logic::trailing_zeros::select_inputs;
use std::str::FromStr;

#[test]
fn test_trailing_zeros() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().trailing_zeros(), out);
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
    // x.trailing_zeros().is_none() == (x == 0)
    // (-x).trailing_zeros() == x.trailing_zeros()
    // if x != 0, ((!x).trailing_zeros() == Some(0)) == (x.trailing_zeros() == Some(0))
    // if x != 0, x >> x.trailing_zeros() is odd
    // if x != 0, x >> x.trailing_zeros() << x.trailing_zeros() == x
    let one_integer = |x: Integer| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(trailing_zeros.is_none(), x == 0);
        assert_eq!((-&x).trailing_zeros(), trailing_zeros);
        if x != 0 {
            let trailing_zeros = trailing_zeros.unwrap();
            assert_ne!((!&x).trailing_zeros() == Some(0), trailing_zeros == 0);
            if trailing_zeros <= u32::max_value().into() {
                let trailing_zeros = trailing_zeros as u32;
                assert!((&x >> trailing_zeros).is_odd());
                assert_eq!(&x >> trailing_zeros << trailing_zeros, x);
            }
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
