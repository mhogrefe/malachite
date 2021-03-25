use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::biguint_to_natural;
use malachite_nz_test_util::natural::arithmetic::checked_sub::checked_sub;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_checked_sub_natural() {
    let test = |u, v, out| {
        let on = Natural::from_str(u)
            .unwrap()
            .checked_sub(Natural::from_str(v).unwrap());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::from_str(u)
            .unwrap()
            .checked_sub(&Natural::from_str(v).unwrap());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&Natural::from_str(u).unwrap()).checked_sub(Natural::from_str(v).unwrap());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&Natural::from_str(u).unwrap()).checked_sub(&Natural::from_str(v).unwrap());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = checked_sub(BigUint::from_str(u).unwrap(), BigUint::from_str(v).unwrap())
            .map(|x| biguint_to_natural(&x));
        assert_eq!(on.to_debug_string(), out);

        let on = checked_sub(
            rug::Integer::from_str(u).unwrap(),
            rug::Integer::from_str(v).unwrap(),
        );
        assert_eq!(on.to_debug_string(), out);
    };
    test("0", "0", "Some(0)");
    test("0", "123", "None");
    test("123", "0", "Some(123)");
    test("456", "123", "Some(333)");
    test("1000000000000", "123", "Some(999999999877)");
    test("123", "1000000000000", "None");
    test(
        "12345678987654321",
        "314159265358979",
        "Some(12031519722295342)",
    );
    test("4294967296", "1", "Some(4294967295)");
    test("4294967295", "4294967295", "Some(0)");
    test("4294967296", "4294967295", "Some(1)");
    test("4294967296", "4294967296", "Some(0)");
    test("4294967295", "4294967296", "None");
    test("18446744073709551616", "1", "Some(18446744073709551615)");
    test("18446744073709551615", "18446744073709551615", "Some(0)");
    test("18446744073709551616", "18446744073709551615", "Some(1)");
    test("18446744073709551615", "18446744073709551616", "None");
    test(
        "70734740290631708",
        "282942734368",
        "Some(70734457347897340)",
    );
    test("282942734368", "70734740290631708", "None");
}
