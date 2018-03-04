use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::base::vecs_of_unsigned_var_1;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions3, benchmark_1, benchmark_3};

pub fn demo_limbs_significant_bits(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_significant_bits({:?}) = {}",
            limbs,
            limbs_significant_bits(&limbs)
        );
    }
}

pub fn demo_natural_significant_bits(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn benchmark_limbs_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} limbs_significant_bits(&[u32])", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned_var_1(gm),
        function_f: &mut (|ref limbs: Vec<u32>| limbs_significant_bits(limbs)),
        x_cons: &(|x| x.clone()),
        x_param: &(|limbs| limbs.len()),
        limit,
        f_name: "malachite",
        title: "limbs\\\\_significant\\\\_bits(\\\\&[u32])",
        x_axis_label: "limbs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.significant_bits()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.significant_bits()),
        function_g: &mut (|n: BigUint| n.bits()),
        function_h: &mut (|n: rug::Integer| n.significant_bits()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| natural_to_biguint(x)),
        z_cons: &(|x| natural_to_rug_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural.significant\\\\_bits()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
