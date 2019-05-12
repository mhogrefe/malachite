use malachite_base::crement::Crementable;

#[test]
fn test_bool_increment() {
    let test = |mut b: bool, out| {
        b.increment();
        assert_eq!(b, out);
    };
    test(false, true);
}

#[test]
#[should_panic]
fn bool_increment_fail() {
    let mut b = true;
    b.increment();
}
