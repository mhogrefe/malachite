use common::GenerationMode;
use inputs::base::vecs_of_unsigned_var_1;
use inputs::natural::positive_naturals;
use malachite_base::num::{CeilingLogTwo, FloorLogTwo};
use malachite_nz::natural::arithmetic::log_two::{limbs_ceiling_log_two, limbs_floor_log_two};
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_limbs_floor_log_two(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_floor_log_two({:?}) = {}",
            limbs,
            limbs_floor_log_two(&limbs)
        );
    }
}

pub fn demo_limbs_ceiling_log_two(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_ceiling_log_two({:?}) = {}",
            limbs,
            limbs_ceiling_log_two(&limbs)
        );
    }
}

pub fn demo_natural_floor_log_two(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        println!("floor_log_two({}) = {}", n, n.floor_log_two());
    }
}

pub fn demo_natural_ceiling_log_two(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        println!("ceiling_log_two({}) = {}", n, n.ceiling_log_two());
    }
}

pub fn benchmark_limbs_floor_log_two(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} limbs_floor_log_two(&[u32])", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned_var_1(gm),
        function_f: &mut (|ref limbs: Vec<u32>| limbs_floor_log_two(limbs)),
        x_cons: &(|x| x.clone()),
        x_param: &(|limbs| limbs.len()),
        limit,
        f_name: "malachite",
        title: "limbs\\\\_floor\\\\_log\\\\_two(\\\\&[u32])",
        x_axis_label: "limbs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_limbs_ceiling_log_two(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} limbs_ceiling_log_two(&[u32])", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned_var_1(gm),
        function_f: &mut (|ref limbs: Vec<u32>| limbs_ceiling_log_two(limbs)),
        x_cons: &(|x| x.clone()),
        x_param: &(|limbs| limbs.len()),
        limit,
        f_name: "malachite",
        title: "limbs\\\\_ceiling\\\\_log\\\\_two(\\\\&[u32])",
        x_axis_label: "limbs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_floor_log_two(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.floor_log_two()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: positive_naturals(gm),
        function_f: &mut (|n: Natural| n.floor_log_two()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.floor_log_two() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.floor\\\\_log\\\\two()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_ceiling_log_two(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.ceiling_log_two()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: positive_naturals(gm),
        function_f: &mut (|n: Natural| n.ceiling_log_two()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.floor_log_two() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.ceiling\\\\_log\\\\_two()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
