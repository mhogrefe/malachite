use itertools::Itertools;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::exhaustive::exhaustive_natural_range;
use malachite_nz::natural::Natural;

fn expected_range_len(a: &Natural, b: &Natural) -> usize {
    usize::exact_from(b) - usize::exact_from(a)
}

fn exhaustive_natural_range_helper(a: Natural, b: Natural, values: &str) {
    let xs = exhaustive_natural_range(a.clone(), b.clone())
        .take(20)
        .collect_vec()
        .to_debug_string();
    assert_eq!(xs, values);
    let len = expected_range_len(&a, &b);
    assert_eq!(exhaustive_natural_range(a.clone(), b.clone()).count(), len);
    let mut init = exhaustive_natural_range(a, b)
        .rev()
        .skip(len.saturating_sub(20))
        .collect_vec();
    init.reverse();
    assert_eq!(xs, init.to_debug_string());
}

#[test]
fn test_exhaustive_natural_range() {
    exhaustive_natural_range_helper(Natural::ZERO, Natural::ZERO, "[]");
    exhaustive_natural_range_helper(Natural::ZERO, Natural::ONE, "[0]");
    exhaustive_natural_range_helper(
        Natural::ZERO,
        Natural::exact_from(10),
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]",
    );
    exhaustive_natural_range_helper(
        Natural::exact_from(10),
        Natural::exact_from(20),
        "[10, 11, 12, 13, 14, 15, 16, 17, 18, 19]",
    );
    exhaustive_natural_range_helper(
        Natural::exact_from(10),
        Natural::exact_from(100),
        "[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]",
    );
}

#[test]
#[should_panic]
fn exhaustive_natural_range_fail() {
    exhaustive_natural_range(Natural::ONE, Natural::ZERO);
}
