use malachite_base::num::logic::traits::CountOnes;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::integer::logic::checked_count_ones::{
    integer_checked_count_ones_alt_1, integer_checked_count_ones_alt_2,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::natural_signeds;
use malachite_test::inputs::integer::integers;

#[test]
fn checked_count_ones_properties() {
    test_properties(integers, |x| {
        let ones = x.checked_count_ones();
        assert_eq!(integer_checked_count_ones_alt_1(x), ones);
        assert_eq!(integer_checked_count_ones_alt_2(x), ones);
        assert_eq!(ones == Some(0), *x == 0);
        assert_eq!((!x).checked_count_zeros(), ones);
    });

    test_properties(natural_signeds::<SignedLimb>, |&i| {
        assert_eq!(
            Integer::from(i).checked_count_ones(),
            Some(CountOnes::count_ones(i))
        );
    });
}
