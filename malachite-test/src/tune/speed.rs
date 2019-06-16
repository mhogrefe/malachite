use malachite_nz::platform::Limb;
use stats::median;
use time::precise_time_ns;

#[derive(Default)]
pub struct SpeedParams {
    // how many times to run the routine
    pub reps: u64,
    // first argument
    pub xp: Vec<Limb>,
    // second argument
    pub yp: Vec<Limb>,
    // size of both arguments
    pub size: usize,
    // user supplied parameter
    pub r: Limb,
    // alignment of xp
    pub align_xp: usize,
    // alignment of yp
    pub align_yp: usize,
    // intended alignment of wp
    pub align_wp: usize,
    // intended alignment of wp2
    pub align_wp2: usize,
    // first special SPEED_BLOCK_SIZE block
    pub xp_block: Vec<Limb>,
    // second special SPEED_BLOCK_SIZE block
    pub yp_block: Vec<Limb>,
    // optionally set by the speed routine
    pub time_divisor: f64,
    // used by the cache priming things
    pub cache: u64,
    pub src_num: u64,
    pub dst_num: u64,
    pub src: [Vec<Limb>; 5],
    pub dst: [Vec<Limb>; 4],
}

pub fn speed_measure(fun: &Fn(&SpeedParams) -> f64, s: &SpeedParams) -> u64 {
    let mut durations_vec = Vec::with_capacity(10);
    for _ in 0..10 {
        let start_time = precise_time_ns();
        fun(s);
        let end_time = precise_time_ns();
        durations_vec.push(end_time - start_time);
    }
    median(durations_vec.iter().cloned()).unwrap() as u64
}
