use std::collections::HashMap;

use generators::common::{GenConfig, GenMode};

pub type DemoFn = &'static dyn Fn(GenMode, GenConfig, usize);
pub type BenchFn = &'static dyn Fn(GenMode, GenConfig, usize, &str);

pub struct Runner {
    demo_map: HashMap<&'static str, DemoFn>,
    bench_map: HashMap<&'static str, BenchFn>,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            demo_map: HashMap::new(),
            bench_map: HashMap::new(),
        }
    }

    pub fn run_demo(&self, key: &str, gm: GenMode, config: GenConfig, limit: usize) {
        self.demo_map.get(key).expect("Invalid demo key")(gm, config, limit);
    }

    pub fn run_bench(
        &self,
        key: &str,
        gm: GenMode,
        config: GenConfig,
        limit: usize,
        file_name: &str,
    ) {
        self.bench_map.get(key).expect("Invalid bench key")(gm, config, limit, file_name);
    }

    pub fn register_demo(&mut self, key: &'static str, f: DemoFn) {
        assert!(
            self.demo_map.insert(key, f).is_none(),
            "Duplicate demo with key {}",
            key
        );
    }

    pub fn register_bench(&mut self, key: &'static str, f: BenchFn) {
        assert!(
            self.bench_map.insert(key, f).is_none(),
            "Duplicate bench with key {}",
            key
        );
    }
}

impl Default for Runner {
    fn default() -> Runner {
        Runner::new()
    }
}

#[macro_export]
macro_rules! register_demo {
    ($runner: ident, $f: ident) => {
        $runner.register_demo(stringify!($f), &$f);
    };
}

#[macro_export]
macro_rules! register_generic_demos {
    ($runner: ident, $f: ident $(,$t:ty)*) => {
        $(
            $runner.register_demo(concat!(stringify!($f), "_", stringify!($t)), &$f::<$t>);
        )*
    };
}

#[macro_export]
macro_rules! register_generic_demos_2 {
    ($runner: ident, $f: ident $(,[$t:ty, $u:ty])*) => {
        $(
            $runner.register_demo(
                concat!(stringify!($f), "_", stringify!($t), "_", stringify!($u)),
                &$f::<$t, $u>
            );
        )*
    };
}

#[macro_export]
macro_rules! register_unsigned_demos {
    ($runner: ident, $f: ident) => {
        register_generic_demos!($runner, $f, u8, u16, u32, u64, u128, usize);
    };
}

#[macro_export]
macro_rules! register_signed_demos {
    ($runner: ident, $f: ident) => {
        register_generic_demos!($runner, $f, i8, i16, i32, i64, i128, isize);
    };
}

#[macro_export]
macro_rules! register_unsigned_unsigned_demos {
    ($runner: ident, $f: ident) => {
        register_generic_demos_2!(
            $runner,
            $f,
            [u8, u8],
            [u8, u16],
            [u8, u32],
            [u8, u64],
            [u8, u128],
            [u8, usize],
            [u16, u8],
            [u16, u16],
            [u16, u32],
            [u16, u64],
            [u16, u128],
            [u16, usize],
            [u32, u8],
            [u32, u16],
            [u32, u32],
            [u32, u64],
            [u32, u128],
            [u32, usize],
            [u64, u8],
            [u64, u16],
            [u64, u32],
            [u64, u64],
            [u64, u128],
            [u64, usize],
            [u128, u8],
            [u128, u16],
            [u128, u32],
            [u128, u64],
            [u128, u128],
            [u128, usize],
            [usize, u8],
            [usize, u16],
            [usize, u32],
            [usize, u64],
            [usize, u128],
            [usize, usize]
        );
    };
}

#[macro_export]
macro_rules! register_signed_unsigned_demos {
    ($runner: ident, $f: ident) => {
        register_generic_demos_2!(
            $runner,
            $f,
            [i8, u8],
            [i8, u16],
            [i8, u32],
            [i8, u64],
            [i8, u128],
            [i8, usize],
            [i16, u8],
            [i16, u16],
            [i16, u32],
            [i16, u64],
            [i16, u128],
            [i16, usize],
            [i32, u8],
            [i32, u16],
            [i32, u32],
            [i32, u64],
            [i32, u128],
            [i32, usize],
            [i64, u8],
            [i64, u16],
            [i64, u32],
            [i64, u64],
            [i64, u128],
            [i64, usize],
            [i128, u8],
            [i128, u16],
            [i128, u32],
            [i128, u64],
            [i128, u128],
            [i128, usize],
            [isize, u8],
            [isize, u16],
            [isize, u32],
            [isize, u64],
            [isize, u128],
            [isize, usize]
        );
    };
}

