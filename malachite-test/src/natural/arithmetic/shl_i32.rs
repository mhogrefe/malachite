use common::{gmp_natural_to_native, gmp_natural_to_rugint_integer, GenerationMode};
use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShlRound, ShlRoundAssign};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::i32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{lex_pairs, log_pairs, random_pairs, random_triples};

type It1 = Iterator<Item = (gmp::Natural, i32)>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_i()))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| i32s_geometric(seed, scale)),
    ))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (gmp::Natural, i32, RoundingMode)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(
        lex_pairs(exhaustive_inputs_1(), exhaustive_rounding_modes())
            .filter(|&((ref n, i), rm)| {
                i >= 0 || rm != RoundingMode::Exact
                    || n.divisible_by_power_of_2(i.wrapping_neg() as u32)
            })
            .map(|((n, i), rm)| (n, i, rm)),
    )
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(
        random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, i, rm)| {
            i >= 0 || rm != RoundingMode::Exact
                || n.divisible_by_power_of_2(i.wrapping_neg() as u32)
        }),
    )
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

pub fn demo_natural_shl_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        n <<= i;
        println!("x := {}; x <<= {}; x = {}", n_old, i, n);
    }
}

pub fn demo_natural_shl_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} << {} = {}", n_old, i, n << i);
    }
}

pub fn demo_natural_shl_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in select_inputs_1(gm).take(limit) {
        println!("&{} << {} = {}", n, i, &n << i);
    }
}

pub fn demo_natural_shl_round_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i, rm) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        n.shl_round_assign(i, rm);
        println!(
            "x := {}; x.shl_round_assign({}, {:?}); x = {}",
            n_old, i, rm, n
        );
    }
}

pub fn demo_natural_shl_round_i32(gm: GenerationMode, limit: usize) {
    for (n, i, rm) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.shl_round({}, {:?}) = {}",
            n_old,
            i,
            rm,
            n.shl_round(i, rm)
        );
    }
}

pub fn demo_natural_shl_round_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i, rm) in select_inputs_2(gm).take(limit) {
        println!(
            "(&{}).shl_round({}, {:?}) = {}",
            n,
            i,
            rm,
            (&n).shl_round(i, rm)
        );
    }
}

pub fn benchmark_natural_shl_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural <<= i32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_1(gm),
        function_f: &(|(mut n, i)| n <<= i),
        function_g: &(|(mut n, i): (native::Natural, i32)| n <<= i),
        function_h: &(|(mut n, i): (rugint::Integer, i32)| n <<= i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural <<= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_shl_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural << i32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, i)| n << i),
        function_g: &(|(n, i): (native::Natural, i32)| n << i),
        function_h: &(|(n, i): (rugint::Integer, i32)| n << i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural << i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_shl_i32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} &Natural << i32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, i)| &n << i),
        function_g: &(|(n, i): (native::Natural, i32)| &n << i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "\\\\&Natural << i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_shl_round_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.shl_round_assign(i32, RoundingMode)",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(mut n, i, rm): (gmp::Natural, i32, RoundingMode)| {
            n.shl_round_assign(i, rm)
        }),
        function_g: &(|(mut n, i, rm): (native::Natural, i32, RoundingMode)| {
            n.shl_round_assign(i, rm)
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_natural_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.shl\\\\_round\\\\_assign(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_shl_round_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.shl_round(i32, RoundingMode)",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(n, i, rm): (gmp::Natural, i32, RoundingMode)| n.shl_round(i, rm)),
        function_g: &(|(n, i, rm): (native::Natural, i32, RoundingMode)| n.shl_round(i, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_natural_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.shl\\\\_round(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_shl_round_i32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} (&Natural).shl_round(i32, RoundingMode)",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(n, i, rm): (gmp::Natural, i32, RoundingMode)| (&n).shl_round(i, rm)),
        function_g: &(|(n, i, rm): (native::Natural, i32, RoundingMode)| (&n).shl_round(i, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_natural_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "(\\\\&Natural).shl\\\\_round(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
