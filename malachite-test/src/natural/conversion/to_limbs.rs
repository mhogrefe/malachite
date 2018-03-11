use common::GenerationMode;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

pub fn demo_natural_to_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_limbs_asc({}) = {:?}", n, n.to_limbs_asc());
    }
}

pub fn demo_natural_to_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_limbs_desc({}) = {:?}", n, n.to_limbs_desc());
    }
}

pub fn demo_natural_into_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("into_limbs_asc({}) = {:?}", n, n.clone().into_limbs_asc());
    }
}

pub fn demo_natural_into_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("into_limbs_desc({}) = {:?}", n, n.clone().into_limbs_desc());
    }
}

pub fn demo_natural_limbs(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limbs({}) = {:?}", n, n.limbs().collect::<Vec<u32>>());
    }
}

pub fn demo_natural_limbs_rev(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "limbs({}).rev() = {:?}",
            n,
            n.limbs().rev().collect::<Vec<u32>>()
        );
    }
}

pub fn benchmark_natural_limbs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.limbs() evaluation strategy",
        gm.name()
    );
    benchmark_3(BenchmarkOptions3 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.to_limbs_asc()),
        function_g: &mut (|n: Natural| n.into_limbs_asc()),
        function_h: &mut (|n: Natural| n.limbs().collect::<Vec<u32>>()),
        x_cons: &(|n| n.clone()),
        y_cons: &(|n| n.clone()),
        z_cons: &(|n| n.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "Natural.to_limbs_asc()",
        g_name: "Natural.into_limbs_asc()",
        h_name: "Natural.limbs().collect::<Vec<u32>>()",
        title: "Natural.limbs_asc() evaluation strategy",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_limbs_rev_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.limbs().rev() evaluation strategy",
        gm.name()
    );
    benchmark_3(BenchmarkOptions3 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.to_limbs_desc()),
        function_g: &mut (|n: Natural| n.into_limbs_desc()),
        function_h: &mut (|n: Natural| n.limbs().rev().collect::<Vec<u32>>()),
        x_cons: &(|n| n.clone()),
        y_cons: &(|n| n.clone()),
        z_cons: &(|n| n.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "Natural.to_limbs_desc()",
        g_name: "Natural.into_limbs_desc()",
        h_name: "Natural.limbs().rev().collect::<Vec<u32>>()",
        title: "Natural.limbs_desc() evaluation strategy",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
