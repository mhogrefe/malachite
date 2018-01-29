use common::GenerationMode;
use inputs::natural::{triples_of_naturals, triples_of_naturals_var_1};
use malachite_base::num::SignificantBits;
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};
use std::cmp::max;

pub fn demo_natural_sub_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, &c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, &{}); x = {}",
            a_old, b, c, a
        );
    }
}

pub fn demo_natural_sub_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
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

pub fn demo_natural_sub_mul_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
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

pub fn benchmark_natural_sub_mul_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.sub_mul_assign(&Natural, &Natural)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_naturals_var_1(gm),
        function_f: &(|(mut a, b, c): (Natural, Natural, Natural)| a.sub_mul_assign(&b, &c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_assign_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.sub_mul_assign(&Natural, &Natural) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_naturals_var_1(gm),
        function_f: &(|(mut a, b, c): (Natural, Natural, Natural)| a.sub_mul_assign(&b, &c)),
        function_g: &(|(mut a, b, c): (Natural, Natural, Natural)| a -= &(&b * &c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
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

pub fn benchmark_natural_sub_mul(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.sub_mul(&Natural, &Natural)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_naturals(gm),
        function_f: &(|(a, b, c): (Natural, Natural, Natural)| a.sub_mul(&b, &c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite",
        title: "Natural.sub\\\\_mul(\\\\&Natural, \\\\&Natural)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.sub_mul(&Natural, &Natural) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_naturals(gm),
        function_f: &(|(a, b, c): (Natural, Natural, Natural)| a.sub_mul(&b, &c)),
        function_g: &(|(a, b, c): (Natural, Natural, Natural)| (&a).sub_mul(&b, &c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
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

pub fn benchmark_natural_sub_mul_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.sub_mul(&Natural, &Natural) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_naturals(gm),
        function_f: &(|(a, b, c): (Natural, Natural, Natural)| a.sub_mul(&b, &c)),
        function_g: &(|(a, b, c): (Natural, Natural, Natural)| a - &(&b * &c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
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

pub fn benchmark_natural_sub_mul_ref_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} (&Natural).sub_mul(&Natural, &Natural) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_naturals(gm),
        function_f: &(|(a, b, c): (Natural, Natural, Natural)| (&a).sub_mul(&b, &c)),
        function_g: &(|(a, b, c): (Natural, Natural, Natural)| &a - &(&b * &c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
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
