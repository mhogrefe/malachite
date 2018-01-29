use common::{natural_to_rugint_integer, GenerationMode};
use inputs::natural::pairs_of_natural_and_small_u64;
use malachite_base::num::{BitAccess, SignificantBits};
use malachite_nz::natural::Natural;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use std::cmp::max;

pub fn demo_natural_flip_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_natural_and_small_u64(gm).take(limit) {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

pub fn benchmark_natural_flip_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.flip_bit(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_small_u64(gm),
        function_f: &(|(mut n, index): (Natural, u64)| n.flip_bit(index)),
        function_g: &(|(mut n, index): (rugint::Integer, u64)| {
            n.invert_bit(index as u32);
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (natural_to_rugint_integer(n), index)),
        x_param: &(|&(ref n, index)| max(n.significant_bits(), index) as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Natural.flip\\\\_bit(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
