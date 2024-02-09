use crate::test_util::stats::common_values_map::common_values_map_debug;
use crate::test_util::stats::median;
use itertools::Itertools;
use std::fmt::Debug;
use std::hash::Hash;

pub fn random_vecs_helper_helper<T, I: Clone + Iterator<Item = Vec<T>>>(
    xss: I,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) where
    T: Clone + Debug + Eq + Hash + Ord,
{
    let values = xss.clone().take(20).collect_vec();
    let values = values.iter().map(Vec::as_slice).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xss.clone());
    let common_values = common_values
        .iter()
        .map(|(xs, f)| (xs.as_slice(), *f))
        .collect_vec();
    let (a, ob) = median(xss.take(1000000));
    let median = (a.as_slice(), ob.as_deref());
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}
