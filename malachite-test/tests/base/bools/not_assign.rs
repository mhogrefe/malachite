use malachite_base::num::traits::NotAssign;

use common::test_properties_no_limit_exhaustive_no_special;
use malachite_test::inputs::base::bools;

#[test]
fn test_bool_not_assign() {
    let test = |mut b: bool, out| {
        b.not_assign();
        assert_eq!(b, out);
    };
    test(false, true);
    test(true, false);
}

#[test]
fn bool_not_assign_properties() {
    test_properties_no_limit_exhaustive_no_special(bools, |&b| {
        let mut mut_b = b;
        mut_b.not_assign();
        assert_ne!(mut_b, b);
        assert_eq!(mut_b, !b);
        mut_b.not_assign();
        assert_eq!(mut_b, b);
    });
}
