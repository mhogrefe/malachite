use std::ops::Sub;

pub fn checked_sub<T: Ord + Sub>(x: T, y: T) -> Option<<T as Sub>::Output> {
    if x >= y {
        Some(x - y)
    } else {
        None
    }
}
