use std::fmt::Debug;
use std::str::FromStr;

pub fn test_eq_helper<T: Debug + Eq + FromStr>(strings: &[&str])
    where T::Err: Debug
{
    let xs: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    let ys: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            if i == j {
                assert_eq!(x, y);
            } else {
                assert_ne!(x, y);
            }
        }
    }
}
