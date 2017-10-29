use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_native::integer as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};

pub fn demo_exhaustive_integer_mul_assign(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {}; x *= {}; x = {}", x_old, y, x);
    }
}

pub fn demo_random_integer_mul_assign(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {}; x *= {}; x = {}", x_old, y, x);
    }
}

pub fn demo_exhaustive_integer_mul_assign_ref(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {}; x *= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_random_integer_mul_assign_ref(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {}; x *= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_exhaustive_integer_mul(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

pub fn demo_random_integer_mul(limit: usize) {
    for (x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

pub fn demo_exhaustive_integer_mul_val_ref(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

pub fn demo_random_integer_mul_val_ref(limit: usize) {
    for (x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

pub fn demo_exhaustive_integer_mul_ref_val(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

pub fn demo_random_integer_mul_ref_val(limit: usize) {
    for (x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

pub fn demo_exhaustive_integer_mul_ref_ref(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

pub fn demo_random_integer_mul_ref_ref(limit: usize) {
    for (x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

pub fn benchmark_exhaustive_integer_mul_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer *= Integer");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs_from_single(exhaustive_integers()),
        function_f: &(|(mut x, y)| x *= y),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x *= y),
        function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x *= y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer *= Integer",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mul_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer *= Integer");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut x, y)| x *= y),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x *= y),
        function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x *= y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer *= Integer",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_mul_assign_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer *= Integer evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs_from_single(exhaustive_integers()),
        function_f: &(|(mut x, y)| x *= y),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x *= &y),
        x_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "Integer *= Integer",
        g_name: "Integer *= \\\\&Integer",
        title: "Integer *= Integer evaluation strategy",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mul_assign_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer *= Integer evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut x, y)| x *= y),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x *= &y),
        x_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "Integer *= Integer",
        g_name: "Integer *= \\\\&Integer",
        title: "Integer *= Integer evaluation strategy",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_mul(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer * Integer");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs_from_single(exhaustive_integers()),
        function_f: &(|(x, y)| x * y),
        function_g: &(|(x, y)| x * y),
        function_h: &(|(x, y)| x * y),
        function_i: &(|(x, y)| x * y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_num_bigint(x), gmp_integer_to_num_bigint(y))),
        w_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer * Integer",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mul(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer * Integer");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(x, y)| x * y),
        function_g: &(|(x, y)| x * y),
        function_h: &(|(x, y)| x * y),
        function_i: &(|(x, y)| x * y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_num_bigint(x), gmp_integer_to_num_bigint(y))),
        w_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer * Integer",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_mul_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer * Integer evaluation strategy");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs_from_single(exhaustive_integers()),
        function_f: &(|(x, y)| x * y),
        function_g: &(|(x, y)| x * &y),
        function_h: &(|(x, y)| &x * y),
        function_i: &(|(x, y)| &x * &y),
        x_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        w_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "Integer * Integer",
        g_name: "Integer * \\\\&Integer",
        h_name: "\\\\&Integer * Integer",
        i_name: "\\\\&Integer * \\\\&Integer",
        title: "Integer * Integer evaluation strategy",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mul_evaluation_strategy(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer * Integer evaluation strategy");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(x, y)| x * y),
        function_g: &(|(x, y)| x * &y),
        function_h: &(|(x, y)| &x * y),
        function_i: &(|(x, y)| &x * &y),
        x_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        w_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "Integer * Integer",
        g_name: "Integer * \\\\&Integer",
        h_name: "\\\\&Integer * Integer",
        i_name: "\\\\&Integer * \\\\&Integer",
        title: "Integer * Integer evaluation strategy",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
