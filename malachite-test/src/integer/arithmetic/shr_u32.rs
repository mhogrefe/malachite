use common::{integer_to_rugint_integer, GenerationMode};
use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShrRound, ShrRoundAssign};
use malachite_nz::integer::Integer;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{lex_pairs, log_pairs, random_pairs, random_triples};

type It1 = Iterator<Item = (Integer, u32)>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_u()))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| natural_u32s_geometric(seed, scale)),
    ))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (Integer, u32, RoundingMode)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(
        lex_pairs(exhaustive_inputs_1(), exhaustive_rounding_modes())
            .filter(|&((ref n, u), rm)| rm != RoundingMode::Exact || n.divisible_by_power_of_2(u))
            .map(|((n, u), rm)| (n, u, rm)),
    )
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(
        random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, u, rm)| rm != RoundingMode::Exact || n.divisible_by_power_of_2(u)),
    )
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

pub fn demo_integer_shr_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        n >>= u;
        println!("x := {}; x >>= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_integer_shr_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

pub fn demo_integer_shr_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs_1(gm).take(limit) {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

pub fn demo_integer_shr_round_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u, rm) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        n.shr_round_assign(u, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {:?}); x = {}",
            n_old, u, rm, n
        );
    }
}

pub fn demo_integer_shr_round_u32(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {:?}) = {}",
            n_old,
            u,
            rm,
            n.shr_round(u, rm)
        );
    }
}

pub fn demo_integer_shr_round_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in select_inputs_2(gm).take(limit) {
        println!(
            "(&{}).shr_round({}, {:?}) = {}",
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
        xs: select_inputs_1(gm),
        function_f: &(|(mut n, u)| n >>= u),
        function_g: &(|(mut n, u): (rugint::Integer, u32)| n >>= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Integer >>= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer >> u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, u)| n >> u),
        function_g: &(|(n, u): (rugint::Integer, u32)| n >> u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Integer >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shr_u32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} &Integer >> u32", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, u)| &n >> u),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "\\\\&Integer >> u32",
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
        xs: select_inputs_2(gm),
        function_f: &(|(mut n, u, rm): (Integer, u32, RoundingMode)| n.shr_round_assign(u, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.shr\\\\_round\\\\_assign(u32, RoundingMode)",
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
        xs: select_inputs_2(gm),
        function_f: &(|(n, u, rm): (Integer, u32, RoundingMode)| n.shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.shr\\\\_round(u32, RoundingMode)",
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
        xs: select_inputs_2(gm),
        function_f: &(|(n, u, rm): (Integer, u32, RoundingMode)| (&n).shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite",
        title: "(\\\\&Integer).shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
