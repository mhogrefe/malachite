use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::natural::logic::not::limbs_not_in_place;
use crate::platform::Limb;

pub fn limbs_twos_complement_in_place_alt_1(limbs: &mut [Limb]) -> bool {
    let i = limbs.iter().cloned().take_while(|&x| x == 0).count();
    let len = limbs.len();
    if i == len {
        return true;
    }
    limbs[i].wrapping_neg_assign();
    let j = i + 1;
    if j != len {
        limbs_not_in_place(&mut limbs[j..]);
    }
    false
}

pub fn limbs_twos_complement_in_place_alt_2(limbs: &mut [Limb]) -> bool {
    let carry = limbs_sub_limb_in_place(limbs, 1);
    limbs_not_in_place(limbs);
    carry
}
