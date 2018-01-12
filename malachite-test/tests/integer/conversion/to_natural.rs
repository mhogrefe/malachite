use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::integer::conversion::to_natural::select_inputs;
use std::str::FromStr;

#[test]
fn test_into_natural() {
    let test = |n, out| {
        let on = Integer::from_str(n).unwrap().into_natural();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Integer::from_str(n).unwrap().to_natural();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Some(0)");
    test("123", "Some(123)");
    test("-123", "None");
    test("1000000000000", "Some(1000000000000)");
    test("-1000000000000", "None");
    test("2147483647", "Some(2147483647)");
    test("2147483648", "Some(2147483648)");
    test("-2147483648", "None");
    test("-2147483649", "None");
}

#[test]
fn to_natural_properties() {
    // x.into_natural() is valid.
    // x.into_natural().to_string() == x.to_string()
    //
    // x.to_natural() is valid.
    // x.to_natural() == x.into_natural()
    //
    // x.to_natural().is_some() == x >= 0
    // if x >= 0, x.to_natural().to_integer() == x
    let one_integer = |x: Integer| {
        let on = x.clone().into_natural();
        assert!(on.clone().map_or(true, |n| n.is_valid()));

        let on_2 = x.to_natural();
        assert!(on_2.clone().map_or(true, |n| n.is_valid()));

        assert_eq!(on.is_some(), x >= 0);
        if let Some(n) = on_2 {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(n.to_integer(), x);
            assert_eq!(n.into_integer(), x);
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
