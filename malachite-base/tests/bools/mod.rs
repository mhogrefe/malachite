use malachite_base::comparison::traits::{Max, Min};
use malachite_base::crement::Crementable;
use malachite_base::num::logic::traits::NotAssign;

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

#[test]
fn test_min() {
    assert_eq!(bool::MIN, false);
}

#[test]
fn test_max() {
    assert_eq!(bool::MAX, true);
}

#[test]
fn test_bool_not_assign() {
    let test = |mut b: bool, out| {
        b.not_assign();
        assert_eq!(b, out);
    };
    test(false, true);
    test(true, false);
}