#[macro_export]
macro_rules! register_primitive_int_unsigned_demos {
    ($runner: ident, $f: ident) => {
        register_unsigned_unsigned_demos!($runner, $f);
        register_signed_unsigned_demos!($runner, $f);
    };
}

#[macro_export]
macro_rules! register_unsigned_signed_demos {
    ($runner: ident, $f: ident) => {
        register_generic_demos_2!(
            $runner,
            $f,
            [u8, i8],
            [u8, i16],
            [u8, i32],
            [u8, i64],
            [u8, i128],
            [u8, isize],
            [u16, i8],
            [u16, i16],
            [u16, i32],
            [u16, i64],
            [u16, i128],
            [u16, isize],
            [u32, i8],
            [u32, i16],
            [u32, i32],
            [u32, i64],
            [u32, i128],
            [u32, isize],
            [u64, i8],
            [u64, i16],
            [u64, i32],
            [u64, i64],
            [u64, i128],
            [u64, isize],
            [u128, i8],
            [u128, i16],
            [u128, i32],
            [u128, i64],
            [u128, i128],
            [u128, isize],
            [usize, i8],
            [usize, i16],
            [usize, i32],
            [usize, i64],
            [usize, i128],
            [usize, isize]
        );
    };
}

#[macro_export]
macro_rules! register_signed_signed_demos {
    ($runner: ident, $f: ident) => {
        register_generic_demos_2!(
            $runner,
            $f,
            [i8, i8],
            [i8, i16],
            [i8, i32],
            [i8, i64],
            [i8, i128],
            [i8, isize],
            [i16, i8],
            [i16, i16],
            [i16, i32],
            [i16, i64],
            [i16, i128],
            [i16, isize],
            [i32, i8],
            [i32, i16],
            [i32, i32],
            [i32, i64],
            [i32, i128],
            [i32, isize],
            [i64, i8],
            [i64, i16],
            [i64, i32],
            [i64, i64],
            [i64, i128],
            [i64, isize],
            [i128, i8],
            [i128, i16],
            [i128, i32],
            [i128, i64],
            [i128, i128],
            [i128, isize],
            [isize, i8],
            [isize, i16],
            [isize, i32],
            [isize, i64],
            [isize, i128],
            [isize, isize]
        );
    };
}

#[macro_export]
macro_rules! register_primitive_int_signed_demos {
    ($runner: ident, $f: ident) => {
        register_unsigned_signed_demos!($runner, $f);
        register_signed_signed_demos!($runner, $f);
    };
}

#[macro_export]
macro_rules! register_bench {
    ($runner: ident, $f: ident) => {
        $runner.register_bench(stringify!($f), &$f);
    };
}

#[macro_export]
macro_rules! register_generic_benches {
    ($runner: ident, $f: ident $(,$t:ty)*) => {
        $(
            $runner.register_bench(concat!(stringify!($f), "_", stringify!($t)), &$f::<$t>);
        )*
    };
}

#[macro_export]
macro_rules! register_generic_benches_2 {
    ($runner: ident, $f: ident $(,[$t:ty, $u:ty])*) => {
        $(
            $runner.register_bench(
                concat!(stringify!($f), "_", stringify!($t), "_", stringify!($u)),
                &$f::<$t, $u>
            );
        )*
    };
}

#[macro_export]
macro_rules! register_unsigned_benches {
    ($runner: ident, $f: ident) => {
        register_generic_benches!($runner, $f, u8, u16, u32, u64, u128, usize);
    };
}

#[macro_export]
macro_rules! register_signed_benches {
    ($runner: ident, $f: ident) => {
        register_generic_benches!($runner, $f, i8, i16, i32, i64, i128, isize);
    };
}

