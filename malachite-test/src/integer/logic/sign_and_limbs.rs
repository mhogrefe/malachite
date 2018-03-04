use common::GenerationMode;
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_integer_sign_and_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("sign_and_limbs_asc({}) = {:?}", n, n.sign_and_limbs_asc());
    }
}

pub fn demo_integer_sign_and_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("sign_and_limbs_desc({}) = {:?}", n, n.sign_and_limbs_desc());
    }
}

pub fn benchmark_integer_sign_and_limbs_asc(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.sign_and_limbs_asc()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.sign_and_limbs_asc()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.sign_and_limbs_le()",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sign_and_limbs_desc(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.sign_and_limbs_desc()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.sign_and_limbs_desc()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.sign_and_limbs_be()",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
