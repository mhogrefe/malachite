use malachite_nz::natural::arithmetic::mul::poly_interpolate::_limbs_shl_and_sub_same_length;
use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use time::precise_time_ns;

use common::GenerationMode;
use inputs::base::pairs_of_unsigned_vec_var_1;

pub(crate) fn tune() -> Vec<String> {
    let mut aorsmul_faster_2aorslsh_vote = 0;
    let mut aorsmul_faster_3aorslsh_vote = 0;
    let mut aorsmul_faster_aors_aorslsh_vote = 0;
    let mut aorsmul_faster_aors_2aorslsh_vote = 0;
    let limit = 100_000;
    for (xs, ys) in pairs_of_unsigned_vec_var_1(GenerationMode::Random(32)).take(limit) {
        let mut mut_xs = xs.to_vec();
        let start_time = precise_time_ns();
        limbs_sub_mul_limb_same_length_in_place_left(&mut mut_xs, &ys, 257);
        let end_time = precise_time_ns();
        let aorsmul_time = end_time - start_time;

        let mut mut_xs = xs.to_vec();
        let start_time = precise_time_ns();
        limbs_sub_same_length_in_place_left(&mut mut_xs, &ys);
        let end_time = precise_time_ns();
        let aors_time = end_time - start_time;

        let mut mut_xs = xs.to_vec();
        let mut scratch = vec![0; xs.len()];
        let start_time = precise_time_ns();
        _limbs_shl_and_sub_same_length(&mut mut_xs, &ys, 12, &mut scratch);
        let end_time = precise_time_ns();
        let aorslsh_time = end_time - start_time;

        if aorsmul_time < 2 * aorslsh_time {
            aorsmul_faster_2aorslsh_vote += 1;
            aorsmul_faster_3aorslsh_vote += 1;
        } else if aorsmul_time < 3 * aorslsh_time {
            aorsmul_faster_3aorslsh_vote += 1;
        }
        if aorsmul_time < aors_time + aorslsh_time {
            aorsmul_faster_aors_aorslsh_vote += 1;
            aorsmul_faster_aors_2aorslsh_vote += 1;
        } else if aorsmul_time < aors_time + 2 * aorslsh_time {
            aorsmul_faster_aors_2aorslsh_vote += 1;
        }
    }
    let half_limit = limit >> 1;
    let mut lines = Vec::new();
    lines.push(format!(
        "pub const AORSMUL_FASTER_2AORSLSH: bool = {};",
        aorsmul_faster_2aorslsh_vote >= half_limit
    ));
    lines.push(format!(
        "pub const AORSMUL_FASTER_3AORSLSH: bool = {};",
        aorsmul_faster_3aorslsh_vote >= half_limit
    ));
    lines.push(format!(
        "pub const AORSMUL_FASTER_AORS_AORSLSH: bool = {};",
        aorsmul_faster_aors_aorslsh_vote >= half_limit
    ));
    lines.push(format!(
        "pub const AORSMUL_FASTER_AORS_2AORSLSH: bool = {};",
        aorsmul_faster_aors_2aorslsh_vote >= half_limit
    ));
    lines
}
