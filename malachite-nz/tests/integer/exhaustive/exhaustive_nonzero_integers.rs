use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::exhaustive::exhaustive_nonzero_integers;

#[test]
fn test_exhaustive_nonzero_integers() {
    assert_eq!(
        exhaustive_nonzero_integers()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10, -10]"
    )
}
