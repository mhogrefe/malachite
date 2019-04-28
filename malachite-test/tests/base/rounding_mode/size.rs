use std::mem::size_of;

use malachite_base::round::RoundingMode;

#[test]
fn test_size() {
    assert_eq!(size_of::<RoundingMode>(), 1);
}
