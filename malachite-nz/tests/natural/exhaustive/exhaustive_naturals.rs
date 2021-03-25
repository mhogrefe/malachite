use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::exhaustive::exhaustive_naturals;

#[test]
fn test_exhaustive_naturals() {
    assert_eq!(
        exhaustive_naturals()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]"
    )
}
