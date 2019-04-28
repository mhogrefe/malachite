extern crate malachite_test; // TODO why is this needed?

use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

use malachite_test::common::{GenerationMode, NoSpecialGenerationMode};

pub const SMALL_LIMIT: usize = 1_000;
pub const LARGE_LIMIT: usize = 10_000;

pub fn test_eq_helper<T: Debug + Eq + FromStr>(strings: &[&str])
where
    T::Err: Debug,
{
    let xs: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    let ys: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(i == j, x == y);
        }
    }
}

pub fn test_cmp_helper<T: Debug + FromStr + Ord>(strings: &[&str])
where
    T::Err: Debug,
{
    let xs: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    let ys: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(i.cmp(&j), x.cmp(y));
        }
    }
}

pub fn test_custom_cmp_helper<T: Debug + FromStr + Ord, F: FnMut(&T, &T) -> Ordering>(
    strings: &[&str],
    mut compare: F,
) where
    T::Err: Debug,
{
    let xs: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    let ys: Vec<T> = strings.iter().map(|s| s.parse().unwrap()).collect();
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(i.cmp(&j), compare(x, y));
        }
    }
}

pub fn test_properties<T, G: Fn(GenerationMode) -> Box<Iterator<Item = T>>, F: FnMut(&T)>(
    gen: G,
    mut test: F,
) {
    for &gm in &[
        GenerationMode::Exhaustive,
        GenerationMode::Random(32),
        GenerationMode::SpecialRandom(32),
    ] {
        for x in gen(gm).take(LARGE_LIMIT) {
            test(&x);
        }
    }
}

pub fn test_properties_no_special<
    T,
    G: Fn(NoSpecialGenerationMode) -> Box<Iterator<Item = T>>,
    F: FnMut(&T),
>(
    gen: G,
    mut test: F,
) {
    for &gm in &[
        NoSpecialGenerationMode::Exhaustive,
        NoSpecialGenerationMode::Random(32),
    ] {
        for x in gen(gm).take(LARGE_LIMIT) {
            test(&x);
        }
    }
}

pub fn test_properties_custom_scale<
    T,
    G: Fn(GenerationMode) -> Box<Iterator<Item = T>>,
    F: FnMut(&T),
>(
    scale: u32,
    gen: G,
    mut test: F,
) {
    for &gm in &[
        GenerationMode::Exhaustive,
        GenerationMode::Random(scale),
        GenerationMode::SpecialRandom(scale),
    ] {
        for x in gen(gm).take(LARGE_LIMIT) {
            test(&x);
        }
    }
}

pub fn test_properties_no_limit_exhaustive_no_special<
    T,
    G: Fn(NoSpecialGenerationMode) -> Box<Iterator<Item = T>>,
    F: FnMut(&T),
>(
    gen: G,
    mut test: F,
) {
    for x in gen(NoSpecialGenerationMode::Exhaustive) {
        test(&x);
    }

    for x in gen(NoSpecialGenerationMode::Random(32)).take(LARGE_LIMIT) {
        test(&x);
    }
}
