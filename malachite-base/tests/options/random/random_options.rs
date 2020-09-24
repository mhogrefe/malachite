use core::hash::Hash;
use std::fmt::Debug;

use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;

use malachite_base::num::random::random_primitive_ints;
use malachite_base::options::random::random_options;
use malachite_base::random::EXAMPLE_SEED;

fn random_options_helper<I: Clone + Iterator>(
    xs: I,
    w_numerator: u64,
    w_denominator: u64,
    expected_values: &[Option<I::Item>],
    expected_common_values: &[(Option<I::Item>, usize)],
    expected_median: (Option<I::Item>, Option<Option<I::Item>>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_options(EXAMPLE_SEED, xs, w_numerator, w_denominator);
    let values = xs.clone().take(20).collect::<Vec<_>>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_random_options() {
    // w = 1
    random_options_helper(
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("xs")),
        1,
        1,
        &[
            None,
            Some(85),
            Some(11),
            Some(136),
            None,
            None,
            None,
            Some(200),
            None,
            None,
            None,
            None,
            Some(235),
            None,
            None,
            None,
            None,
            Some(134),
            None,
            Some(203),
        ],
        &[
            (None, 500473),
            (Some(81), 2076),
            (Some(208), 2066),
            (Some(35), 2065),
            (Some(211), 2045),
            (Some(112), 2042),
            (Some(143), 2039),
            (Some(162), 2037),
            (Some(170), 2036),
            (Some(58), 2035),
        ],
        (None, None),
    );
    // w = 1/50
    random_options_helper(
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("xs")),
        1,
        50,
        &[None; 20],
        &[
            (None, 980406),
            (Some(18), 101),
            (Some(25), 99),
            (Some(116), 97),
            (Some(226), 97),
            (Some(237), 97),
            (Some(23), 95),
            (Some(185), 95),
            (Some(30), 94),
            (Some(73), 94),
        ],
        (None, None),
    );
    // w = 50
    random_options_helper(
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("xs")),
        50,
        1,
        &[
            Some(85),
            Some(11),
            Some(136),
            Some(200),
            Some(235),
            Some(134),
            Some(203),
            Some(223),
            Some(38),
            Some(235),
            Some(217),
            Some(177),
            Some(162),
            Some(32),
            Some(166),
            Some(234),
            None,
            Some(30),
            Some(218),
            Some(90),
        ],
        &[
            (None, 19398),
            (Some(58), 4030),
            (Some(81), 4002),
            (Some(194), 3981),
            (Some(66), 3973),
            (Some(64), 3969),
            (Some(4), 3965),
            (Some(143), 3965),
            (Some(196), 3953),
            (Some(208), 3942),
        ],
        (Some(125), None),
    );
    // w = 10
    random_options_helper(
        random_options(
            EXAMPLE_SEED.fork("inner"),
            random_primitive_ints::<u8>(EXAMPLE_SEED.fork("xs")),
            10,
            1,
        ),
        10,
        1,
        &[
            Some(Some(85)),
            Some(Some(11)),
            Some(None),
            Some(Some(136)),
            Some(None),
            Some(Some(200)),
            Some(Some(235)),
            Some(Some(134)),
            None,
            Some(Some(203)),
            None,
            Some(Some(223)),
            Some(Some(38)),
            Some(Some(235)),
            Some(Some(217)),
            Some(Some(177)),
            Some(Some(162)),
            Some(Some(32)),
            Some(Some(166)),
            Some(Some(234)),
        ],
        &[
            (None, 90786),
            (Some(None), 82785),
            (Some(Some(58)), 3416),
            (Some(Some(81)), 3403),
            (Some(Some(37)), 3363),
            (Some(Some(4)), 3359),
            (Some(Some(220)), 3358),
            (Some(Some(162)), 3353),
            (Some(Some(143)), 3344),
            (Some(Some(150)), 3334),
        ],
        (Some(Some(101)), None),
    );
}

#[test]
#[should_panic]
fn random_options_fail_1() {
    random_options(
        EXAMPLE_SEED,
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("xs")),
        0,
        1,
    );
}

#[test]
#[should_panic]
fn random_options_fail_2() {
    random_options(
        EXAMPLE_SEED,
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("xs")),
        1,
        0,
    );
}

#[test]
#[should_panic]
fn random_options_fail_3() {
    random_options(
        EXAMPLE_SEED,
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("xs")),
        1,
        u64::MAX,
    );
}

#[test]
#[should_panic]
fn random_options_fail_4() {
    random_options(
        EXAMPLE_SEED,
        random_primitive_ints::<u8>(EXAMPLE_SEED.fork("xs")),
        u64::MAX,
        1,
    );
}
