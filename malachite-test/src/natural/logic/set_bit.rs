use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::natural::pairs_of_natural_and_small_u64;
use malachite_base::num::BitAccess;
use malachite_nz::natural::Natural;
use num::{BigUint, One};
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

pub fn num_set_bit(x: &mut BigUint, index: u64) {
    *x = x.clone() | (BigUint::one() << index as usize);
}

pub fn demo_natural_set_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_natural_and_small_u64(gm).take(limit) {
        let n_old = n.clone();
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

pub fn benchmark_natural_set_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.set_bit(u64)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_natural_and_small_u64(gm),
        function_f: &mut (|(mut n, index): (Natural, u64)| n.set_bit(index)),
        function_g: &mut (|(mut n, index): (BigUint, u64)| num_set_bit(&mut n, index)),
        function_h: &mut (|(mut n, index): (rug::Integer, u64)| {
            n.set_bit(index as u32, true);
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (natural_to_biguint(n), index)),
        z_cons: &(|&(ref n, index)| (natural_to_rug_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural.set_bit(u64)",
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
