use malachite_base::comparison::Max;
use malachite_base::limbs::limbs_pad_left;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{CheckedFrom, WrappingFrom};
use malachite_nz::natural::random::random_natural_up_to_bits::limbs_random_up_to_bits;
use malachite_nz::platform::Limb;
use rand::{IsaacRng, Rng, SeedableRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use std::cmp::max;
use tune::speed::{speed_measure, SpeedParams};

const DATA_HIGH_LT_R: u64 = 1;
const DATA_HIGH_GE_R: u64 = 2;
// limbs
const DEFAULT_MAX_SIZE: usize = 1000;
const OPTION_TRACE: u32 = 0;

pub struct Param<'a> {
    name: String,
    function: &'a Fn(&SpeedParams) -> f64,
    function2: &'a Fn(&SpeedParams) -> f64,
    // how much to step relatively
    step_factor: f64,
    // how much to step absolutely
    step: u64,
    // multiplier for "function" speeds
    function_fudge: f64,
    stop_since_change: u64,
    stop_factor: f64,
    min_size: usize,
    min_is_always: u64,
    max_size: usize,
    check_size: usize,
    size_extra: usize,
    data_high: u64,
    noprint: bool,
}

pub struct Data {
    size: usize,
    d: f64,
}

fn add_dat(dat: &mut Vec<Data>, size: usize, d: f64) {
    dat.push(Data { size, d });
}

// Return the threshold size based on the data accumulated.
fn analyze_dat(dat: &[Data], final0: u32) -> usize {
    //double  x, min_x;
    //int     j, min_j;

    // If the threshold is set at dat[0].size, any positive values are bad.
    let mut x: f64 = 0.0;
    for j in 0..dat.len() {
        if dat[j].d > 0.0 {
            x += dat[j].d;
        }
    }

    if OPTION_TRACE >= 2 && final0 != 0 {
        println!("x is the sum of the badness from setting thresh at given size");
        println!("(minimum x is sought)");
        println!("size={}, first x={}", dat.last().unwrap().size, x);
    }

    let mut min_x = x;
    let mut min_j = 0;

    // When stepping to the next dat[j].size, positive values are no longer
    // bad (so subtracted), negative values become bad (so add the absolute
    // value, meaning subtract).
    for j in 0..dat.len() {
        if OPTION_TRACE >= 2 && final0 != 0 {
            println!("size={}  x={}", dat[j].size, x);
        }

        if x < min_x {
            min_x = x;
            min_j = j;
        }
        x -= dat[j].d;
    }

    min_j
}

fn print_define_start(name: &str) {
    print!("{}: ", name);
    if OPTION_TRACE != 0 {
        println!("...\n");
    }
}

fn print_define_end_remark(name: &str, value: usize, remark: &str) {
    if OPTION_TRACE != 0 {
        print!("{}: ", name);
    }
    if value == usize::MAX {
        print!("usize::MAX");
    } else {
        print!("{}", value);
    }

    if remark.len() != 0 {
        print!("  /* {} */", remark);
    }
    print!("\n");
}

fn print_define_end(name: &str, value: usize) {
    let remark = if value == usize::MAX {
        "never"
    } else if value == 0 {
        "always"
    } else {
        ""
    };
    print_define_end_remark(name, value, remark);
}

fn print_define(name: &str, value: usize) {
    print_define_start(name);
    print_define_end(name, value);
}

fn set_default<T: Default + PartialEq>(reference: &mut T, default_value: T) {
    if *reference == T::default() {
        *reference = default_value;
    }
}

pub fn tuneup_measure<R: Rng>(
    rng: &mut R,
    fun: &Fn(&SpeedParams) -> f64,
    param: &Param,
    s: &mut SpeedParams,
) -> u64 {
    s.size += param.size_extra;
    let size = u64::wrapping_from(s.size * usize::wrapping_from(Limb::WIDTH));
    s.xp = limbs_random_up_to_bits(rng, size);
    let x_len = s.xp.len();
    if x_len < s.size {
        limbs_pad_left(&mut s.xp, s.size - x_len, 0);
    }
    s.yp = limbs_random_up_to_bits(rng, size);
    let y_len = s.yp.len();
    if y_len < s.size {
        limbs_pad_left(&mut s.yp, s.size - y_len, 0);
    }

    match param.data_high {
        DATA_HIGH_LT_R => {
            s.xp[s.size - 1] %= s.r;
            s.yp[s.size - 1] %= s.r;
        }
        DATA_HIGH_GE_R => {
            s.xp[s.size - 1] |= s.r;
            s.yp[s.size - 1] |= s.r;
        }
        _ => {}
    }

    let t = speed_measure(fun, s);
    s.size -= param.size_extra;
    t
}

