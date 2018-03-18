use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::{BigInt, BigUint};
use rug;
use rust_wheels::benchmarks::{run_benchmark, BenchmarkOptions, BenchmarkSeriesOptions};
use std::str::FromStr;

pub fn biguint_to_natural(n: &BigUint) -> Natural {
    Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn natural_to_biguint(n: &Natural) -> BigUint {
    BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rug_integer_to_natural(n: &rug::Integer) -> Natural {
    Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn natural_to_rug_integer(n: &Natural) -> rug::Integer {
    rug::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn bigint_to_integer(n: &BigInt) -> Integer {
    Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn integer_to_bigint(n: &Integer) -> BigInt {
    BigInt::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rug_integer_to_integer(n: &rug::Integer) -> Integer {
    Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn integer_to_rug_integer(n: &Integer) -> rug::Integer {
    rug::Integer::from_str(n.to_string().as_ref()).unwrap()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NoSpecialGenerationMode {
    Exhaustive,
    Random(u32),
}

impl NoSpecialGenerationMode {
    pub fn name(&self) -> &str {
        match *self {
            NoSpecialGenerationMode::Exhaustive => "exhaustive",
            NoSpecialGenerationMode::Random(_) => "random",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GenerationMode {
    Exhaustive,
    Random(u32),
    SpecialRandom(u32),
}

impl GenerationMode {
    pub fn name(&self) -> &str {
        match *self {
            GenerationMode::Exhaustive => "exhaustive",
            GenerationMode::Random(_) => "random",
            GenerationMode::SpecialRandom(_) => "special_random",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BenchmarkType {
    Single,
    LibraryComparison,
    EvaluationStrategy,
    Algorithms,
}

#[allow(too_many_arguments)]
pub fn m_run_benchmark<'a, I: Iterator>(
    title: &'a str,
    benchmark_type: BenchmarkType,
    generator: I,
    generation_mode_name: &'a str,
    limit: usize,
    file_name: &'a str,
    bucketing_function: &'a Fn(&I::Item) -> usize,
    bucketing_label: &'a str,
    series: &mut [(&'a str, &'a mut FnMut(I::Item))],
) where
    I::Item: Clone,
{
    let title = match benchmark_type {
        BenchmarkType::Single => title.to_owned(),
        BenchmarkType::LibraryComparison => format!("{} library comparison", title),
        BenchmarkType::EvaluationStrategy => format!("{} evaluation strategy", title),
        BenchmarkType::Algorithms => format!("{} algorithms", title),
    };
    println!("benchmarking {} {}", generation_mode_name, title);
    let colors = vec!["green", "blue", "red", "black", "orange"];
    if series.len() > colors.len() {
        panic!("not enough available colors");
    }
    if (benchmark_type == BenchmarkType::Single) != (series.len() == 1) {
        panic!("Benchmarks have type Single iff they have only one series");
    }
    let mut series_options = Vec::new();
    for (&mut (label, ref mut function), color) in series.iter_mut().zip(colors.iter()) {
        series_options.push(BenchmarkSeriesOptions {
            name: label,
            function,
            color,
        });
    }
    let options = BenchmarkOptions {
        generator,
        title: &title,
        limit,
        bucketing_function,
        x_axis_label: bucketing_label,
        y_axis_label: "time (ns)",
        file_name: format!("benchmarks/{}", file_name),
        series_options,
    };
    run_benchmark(options);
}

macro_rules! no_out {
    ($e: expr) => {{
        $e;
    }};
}
