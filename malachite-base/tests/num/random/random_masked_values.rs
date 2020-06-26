use malachite_base_test_util::stats::common_values_map::common_values_map;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::random::random_masked_values;
use malachite_base::random::{standard_random_values, EXAMPLE_SEED};

fn random_masked_values_helper<T: PrimitiveInteger>(
    pow: u64,
    values: &[T],
    common_values: &[(T, usize)],
) {
    let xs = random_masked_values(standard_random_values::<T>(EXAMPLE_SEED), pow);
    assert_eq!(xs.clone().take(20).collect::<Vec<T>>(), values);
    assert_eq!(common_values_map(1_000_000, 10, xs), common_values)
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_random_masked_values() {
    random_masked_values_helper::<u8>(0, &[0; 20], &[(0, 1_000_000)]);
    random_masked_values_helper::<u8>(
        1,
        &[1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0],
        &[(0, 500187), (1, 499813)],
    );
    random_masked_values_helper::<u8>(
        2,
        &[1, 0, 3, 0, 1, 1, 1, 3, 3, 0, 1, 0, 2, 3, 0, 0, 2, 3, 3, 0],
        &[(3, 250600), (0, 250417), (2, 249770), (1, 249213)],
    );
    random_masked_values_helper::<u8>(
        3,
        &[1, 4, 7, 4, 5, 5, 5, 7, 7, 0, 1, 4, 2, 7, 0, 4, 2, 7, 3, 0],
        &[
            (7, 125446),
            (0, 125424),
            (2, 125277),
            (3, 125154),
            (4, 124993),
            (1, 124724),
            (6, 124493),
            (5, 124489),
        ],
    );
    random_masked_values_helper::<u8>(
        7,
        &[
            113, 100, 87, 60, 93, 61, 117, 23, 7, 72, 105, 12, 114, 39, 104, 100, 114, 111, 107, 72,
        ],
        &[
            (121, 8065),
            (55, 8045),
            (88, 8031),
            (80, 8005),
            (27, 8004),
            (45, 7997),
            (74, 7997),
            (63, 7966),
            (2, 7958),
            (68, 7954),
        ],
    );
    random_masked_values_helper::<u8>(
        8,
        &[
            113, 228, 87, 188, 93, 189, 117, 151, 7, 72, 233, 12, 114, 39, 104, 228, 242, 239, 235,
            200,
        ],
        &[
            (88, 4062),
            (121, 4052),
            (173, 4045),
            (47, 4041),
            (27, 4036),
            (123, 4034),
            (74, 4032),
            (183, 4030),
            (16, 4021),
            (55, 4015),
        ],
    );

    random_masked_values_helper::<i8>(0, &[0; 20], &[(0, 1_000_000)]);
    random_masked_values_helper::<i8>(
        1,
        &[1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0],
        &[(0, 500187), (1, 499813)],
    );
    random_masked_values_helper::<i8>(
        2,
        &[1, 0, 3, 0, 1, 1, 1, 3, 3, 0, 1, 0, 2, 3, 0, 0, 2, 3, 3, 0],
        &[(3, 250600), (0, 250417), (2, 249770), (1, 249213)],
    );
    random_masked_values_helper::<i8>(
        3,
        &[1, 4, 7, 4, 5, 5, 5, 7, 7, 0, 1, 4, 2, 7, 0, 4, 2, 7, 3, 0],
        &[
            (7, 125446),
            (0, 125424),
            (2, 125277),
            (3, 125154),
            (4, 124993),
            (1, 124724),
            (6, 124493),
            (5, 124489),
        ],
    );
    random_masked_values_helper::<i8>(
        7,
        &[
            113, 100, 87, 60, 93, 61, 117, 23, 7, 72, 105, 12, 114, 39, 104, 100, 114, 111, 107, 72,
        ],
        &[
            (121, 8065),
            (55, 8045),
            (88, 8031),
            (80, 8005),
            (27, 8004),
            (45, 7997),
            (74, 7997),
            (63, 7966),
            (2, 7958),
            (68, 7954),
        ],
    );
    random_masked_values_helper::<i8>(
        8,
        &[
            113, -28, 87, -68, 93, -67, 117, -105, 7, 72, -23, 12, 114, 39, 104, -28, -14, -17,
            -21, -56,
        ],
        &[
            (88, 4062),
            (121, 4052),
            (-83, 4045),
            (47, 4041),
            (27, 4036),
            (123, 4034),
            (74, 4032),
            (-73, 4030),
            (16, 4021),
            (55, 4015),
        ],
    );
}
