use malachite_nz::integer::Integer;
use std::mem::size_of;

#[test]
fn test_size() {
    if size_of::<usize>() == 8 {
        assert_eq!(size_of::<Integer>(), 32);
    }
}
