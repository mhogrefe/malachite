use malachite_base::rounding_modes::RoundingMode;
use std::mem::size_of;

#[test]
fn test_size() {
    assert_eq!(size_of::<RoundingMode>(), 1);
}
