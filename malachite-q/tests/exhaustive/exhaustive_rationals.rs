use itertools::Itertools;
use malachite_base::strings::ExtraToString;
use malachite_q::exhaustive::exhaustive_rationals;

#[test]
fn test_exhaustive_rationals() {
    assert_eq!(
        exhaustive_rationals().take(20).collect_vec().to_string(),
        "[0, 1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, -3/2, 2/3, -2/3, 3, -3, 1/4, -1/4, 4/3, \
        -4/3, 3/5]"
    )
}
