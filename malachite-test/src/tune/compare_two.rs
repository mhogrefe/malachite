use std::collections::{BTreeMap, HashMap};

use stats::{mean, median};
use time::precise_time_ns;

#[derive(Debug)]
pub enum ComparisonResult {
    NeitherBetter,
    FirstAlwaysBetter,
    SecondAlwaysBetter,
    FirstBetterAbove(usize),
    SecondBetterAbove(usize),
}

pub fn compare_two<'a, I: Iterator>(
    first: &'a mut FnMut(I::Item),
    second: &'a mut FnMut(I::Item),
    generator: I,
    limit: usize,
    bucketing_function: &'a Fn(&I::Item) -> usize,
) -> ComparisonResult
where
    I::Item: Clone,
{
    let reps = 10;
    let min_bucket_size = 2;
    let mut durations_map_1 = HashMap::new();
    let mut durations_map_2 = HashMap::new();
    for x in generator.take(limit) {
        let size = bucketing_function(&x);
        let mut durations_vec_1 = Vec::new();
        let mut durations_vec_2 = Vec::new();
        for _ in 0..reps {
            let x_1 = x.clone();
            let start_time = precise_time_ns();
            first(x_1);
            let end_time = precise_time_ns();
            durations_vec_1.push(end_time - start_time);
            let x_2 = x.clone();
            let start_time = precise_time_ns();
            second(x_2);
            let end_time = precise_time_ns();
            durations_vec_2.push(end_time - start_time);
        }
        let median_duration_1 = median(durations_vec_1.iter().cloned()).unwrap();
        let median_duration_2 = median(durations_vec_2.iter().cloned()).unwrap();
        durations_map_1
            .entry(size)
            .or_insert_with(Vec::new)
            .push(median_duration_1);
        durations_map_2
            .entry(size)
            .or_insert_with(Vec::new)
            .push(median_duration_2);
    }

    let mut median_durations_map_1: BTreeMap<usize, u64> = BTreeMap::new();
    let mut median_durations_map_2: BTreeMap<usize, u64> = BTreeMap::new();
    for (&size, durations) in &durations_map_1 {
        if durations.len() >= min_bucket_size {
            median_durations_map_1.insert(size, mean(durations.iter().cloned()) as u64);
        }
    }
    for (&size, durations) in &durations_map_2 {
        if durations.len() >= min_bucket_size {
            median_durations_map_2.insert(size, mean(durations.iter().cloned()) as u64);
        }
    }

    let mut win_map: BTreeMap<usize, bool> = BTreeMap::new();
    let mut firsts = 0;
    let mut seconds = 0;
    for s in median_durations_map_1.keys() {
        let w = median_durations_map_1[s] >= median_durations_map_2[s];
        win_map.insert(*s, w);
        if w {
            seconds += 1;
        } else {
            firsts += 1;
        }
    }
    let total = firsts + seconds;
    let mut max_successes = None;
    let mut first_then_second_cutoff = 0;
    let mut min_successes = None;
    let mut second_then_first_cutoff = 0;
    let mut firsts_lt = 0;
    let mut seconds_lt = 0;
    for (s, w) in win_map {
        if w {
            seconds_lt += 1;
        } else {
            firsts_lt += 1;
        }
        let firsts_ge = firsts - firsts_lt;
        let seconds_ge = seconds - seconds_lt;
        let first_then_second_successes = firsts_lt + seconds_ge - firsts_ge - seconds_lt;
        if max_successes.map_or(true, |m| first_then_second_successes > m) {
            max_successes = Some(first_then_second_successes);
            first_then_second_cutoff = s;
        }
        if min_successes.map_or(true, |m| first_then_second_successes < m) {
            min_successes = Some(first_then_second_successes);
            second_then_first_cutoff = s;
        }
    }
    let min_successes = min_successes.unwrap();
    let max_successes = max_successes.unwrap();
    let result = if firsts * 100 < seconds {
        ComparisonResult::SecondAlwaysBetter
    } else if seconds * 100 < firsts {
        ComparisonResult::FirstAlwaysBetter
    } else if max_successes * 10 >= total {
        ComparisonResult::SecondBetterAbove(first_then_second_cutoff)
    } else if -min_successes * 10 >= total {
        ComparisonResult::FirstBetterAbove(second_then_first_cutoff)
    } else {
        ComparisonResult::NeitherBetter
    };
    //println!("max_successes: {}", max_successes);
    //println!("first_then_second_cutoff: {}", first_then_second_cutoff);
    //println!("min_successes: {}", min_successes);
    //println!("second_then_first_cutoff: {}", second_then_first_cutoff);
    //println!("result: {:?}", result);
    result
}
