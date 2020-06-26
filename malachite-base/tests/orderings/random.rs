use std::cmp::Ordering::{self, Equal, Greater, Less};

use malachite_base_test_util::stats::common_values_map::common_values_map_debug;

use malachite_base::random::{standard_random_values, EXAMPLE_SEED};

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_standard_random_values() {
    let xs = standard_random_values::<Ordering>(EXAMPLE_SEED);
    assert_eq!(
        xs.clone().take(20).collect::<Vec<Ordering>>(),
        &[
            Less, Less, Greater, Equal, Less, Less, Less, Greater, Less, Greater, Equal, Greater,
            Greater, Greater, Greater, Equal, Greater, Equal, Equal, Greater
        ]
    );
    assert_eq!(
        common_values_map_debug(1_000_000, 10, xs),
        &[(Greater, 333624), (Equal, 333501), (Less, 332875)]
    )
}
