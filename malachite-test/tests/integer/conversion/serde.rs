extern crate serde;
extern crate serde_json;

use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_test::inputs::integer::integers;
use std::str::FromStr;

use rust_wheels::prim_utils::string_utils::string_is_subset;

#[test]
fn test_serde() {
    let test = |n, out| {
        assert_eq!(
            serde_json::to_string(&Integer::from_str(n).unwrap()).unwrap(),
            out
        );
        assert_eq!(serde_json::from_str::<Integer>(out).unwrap().to_string(), n);
    };
    test("0", r#"{"sign":true,"abs":{"Small":0}}"#);
    test("100", r#"{"sign":true,"abs":{"Small":100}}"#);
    test(
        "1000000000000",
        r#"{"sign":true,"abs":{"Large":[3567587328,232]}}"#,
    );
    test("4294967295", r#"{"sign":true,"abs":{"Small":4294967295}}"#);
    test("4294967296", r#"{"sign":true,"abs":{"Large":[0,1]}}"#);
    test(
        "18446744073709551615",
        r#"{"sign":true,"abs":{"Large":[4294967295,4294967295]}}"#,
    );
    test(
        "18446744073709551616",
        r#"{"sign":true,"abs":{"Large":[0,0,1]}}"#,
    );
    test("-100", r#"{"sign":false,"abs":{"Small":100}}"#);
    test(
        "-1000000000000",
        r#"{"sign":false,"abs":{"Large":[3567587328,232]}}"#,
    );
    test(
        "-4294967295",
        r#"{"sign":false,"abs":{"Small":4294967295}}"#,
    );
    test("-4294967296", r#"{"sign":false,"abs":{"Large":[0,1]}}"#);
    test(
        "-18446744073709551615",
        r#"{"sign":false,"abs":{"Large":[4294967295,4294967295]}}"#,
    );
    test(
        "-18446744073709551616",
        r#"{"sign":false,"abs":{"Large":[0,0,1]}}"#,
    );
}

#[test]
fn serde_properties() {
    test_properties(integers, |x| {
        let s = serde_json::to_string(&x).unwrap();
        assert_eq!(serde_json::from_str::<Integer>(&s).unwrap(), *x);
        assert!(string_is_subset(&s, r#"",0123456789:LS[]abefgilmnrstu{}"#));
    });
}
