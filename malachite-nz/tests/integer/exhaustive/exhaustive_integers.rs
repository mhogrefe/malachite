use malachite_base::strings::ToDebugString;

use malachite_nz::integer::exhaustive::exhaustive_integers;

#[test]
fn test_exhaustive_integers() {
    assert_eq!(
        exhaustive_integers()
            .take(20)
            .collect::<Vec<_>>()
            .to_debug_string(),
        "[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10]"
    )
}
