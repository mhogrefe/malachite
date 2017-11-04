use common::gmp_natural_to_native;
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use rust_wheels::benchmarks::{benchmark_2, BenchmarkOptions2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_triples_from_single, random_triples_from_single};
use std::cmp::max;

pub fn demo_exhaustive_natural_sub_mul_assign(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_naturals())
        .filter(|&(ref a, ref b, ref c)| a >= &(b * c))
        .take(limit)
    {
        let a_old = a.clone();
        a.sub_mul_assign(&b, &c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, &{}); x = {}",
            a_old,
            b,
            c,
            a
        );
    }
}

pub fn demo_random_natural_sub_mul_assign(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32))
        .filter(|&(ref a, ref b, ref c)| a >= &(b * c))
        .take(limit)
    {
        let a_old = a.clone();
        a.sub_mul_assign(&b, &c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, &{}); x = {}",
            a_old,
            b,
            c,
            a
        );
    }
}

pub fn demo_exhaustive_natural_sub_mul(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
        let a_old = a.clone();
        println!(
            "{}.sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            a.sub_mul(&b, &c)
        );
    }
}

pub fn demo_random_natural_sub_mul(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        let a_old = a.clone();
        println!(
            "{}.sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            a.sub_mul(&b, &c)
        );
    }
}

pub fn demo_exhaustive_natural_sub_mul_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, &c)
        );
    }
}

pub fn demo_random_natural_sub_mul_ref(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, &c)
        );
    }
}

pub fn benchmark_exhaustive_natural_sub_mul_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.sub_mul_assign(&Natural, &Natural)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals())
            .filter(|&(ref a, ref b, ref c)| a >= &(b * c)),
        function_f: &(|(mut a, b, c): (gmp::Natural, gmp::Natural, gmp::Natural)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_sub_mul_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.sub_mul_assign(&Natural, &Natural)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale))
            .filter(|&(ref a, ref b, ref c)| a >= &(b * c)),
        function_f: &(|(mut a, b, c): (gmp::Natural, gmp::Natural, gmp::Natural)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_sub_mul_assign_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.sub_mul_assign(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals())
            .filter(|&(ref a, ref b, ref c)| a >= &(b * c)),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a -= &(&b * &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        g_name: "Natural -= \\\\&Natural * \\\\&Natural",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_sub_mul_assign_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.sub_mul_assign(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale))
            .filter(|&(ref a, ref b, ref c)| a >= &(b * c)),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a -= &(&b * &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        g_name: "Natural -= \\\\&Natural * \\\\&Natural",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_sub_mul(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.sub_mul(&Natural, &Natural)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (gmp::Natural, gmp::Natural, gmp::Natural)| a.sub_mul(&b, &c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul(&b, &c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_sub_mul(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.sub_mul(&Natural, &Natural)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (gmp::Natural, gmp::Natural, gmp::Natural)| a.sub_mul(&b, &c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul(&b, &c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_sub_mul_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.sub_mul(&Natural, &Natural) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          (&a).sub_mul(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        title: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_sub_mul_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.sub_mul(&Natural, &Natural) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          (&a).sub_mul(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        title: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_sub_mul_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.sub_mul(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a - &(&b * &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "Natural - \\\\&Natural * \\\\&Natural",
        title: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_sub_mul_algorithms(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.sub_mul(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a - &(&b * &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "Natural - \\\\&Natural * \\\\&Natural",
        title: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_sub_mul_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Natural).sub_mul(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          (&a).sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          &a - &(&b * &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "(\\\\&Natural) - \\\\&Natural * \\\\&Natural",
        title: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_sub_mul_ref_algorithms(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random (&Natural).sub_mul(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          (&a).sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          &a - &(&b * &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "(\\\\&Natural) - \\\\&Natural * \\\\&Natural",
        title: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