#[macro_export]
macro_rules! register_unsigned_unsigned_benches {
    ($runner: ident, $f: ident) => {
        register_generic_benches_2!(
            $runner,
            $f,
            [u8, u8],
            [u8, u16],
            [u8, u32],
            [u8, u64],
            [u8, u128],
            [u8, usize],
            [u16, u8],
            [u16, u16],
            [u16, u32],
            [u16, u64],
            [u16, u128],
            [u16, usize],
            [u32, u8],
            [u32, u16],
            [u32, u32],
            [u32, u64],
            [u32, u128],
            [u32, usize],
            [u64, u8],
            [u64, u16],
            [u64, u32],
            [u64, u64],
            [u64, u128],
            [u64, usize],
            [u128, u8],
            [u128, u16],
            [u128, u32],
            [u128, u64],
            [u128, u128],
            [u128, usize],
            [usize, u8],
            [usize, u16],
            [usize, u32],
            [usize, u64],
            [usize, u128],
            [usize, usize]
        );
    };
}

#[macro_export]
macro_rules! register_signed_unsigned_benches {
    ($runner: ident, $f: ident) => {
        register_generic_benches_2!(
            $runner,
            $f,
            [i8, u8],
            [i8, u16],
            [i8, u32],
            [i8, u64],
            [i8, u128],
            [i8, usize],
            [i16, u8],
            [i16, u16],
            [i16, u32],
            [i16, u64],
            [i16, u128],
            [i16, usize],
            [i32, u8],
            [i32, u16],
            [i32, u32],
            [i32, u64],
            [i32, u128],
            [i32, usize],
            [i64, u8],
            [i64, u16],
            [i64, u32],
            [i64, u64],
            [i64, u128],
            [i64, usize],
            [i128, u8],
            [i128, u16],
            [i128, u32],
            [i128, u64],
            [i128, u128],
            [i128, usize],
            [isize, u8],
            [isize, u16],
            [isize, u32],
            [isize, u64],
            [isize, u128],
            [isize, usize]
        );
    };
}

#[macro_export]
macro_rules! register_primitive_int_unsigned_benches {
    ($runner: ident, $f: ident) => {
        register_unsigned_unsigned_benches!($runner, $f);
        register_signed_unsigned_benches!($runner, $f);
    };
}

#[macro_export]
macro_rules! register_unsigned_signed_benches {
    ($runner: ident, $f: ident) => {
        register_generic_benches_2!(
            $runner,
            $f,
            [u8, i8],
            [u8, i16],
            [u8, i32],
            [u8, i64],
            [u8, i128],
            [u8, isize],
            [u16, i8],
            [u16, i16],
            [u16, i32],
            [u16, i64],
            [u16, i128],
            [u16, isize],
            [u32, i8],
            [u32, i16],
            [u32, i32],
            [u32, i64],
            [u32, i128],
            [u32, isize],
            [u64, i8],
            [u64, i16],
            [u64, i32],
            [u64, i64],
            [u64, i128],
            [u64, isize],
            [u128, i8],
            [u128, i16],
            [u128, i32],
            [u128, i64],
            [u128, i128],
            [u128, isize],
            [usize, i8],
            [usize, i16],
            [usize, i32],
            [usize, i64],
            [usize, i128],
            [usize, isize]
        );
    };
}

#[macro_export]
macro_rules! register_signed_signed_benches {
    ($runner: ident, $f: ident) => {
        register_generic_benches_2!(
            $runner,
            $f,
            [i8, i8],
            [i8, i16],
            [i8, i32],
            [i8, i64],
            [i8, i128],
            [i8, isize],
            [i16, i8],
            [i16, i16],
            [i16, i32],
            [i16, i64],
            [i16, i128],
            [i16, isize],
            [i32, i8],
            [i32, i16],
            [i32, i32],
            [i32, i64],
            [i32, i128],
            [i32, isize],
            [i64, i8],
            [i64, i16],
            [i64, i32],
            [i64, i64],
            [i64, i128],
            [i64, isize],
            [i128, i8],
            [i128, i16],
            [i128, i32],
            [i128, i64],
            [i128, i128],
            [i128, isize],
            [isize, i8],
            [isize, i16],
            [isize, i32],
            [isize, i64],
            [isize, i128],
            [isize, isize]
        );
    };
}

#[macro_export]
macro_rules! register_primitive_int_signed_benches {
    ($runner: ident, $f: ident) => {
        register_unsigned_signed_benches!($runner, $f);
        register_signed_signed_benches!($runner, $f);
    };
}

pub mod cmd;