pub fn one(threshold: &mut usize, param: &mut Param, s: &mut SpeedParams, dat: &mut Vec<Data>) {
    //int  since_positive, since_thresh_change;
    //int  thresh_idx, new_thresh_idx;

    set_default(&mut param.function_fudge, 1.0);
    // small steps by default
    set_default(&mut param.step_factor, 0.01);
    // small steps by default
    set_default(&mut param.step, 1);
    set_default(&mut param.stop_since_change, 80);
    set_default(&mut param.stop_factor, 1.2);
    set_default(&mut param.min_size, 10);
    set_default(&mut param.max_size, DEFAULT_MAX_SIZE);
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    if param.check_size != 0 {
        //double   t1, t2;
        s.size = param.check_size;

        *threshold = s.size + 1;
        let mut t1 = tuneup_measure(&mut rng, param.function, param, s) as f64;

        *threshold = s.size;
        let t2 = tuneup_measure(&mut rng, param.function2, param, s) as f64;
        if t1 == -1.0 || t2 == -1.0 {
            panic!("Oops, can't run both functions at size {}", s.size);
        }
        t1 *= param.function_fudge;

        // ask that t2 is at least 4% below t1
        if t1 < t2 * 1.04 {
            if OPTION_TRACE != 0 {
                println!("function2 never enough faster: t1={} t2={}", t1, t2);
            }
            *threshold = usize::MAX;
            if !param.noprint {
                print_define(&param.name, *threshold);
            }
            return;
        }

        if OPTION_TRACE >= 2 {
            println!(
                "function2 enough faster at size={}: t1={} t2={}",
                s.size, t1, t2
            );
        }
    }

    if !param.noprint || OPTION_TRACE != 0 {
        print_define_start(&param.name);
    }

    let mut since_positive = 0;
    let mut since_thresh_change = 0;
    let mut thresh_idx = 0;

    if OPTION_TRACE >= 2 {
        println!("             algorithm-A  algorithm-B   ratio  possible");
        println!("              (seconds)    (seconds)    diff    thresh");
    }

    s.size = param.min_size;
    while s.size < param.max_size {
        //double   ti, tiplus1, d;

        // using method A at this size
        *threshold = s.size + 1;
        let mut ti = tuneup_measure(&mut rng, param.function, param, s) as f64;
        if ti == -1.0 {
            panic!();
        }
        ti *= param.function_fudge;

        // using method B at this size
        *threshold = s.size;
        let tiplus1 = tuneup_measure(&mut rng, param.function2, param, s) as f64;
        if tiplus1 == -1.0 {
            panic!();
        }

        // Calculate the fraction by which the one or the other routine is slower.
        let d = if tiplus1 >= ti {
            (tiplus1 - ti) / tiplus1 // negative
        } else {
            (tiplus1 - ti) / ti // positive
        };
        add_dat(dat, s.size, d);
        let new_thresh_idx = analyze_dat(dat, 0);

        if OPTION_TRACE >= 2 {
            println!(
                "size={}  {}  {}  {} {} {}",
                s.size,
                ti,
                tiplus1,
                d,
                if ti > tiplus1 { '#' } else { ' ' },
                dat[new_thresh_idx].size
            );
        }

        // Stop if the last time method i was faster was more than a
        // certain number of measurements ago.
        const STOP_SINCE_POSITIVE: usize = 200;
        if d >= 0.0 {
            since_positive = 0;
        } else {
            since_positive += 1;
            if since_positive > STOP_SINCE_POSITIVE {
                if OPTION_TRACE >= 1 {
                    println!("stopped due to since_positive {}", STOP_SINCE_POSITIVE);
                }
                break;
            }
        }

        // Stop if method A has become slower by a certain factor.
        if ti >= tiplus1 * param.stop_factor {
            if OPTION_TRACE >= 1 {
                println!(
                    "stopped due to ti >= tiplus1 * factor ({})",
                    param.stop_factor
                );
            }
            break;
        }

        // Stop if the threshold implied hasn't changed in a certain
        // number of measurements.  (It's this condition that usually
        // stops the loop.) */
        if thresh_idx != new_thresh_idx {
            since_thresh_change = 0;
            thresh_idx = new_thresh_idx;
        } else {
            since_thresh_change += 1;
            if since_thresh_change > param.stop_since_change {
                if OPTION_TRACE >= 1 {
                    println!(
                        "stopped due to since_thresh_change ({})",
                        param.stop_since_change
                    );
                }
                break;
            }
        }

        // Stop if the threshold implied is more than a certain number of
        // measurements ago.
        const STOP_SINCE_AFTER: usize = 500;
        if dat.len() - thresh_idx > STOP_SINCE_AFTER {
            if OPTION_TRACE >= 1 {
                println!(
                    "stopped due to ndat - thresh_idx > amount ({})",
                    STOP_SINCE_AFTER
                );
            }
            break;
        }

        // Stop when the size limit is reached before the end of the
        // crossover, but only show this as an error for >= the default max
        // size.
        if s.size >= param.max_size && param.max_size >= DEFAULT_MAX_SIZE {
            panic!();
            //fprintf (stderr, "%s\n", param->name);
            //fprintf (stderr, "sizes %ld to %ld total %d measurements\n",
            //         (long) dat[0].size, (long) dat[ndat-1].size, ndat);
            //fprintf (stderr, "    max size reached before end of crossover\n");
            // break;
        }

        s.size += max(
            (s.size as f64 * param.step_factor).floor() as usize,
            usize::checked_from(param.step).unwrap(),
        )
    }

    if OPTION_TRACE >= 1 {
        println!(
            "sizes {} to {} total {} measurements",
            dat[0].size,
            dat[dat.len() - 1].size,
            dat.len()
        );
    }

    *threshold = dat[analyze_dat(&dat, 1)].size;

    if param.min_is_always != 0 {
        if *threshold == param.min_size {
            *threshold = 0;
        }
    }

    if !param.noprint || OPTION_TRACE != 0 {
        print_define_end(&param.name, *threshold);
    }
}
