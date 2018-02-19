use common::GenerationMode;
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_integer_twos_complement_limbs_le(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_limbs_le({}) = {:?}",
            n,
            n.twos_complement_limbs_le()
        );
    }
}

pub fn demo_integer_twos_complement_limbs_be(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_limbs_be({}) = {:?}",
            n,
            n.twos_complement_limbs_be()
        );
    }
}

pub fn benchmark_integer_twos_complement_limbs_le(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.twos_complement_limbs_le()",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.twos_complement_limbs_le()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.twos\\\\_complement\\\\_limbs\\\\_le()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_twos_complement_limbs_be(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.twos_complement_limbs_be()",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.twos_complement_limbs_be()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.twos\\\\_complement\\\\_limbs\\\\_be()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
