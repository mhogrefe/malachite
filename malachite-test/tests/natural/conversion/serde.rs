extern crate serde;
extern crate serde_json;

use common::test_properties;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;

use rust_wheels::prim_utils::string_utils::string_is_subset;

//TODO just use a simple hex string (or base64)

#[test]
fn test_serde() {
    let test = |n, out| {
        assert_eq!(
            serde_json::to_string(&Natural::from_str(n).unwrap()).unwrap(),
            out
        );
        assert_eq!(serde_json::from_str::<Natural>(out).unwrap().to_string(), n);
    };
    test("0", r#"{"Small":0}"#);
    test("100", r#"{"Small":100}"#);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000", r#"{"Large":[3567587328,232]}"#);
        test("4294967295", r#"{"Small":4294967295}"#);
        test("4294967296", r#"{"Large":[0,1]}"#);
        test(
            "18446744073709551615",
            r#"{"Large":[4294967295,4294967295]}"#,
        );
        test("18446744073709551616", r#"{"Large":[0,0,1]}"#);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test(
            "1000000000000000000000000",
            r#"{"Large":[2003764205206896640,54210]}"#,
        );
        test("18446744073709551615", r#"{"Small":18446744073709551615}"#);
        test("18446744073709551616", r#"{"Large":[0,1]}"#);
        test(
            "340282366920938463463374607431768211455",
            r#"{"Large":[18446744073709551615,18446744073709551615]}"#,
        );
        test(
            "340282366920938463463374607431768211456",
            r#"{"Large":[0,0,1]}"#,
        );
    }
}

#[test]
fn serde_properties() {
    test_properties(naturals, |x| {
        let s = serde_json::to_string(&x).unwrap();
        assert_eq!(serde_json::from_str::<Natural>(&s).unwrap(), *x);
        assert!(string_is_subset(&s, r#"",0123456789:LS[]aeglmr{}"#));
    });
}
