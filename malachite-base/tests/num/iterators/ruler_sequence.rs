use itertools::Itertools;
use malachite_base::num::iterators::ruler_sequence;

#[test]
pub fn test_ruler_sequence() {
    assert_eq!(
        ruler_sequence::<u32>().take(100).collect_vec(),
        &[
            0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0,
            1, 0, 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1,
            0, 2, 0, 1, 0, 6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2, 0, 1, 0,
            3, 0, 1, 0, 2, 0, 1, 0, 5, 0, 1, 0, 2
        ]
    );
}
