use common::{integer_to_rug_integer, GenerationMode};
use inputs::integer::{pairs_of_integer_and_small_i32,
                      triples_of_integer_small_i32_and_rounding_mode_var_2};
use malachite_base::round::RoundingMode;
use malachite_base::num::{ShrRound, ShrRoundAssign};
use malachite_nz::integer::Integer;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};

pub fn demo_integer_shr_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_small_i32(gm).take(limit) {
        let n_old = n.clone();
        n >>= i;
        println!("x := {}; x >>= {}; x = {}", n_old, i, n);
    }
}

pub fn demo_integer_shr_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_small_i32(gm).take(limit) {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, i, n >> i);
    }
}

pub fn demo_integer_shr_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_small_i32(gm).take(limit) {
        println!("&{} >> {} = {}", n, i, &n >> i);
    }
}

pub fn demo_integer_shr_round_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i, rm) in triples_of_integer_small_i32_and_rounding_mode_var_2(gm).take(limit) {
        let n_old = n.clone();
        n.shr_round_assign(i, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {}); x = {}",
            n_old, i, rm, n
        );
    }
}

pub fn demo_integer_shr_round_i32(gm: GenerationMode, limit: usize) {
    for (n, i, rm) in triples_of_integer_small_i32_and_rounding_mode_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {}) = {}",
            n_old,
            i,
            rm,
            n.shr_round(i, rm)
        );
    }
}

pub fn demo_integer_shr_round_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i, rm) in triples_of_integer_small_i32_and_rounding_mode_var_2(gm).take(limit) {
        println!(
            "(&{}).shr_round({}, {}) = {}",
            n,
            i,
            rm,
            (&n).shr_round(i, rm)
        );
    }
}

pub fn benchmark_integer_shr_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer >>= i32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_small_i32(gm),
        function_f: &(|(mut n, i)| n >>= i),
        function_g: &(|(mut n, i): (rug::Integer, i32)| n >>= i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rug_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer >>= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer >> i32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_small_i32(gm),
        function_f: &(|(n, i)| n >> i),
        function_g: &(|(n, i): (rug::Integer, i32)| n >> i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rug_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer >> i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_i32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} &Integer >> i32", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_i32(gm),
        function_f: &(|(n, i)| &n >> i),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "\\\\&Integer >> i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_round_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.shr_round_assign(i32, RoundingMode)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_integer_small_i32_and_rounding_mode_var_2(gm),
        function_f: &(|(mut n, i, rm): (Integer, i32, RoundingMode)| n.shr_round_assign(i, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.shr\\\\_round\\\\_assign(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_round_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.shr_round(i32, RoundingMode)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_integer_small_i32_and_rounding_mode_var_2(gm),
        function_f: &(|(n, i, rm): (Integer, i32, RoundingMode)| n.shr_round(i, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.shr\\\\_round(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_round_i32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} (&Integer).shr_round(i32, RoundingMode)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_integer_small_i32_and_rounding_mode_var_2(gm),
        function_f: &(|(n, i, rm): (Integer, i32, RoundingMode)| (&n).shr_round(i, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "(\\\\&Integer).shr\\\\_round(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
