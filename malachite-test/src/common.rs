use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

use malachite_bench::benchmarks::{run_benchmark, BenchmarkOptions, BenchmarkSeriesOptions};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::{BigInt, BigUint};
use rug;

pub const SMALL_LIMIT: usize = 1_000;
pub const LARGE_LIMIT: usize = 10_000;

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
pub enum ScaleType {
    None,
    Small,
    Large,
}

pub fn get_gm(gm_string: &str, scale_type: ScaleType) -> GenerationMode {
    let scale = match scale_type {
        ScaleType::None => 0,
        ScaleType::Small => 128,
        ScaleType::Large => 1024,
    };
    match gm_string {
        "exhaustive" => GenerationMode::Exhaustive,
        "random" => GenerationMode::Random(scale),
        "special_random" => GenerationMode::SpecialRandom(scale),
        _ => panic!(),
    }
}

pub fn get_no_special_gm(gm_string: &str, scale_type: ScaleType) -> NoSpecialGenerationMode {
    let scale = match scale_type {
        ScaleType::None => 0,
        ScaleType::Small => 128,
        ScaleType::Large => 1024,
    };
    match gm_string {
        "exhaustive" => NoSpecialGenerationMode::Exhaustive,
        "random" => NoSpecialGenerationMode::Random(scale),
        _ => panic!(),
    }
}

type DemoFn = &'static dyn Fn(GenerationMode, usize) -> ();
type BenchFn = &'static dyn Fn(GenerationMode, usize, &str) -> ();
type NoSpecialDemoFn = &'static dyn Fn(NoSpecialGenerationMode, usize) -> ();
type NoSpecialBenchFn = &'static dyn Fn(NoSpecialGenerationMode, usize, &str) -> ();

#[derive(Default)]
pub struct DemoBenchRegistry {
    demo_map: BTreeMap<&'static str, DemoFn>,
    bench_map: BTreeMap<&'static str, (ScaleType, BenchFn)>,
    no_special_demo_map: BTreeMap<&'static str, NoSpecialDemoFn>,
    no_special_bench_map: BTreeMap<&'static str, (ScaleType, NoSpecialBenchFn)>,
}

impl DemoBenchRegistry {
    pub(crate) fn register_demo(&mut self, name: &'static str, f: DemoFn) {
        assert!(
            self.demo_map.insert(name, f).is_none(),
            "Duplicate demo with name {}",
            name
        );
    }

    pub fn lookup_demo(&self, name: &str) -> Option<&DemoFn> {
        self.demo_map.get(name)
    }

    pub(crate) fn register_bench(&mut self, scale_type: ScaleType, name: &'static str, f: BenchFn) {
        f(GenerationMode::Exhaustive, 0, "validation");
        assert!(
            self.bench_map.insert(name, (scale_type, f)).is_none(),
            "Duplicate bench with name {}",
            name
        );
    }

    pub fn lookup_bench(&self, name: &str) -> Option<&(ScaleType, BenchFn)> {
        self.bench_map.get(name)
    }

    pub(crate) fn register_no_special_demo(&mut self, name: &'static str, f: NoSpecialDemoFn) {
        assert!(
            self.no_special_demo_map.insert(name, f).is_none(),
            "Duplicate demo with name {}",
            name
        );
    }

    pub fn lookup_no_special_demo(&self, name: &str) -> Option<&NoSpecialDemoFn> {
        self.no_special_demo_map.get(name)
    }

    pub(crate) fn register_no_special_bench(
        &mut self,
        scale_type: ScaleType,
        name: &'static str,
        f: NoSpecialBenchFn,
    ) {
        f(NoSpecialGenerationMode::Exhaustive, 0, "validation");
        assert!(
            self.no_special_bench_map
                .insert(name, (scale_type, f))
                .is_none(),
            "Duplicate bench with name {}",
            name
        );
    }

    pub fn lookup_no_special_bench(&self, name: &str) -> Option<&(ScaleType, NoSpecialBenchFn)> {
        self.no_special_bench_map.get(name)
    }

    pub fn benchmark_all(&self, limit: usize) {
        let files: Vec<String> = fs::read_dir("benchmarks/")
            .unwrap()
            .into_iter()
            .map(|file| file.unwrap().path().display().to_string())
            .filter(|file| file.ends_with(".gp"))
            .collect();
        for file in files {
            fs::remove_file(file);
        }
        for (name, &(st, f)) in &self.no_special_bench_map {
            for gm_string in &["exhaustive", "random"] {
                let gm = get_no_special_gm(gm_string, st);
                f(gm, limit, &format!("{}_{}.gp", gm_string, name));
            }
        }
        for (name, &(st, f)) in &self.bench_map {
            for gm_string in &["exhaustive", "random", "special_random"] {
                let gm = get_gm(gm_string, st);
                f(gm, limit, &format!("{}_{}.gp", gm_string, name));
            }
        }
    }
}

macro_rules! register_demo {
    ($registry:ident, $f:ident) => {{
        $registry.register_demo(stringify!($f), &$f);
    }};
}

macro_rules! register_ns_demo {
    ($registry:ident, $f:ident) => {{
        $registry.register_no_special_demo(stringify!($f), &$f);
    }};
}

macro_rules! register_bench {
    ($registry:ident, $st:ident, $f:ident) => {{
        $registry.register_bench(ScaleType::$st, stringify!($f), &$f);
    }};
}

