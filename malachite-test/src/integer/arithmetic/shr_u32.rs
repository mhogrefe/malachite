use common::{gmp_integer_to_native, gmp_integer_to_rugint};
use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShrRound, ShrRoundAssign};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{lex_pairs, log_pairs, random_pairs, random_triples};

pub fn demo_exhaustive_integer_shr_assign_u32(limit: usize) {
    for (mut n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n >>= u;
        println!("x := {}; x >>= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_random_integer_shr_assign_u32(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!("x := {}; x >>= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_exhaustive_integer_shr_u32(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

pub fn demo_random_integer_shr_u32(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

pub fn demo_exhaustive_integer_shr_u32_ref(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

pub fn demo_random_integer_shr_u32_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

pub fn demo_exhaustive_integer_shr_round_assign_u32(limit: usize) {
    for ((mut n, u), rm) in lex_pairs(
        log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, u), rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        let n_old = n.clone();
        n.shr_round_assign(u, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {:?}); x = {}",
            n_old,
            u,
            rm,
            n
        );
    }
}

pub fn demo_random_integer_shr_round_assign_u32(limit: usize) {
    for (mut n, u, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, u, rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        let n_old = n.clone();
        n.shr_round_assign(u, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {:?}); x = {}",
            n_old,
            u,
            rm,
            n
        );
    }
}

pub fn demo_exhaustive_integer_shr_round_u32(limit: usize) {
    for ((n, u), rm) in lex_pairs(
        log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, u), rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
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

pub fn demo_random_integer_shr_round_u32(limit: usize) {
    for (n, u, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, u, rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
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

pub fn demo_exhaustive_integer_shr_round_u32_ref(limit: usize) {
    for ((n, u), rm) in lex_pairs(
        log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, u), rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        println!(
            "(&{}).shr_round({}, {:?}) = {}",
            n,
            u,
            rm,
            (&n).shr_round(u, rm)
        );
    }
}

pub fn demo_random_integer_shr_round_u32_ref(limit: usize) {
    for (n, u, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, u, rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        println!(
            "(&{}).shr_round({}, {:?}) = {}",
            n,
            u,
            rm,
            (&n).shr_round(u, rm)
        );
    }
}

pub fn benchmark_exhaustive_integer_shr_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer >>= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u)| n >>= u),
        function_g: &(|(mut n, u): (native::Integer, u32)| n >>= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n >>= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer >>= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer >>= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, u)| n >>= u),
        function_g: &(|(mut n, u): (native::Integer, u32)| n >>= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n >>= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer >>= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer >> u32");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| n >> u),
        function_g: &(|(n, u): (native::Integer, u32)| n >> u),
        function_h: &(|(n, u): (rugint::Integer, u32)| n >> u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer >>= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u)| n >> u),
        function_g: &(|(n, u): (native::Integer, u32)| n >> u),
        function_h: &(|(n, u): (rugint::Integer, u32)| n >> u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_u32_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive &Integer >> u32");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| &n >> u),
        function_g: &(|(n, u): (native::Integer, u32)| &n >> u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "\\\\&Integer >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_u32_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random &Integer >>= u32");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u)| &n >> u),
        function_g: &(|(n, u): (native::Integer, u32)| &n >> u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "\\\\&Integer >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_round_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.shr_round_assign(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, u), rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|((mut n, u), rm): ((gmp::Integer, u32), RoundingMode)| {
                          n.shr_round_assign(u, rm)
                      }),
        function_g: &(|((mut n, u), rm): ((native::Integer, u32), RoundingMode)| {
                          n.shr_round_assign(u, rm)
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_integer_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.shr\\\\_round\\\\_assign(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_round_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.shr_round_assign(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, u, rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|(mut n, u, rm): (gmp::Integer, u32, RoundingMode)| {
            n.shr_round_assign(u, rm)
        }),
        function_g: &(|(mut n, u, rm): (native::Integer, u32, RoundingMode)| {
                          n.shr_round_assign(u, rm)
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_integer_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.shr\\\\_round\\\\_assign(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_round_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.shr_round(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, u), rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|((n, u), rm): ((gmp::Integer, u32), RoundingMode)| n.shr_round(u, rm)),
        function_g: &(|((n, u), rm): ((native::Integer, u32), RoundingMode)| n.shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_integer_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_round_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.shr_round(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, u, rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|(n, u, rm): (gmp::Integer, u32, RoundingMode)| n.shr_round(u, rm)),
        function_g: &(|(n, u, rm): (native::Integer, u32, RoundingMode)| n.shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_integer_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_round_u32_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Integer).shr_round(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, u), rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|((n, u), rm): ((gmp::Integer, u32), RoundingMode)| (&n).shr_round(u, rm)),
        function_g: &(|((n, u), rm): ((native::Integer, u32), RoundingMode)| (&n).shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_integer_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "(\\\\&Integer).shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_round_u32_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random (&Integer).shr_round(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, u, rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|(n, u, rm): (gmp::Integer, u32, RoundingMode)| (&n).shr_round(u, rm)),
        function_g: &(|(n, u, rm): (native::Integer, u32, RoundingMode)| (&n).shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_integer_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "(\\\\&Integer).shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
