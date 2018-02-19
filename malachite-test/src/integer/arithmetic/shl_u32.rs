use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::pairs_of_integer_and_small_u32;
use num::BigInt;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};

pub fn demo_integer_shl_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n <<= u;
        println!("x := {}; x <<= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_integer_shl_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!("{} << {} = {}", n_old, u, n << u);
    }
}

pub fn demo_integer_shl_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        println!("&{} << {} = {}", n, u, &n << u);
    }
}

pub fn benchmark_integer_shl_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer <<= u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &mut (|(mut n, u)| n <<= u),
        function_g: &mut (|(mut n, u): (rug::Integer, u32)| n <<= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rug_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer <<= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shl_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer << u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &mut (|(n, u)| n << u),
        function_g: &mut (|(n, u): (BigInt, u32)| n << u as usize),
        function_h: &mut (|(n, u): (rug::Integer, u32)| n << u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_bigint(n), index)),
        z_cons: &(|&(ref n, index)| (integer_to_rug_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer << u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shl_u32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} &Integer << u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &mut (|(n, u)| &n << u),
        function_g: &mut (|(n, u): (BigInt, u32)| &n << u as usize),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_bigint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "\\\\&Integer << u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
