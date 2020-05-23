use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

fn read_strings<T: FromStr>(strings: &[&str]) -> Vec<T>
where
    T::Err: Debug,
{
    strings.iter().map(|s| s.parse().unwrap()).collect()
}

fn test_helper_helper<
    T: Debug + FromStr,
    U: Debug + Eq,
    F: FnMut(usize, usize) -> U,
    G: FnMut(&T, &T) -> U,
>(
    strings: &[&str],
    mut compare_indices: F,
    mut compare_elements: G,
) where
    T::Err: Debug,
{
    let xs = read_strings::<T>(strings);
    let ys = read_strings::<T>(strings);
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(compare_indices(i, j), compare_elements(x, y));
        }
    }
}

pub fn test_eq_helper<T: Debug + Eq + FromStr>(strings: &[&str])
where
    T::Err: Debug,
{
    test_helper_helper(strings, |i, j| i == j, |x: &T, y: &T| x == y);
}

pub fn test_cmp_helper<T: Debug + FromStr + Ord>(strings: &[&str])
where
    T::Err: Debug,
{
    test_helper_helper(strings, |i, j| i.cmp(&j), |x: &T, y: &T| x.cmp(y));
}

pub fn test_custom_cmp_helper<T: Debug + FromStr, F: FnMut(&T, &T) -> Ordering>(
    strings: &[&str],
    compare: F,
) where
    T::Err: Debug,
{
    test_helper_helper(strings, |i, j| i.cmp(&j), compare);
}
