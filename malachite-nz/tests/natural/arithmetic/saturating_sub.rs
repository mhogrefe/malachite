use malachite_base::num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_saturating_sub_natural() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_assign(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_assign(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).saturating_sub(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).saturating_sub(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("123", "1000000000000", "0");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("4294967295", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
    test("18446744073709551615", "18446744073709551616", "0");
    test("70734740290631708", "282942734368", "70734457347897340");
    test("282942734368", "70734740290631708", "0");
}
