use crate::extra_variadic::Union3;
use malachite_base::test_util::common::test_eq_helper;

#[test]
fn test_eq() {
    test_eq_helper::<Union3<char, u32, bool>>(&[
        "B(8)", "A(d)", "C(true)", "B(5)", "C(false)", "A(a)",
    ]);
}
