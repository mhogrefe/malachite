use malachite_base::slices::slice_set_zero::slice_set_zero;
use malachite_base::slices::slice_test_zero::slice_test_zero;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn slice_test_zero_properties() {
    test_properties(vecs_of_unsigned, |xs: &Vec<u32>| {
        let xs_are_zero = slice_test_zero(xs);
        let mut new_xs = xs.clone();
        slice_set_zero(&mut new_xs);
        assert_eq!(*xs == new_xs, xs_are_zero);
    });
}
