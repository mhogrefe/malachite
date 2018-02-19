use common::{natural_to_rug_integer, GenerationMode};
use inputs::natural::triples_of_natural_small_u64_and_bool;
use malachite_base::num::{BitAccess, SignificantBits};
use malachite_nz::natural::Natural;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use std::cmp::max;

pub fn demo_natural_assign_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index, bit) in triples_of_natural_small_u64_and_bool(gm).take(limit) {
        let n_old = n.clone();
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

pub fn benchmark_natural_assign_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.assign_bit(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_natural_small_u64_and_bool(gm),
        function_f: &mut (|(mut n, index, bit): (Natural, u64, bool)| n.assign_bit(index, bit)),
        function_g: &mut (|(mut n, index, bit): (rug::Integer, u64, bool)| {
            n.set_bit(index as u32, bit);
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, bit)| (natural_to_rug_integer(n), index, bit)),
        x_param: &(|&(ref n, index, _)| max(n.significant_bits(), index) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Natural.assign\\\\_bit(u64, bool)",
        x_axis_label: "max(n.significant\\\\_bits(), index)",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
