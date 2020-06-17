use std::mem::size_of;

use malachite_nz::natural::Natural;

#[test]
fn test_size() {
    if size_of::<usize>() == 8 {
        assert_eq!(size_of::<Natural>(), 32);
    }
}
