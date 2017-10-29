use common::gmp_natural_to_native;
use malachite_base::traits::{AddMul, AddMulAssign};
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use rust_wheels::benchmarks::{benchmark_2, BenchmarkOptions2, benchmark_4, BenchmarkOptions4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_triples, random_triples};
use std::cmp::max;

pub fn demo_exhaustive_natural_add_mul_assign_u32(limit: usize) {
    for (mut a, b, c) in exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old,
            b_old,
            c,
            a
        );
    }
}

pub fn demo_random_natural_add_mul_assign_u32(limit: usize) {
    for (mut a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old,
            b_old,
            c,
            a
        );
    }
}

pub fn demo_exhaustive_natural_add_mul_assign_u32_ref(limit: usize) {
    for (mut a, b, c) in exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        a.add_mul_assign(&b, c);
        println!("a := {}; x.add_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_random_natural_add_mul_assign_u32_ref(limit: usize) {
    for (mut a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        a.add_mul_assign(&b, c);
        println!("a := {}; x.add_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_exhaustive_natural_add_mul_u32(limit: usize) {
    for (a, b, c) in exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.add_mul({}, {}) = {}", a_old, b_old, c, a.add_mul(b, c));
    }
}

pub fn demo_random_natural_add_mul_u32(limit: usize) {
    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.add_mul({}, {}) = {}", a_old, b_old, c, a.add_mul(b, c));
    }
}

pub fn demo_exhaustive_natural_add_mul_u32_val_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, {}) = {}", a_old, b, c, a.add_mul(&b, c));
    }
}

pub fn demo_random_natural_add_mul_u32_val_ref(limit: usize) {
    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, {}) = {}", a_old, b, c, a.add_mul(&b, c));
    }
}

pub fn demo_exhaustive_natural_add_mul_u32_ref_val(limit: usize) {
    for (a, b, c) in exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).add_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).add_mul(b, c)
        );
    }
}

pub fn demo_random_natural_add_mul_u32_ref_val(limit: usize) {
    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).add_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).add_mul(b, c)
        );
    }
}

pub fn demo_exhaustive_natural_add_mul_u32_ref_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        println!(
            "(&{}).add_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).add_mul(&b, c)
        );
    }
}

pub fn demo_random_natural_add_mul_u32_ref_ref(limit: usize) {
    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        println!(
            "(&{}).add_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).add_mul(&b, c)
        );
    }
}

pub fn benchmark_exhaustive_natural_add_mul_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul_assign(Natural, u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(mut a, b, c): (gmp::Natural, gmp::Natural, u32)| a.add_mul_assign(b, c)),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.add_mul_assign(Natural, u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(mut a, b, c): (gmp::Natural, gmp::Natural, u32)| a.add_mul_assign(b, c)),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_assign_u32_evaluation_strategy(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.add_mul_assign(Natural, u32) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(&b, c)
                      }),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        g_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_u32_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul_assign(Natural, u32) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(&b, c)
                      }),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        g_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_assign_u32_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul_assign(Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| a += b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        g_name: "Natural += Natural * u32",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_u32_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul_assign(Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| a += b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        g_name: "Natural += Natural * u32",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_assign_u32_ref_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.add_mul_assign(&Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(&b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| a += &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        g_name: "Natural += \\\\&Natural * u32",
        title: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_u32_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul_assign(&Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
                          a.add_mul_assign(&b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| a += &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        g_name: "Natural += \\\\&Natural * u32",
        title: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(Natural, u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (gmp::Natural, gmp::Natural, u32)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.add\\\\_mul(Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.add_mul(Natural, u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (gmp::Natural, gmp::Natural, u32)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.add\\\\_mul(Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_u32_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(Natural, u32) evaluation strategy");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(&b, c)),
        function_h: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).add_mul(b, c)),
        function_i: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).add_mul(&b, c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        z_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        w_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, u32)",
        g_name: "Natural.add\\\\_mul(\\\\&Natural, u32)",
        h_name: "(\\\\&Natural).add\\\\_mul(Natural, u32)",
        i_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32)",
        title: "Natural.add\\\\_mul(\\\\&Natural, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_u32_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul(Natural, u32) evaluation strategy");
    benchmark_4(BenchmarkOptions4 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(&b, c)),
        function_h: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).add_mul(b, c)),
        function_i: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).add_mul(&b, c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        z_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        w_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, u32)",
        g_name: "Natural.add\\\\_mul(\\\\&Natural, u32)",
        h_name: "(\\\\&Natural).add\\\\_mul(Natural, u32)",
        i_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32)",
        title: "Natural.add\\\\_mul(\\\\&Natural, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_u32_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a + b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, u32)",
        g_name: "Natural + Natural * u32",
        title: "Natural.add\\\\_mul(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_u32_algorithms(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.add_mul(Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a + b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, u32)",
        g_name: "Natural + Natural * u32",
        title: "Natural.add\\\\_mul(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_u32_val_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(&Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a + &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(\\\\&Natural, u32)",
        g_name: "Natural + \\\\&Natural * u32",
        title: "Natural.add\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_u32_val_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul(&Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.add_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a + &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(\\\\&Natural, u32)",
        g_name: "Natural + \\\\&Natural * u32",
        title: "Natural.add\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_u32_ref_val_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Natural).add_mul(Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| &a + b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Natural).add\\\\_mul(Natural, u32)",
        g_name: "(\\\\&Natural) + Natural * u32",
        title: "(\\\\&Natural).add\\\\_mul(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_u32_ref_val_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random (&Natural).add_mul(Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| &a + b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Natural).add\\\\_mul(Natural, u32)",
        g_name: "(\\\\&Natural) + Natural * u32",
        title: "(\\\\&Natural).add\\\\_mul(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_u32_ref_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Natural).add_mul(&Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_naturals(),
            exhaustive_naturals(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).add_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| &a + &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32)",
        g_name: "(\\\\&Natural) + \\\\&Natural * u32",
        title: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_u32_ref_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random (&Natural).add_mul(&Natural, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).add_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| &a + &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32)",
        g_name: "(\\\\&Natural) + \\\\&Natural * u32",
        title: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
