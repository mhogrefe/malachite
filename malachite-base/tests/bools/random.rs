use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{disc_uniform_dist_assertions, MomentStats};

use malachite_base::random::{standard_random_values, EXAMPLE_SEED};

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_standard_random_values() {
    let xs = standard_random_values::<bool>(EXAMPLE_SEED);
    disc_uniform_dist_assertions(
        xs,
        &false,
        &true,
        &[
            false, true, false, true, false, true, false, false, true, true, false, true, true,
            true, true, false, true, true, false, true,
        ],
        &[(true, 500680), (false, 499320)],
        NiceFloat(0.5),
        (true, None),
        MomentStats {
            mean: NiceFloat(0.5),
            stdev: NiceFloat(0.5),
            skewness: NiceFloat(0.0),
            kurtosis: NiceFloat(-2.0),
        },
        MomentStats {
            mean: NiceFloat(0.5006800000000157),
            stdev: NiceFloat(0.49999978759972746),
            skewness: NiceFloat(-0.0027200025154596134),
            kurtosis: NiceFloat(-1.9999926015862308),
        },
    );
}
