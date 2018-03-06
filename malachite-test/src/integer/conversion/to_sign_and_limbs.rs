use common::GenerationMode;
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn demo_integer_to_sign_and_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "to_sign_and_limbs_asc({}) = {:?}",
            n,
            n.to_sign_and_limbs_asc()
        );
    }
}

pub fn demo_integer_to_sign_and_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "to_sign_and_limbs_desc({}) = {:?}",
            n,
            n.to_sign_and_limbs_desc()
        );
    }
}

pub fn demo_integer_into_sign_and_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "into_sign_and_limbs_asc({}) = {:?}",
            n,
            n.clone().into_sign_and_limbs_asc()
        );
    }
}

pub fn demo_integer_into_sign_and_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "into_sign_and_limbs_desc({}) = {:?}",
            n,
            n.clone().into_sign_and_limbs_desc()
        );
    }
}

pub fn benchmark_integer_to_sign_and_limbs_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.to_sign_and_limbs_asc() evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.to_sign_and_limbs_asc()),
        function_g: &mut (|n: Integer| n.into_sign_and_limbs_asc()),
        x_cons: &(|n| n.clone()),
        y_cons: &(|n| n.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "Integer.to_sign_and_limbs_asc()",
        g_name: "Integer.into_sign_and_limbs_asc()",
        title: "Integer.to_sign_and_limbs_asc() evaluation strategy",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_to_sign_and_limbs_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.to_sign_and_limbs_desc() evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.to_sign_and_limbs_desc()),
        function_g: &mut (|n: Integer| n.into_sign_and_limbs_desc()),
        x_cons: &(|n| n.clone()),
        y_cons: &(|n| n.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "Integer.to_sign_and_limbs_desc()",
        g_name: "Integer.into_sign_and_limbs_desc()",
        title: "Integer.to_sign_and_limbs_desc() evaluation strategy",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
