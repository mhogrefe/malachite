use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::exhaustive::exhaustive_natural_integers;

#[test]
fn test_exhaustive_natural_integers() {
    assert_eq!(
        exhaustive_natural_integers()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]"
    )
}
