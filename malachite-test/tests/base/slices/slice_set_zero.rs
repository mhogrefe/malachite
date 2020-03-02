use malachite_base::slices::{slice_set_zero, slice_test_zero};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn slice_set_zero_properties() {
    test_properties(vecs_of_unsigned, |xs: &Vec<u32>| {
        let mut mut_xs = xs.clone();
        slice_set_zero(&mut mut_xs);
        assert_eq!(xs.len(), mut_xs.len());
        assert!(slice_test_zero(&mut_xs));
    });
}
