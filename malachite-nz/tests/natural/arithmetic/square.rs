use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{Square, SquareAssign};

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::square::{
    _limbs_square_to_out_basecase, SQR_TOOM2_THRESHOLD,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
fn limbs_square_basecase_helper(out: &[Limb], xs: &[Limb]) -> Vec<Limb> {
    let mut out = out.to_vec();
    let old_out = out.clone();
    _limbs_square_to_out_basecase(&mut out, xs);
    let n = Natural::from_limbs_asc(xs).square();
    let len = xs.len() << 1;
    let mut limbs = n.into_limbs_asc();
    limbs.resize(len, 0);
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
    out
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_square_to_out_basecase() {
    let test = |out_before: &[Limb], xs: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        _limbs_square_to_out_basecase(&mut out, xs);
        assert_eq!(out, out_after);

        limbs_square_basecase_helper(out_before, xs);
    };
    test(&[10; 3], &[0], &[0, 0, 10]);
    test(&[10; 3], &[5], &[25, 0, 10]);
    test(&[10; 6], &[1, 2, 3], &[1, 4, 10, 12, 9, 0]);
    test(
        &[10; 6],
        &[u32::MAX, u32::MAX],
        &[1, 0, u32::MAX - 1, u32::MAX, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_to_out_basecase_fail_1() {
    let mut out = vec![10; 3];
    _limbs_square_to_out_basecase(&mut out, &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_to_out_basecase_fail_2() {
    let mut out = vec![10; 3];
    _limbs_square_to_out_basecase(&mut out, &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_to_out_basecase_fail_3() {
    let mut out = vec![0; (SQR_TOOM2_THRESHOLD + 1) << 1];
    _limbs_square_to_out_basecase(&mut out, &[10; SQR_TOOM2_THRESHOLD + 1]);
}

#[test]
fn test_square() {
    let test = |x, out| {
        assert_eq!(Natural::from_str(x).unwrap().square().to_string(), out);

        let mut x = Natural::from_str(x).unwrap();
        x.square_assign();
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "4");
    test("3", "9");
    test("10", "100");
    test("123", "15129");
    test("1000", "1000000");
    test("123456789", "15241578750190521");
}
