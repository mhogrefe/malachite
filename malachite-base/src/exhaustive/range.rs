use std::fmt::Display;

use comparison::traits::{Max, Min};
use crement::Crementable;

//TODO to be replaced by std ranges once relevant traits are stable

#[derive(Clone)]
pub struct RangeIncreasing<T: Crementable> {
    i: T,
    b: T,
    done: bool,
}

impl<T: Clone + Crementable> Iterator for RangeIncreasing<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.done = self.i == self.b;
            let ret = self.i.clone();
            if !self.done {
                self.i.increment();
            }
            Some(ret)
        }
    }
}

pub fn range_increasing<T: Display + Crementable>(a: T, b: T) -> RangeIncreasing<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeIncreasing {
        i: a,
        b,
        done: false,
    }
}

#[derive(Clone)]
pub struct RangeDecreasing<T: Crementable> {
    a: T,
    i: T,
    done: bool,
}

impl<T: Clone + Crementable> Iterator for RangeDecreasing<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.done = self.i == self.a;
            let ret = self.i.clone();
            if !self.done {
                self.i.decrement();
            }
            Some(ret)
        }
    }
}

pub fn range_decreasing<T: Display + Crementable>(a: T, b: T) -> RangeDecreasing<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeDecreasing {
        a,
        i: b,
        done: false,
    }
}

#[inline]
pub fn range_up_increasing<T: Crementable + Display + Max>(a: T) -> RangeIncreasing<T> {
    range_increasing(a, T::MAX)
}

#[inline]
pub fn range_up_decreasing<T: Crementable + Display + Max>(a: T) -> RangeDecreasing<T> {
    range_decreasing(a, T::MAX)
}

#[inline]
pub fn range_down_increasing<T: Crementable + Display + Min>(b: T) -> RangeIncreasing<T> {
    range_increasing(T::MIN, b)
}

#[inline]
pub fn range_down_decreasing<T: Crementable + Display + Min>(b: T) -> RangeDecreasing<T> {
    range_decreasing(T::MIN, b)
}

#[inline]
pub fn increasing<T: Crementable + Display + Max + Min>() -> RangeIncreasing<T> {
    range_increasing(T::MIN, T::MAX)
}

#[inline]
pub fn decreasing<T: Crementable + Display + Max + Min>() -> RangeDecreasing<T> {
    range_decreasing(T::MIN, T::MAX)
}
