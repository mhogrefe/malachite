use common::{gmp_integer_to_native, GenerationMode};
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions4, benchmark_2, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_triples, random_triples};
use std::cmp::max;

type It = Iterator<Item = (gmp::Integer, gmp::Integer, u32)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_triples(
        exhaustive_integers(),
        exhaustive_integers(),
        exhaustive_u(),
    ))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_sub_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

pub fn demo_integer_sub_mul_assign_u32_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_integer_sub_mul_u32(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.sub_mul({}, {}) = {}", a_old, b_old, c, a.sub_mul(b, c));
    }
}

pub fn demo_integer_sub_mul_u32_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {}", a_old, b, c, a.sub_mul(&b, c));
    }
}

pub fn demo_integer_sub_mul_u32_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).sub_mul(b, c)
        );
    }
}

pub fn demo_integer_sub_mul_u32_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

pub fn benchmark_integer_sub_mul_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.sub_mul_assign(Integer, u32)",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (gmp::Integer, gmp::Integer, u32)| a.sub_mul_assign(b, c)),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
            a.sub_mul_assign(b, c)
        }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_assign_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul_assign(Integer, u32) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
            a.sub_mul_assign(b, c)
        }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
            a.sub_mul_assign(&b, c)
        }),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        g_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32)",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul_assign(Integer, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
            a.sub_mul_assign(b, c)
        }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| a -= b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        g_name: "Integer -= Integer * u32",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_assign_u32_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul_assign(&Integer, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
            a.sub_mul_assign(&b, c)
        }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| a -= &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32)",
        g_name: "Integer -= \\\\&Integer * u32",
        title: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.sub_mul(Integer, u32)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (gmp::Integer, gmp::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul(Integer, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul(Integer, u32) evaluation strategy",
        gm.name()
    );
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(&b, c)),
        function_h: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(b, c)),
        function_i: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(&b, c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        z_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        w_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub\\\\_mul(Integer, u32)",
        g_name: "Integer.sub\\\\_mul(\\\\&Integer, u32)",
        h_name: "(\\\\&Integer).sub\\\\_mul(Integer, u32)",
        i_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32)",
        title: "Integer.sub\\\\_mul(\\\\&Integer, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.sub_mul(Integer, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a - b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub\\\\_mul(Integer, u32)",
        g_name: "Integer - Integer * u32",
        title: "Integer.sub\\\\_mul(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_u32_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul(&Integer, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a - &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub\\\\_mul(\\\\&Integer, u32)",
        g_name: "Integer - \\\\&Integer * u32",
        title: "Integer.sub\\\\_mul(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_u32_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Integer).sub_mul(Integer, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| &a - b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Integer).sub\\\\_mul(Integer, u32)",
        g_name: "(\\\\&Integer) - Integer * u32",
        title: "(\\\\&Integer).sub\\\\_mul(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_u32_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Integer).sub_mul(&Integer, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| &a - &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32)",
        g_name: "(\\\\&Integer) - \\\\&Integer * u32",
        title: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
