use crate::extra_variadic::Union3;
use malachite_base::test_util::common::test_cmp_helper;

#[test]
fn test_cmp() {
    test_cmp_helper::<Union3<char, u32, bool>>(&[
        "A(a)", "A(d)", "B(5)", "B(8)", "C(false)", "C(true)",
    ]);
}
