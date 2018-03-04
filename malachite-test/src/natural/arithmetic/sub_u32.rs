use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural,
                      pairs_of_natural_and_u32_var_1};
use malachite_base::num::SignificantBits;
use natural::comparison::partial_ord_u32::num_partial_cmp_u32;
use num::BigUint;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, BenchmarkOptions3,
                              benchmark_1, benchmark_2, benchmark_3};
use std::cmp::Ordering;

pub fn num_sub_u32(x: BigUint, u: u32) -> Option<BigUint> {
    if num_partial_cmp_u32(&x, u) != Some(Ordering::Less) {
        Some(x - BigUint::from(u))
    } else {
        None
    }
}

pub fn rug_sub_u32(x: rug::Integer, u: u32) -> Option<rug::Integer> {
    if x >= u {
        Some(x - u)
    } else {
        None
    }
}

pub fn demo_natural_sub_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_u32_var_1(gm).take(limit) {
        let n_old = n.clone();
        n -= u;
        println!("x := {}; x -= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_natural_sub_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {:?}", n_old, u, n - u);
    }
}

pub fn demo_natural_sub_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} - {} = {:?}", n, u, &n - u);
    }
}

pub fn demo_u32_sub_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {:?}", u, n_old, u - &n);
    }
}

pub fn benchmark_natural_sub_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural -= u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_u32_var_1(gm),
        function_f: &mut (|(mut n, u)| n -= u),
        function_g: &mut (|(mut n, u): (rug::Integer, u32)| n -= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Natural -= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural - u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_natural_and_unsigned::<u32>(gm),
        function_f: &mut (|(n, u)| n - u),
        function_g: &mut (|(n, u): (BigUint, u32)| num_sub_u32(n, u)),
        function_h: &mut (|(n, u): (rug::Integer, u32)| n - u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_biguint(n), u)),
        z_cons: &(|&(ref n, u)| (natural_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural - u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_u32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} &Natural - u32", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_natural_and_unsigned::<u32>(gm),
        function_f: &mut (|(n, u)| &n - u),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "&Natural - u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_sub_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32 - Natural", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_unsigned_and_natural::<u32>(gm),
        function_f: &mut (|(u, n)| u - &n),
        function_g: &mut (|(u, n): (u32, rug::Integer)| u - n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, natural_to_rug_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "u32 - Natural",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
