use malachite_base::num::comparison::traits::OrdAbs;
use malachite_base_test_util::common::test_custom_cmp_helper;
use malachite_nz::integer::Integer;
use rug;

#[test]
fn test_ord_abs() {
    let strings =
        &["0", "1", "-2", "123", "-124", "999999999999", "-1000000000000", "1000000000001"];
    test_custom_cmp_helper::<Integer, _>(strings, OrdAbs::cmp_abs);
    test_custom_cmp_helper::<rug::Integer, _>(strings, rug::Integer::cmp_abs);
}
