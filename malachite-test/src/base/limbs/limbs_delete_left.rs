use common::GenerationMode;
use inputs::base::pairs_of_unsigned_vec_and_small_usize_var_1;
use malachite_base::limbs::limbs_delete_left;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_limbs_delete_left(gm: GenerationMode, limit: usize) {
    for (limbs, delete_size) in pairs_of_unsigned_vec_and_small_usize_var_1(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_delete_left(&mut mut_limbs, delete_size);
        println!(
            "limbs := {:?}; limbs_delete_left(&mut limbs, {}); x = {:?}",
            limbs, delete_size, mut_limbs
        );
    }
}

pub fn benchmark_limbs_delete_left(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} limbs_delete_left(&mut Vec<u32>, usize)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_unsigned_vec_and_small_usize_var_1(gm),
        function_f: &mut (|(mut limbs, delete_size): (Vec<u32>, usize)| {
            limbs_delete_left(&mut limbs, delete_size)
        }),
        x_cons: &(|ps| ps.clone()),
        x_param: &(|&(ref limbs, _)| limbs.len()),
        limit,
        f_name: "malachite",
        title: "limbs_delete_left(&mut Vec<u32>, usize)",
        x_axis_label: "limbs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
