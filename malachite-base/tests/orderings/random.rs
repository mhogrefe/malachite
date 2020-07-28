use std::cmp::Ordering::{Equal, Greater, Less};

use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;

use malachite_base::orderings::random::random_orderings;
use malachite_base::random::EXAMPLE_SEED;

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_random_orderings() {
    let xs = random_orderings(EXAMPLE_SEED);
    let values = xs.clone().take(20).collect::<Vec<_>>();
    let common_values = common_values_map_debug(1_000_000, 10, xs.clone());
    let median = median(xs.take(1_000_000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (
            &[
                Less, Equal, Less, Greater, Less, Less, Equal, Less, Equal, Greater, Less, Equal,
                Less, Greater, Greater, Equal, Less, Equal, Greater, Greater
            ][..],
            &[(Less, 333784), (Greater, 333516), (Equal, 332700)][..],
            (Equal, None)
        )
    );
}
