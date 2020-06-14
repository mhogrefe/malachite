use malachite_base::slices::slice_leading_zeros::slice_leading_zeros;
use malachite_base::slices::slice_test_zero::slice_test_zero;
use malachite_base::slices::slice_trailing_zeros::slice_trailing_zeros;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn slice_leading_zeros_properties() {
    test_properties(vecs_of_unsigned, |xs: &Vec<u32>| {
        let leading_zeros = slice_leading_zeros(xs);
        assert!(leading_zeros <= xs.len());
        assert_eq!(leading_zeros == xs.len(), slice_test_zero(xs));
        let mut new_xs = xs.clone();
        new_xs.reverse();
        assert_eq!(slice_trailing_zeros(&new_xs), leading_zeros);
    });
}
