use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::iter::Iterator;

use gnuplot::{AxesCommon, Caption, Color, Figure};
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::ExactFrom;
use time::precise_time_ns;

use bench::bucketers::Bucketer;

fn escape_label_string(s: &str) -> String {
    let mut escaped = String::new();
    for c in s.chars() {
        if c == '_' || c == '&' {
            escaped.push_str("\\\\");
        }
        escaped.push(c);
    }
    escaped
}

struct BenchmarkSeriesOptions<'a, T: 'a> {
    pub name: &'a str,
    pub function: &'a mut dyn FnMut(T),
    pub color: &'a str,
}

struct BenchmarkOptions<'a, I: Iterator>
where
    I::Item: 'a,
{
    pub generator: I,
    pub title: &'a str,
    pub limit: usize,
    pub bucketing_function: &'a dyn Fn(&I::Item) -> usize,
    pub x_axis_label: &'a str,
    pub y_axis_label: &'a str,
    pub file_name: String,
    pub series_options: Vec<BenchmarkSeriesOptions<'a, I::Item>>,
}

fn quick_median(mut xs: Vec<u64>) -> u64 {
    assert!(!xs.is_empty());
    //let mut xs = xs.clone();
    xs.sort_unstable();
    let half_index = xs.len() >> 1;
    if xs.len().odd() {
        xs[half_index]
    } else {
        (xs[half_index - 1] + xs[half_index]) >> 1
    }
}

fn quick_mean(xs: &[u64]) -> u64 {
    assert!(!xs.is_empty());
    let sum: u64 = xs.iter().sum();
    sum / u64::exact_from(xs.len())
}

fn run_benchmark_internal<I: Iterator>(mut options: BenchmarkOptions<I>)
where
    I::Item: Clone,
{
    let reps = 5;
    let min_bucket_size = 2;

    let mut durations_maps = Vec::new();
    for _ in 0..options.series_options.len() {
        durations_maps.push(HashMap::new());
    }
    for x in options.generator.take(options.limit) {
        let size = (options.bucketing_function)(&x);
        for (i, series) in options.series_options.iter_mut().enumerate() {
            let mut durations_vec = Vec::new();
            for _ in 0..reps {
                let x = x.clone();
                let start_time = precise_time_ns();
                (series.function)(x);
                let end_time = precise_time_ns();
                durations_vec.push(end_time - start_time);
            }
            let median_duration = quick_median(durations_vec);
            durations_maps[i]
                .entry(size)
                .or_insert_with(Vec::new)
                .push(median_duration);
        }
    }
    let mut median_durations_maps = Vec::new();
    for durations_map in durations_maps {
        let mut median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
        for (&size, durations) in &durations_map {
            if durations.len() >= min_bucket_size {
                median_durations_map.insert(size, quick_mean(durations) as u64);
            }
        }
        median_durations_maps.push(median_durations_map);
    }

    let mut fg = Figure::new();
    {
        let axes = fg.axes2d();
        axes.set_title(&escape_label_string(options.title), &[]);
        axes.set_x_label(&escape_label_string(options.x_axis_label), &[]);
        axes.set_y_label(&escape_label_string(options.y_axis_label), &[]);
        for (median_durations_map, options) in median_durations_maps
            .iter()
            .zip(options.series_options.iter())
        {
            let sizes = median_durations_map
                .iter()
                .map(|entry| *entry.0)
                .collect_vec();
            let durations = median_durations_map
                .iter()
                .map(|entry| *entry.1)
                .collect_vec();
            axes.lines(
                &sizes,
                &durations,
                &[
                    Caption(&escape_label_string(options.name)),
                    Color(options.color),
                ],
            );
        }
    }
    fg.echo_to_file(&options.file_name);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BenchmarkType {
    Single,
    LibraryComparison,
    EvaluationStrategy,
    Algorithms,
}

#[allow(clippy::type_complexity)]
pub fn run_benchmark<'a, I: Iterator>(
    title: &'a str,
    benchmark_type: BenchmarkType,
    generator: I,
    generation_mode_name: &'a str,
    limit: usize,
    file_name: &'a str,
    bucketer: &Bucketer<'a, I::Item>,
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
        bucketing_function: bucketer.bucketing_function,
        x_axis_label: &bucketer.bucketing_label,
        y_axis_label: "time (ns)",
        file_name: format!("benchmarks/{}", file_name),
        series_options,
    };
    run_benchmark_internal(options);
}

#[allow(clippy::type_complexity)]
pub fn run_benchmark_old<'a, I: Iterator>(
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
    run_benchmark_internal(options);
}

#[macro_export]
macro_rules! no_out {
    ($e:expr) => {{
        $e;
    }};
}

pub mod bucketers;
