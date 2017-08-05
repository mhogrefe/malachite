use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use num::bigint::Sign;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::cmp::Ordering;

pub fn num_sign(x: &num::BigInt) -> Ordering {
    match x.sign() {
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
        Sign::Minus => Ordering::Less,
    }
}

pub fn demo_exhaustive_integer_sign(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

pub fn demo_random_integer_sign(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

pub fn benchmark_exhaustive_integer_sign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer sign");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.sign()),
        function_g: &(|n: native::Integer| n.sign()),
        function_h: &(|n: num::BigInt| num_sign(&n)),
        function_i: &(|n: rugint::Integer| n.sign()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        z_cons: &(|x| gmp_integer_to_num_bigint(x)),
        w_cons: &(|x| gmp_integer_to_rugint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer sign",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer sign");
    benchmark_4(BenchmarkOptions4 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Integer| n.sign()),
        function_g: &(|n: native::Integer| n.sign()),
        function_h: &(|n: num::BigInt| num_sign(&n)),
        function_i: &(|n: rugint::Integer| n.sign()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        z_cons: &(|x| gmp_integer_to_num_bigint(x)),
        w_cons: &(|x| gmp_integer_to_rugint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer sign",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
