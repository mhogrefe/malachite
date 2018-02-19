use common::GenerationMode;
use inputs::natural::{triples_of_natural_natural_and_unsigned,
                      triples_of_natural_natural_and_u32_var_1};
use malachite_base::num::SignificantBits;
use malachite_base::num::{SubMul, SubMulAssign};
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};
use std::cmp::max;

pub fn demo_natural_sub_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_u32_var_1(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_natural_sub_mul_u32(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {:?}", a_old, b, c, a.sub_mul(&b, c));
    }
}

pub fn demo_natural_sub_mul_u32_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {:?}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

pub fn benchmark_natural_sub_mul_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.sub_mul_assign(&Natural, u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_natural_natural_and_u32_var_1(gm),
        function_f: &mut (|(mut a, b, c): (Natural, Natural, u32)| a.sub_mul_assign(&b, c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.sub_mul_assign(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_natural_natural_and_u32_var_1(gm),
        function_f: &mut (|(mut a, b, c): (Natural, Natural, u32)| a.sub_mul_assign(&b, c)),
        function_g: &mut (|(mut a, b, c): (Natural, Natural, u32)| a -= &(&b * c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        g_name: "Natural -= \\\\&Natural * u32",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.sub_mul(&Natural, u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_natural_natural_and_unsigned::<u32>(gm),
        function_f: &mut (|(a, b, c): (Natural, Natural, u32)| a.sub_mul(&b, c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        title: "Natural.sub\\\\_mul(\\\\&Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.sub_mul(&Natural, u32) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_natural_natural_and_unsigned::<u32>(gm),
        function_f: &mut (|(a, b, c): (Natural, Natural, u32)| a.sub_mul(&b, c)),
        function_g: &mut (|(a, b, c): (Natural, Natural, u32)| (&a).sub_mul(&b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.sub\\\\_mul(\\\\&Natural, u32)",
        g_name: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, u32)",
        title: "Natural.sub\\\\_mul(\\\\&Natural, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.sub_mul(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_natural_natural_and_unsigned::<u32>(gm),
        function_f: &mut (|(a, b, c): (Natural, Natural, u32)| a.sub_mul(&b, c)),
        function_g: &mut (|(a, b, c): (Natural, Natural, u32)| a - &(&b * c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.sub\\\\_mul(\\\\&Natural, u32)",
        g_name: "Natural - \\\\&Natural * u32",
        title: "Natural.sub\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_u32_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Natural).sub_mul(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_natural_natural_and_unsigned::<u32>(gm),
        function_f: &mut (|(a, b, c): (Natural, Natural, u32)| (&a).sub_mul(&b, c)),
        function_g: &mut (|(a, b, c): (Natural, Natural, u32)| &a - &(&b * c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, u32)",
        g_name: "(\\\\&Natural) - \\\\&Natural * u32",
        title: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
