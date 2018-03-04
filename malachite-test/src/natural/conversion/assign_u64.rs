use common::{natural_to_biguint, GenerationMode};
use inputs::natural::pairs_of_natural_and_unsigned;
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use malachite_nz::natural::Natural;
use num::BigUint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn num_assign_u64(x: &mut BigUint, u: u64) {
    *x = BigUint::from(u);
}

pub fn demo_natural_assign_u64(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<u64>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_natural_assign_u64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.assign(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_unsigned::<u64>(gm),
        function_f: &mut (|(mut n, u): (Natural, u64)| n.assign(u)),
        function_g: &mut (|(mut n, u): (BigUint, u64)| num_assign_u64(&mut n, u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_biguint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Natural.assign(u64)",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