macro_rules! register_ns_bench {
    ($registry:ident, $st:ident, $f:ident) => {{
        $registry.register_no_special_bench(ScaleType::$st, stringify!($f), &$f);
    }};
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NoSpecialGenerationMode {
    Exhaustive,
    Random(u32),
}

impl NoSpecialGenerationMode {
    pub(crate) fn name(self) -> &'static str {
        match self {
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
    pub(crate) fn name(self) -> &'static str {
        match self {
            GenerationMode::Exhaustive => "exhaustive",
            GenerationMode::Random(_) => "random",
            GenerationMode::SpecialRandom(_) => "special_random",
        }
    }

    pub(crate) fn get_scale(self) -> Option<u32> {
        match self {
            GenerationMode::Exhaustive => None,
            GenerationMode::Random(scale) => Some(scale),
            GenerationMode::SpecialRandom(scale) => Some(scale),
        }
    }

    pub(crate) fn with_scale(self, scale: u32) -> GenerationMode {
        match self {
            GenerationMode::Exhaustive => GenerationMode::Exhaustive,
            GenerationMode::Random(_) => GenerationMode::Random(scale),
            GenerationMode::SpecialRandom(_) => GenerationMode::SpecialRandom(scale),
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
pub(crate) fn m_run_benchmark<'a, I: Iterator>(
    title: &'a str,
    benchmark_type: BenchmarkType,
    generator: I,
    generation_mode_name: &'a str,
    limit: usize,
    file_name: &'a str,
    bucketing_function: &'a dyn Fn(&I::Item) -> usize,
    bucketing_label: &'a str,
    series: &mut [(&'a str, &'a mut dyn FnMut(I::Item))],
) where
    I::Item: Clone,
{
    if (benchmark_type == BenchmarkType::Single) != (series.len() == 1) {
        panic!(
            "Bad benchmark: {}. \
             Benchmarks should have type Single iff they have only one series.",
            title
        );
    }
    if limit == 0 {
        return;
    }
    let title = match benchmark_type {
        BenchmarkType::Single => title.to_string(),
        BenchmarkType::LibraryComparison => format!("{} library comparison", title),
        BenchmarkType::EvaluationStrategy => format!("{} evaluation strategy", title),
        BenchmarkType::Algorithms => format!("{} algorithms", title),
    };
    println!("benchmarking {} {}", generation_mode_name, title);
    let colors = vec![
        "green", "blue", "red", "black", "orange", "yellow", "gray", "purple",
    ];
    if series.len() > colors.len() {
        panic!("not enough available colors");
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
    ($e:expr) => {{
        $e;
    }};
}

pub fn test_properties<T, G: Fn(GenerationMode) -> Box<dyn Iterator<Item = T>>, F: FnMut(&T)>(
    gen: G,
    mut test: F,
) {
    for &gm in &[
        GenerationMode::Exhaustive,
        GenerationMode::Random(32),
        GenerationMode::SpecialRandom(32),
    ] {
        for x in gen(gm).take(LARGE_LIMIT) {
            test(&x);
        }
    }
}

pub fn test_properties_no_special<
    T,
    G: Fn(NoSpecialGenerationMode) -> Box<dyn Iterator<Item = T>>,
    F: FnMut(&T),
>(
    gen: G,
    mut test: F,
) {
    for &gm in &[
        NoSpecialGenerationMode::Exhaustive,
        NoSpecialGenerationMode::Random(32),
    ] {
        for x in gen(gm).take(LARGE_LIMIT) {
            test(&x);
        }
    }
}

pub fn test_properties_custom_scale<
    T,
    G: Fn(GenerationMode) -> Box<dyn Iterator<Item = T>>,
    F: FnMut(&T),
>(
    scale: u32,
    gen: G,
    mut test: F,
) {
    for &gm in &[
        GenerationMode::Exhaustive,
        GenerationMode::Random(scale),
        GenerationMode::SpecialRandom(scale),
    ] {
        for x in gen(gm).take(LARGE_LIMIT) {
            test(&x);
        }
    }
}

pub fn test_properties_custom_limit<
    T,
    G: Fn(GenerationMode) -> Box<dyn Iterator<Item = T>>,
    F: FnMut(&T),
>(
    limit: usize,
    gen: G,
    mut test: F,
) {
    for &gm in &[
        GenerationMode::Exhaustive,
        GenerationMode::Random(32),
        GenerationMode::SpecialRandom(32),
    ] {
        for x in gen(gm).take(limit) {
            test(&x);
        }
    }
}

pub fn test_properties_custom_limit_no_special<
    T,
    G: Fn(NoSpecialGenerationMode) -> Box<dyn Iterator<Item = T>>,
    F: FnMut(&T),
>(
    limit: usize,
    gen: G,
    mut test: F,
) {
    for &gm in &[
        NoSpecialGenerationMode::Exhaustive,
        NoSpecialGenerationMode::Random(32),
    ] {
        for x in gen(gm).take(limit) {
            test(&x);
        }
    }
}

pub fn test_properties_no_limit_exhaustive_no_special<
    T,
    G: Fn(NoSpecialGenerationMode) -> Box<dyn Iterator<Item = T>>,
    F: FnMut(&T),
>(
    gen: G,
    mut test: F,
) {
    for x in gen(NoSpecialGenerationMode::Exhaustive) {
        test(&x);
    }

    for x in gen(NoSpecialGenerationMode::Random(32)).take(LARGE_LIMIT) {
        test(&x);
    }
}
