use malachite_base::crement::Crementable;

#[test]
fn test_bool_decrement() {
    let test = |mut b: bool, out| {
        b.decrement();
        assert_eq!(b, out);
    };
    test(true, false);
}

#[test]
#[should_panic]
fn bool_decrement_fail() {
    let mut b = false;
    b.decrement();
}
