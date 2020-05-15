use malachite_base::num::logic::traits::NotAssign;

use common::{DemoBenchRegistry, NoSpecialGenerationMode};
use inputs::base::bools;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_bool_not_assign);
}

fn demo_bool_not_assign(gm: NoSpecialGenerationMode, limit: usize) {
    for mut b in bools(gm).take(limit) {
        let b_old = b;
        b.not_assign();
        println!("b := {:?}; b.not_assign(); b = {:?}", b_old, b);
    }
}
