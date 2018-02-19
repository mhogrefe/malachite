use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer};
use malachite_base::num::SignificantBits;
use num::BigInt;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};

pub fn num_partial_eq_i32(x: &BigInt, i: i32) -> bool {
    *x == BigInt::from(i)
}

pub fn demo_integer_partial_eq_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        if n == i {
            println!("{} = {}", n, i);
        } else {
            println!("{} ≠ {}", n, i);
        }
    }
}

pub fn demo_i32_partial_eq_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        if i == n {
            println!("{} = {}", i, n);
        } else {
            println!("{} ≠ {}", i, n);
        }
    }
}

pub fn benchmark_integer_partial_eq_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer == i32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integer_and_signed::<i32>(gm),
        function_f: &mut (|(n, i)| n == i),
        function_g: &mut (|(n, i): (BigInt, i32)| num_partial_eq_i32(&n, i)),
        function_h: &mut (|(n, i): (rug::Integer, i32)| n == i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (integer_to_bigint(n), i)),
        z_cons: &(|&(ref n, i)| (integer_to_rug_integer(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer == i32",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_i32_partial_eq_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} i32 == Integer", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_signed_and_integer::<i32>(gm),
        function_f: &mut (|(i, n)| i == n),
        function_g: &mut (|(i, n): (i32, rug::Integer)| i == n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, integer_to_rug_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "i32 == Integer",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
