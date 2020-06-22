use malachite_base::slices::{slice_leading_zeros, slice_test_zero, slice_trailing_zeros};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn slice_trailing_zeros_properties() {
    test_properties(vecs_of_unsigned, |xs: &Vec<u32>| {
        let trailing_zeros = slice_trailing_zeros(xs);
        assert!(trailing_zeros <= xs.len());
        assert_eq!(trailing_zeros == xs.len(), slice_test_zero(xs));
        let mut new_xs = xs.clone();
        new_xs.reverse();
        assert_eq!(slice_leading_zeros(&new_xs), trailing_zeros);
    });
}
