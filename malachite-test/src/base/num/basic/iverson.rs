use malachite_base::num::basic::integers::PrimitiveInteger;

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode};
use malachite_test::inputs::base::bools;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_iverson);
    register_ns_demo!(registry, demo_u16_iverson);
    register_ns_demo!(registry, demo_u32_iverson);
    register_ns_demo!(registry, demo_u64_iverson);
    register_ns_demo!(registry, demo_u128_iverson);
    register_ns_demo!(registry, demo_usize_iverson);
    register_ns_demo!(registry, demo_i8_iverson);
    register_ns_demo!(registry, demo_i16_iverson);
    register_ns_demo!(registry, demo_i32_iverson);
    register_ns_demo!(registry, demo_i64_iverson);
    register_ns_demo!(registry, demo_i128_iverson);
    register_ns_demo!(registry, demo_isize_iverson);
}

fn demo_iverson<T: PrimitiveInteger>(gm: NoSpecialGenerationMode, limit: usize) {
    for b in bools(gm).take(limit) {
        println!("iverson({}) = {}", b, T::iverson(b));
    }
}

macro_rules! demo {
    ($t:ident, $demo_name:ident) => {
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_iverson::<$t>(gm, limit);
        }
    };
}

demo!(u8, demo_u8_iverson);
demo!(u16, demo_u16_iverson);
demo!(u32, demo_u32_iverson);
demo!(u64, demo_u64_iverson);
demo!(u128, demo_u128_iverson);
demo!(usize, demo_usize_iverson);
demo!(i8, demo_i8_iverson);
demo!(i16, demo_i16_iverson);
demo!(i32, demo_i32_iverson);
demo!(i64, demo_i64_iverson);
demo!(i128, demo_i128_iverson);
demo!(isize, demo_isize_iverson);
