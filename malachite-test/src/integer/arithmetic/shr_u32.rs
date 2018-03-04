use common::{integer_to_rug_integer, GenerationMode};
use inputs::integer::{pairs_of_integer_and_small_u32,
                      triples_of_integer_small_u32_and_rounding_mode_var_1};
use malachite_base::round::RoundingMode;
use malachite_base::num::{ShrRound, ShrRoundAssign};
use malachite_nz::integer::Integer;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};

pub fn demo_integer_shr_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n >>= u;
        println!("x := {}; x >>= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_integer_shr_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

pub fn demo_integer_shr_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

pub fn demo_integer_shr_round_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u, rm) in triples_of_integer_small_u32_and_rounding_mode_var_1(gm).take(limit) {
        let n_old = n.clone();
        n.shr_round_assign(u, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {}); x = {}",
            n_old, u, rm, n
        );
    }
}

pub fn demo_integer_shr_round_u32(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in triples_of_integer_small_u32_and_rounding_mode_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {}) = {}",
            n_old,
            u,
            rm,
            n.shr_round(u, rm)
        );
    }
}

pub fn demo_integer_shr_round_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in triples_of_integer_small_u32_and_rounding_mode_var_1(gm).take(limit) {
        println!(
            "(&{}).shr_round({}, {}) = {}",
            n,
            u,
            rm,
            (&n).shr_round(u, rm)
        );
    }
}

pub fn benchmark_integer_shr_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer >>= u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &mut (|(mut n, u)| n >>= u),
        function_g: &mut (|(mut n, u): (rug::Integer, u32)| n >>= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rug_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer >>= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer >> u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &mut (|(n, u)| n >> u),
        function_g: &mut (|(n, u): (rug::Integer, u32)| n >> u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rug_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_u32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} &Integer >> u32", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &mut (|(n, u)| &n >> u),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "&Integer >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_round_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.shr_round_assign(u32, RoundingMode)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_integer_small_u32_and_rounding_mode_var_1(gm),
        function_f: &mut (|(mut n, u, rm): (Integer, u32, RoundingMode)| n.shr_round_assign(u, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.shr_round_assign(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_round_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.shr_round(u32, RoundingMode)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_integer_small_u32_and_rounding_mode_var_1(gm),
        function_f: &mut (|(n, u, rm): (Integer, u32, RoundingMode)| n.shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.shr_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_round_u32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} (&Integer).shr_round(u32, RoundingMode)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_integer_small_u32_and_rounding_mode_var_1(gm),
        function_f: &mut (|(n, u, rm): (Integer, u32, RoundingMode)| (&n).shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "(&Integer).shr_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
