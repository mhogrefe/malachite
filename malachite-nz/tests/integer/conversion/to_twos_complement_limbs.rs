#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::WrappingNegAssign;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::Integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::sub::limbs_sub_limb_in_place;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::not::limbs_not_in_place;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
fn limbs_twos_complement_in_place_alt_1(xs: &mut [Limb]) -> bool {
    let i = xs.iter().cloned().take_while(|&x| x == 0).count();
    let len = xs.len();
    if i == len {
        return true;
    }
    xs[i].wrapping_neg_assign();
    let j = i + 1;
    if j != len {
        limbs_not_in_place(&mut xs[j..]);
    }
    false
}

#[cfg(feature = "32_bit_limbs")]
fn limbs_twos_complement_in_place_alt_2(xs: &mut [Limb]) -> bool {
    let carry = limbs_sub_limb_in_place(xs, 1);
    limbs_not_in_place(xs);
    carry
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_twos_complement() {
    let test = |xs: &[Limb], out: &[Limb]| {
        assert_eq!(limbs_twos_complement(xs), out);
    };
    test(&[1, 2, 3], &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
    test(&[0xffff_ffff, 0xffff_fffd, 0xffff_fffc], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_maybe_sign_extend_non_negative_in_place() {
    let test = |xs: &[Limb], out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_maybe_sign_extend_non_negative_in_place(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[]);
    test(&[1, 2, 3], &[1, 2, 3]);
    test(&[1, 2, 0xffff_ffff], &[1, 2, 0xffff_ffff, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_twos_complement_in_place() {
    let test = |xs: &[Limb], out: &[Limb], carry: bool| {
        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_twos_complement_in_place(&mut mut_xs), carry);
        assert_eq!(mut_xs, out);

        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_twos_complement_in_place_alt_1(&mut mut_xs), carry);
        assert_eq!(mut_xs, out);

        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_twos_complement_in_place_alt_2(&mut mut_xs), carry);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[], true);
    test(&[1, 2, 3], &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc], false);
    test(&[0xffff_ffff, 0xffff_fffd, 0xffff_fffc], &[1, 2, 3], false);
    test(&[0, 0, 0], &[0, 0, 0], true);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_twos_complement_and_maybe_sign_extend_negative_in_place() {
    let test = |xs: &[Limb], out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[1, 2, 3], &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
    test(
        &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc],
        &[1, 2, 3, 0xffff_ffff],
    );
    test(&[0, 0xffff_ffff], &[0, 1, 0xffff_ffff]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_twos_complement_and_maybe_sign_extend_negative_in_place_fail() {
    let mut mut_xs = vec![0, 0, 0];
    limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_xs);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_twos_complement_limbs_asc() {
    let test = |n, out| {
        let n = Integer::from_str(n).unwrap();
        assert_eq!(n.twos_complement_limbs().collect::<Vec<Limb>>(), out);
        assert_eq!(n.to_twos_complement_limbs_asc(), out);
        assert_eq!(n.into_twos_complement_limbs_asc(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4_294_967_173]);
    test("1000000000000", vec![3_567_587_328, 232]);
    test("-1000000000000", vec![727_379_968, 4_294_967_063]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![
            Limb::MAX,
            Limb::MAX - 2,
            Limb::MAX - 3,
            Limb::MAX - 4,
            Limb::MAX - 5,
        ],
    );
    test("4294967295", vec![Limb::MAX, 0]);
    test("-4294967295", vec![1, Limb::MAX]);
    test("4294967296", vec![0, 1]);
    test("-4294967296", vec![0, Limb::MAX]);
    test("18446744073709551615", vec![Limb::MAX, Limb::MAX, 0]);
    test("-18446744073709551615", vec![1, 0, Limb::MAX]);
    test("18446744073709551616", vec![0, 0, 1]);
    test("-18446744073709551616", vec![0, 0, Limb::MAX]);

    let n = Integer::from_str("-1701411834921604967429270619762735448065").unwrap();
    let mut limbs = n.twos_complement_limbs();
    assert_eq!(Some(Limb::MAX), limbs.next());
    assert_eq!(Some(Limb::MAX - 5), limbs.next_back());
    assert_eq!(Some(Limb::MAX - 4), limbs.next_back());
    assert_eq!(Some(Limb::MAX - 2), limbs.next());
    assert_eq!(Some(Limb::MAX - 3), limbs.next());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());

    assert_eq!(limbs.get(0), Limb::MAX);
    assert_eq!(limbs.get(1), Limb::MAX - 2);
    assert_eq!(limbs.get(2), Limb::MAX - 3);
    assert_eq!(limbs.get(3), Limb::MAX - 4);
    assert_eq!(limbs.get(4), Limb::MAX - 5);
    assert_eq!(limbs.get(5), Limb::MAX);

    let mut limbs = n.twos_complement_limbs();
    assert_eq!(Some(Limb::MAX), limbs.next());
    assert_eq!(Some(Limb::MAX - 2), limbs.next());
    assert_eq!(Some(Limb::MAX - 3), limbs.next());
    assert_eq!(Some(Limb::MAX - 5), limbs.next_back());
    assert_eq!(Some(Limb::MAX - 4), limbs.next_back());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_twos_complement_limbs_desc() {
    let test = |n, out| {
        assert_eq!(
            Integer::from_str(n)
                .unwrap()
                .to_twos_complement_limbs_desc(),
            out
        );
        assert_eq!(
            Integer::from_str(n)
                .unwrap()
                .into_twos_complement_limbs_desc(),
            out
        );
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4_294_967_173]);
    test("1000000000000", vec![232, 3_567_587_328]);
    test("-1000000000000", vec![4_294_967_063, 727_379_968]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![
            Limb::MAX - 5,
            Limb::MAX - 4,
            Limb::MAX - 3,
            Limb::MAX - 2,
            Limb::MAX,
        ],
    );
    test("4294967295", vec![0, Limb::MAX]);
    test("-4294967295", vec![Limb::MAX, 1]);
    test("4294967296", vec![1, 0]);
    test("-4294967296", vec![Limb::MAX, 0]);
    test("18446744073709551615", vec![0, Limb::MAX, Limb::MAX]);
    test("-18446744073709551615", vec![Limb::MAX, 0, 1]);
    test("18446744073709551616", vec![1, 0, 0]);
    test("-18446744073709551616", vec![Limb::MAX, 0, 0]);
}
