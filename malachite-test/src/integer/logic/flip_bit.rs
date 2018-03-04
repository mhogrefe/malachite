use common::{integer_to_rug_integer, GenerationMode};
use inputs::integer::pairs_of_integer_and_small_u64;
use malachite_base::num::{BitAccess, SignificantBits};
use malachite_nz::integer::Integer;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use std::cmp::max;

pub fn demo_integer_flip_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_integer_and_small_u64(gm).take(limit) {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

pub fn benchmark_integer_flip_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.flip_bit(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_small_u64(gm),
        function_f: &mut (|(mut n, index): (Integer, u64)| n.flip_bit(index)),
        function_g: &mut (|(mut n, index): (rug::Integer, u64)| {
            n.toggle_bit(index as u32);
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rug_integer(n), index)),
        x_param: &(|&(ref n, index)| max(n.significant_bits(), index) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer.flip_bit(u64)",
        x_axis_label: "max(n.significant_bits(), index)",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
