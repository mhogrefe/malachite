use malachite_q::Rational;
use std::mem::size_of;

#[test]
fn test_size() {
    if size_of::<usize>() == 8 {
        assert_eq!(size_of::<Rational>(), 56);
    }
}
