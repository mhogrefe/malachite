extern crate serde;
extern crate serde_json;

use malachite_base::strings::string_is_subset;
use malachite_nz::integer::Integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::integer::integers;

#[test]
fn serde_properties() {
    test_properties(integers, |x| {
        let s = serde_json::to_string(&x).unwrap();
        assert_eq!(serde_json::from_str::<Integer>(&s).unwrap(), *x);
        assert!(string_is_subset(&s, r#"",0123456789:LS[]abefgilmnrstu{}"#));
    });
}
