use std::fmt::Debug;
use std::str::FromStr;

pub const SMALL_LIMIT: usize = 1000;
pub const LARGE_LIMIT: usize = 10000;

pub fn test_eq_helper<T: Debug + Eq + FromStr>(strings: &[&str])
    where T::Err: Debug
{
    let xs: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    let ys: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(i == j, x == y);
        }
    }
}

pub fn test_ord_helper<T: Debug + FromStr + Ord>(strings: &[&str])
    where T::Err: Debug
{
    let xs: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    let ys: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(i.cmp(&j), x.cmp(y));
        }
    }
}
