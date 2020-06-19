use malachite_nz::natural::logic::not::{limbs_not, limbs_not_in_place, limbs_not_to_out};
use malachite_nz_test_util::common::{natural_to_rug_integer, rug_integer_to_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigned_vec_var_3, vecs_of_unsigned};
use malachite_test::inputs::natural::naturals;

#[test]
fn limbs_not_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(limbs_not(&limbs_not(limbs)), *limbs);
    });
}

#[test]
fn limbs_not_to_out_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref out, ref limbs_in)| {
        let mut mut_out = out.to_vec();
        limbs_not_to_out(&mut mut_out, limbs_in);
        limbs_not_in_place(&mut mut_out[..limbs_in.len()]);
        assert_eq!(mut_out[..limbs_in.len()], **limbs_in);
    });
}

#[test]
fn limbs_not_in_place_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        let mut mut_limbs = limbs.to_vec();
        limbs_not_in_place(&mut mut_limbs);
        limbs_not_in_place(&mut mut_limbs);
        assert_eq!(mut_limbs, *limbs);
    });
}

#[test]
fn not_properties() {
    test_properties(naturals, |x| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rug_not = !natural_to_rug_integer(x);
        assert_eq!(rug_integer_to_integer(&rug_not), not);

        let not_alt = !x;
        assert!(not_alt.is_valid());
        assert_eq!(not_alt, not);

        assert!(not < 0);
        assert_ne!(not, *x);
        assert_eq!(!&not, *x);
    });
}
