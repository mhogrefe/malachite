use generators::common::{GenConfig, GenMode};
use std::collections::HashMap;

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

pub mod cmd;
