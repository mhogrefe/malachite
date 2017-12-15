use common::{gmp_integer_to_native, gmp_integer_to_rugint};
use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShrRound, ShrRoundAssign};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::i32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{lex_pairs, log_pairs, random_pairs, random_triples};

pub fn demo_exhaustive_integer_shr_assign_i32(limit: usize) {
    for (mut n, i) in log_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        let n_old = n.clone();
        n >>= i;
        println!("x := {}; x >>= {}; x = {}", n_old, i, n);
    }
}

pub fn demo_random_integer_shr_assign_i32(limit: usize) {
    for (mut n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| i32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n >>= i;
        println!("x := {}; x >>= {}; x = {}", n_old, i, n);
    }
}

pub fn demo_exhaustive_integer_shr_i32(limit: usize) {
    for (n, i) in log_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, i, n >> i);
    }
}

pub fn demo_random_integer_shr_i32(limit: usize) {
    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| i32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, i, n >> i);
    }
}

pub fn demo_exhaustive_integer_shr_i32_ref(limit: usize) {
    for (n, i) in log_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        println!("&{} >> {} = {}", n, i, &n >> i);
    }
}

pub fn demo_random_integer_shr_i32_ref(limit: usize) {
    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| i32s_geometric(seed, 32)),
    ).take(limit)
    {
        println!("&{} >> {} = {}", n, i, &n >> i);
    }
}

pub fn demo_exhaustive_integer_shr_round_assign_i32(limit: usize) {
    for ((mut n, i), rm) in lex_pairs(
        log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, i), rm)| {
        i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
    })
        .take(limit)
    {
        let n_old = n.clone();
        n.shr_round_assign(i, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {:?}); x = {}",
            n_old,
            i,
            rm,
            n
        );
    }
}

pub fn demo_random_integer_shr_round_assign_i32(limit: usize) {
    for (mut n, i, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| i32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, i, rm)| {
        i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
    })
        .take(limit)
    {
        let n_old = n.clone();
        n.shr_round_assign(i, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {:?}); x = {}",
            n_old,
            i,
            rm,
            n
        );
    }
}

pub fn demo_exhaustive_integer_shr_round_i32(limit: usize) {
    for ((n, i), rm) in lex_pairs(
        log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, i), rm)| {
        i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
    })
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {:?}) = {}",
            n_old,
            i,
            rm,
            n.shr_round(i, rm)
        );
    }
}

pub fn demo_random_integer_shr_round_i32(limit: usize) {
    for (n, i, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| i32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, i, rm)| {
        i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
    })
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {:?}) = {}",
            n_old,
            i,
            rm,
            n.shr_round(i, rm)
        );
    }
}

pub fn demo_exhaustive_integer_shr_round_i32_ref(limit: usize) {
    for ((n, i), rm) in lex_pairs(
        log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, i), rm)| {
        i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
    })
        .take(limit)
    {
        println!(
            "(&{}).shr_round({}, {:?}) = {}",
            n,
            i,
            rm,
            (&n).shr_round(i, rm)
        );
    }
}

pub fn demo_random_integer_shr_round_i32_ref(limit: usize) {
    for (n, i, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| i32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, i, rm)| {
        i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
    })
        .take(limit)
    {
        println!(
            "(&{}).shr_round({}, {:?}) = {}",
            n,
            i,
            rm,
            (&n).shr_round(i, rm)
        );
    }
}

pub fn benchmark_exhaustive_integer_shr_assign_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer >>= i32");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(mut n, i)| n >>= i),
        function_g: &(|(mut n, i): (native::Integer, i32)| n >>= i),
        function_h: &(|(mut n, i): (rugint::Integer, i32)| n >>= i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer >>= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_assign_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer >>= i32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, i)| n >>= i),
        function_g: &(|(mut n, i): (native::Integer, i32)| n >>= i),
        function_h: &(|(mut n, i): (rugint::Integer, i32)| n >>= i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer >>= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer >> i32");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(n, i)| n >> i),
        function_g: &(|(n, i): (native::Integer, i32)| n >> i),
        function_h: &(|(n, i): (rugint::Integer, i32)| n >> i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer >> i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer >>= i32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, i)| n >> i),
        function_g: &(|(n, i): (native::Integer, i32)| n >> i),
        function_h: &(|(n, i): (rugint::Integer, i32)| n >> i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer >> i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_i32_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive &Integer >> i32");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(n, i)| &n >> i),
        function_g: &(|(n, i): (native::Integer, i32)| &n >> i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "\\\\&Integer >> i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_i32_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random &Integer >>= i32");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, i)| &n >> i),
        function_g: &(|(n, i): (native::Integer, i32)| &n >> i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "\\\\&Integer >> i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_round_assign_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.shr_round_assign(i32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, i), rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
        }),
        function_f: &(|((mut n, i), rm): ((gmp::Integer, i32), RoundingMode)| {
                          n.shr_round_assign(i, rm)
                      }),
        function_g: &(|((mut n, i), rm): ((native::Integer, i32), RoundingMode)| {
                          n.shr_round_assign(i, rm)
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_integer_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.shr\\\\_round\\\\_assign(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_round_assign_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.shr_round_assign(i32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, i, rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
        }),
        function_f: &(|(mut n, i, rm): (gmp::Integer, i32, RoundingMode)| {
            n.shr_round_assign(i, rm)
        }),
        function_g: &(|(mut n, i, rm): (native::Integer, i32, RoundingMode)| {
                          n.shr_round_assign(i, rm)
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_integer_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.shr\\\\_round\\\\_assign(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_round_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.shr_round(i32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, i), rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
        }),
        function_f: &(|((n, i), rm): ((gmp::Integer, i32), RoundingMode)| n.shr_round(i, rm)),
        function_g: &(|((n, i), rm): ((native::Integer, i32), RoundingMode)| n.shr_round(i, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_integer_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.shr\\\\_round(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_round_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.shr_round(i32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, i, rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
        }),
        function_f: &(|(n, i, rm): (gmp::Integer, i32, RoundingMode)| n.shr_round(i, rm)),
        function_g: &(|(n, i, rm): (native::Integer, i32, RoundingMode)| n.shr_round(i, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_integer_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.shr\\\\_round(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shr_round_i32_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Integer).shr_round(i32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, i), rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
        }),
        function_f: &(|((n, i), rm): ((gmp::Integer, i32), RoundingMode)| (&n).shr_round(i, rm)),
        function_g: &(|((n, i), rm): ((native::Integer, i32), RoundingMode)| (&n).shr_round(i, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_integer_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "(\\\\&Integer).shr\\\\_round(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shr_round_i32_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random (&Integer).shr_round(i32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| i32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, i, rm)| {
            i <= 0 || rm != RoundingMode::Exact || n.divisible_by_power_of_2(i as u32)
        }),
        function_f: &(|(n, i, rm): (gmp::Integer, i32, RoundingMode)| (&n).shr_round(i, rm)),
        function_g: &(|(n, i, rm): (native::Integer, i32, RoundingMode)| (&n).shr_round(i, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_integer_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "(\\\\&Integer).shr\\\\_round(i32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
