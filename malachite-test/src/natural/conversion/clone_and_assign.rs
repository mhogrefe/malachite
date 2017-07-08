use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer};
use malachite_native::natural as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::natural as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use num;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4, BenchmarkOptions6, benchmark_6};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

pub fn demo_exhaustive_natural_clone(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        println!("clone({}) = {:?}", n, n.clone());
    }
}

pub fn demo_random_natural_clone(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        println!("clone({}) = {:?}", n, n.clone());
    }
}

pub fn demo_exhaustive_natural_clone_from(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

pub fn demo_random_natural_clone_from(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

pub fn demo_exhaustive_natural_assign(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_random_natural_assign(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_exhaustive_natural_assign_ref(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn demo_random_natural_assign_ref(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_exhaustive_natural_clone(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.clone()");
    benchmark_4(BenchmarkOptions4 {
                    xs: exhaustive_naturals(),
                    function_f: &(|n: gmp::Natural| n.clone()),
                    function_g: &(|n: native::Natural| n.clone()),
                    function_h: &(|n: num::BigUint| n.clone()),
                    function_i: &(|n: rugint::Integer| n.clone()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_natural_to_native(x)),
                    z_cons: &(|x| gmp_natural_to_num_biguint(x)),
                    w_cons: &(|x| gmp_natural_to_rugint_integer(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Natural.clone()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_clone(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.clone()");
    benchmark_4(BenchmarkOptions4 {
                    xs: random_naturals(&EXAMPLE_SEED, scale),
                    function_f: &(|n: gmp::Natural| n.clone()),
                    function_g: &(|n: native::Natural| n.clone()),
                    function_h: &(|n: num::BigUint| n.clone()),
                    function_i: &(|n: rugint::Integer| n.clone()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_natural_to_native(x)),
                    z_cons: &(|x| gmp_natural_to_num_biguint(x)),
                    w_cons: &(|x| gmp_natural_to_rugint_integer(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Natural.clone()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_natural_clone_from(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.clone_from(Natural)");
    benchmark_4(BenchmarkOptions4 {
                    xs: exhaustive_pairs_from_single(exhaustive_naturals()),
                    function_f: &(|(mut x, y): (gmp::Natural, gmp::Natural)| x.clone_from(&y)),
                    function_g: &(|(mut x, y): (native::Natural, native::Natural)| {
                                      x.clone_from(&y)
                                  }),
                    function_h: &(|(mut x, y): (num::BigUint, num::BigUint)| x.clone_from(&y)),
                    function_i: &(|(mut x, y): (rugint::Integer, rugint::Integer)| {
                                      x.clone_from(&y)
                                  }),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))
                              }),
                    w_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Natural.clone\\\\_from(Natural)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_clone_from(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.clone_from(Natural)");
    benchmark_4(BenchmarkOptions4 {
                    xs: random_pairs_from_single(random_naturals(&EXAMPLE_SEED, scale)),
                    function_f: &(|(mut x, y): (gmp::Natural, gmp::Natural)| x.clone_from(&y)),
                    function_g: &(|(mut x, y): (native::Natural, native::Natural)| {
                                      x.clone_from(&y)
                                  }),
                    function_h: &(|(mut x, y): (num::BigUint, num::BigUint)| x.clone_from(&y)),
                    function_i: &(|(mut x, y): (rugint::Integer, rugint::Integer)| {
                                      x.clone_from(&y)
                                  }),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))
                              }),
                    w_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Natural.clone\\\\_from(Natural)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_natural_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.assign(Natural)");
    benchmark_6(BenchmarkOptions6 {
                    xs: exhaustive_pairs_from_single(exhaustive_naturals()),
                    function_f: &(|(mut x, y): (gmp::Natural, gmp::Natural)| x.assign(y)),
                    function_g: &(|(mut x, y): (gmp::Natural, gmp::Natural)| x.assign(&y)),
                    function_h: &(|(mut x, y): (native::Natural, native::Natural)| x.assign(y)),
                    function_i: &(|(mut x, y): (native::Natural, native::Natural)| x.assign(&y)),
                    function_j: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
                    function_k: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(&y)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|p| p.clone()),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    w_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    v_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    u_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp by value",
                    g_name: "malachite-gmp by reference",
                    h_name: "malachite-native by value",
                    i_name: "malachite-native by reference",
                    j_name: "rugint by value",
                    k_name: "rugint by reference",
                    title: "Natural.assign(Natural)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.assign(Natural)");
    benchmark_6(BenchmarkOptions6 {
                    xs: random_pairs_from_single(random_naturals(&EXAMPLE_SEED, scale)),
                    function_f: &(|(mut x, y): (gmp::Natural, gmp::Natural)| x.assign(y)),
                    function_g: &(|(mut x, y): (gmp::Natural, gmp::Natural)| x.assign(&y)),
                    function_h: &(|(mut x, y): (native::Natural, native::Natural)| x.assign(y)),
                    function_i: &(|(mut x, y): (native::Natural, native::Natural)| x.assign(&y)),
                    function_j: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
                    function_k: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(&y)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|p| p.clone()),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    w_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    v_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    u_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp by value",
                    g_name: "malachite-gmp by reference",
                    h_name: "malachite-native by value",
                    i_name: "malachite-native by reference",
                    j_name: "rugint by value",
                    k_name: "rugint by reference",
                    title: "Natural.assign(Natural)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
