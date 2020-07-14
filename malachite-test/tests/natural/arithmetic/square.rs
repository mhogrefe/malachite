use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_nz::natural::arithmetic::square::{
    _limbs_square_to_out_basecase, _limbs_square_to_out_toom_2,
    _limbs_square_to_out_toom_2_scratch_len, _limbs_square_to_out_toom_3,
    _limbs_square_to_out_toom_3_scratch_len,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::arithmetic::square::_limbs_square_to_out_basecase_unrestricted;

use malachite_test::common::{
    test_properties, test_properties_custom_scale, test_properties_no_special,
};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_var_17, pairs_of_unsigned_vec_var_18, pairs_of_unsigned_vec_var_19,
    unsigneds_var_8,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};

fn limbs_square_basecase_helper(out: &[Limb], xs: &[Limb]) -> Vec<Limb> {
    let mut out = out.to_vec();
    let old_out = out.clone();
    _limbs_square_to_out_basecase_unrestricted(&mut out, xs);
    let n = Natural::from_limbs_asc(xs).square();
    let len = xs.len() << 1;
    let mut limbs = n.into_limbs_asc();
    limbs.resize(len, 0);
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
    out
}

#[test]
fn limbs_square_to_out_basecase_properties() {
    test_properties(pairs_of_unsigned_vec_var_17, |&(ref out, ref xs)| {
        let expected_out = limbs_square_basecase_helper(out, xs);
        let mut out = out.to_vec();
        _limbs_square_to_out_basecase(&mut out, xs);
        assert_eq!(out, expected_out);
    });
}

#[test]
fn limbs_square_to_out_toom_2_properties() {
    test_properties_custom_scale(2_048, pairs_of_unsigned_vec_var_18, |&(ref out, ref xs)| {
        let expected_out = limbs_square_basecase_helper(out, xs);
        let mut out = out.to_vec();
        let mut scratch = vec![0; _limbs_square_to_out_toom_2_scratch_len(xs.len())];
        _limbs_square_to_out_toom_2(&mut out, xs, &mut scratch);
        assert_eq!(out, expected_out);
    });
}

#[test]
fn limbs_square_to_out_toom_3_properties() {
    test_properties_custom_scale(2_048, pairs_of_unsigned_vec_var_19, |&(ref out, ref xs)| {
        let expected_out = limbs_square_basecase_helper(out, xs);
        let mut out = out.to_vec();
        let mut scratch = vec![0; _limbs_square_to_out_toom_3_scratch_len(xs.len())];
        _limbs_square_to_out_toom_3(&mut out, xs, &mut scratch);
        assert_eq!(out, expected_out);
    });
}

#[test]
fn square_properties() {
    test_properties(naturals, |x| {
        let square = x.square();
        assert!(square.is_valid());

        let mut mut_x = x.clone();
        mut_x.square_assign();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);

        assert_eq!(x * x, square);
        assert!(square >= *x);
    });

    test_properties(pairs_of_naturals, |(x, y)| {
        assert_eq!((x * y).square(), x.square() * y.square());
    });

    test_properties_no_special(unsigneds_var_8::<Limb>, |&x| {
        assert_eq!(x.square(), Natural::from(x).square());
    });
}
