use common::LARGE_LIMIT;
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::natural::conversion::to_integer::select_inputs;
use std::str::FromStr;

#[test]
fn test_into_integer() {
    let test = |s| {
        let x = Natural::from_str(s).unwrap().into_integer();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = Natural::from_str(s).unwrap().to_integer();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);
    };
    test("0");
    test("123");
    test("1000000000000");
    test("4294967295");
    test("4294967296");
}

#[test]
fn to_integer_properties() {
    // x.into_integer() is valid.
    // x.into_integer().to_string() == x.to_string()
    //
    // x.to_integer() is valid.
    // x.to_integer() == x.into_integer()
    //
    // x.to_integer().to_natural() == x
    let one_natural = |x: Natural| {
        let result = x.clone().into_integer();
        assert!(result.is_valid());
        assert_eq!(result.to_string(), x.to_string());

        let result_2 = x.to_integer();
        assert!(result_2.is_valid());
        assert_eq!(result_2, result);

        assert_eq!(result_2.to_natural().unwrap(), x);
        assert_eq!(result_2.into_natural().unwrap(), x);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
