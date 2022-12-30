use crate::integer::Integer;
use malachite_base::num::basic::traits::Zero;

pub fn integer_sum_alt<I: Iterator<Item = Integer>>(xs: I) -> Integer {
    let mut stack = Vec::new();
    for (i, x) in xs.enumerate().map(|(i, x)| (i + 1, x)) {
        let mut s = x;
        for _ in 0..i.trailing_zeros() {
            s += stack.pop().unwrap();
        }
        stack.push(s);
    }
    let mut s = Integer::ZERO;
    for x in stack {
        s += x;
    }
    s
}
