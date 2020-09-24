use malachite_base_test_util::stats::common_values_map::common_values_map;

use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::random::random_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{Ceiling, Down, Exact, Floor, Nearest, Up};

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_random_rounding_modes() {
    let xs = random_rounding_modes(EXAMPLE_SEED);
    let values = xs.clone().take(20).collect::<Vec<_>>();
    let common_values = common_values_map(1000000, 10, xs.clone());
    assert_eq!(
        (values.as_slice(), common_values.as_slice()),
        (
            &[
                Up, Exact, Ceiling, Up, Floor, Nearest, Exact, Up, Floor, Exact, Nearest, Down,
                Exact, Down, Floor, Exact, Floor, Down, Nearest, Down
            ][..],
            &[
                (Ceiling, 167408),
                (Down, 167104),
                (Nearest, 166935),
                (Exact, 166549),
                (Floor, 166068),
                (Up, 165936)
            ][..]
        )
    );
}
