use malachite_base_test_util::stats::common_values_map::common_values_map;

use malachite_base::random::{standard_random_values, EXAMPLE_SEED};
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_standard_random_values() {
    let xs = standard_random_values::<RoundingMode>(EXAMPLE_SEED);
    assert_eq!(
        xs.clone().take(20).collect::<Vec<RoundingMode>>(),
        &[
            Floor, Exact, Floor, Nearest, Up, Ceiling, Ceiling, Exact, Nearest, Up, Ceiling,
            Nearest, Down, Nearest, Nearest, Nearest, Nearest, Down, Nearest, Up
        ]
    );
    assert_eq!(
        common_values_map(1_000_000, 10, xs),
        &[
            (Exact, 167171),
            (Down, 166909),
            (Floor, 166674),
            (Ceiling, 166617),
            (Nearest, 166350),
            (Up, 166279)
        ]
    )
}
