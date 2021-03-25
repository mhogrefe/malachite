use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::exhaustive::integer_decreasing_range_to_negative_infinity;
use malachite_nz::integer::Integer;

fn integer_decreasing_range_to_negative_infinity_helper(a: Integer, values: &str) {
    let xs = integer_decreasing_range_to_negative_infinity(a)
        .take(20)
        .collect_vec()
        .to_debug_string();
    assert_eq!(xs, values);
}

#[test]
fn test_integer_decreasing_range_to_negative_infinity() {
    integer_decreasing_range_to_negative_infinity_helper(
        Integer::ZERO,
        "[0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19]",
    );
    integer_decreasing_range_to_negative_infinity_helper(
        Integer::from(10),
        "[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9]",
    );
    integer_decreasing_range_to_negative_infinity_helper(
        Integer::from(-10),
        "[-10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20, -21, -22, -23, -24, -25, -26, \
        -27, -28, -29]",
    );
}
