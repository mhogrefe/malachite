use malachite_base::named::Named;
use malachite_nz::integer::Integer;

#[test]
fn test_named() {
    assert_eq!(Integer::NAME, "Integer");
}
