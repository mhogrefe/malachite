use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_small_usize_and_unsigned;
use malachite_base::limbs::limbs_pad_left;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_limbs_pad_left(gm: GenerationMode, limit: usize) {
    for (limbs, pad_size, pad_limb) in
        triples_of_unsigned_vec_small_usize_and_unsigned(gm).take(limit)
    {
        let mut mut_limbs = limbs.clone();
        limbs_pad_left(&mut mut_limbs, pad_size, pad_limb);
        println!(
            "limbs := {:?}; limbs_pad_left(&mut limbs, {}, {}); x = {:?}",
            limbs, pad_size, pad_limb, mut_limbs
        );
    }
}

pub fn benchmark_limbs_pad_left(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} limbs_pad_left(&mut Vec<u32>, usize, u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_unsigned_vec_small_usize_and_unsigned(gm),
        function_f: &(|(mut limbs, pad_size, pad_limb): (Vec<u32>, usize, u32)| {
            limbs_pad_left(&mut limbs, pad_size, pad_limb)
        }),
        x_cons: &(|ts| ts.clone()),
        x_param: &(|&(ref limbs, _, _)| limbs.len()),
        limit,
        f_name: "malachite",
        title: "limbs\\\\_pad\\\\_left(\\\\&mut Vec<u32>, usize, u32)",
        x_axis_label: "limbs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
