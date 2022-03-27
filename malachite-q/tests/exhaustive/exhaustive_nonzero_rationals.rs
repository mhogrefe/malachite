use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_q::exhaustive::exhaustive_nonzero_rationals;

#[test]
fn test_exhaustive_nonzero_rationals() {
    assert_eq!(
        exhaustive_nonzero_rationals()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, -3/2, 2/3, -2/3, 3, -3, 1/4, -1/4, 4/3, -4/3, \
        3/5, -3/5]"
    )
}
