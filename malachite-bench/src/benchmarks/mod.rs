use gnuplot::{AxesCommon, Caption, Color, Figure};
use stats::{mean, median};
use std::collections::{BTreeMap, HashMap};
use time::precise_time_ns;

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

pub struct BenchmarkSeriesOptions<'a, T: 'a> {
    pub name: &'a str,
    pub function: &'a mut dyn FnMut(T),
    pub color: &'a str,
}

pub struct BenchmarkOptions<'a, I: Iterator>
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

pub fn run_benchmark<I: Iterator>(mut options: BenchmarkOptions<I>)
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
            let median_duration = median(durations_vec.iter().cloned()).unwrap();
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
                median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
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
            let sizes: Vec<usize> = median_durations_map.iter().map(|entry| *entry.0).collect();
            let durations: Vec<u64> = median_durations_map.iter().map(|entry| *entry.1).collect();
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
