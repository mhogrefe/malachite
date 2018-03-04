use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};
use malachite_base::num::SignificantBits;
use num::BigInt;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};

pub fn num_add_u32(x: BigInt, u: u32) -> BigInt {
    x + BigInt::from(u)
}

pub fn demo_integer_add_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n += u;
        println!("x := {}; x += {}; x = {}", n_old, u, n);
    }
}

pub fn demo_integer_add_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + {} = {}", n_old, u, n + u);
    }
}

pub fn demo_integer_add_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} + {} = {}", n, u, &n + u);
    }
}

pub fn demo_u32_add_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + {} = {}", u, n_old, u + n);
    }
}

pub fn demo_u32_add_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + &{} = {}", u, n_old, u + &n);
    }
}

pub fn benchmark_integer_add_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer += u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_unsigned::<u32>(gm),
        function_f: &mut (|(mut n, u)| n += u),
        function_g: &mut (|(mut n, u): (rug::Integer, u32)| n += u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (integer_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer += u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer + u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integer_and_unsigned::<u32>(gm),
        function_f: &mut (|(n, u)| n + u),
        function_g: &mut (|(n, u): (BigInt, u32)| num_add_u32(n, u)),
        function_h: &mut (|(n, u): (rug::Integer, u32)| n + u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (integer_to_bigint(n), u)),
        z_cons: &(|&(ref n, u)| (integer_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer + u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer + u32 evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_unsigned::<u32>(gm),
        function_f: &mut (|(n, u)| n + u),
        function_g: &mut (|(n, u)| &n + u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Integer + u32",
        g_name: "&Integer + u32",
        title: "Integer + u32 evaluation strategy",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_add_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32 + Integer", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_unsigned_and_integer::<u32>(gm),
        function_f: &mut (|(u, n)| u + n),
        function_g: &mut (|(u, n): (u32, rug::Integer)| u + n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, integer_to_rug_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "u32 + Integer",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_add_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} u32 + Integer evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_unsigned_and_integer::<u32>(gm),
        function_f: &mut (|(u, n)| u + n),
        function_g: &mut (|(u, n)| u + &n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "u32 + Integer",
        g_name: "u32 + &Integer",
        title: "u32 + Integer evaluation strategy",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
