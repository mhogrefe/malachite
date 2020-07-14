use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mod_op::{
    _limbs_mod_limb_small_normalized_large, _limbs_mod_limb_small_small,
};
use malachite_nz::platform::Limb;

use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::pairs_of_nonempty_unsigned_vec_and_unsigned_var_1;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            let mut len = limbs.len();
            let mut remainder = limbs[len - 1];
            if remainder >= divisor {
                remainder -= divisor;
            }
            len -= 1;
            if len == 0 {
                return;
            }
            let limbs = &limbs[..len];
            _limbs_mod_limb_small_small(&limbs, divisor, remainder);
        }),
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            let mut len = limbs.len();
            let mut remainder = limbs[len - 1];
            if remainder >= divisor {
                remainder -= divisor;
            }
            len -= 1;
            if len == 0 {
                return;
            }
            let limbs = &limbs[..len];
            _limbs_mod_limb_small_normalized_large(&limbs, divisor, remainder);
        }),
        pairs_of_nonempty_unsigned_vec_and_unsigned_var_1(GenerationMode::Random(32)),
        10000,
        &(|&(ref limbs, _)| limbs.len() - 1),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MOD_1_NORM_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected mod limb small normalized tuning result: {:?}",
            result
        );
    }
    lines
}
