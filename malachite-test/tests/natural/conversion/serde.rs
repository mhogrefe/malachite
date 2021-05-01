extern crate serde;
extern crate serde_json;

use malachite_base::strings::string_is_subset;
use malachite_nz::natural::Natural;

use malachite_test::common::test_properties;
use malachite_test::inputs::natural::naturals;

#[test]
fn serde_properties() {
    test_properties(naturals, |x| {
        let s = serde_json::to_string(&x).unwrap();
        assert_eq!(serde_json::from_str::<Natural>(&s).unwrap(), *x);
        assert!(string_is_subset(&s, "\"0123456789abcdefx"));
    });
}
