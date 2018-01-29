use common::GenerationMode;
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_integer_sign_and_limbs_le(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("sign_and_limbs_le({}) = {:?}", n, n.sign_and_limbs_le());
    }
}

pub fn demo_integer_sign_and_limbs_be(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("sign_and_limbs_be({}) = {:?}", n, n.sign_and_limbs_be());
    }
}

pub fn benchmark_integer_sign_and_limbs_le(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.sign_and_limbs_le()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.sign_and_limbs_le()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.sign\\\\_and\\\\_limbs\\\\_le()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sign_and_limbs_be(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.sign_and_limbs_be()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.sign_and_limbs_be()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.sign\\\\_and\\\\_limbs\\\\_be()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
