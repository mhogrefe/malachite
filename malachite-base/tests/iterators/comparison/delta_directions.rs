use std::cmp::Ordering;

use malachite_base::iterators::comparison::delta_directions;

fn delta_directions_helper(xs: &[u8], result: &[Ordering]) {
    assert_eq!(delta_directions(xs.iter()).collect::<Vec<_>>(), result);
    assert_eq!(result.len(), xs.len().saturating_sub(1));
    assert_eq!(
        delta_directions(xs.iter().rev())
            .map(Ordering::reverse)
            .collect::<Vec<_>>(),
        result.iter().cloned().rev().collect::<Vec<_>>()
    );
}

#[test]
fn test_delta_directions() {
    delta_directions_helper(&[], &[]);
    delta_directions_helper(&[5], &[]);
    delta_directions_helper(&[5, 6], &[Ordering::Greater]);
    delta_directions_helper(&[5, 5], &[Ordering::Equal]);
    delta_directions_helper(&[5, 4], &[Ordering::Less]);
    delta_directions_helper(&[1, 2, 3, 4], &[Ordering::Greater; 3]);
    delta_directions_helper(
        &[1, 2, 2, 4],
        &[Ordering::Greater, Ordering::Equal, Ordering::Greater],
    );
    delta_directions_helper(
        &[1, 3, 2, 4],
        &[Ordering::Greater, Ordering::Less, Ordering::Greater],
    );
    delta_directions_helper(
        &[3, 1, 4, 1, 5, 9],
        &[
            Ordering::Less,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Greater,
        ],
    );
}
