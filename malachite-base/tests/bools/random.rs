use malachite_base_test_util::stats::common_values_map::common_values_map;

use malachite_base::random::{standard_random_values, EXAMPLE_SEED};

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_standard_random_values() {
    let xs = standard_random_values::<bool>(EXAMPLE_SEED);
    assert_eq!(
        xs.clone().take(20).collect::<Vec<bool>>(),
        &[
            false, true, false, true, false, true, false, false, true, true, false, true, true,
            true, true, false, true, true, false, true
        ]
    );
    assert_eq!(
        common_values_map(1_000_000, 10, xs),
        &[(true, 500680), (false, 499320)]
    )
}
