use itertools::Itertools;
use num::arithmetic::traits::Parity;

pub fn median<I: Iterator>(xs: I) -> (I::Item, Option<I::Item>)
where
    I::Item: Eq + Ord,
{
    let mut xs = xs.collect_vec();
    assert!(!xs.is_empty());
    xs.sort_unstable();
    let n = xs.len();
    let half_n = n >> 1;
    if n.even() {
        // swap-remove m_2 first because if n == 2 it's the last element of the list.
        let m_2 = xs.swap_remove(half_n);
        let m_1 = xs.swap_remove(half_n - 1);
        if m_1 == m_2 {
            (m_1, None)
        } else {
            (m_1, Some(m_2))
        }
    } else {
        (xs.swap_remove(half_n), None)
    }
}

pub mod common_values_map;
pub mod median;
pub mod moments;
