use malachite_base::num::logic::traits::NotAssign;
use malachite_base_test_util::generators::bool_gen;

#[test]
fn test_not_assign() {
    let test = |mut b: bool, out| {
        b.not_assign();
        assert_eq!(b, out);
    };
    test(false, true);
    test(true, false);
}

#[test]
fn not_assign_properties() {
    bool_gen().test_properties(|b| {
        let mut mut_b = b;
        mut_b.not_assign();
        assert_ne!(mut_b, b);
        assert_eq!(mut_b, !b);
        mut_b.not_assign();
        assert_eq!(mut_b, b);
    });
}
