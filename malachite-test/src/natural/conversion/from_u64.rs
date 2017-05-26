use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rust_wheels::benchmarks::benchmark_3;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

pub fn demo_exhaustive_natural_from_u64(limit: usize) {
    for u in exhaustive_u::<u64>().take(limit) {
        println!("from({}) = {}", u, gmp::Natural::from(u));
    }
}

pub fn demo_random_natural_from_u64(limit: usize) {
    for u in random_x::<u64>(&EXAMPLE_SEED).take(limit) {
        println!("from({}) = {}", u, gmp::Natural::from(u));
    }
}

pub fn benchmark_exhaustive_natural_from_u64(limit: usize, file_name: &str) {
    benchmark_3(exhaustive_u::<u64>(),
                &(|u| gmp::Natural::from(u)),
                &(|u| native::Natural::from(u)),
                &(|u| num::BigUint::from(u)),
                &(|&u| u),
                &(|&u| u),
                &(|&u| (64 - u.leading_zeros()) as usize),
                limit,
                "malachite-gmp",
                "malachite-native",
                "num",
                "Natural::from(u64)",
                "u.significant\\\\_bits()",
                "time (ns)",
                &format!("benchmarks/{}", file_name));
}

pub fn benchmark_random_natural_from_u64(limit: usize, file_name: &str) {
    benchmark_3(random_x::<u64>(&EXAMPLE_SEED),
                &(|u| gmp::Natural::from(u)),
                &(|u| native::Natural::from(u)),
                &(|u| num::BigUint::from(u)),
                &(|&u| u),
                &(|&u| u),
                &(|&u| (64 - u.leading_zeros()) as usize),
                limit,
                "malachite-gmp",
                "malachite-native",
                "num",
                "Natural::from(u64)",
                "u.significant\\\\_bits()",
                "time (ns)",
                &format!("benchmarks/{}", file_name));
}
