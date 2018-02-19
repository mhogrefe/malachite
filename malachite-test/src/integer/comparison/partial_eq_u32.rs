use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};
use malachite_base::num::SignificantBits;
use num::BigInt;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};

pub fn num_partial_eq_u32(x: &BigInt, u: u32) -> bool {
    *x == BigInt::from(u)
}

pub fn demo_integer_partial_eq_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        if n == u {
            println!("{} = {}", n, u);
        } else {
            println!("{} ≠ {}", n, u);
        }
    }
}

pub fn demo_u32_partial_eq_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        if u == n {
            println!("{} = {}", u, n);
        } else {
            println!("{} ≠ {}", u, n);
        }
    }
}

pub fn benchmark_integer_partial_eq_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer == u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integer_and_unsigned::<u32>(gm),
        function_f: &mut (|(n, u)| n == u),
        function_g: &mut (|(n, u): (BigInt, u32)| num_partial_eq_u32(&n, u)),
        function_h: &mut (|(n, u): (rug::Integer, u32)| n == u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (integer_to_bigint(n), u)),
        z_cons: &(|&(ref n, u)| (integer_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer == u32",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_partial_eq_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32 == Integer", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_unsigned_and_integer::<u32>(gm),
        function_f: &mut (|(u, n)| u == n),
        function_g: &mut (|(u, n): (u32, rug::Integer)| u == n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, integer_to_rug_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "u32 == Integer",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
