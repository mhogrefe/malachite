use malachite_base::num::logic::traits::NotAssign;

#[test]
fn test_bool_not_assign() {
    let test = |mut b: bool, out| {
        b.not_assign();
        assert_eq!(b, out);
    };
    test(false, true);
    test(true, false);
}
