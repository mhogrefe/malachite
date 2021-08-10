use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::generators::bool_gen;
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_demos!(runner, demo_iverson_primitive_int);
    register_primitive_float_demos!(runner, demo_iverson_primitive_float);
}

fn demo_iverson_primitive_int<T: PrimitiveInt>(gm: GenMode, config: GenConfig, limit: usize) {
    for b in bool_gen().get(gm, &config).take(limit) {
        println!("iverson({}) = {}", b, T::iverson(b));
    }
}

fn demo_iverson_primitive_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for b in bool_gen().get(gm, &config).take(limit) {
        println!("iverson({}) = {}", b, NiceFloat(T::iverson(b)));
    }
}
