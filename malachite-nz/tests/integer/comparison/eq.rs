use malachite_base_test_util::common::test_eq_helper;
use malachite_base_test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_nz_test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use num::BigInt;
use rug;

#[test]
fn test_eq() {
    let strings = &["0", "1", "-1", "2", "-2", "123", "-123", "1000000000000", "-1000000000000"];
    test_eq_helper::<Integer>(strings);
    test_eq_helper::<BigInt>(strings);
    test_eq_helper::<rug::Integer>(strings);
}

#[allow(clippy::cmp_owned, clippy::eq_op)]
#[test]
fn eq_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(integer_to_bigint(&x) == integer_to_bigint(&y), eq);
        assert_eq!(integer_to_rug_integer(&x) == integer_to_rug_integer(&y), eq);
        assert_eq!(y == x, eq);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(x, x);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        if x == y && y == z {
            assert_eq!(x, z);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x) == Integer::from(&y), x == y);
        assert_eq!(Integer::from(&x) == y, x == y);
        assert_eq!(x == Integer::from(&y), x == y);
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x) == Integer::from(y), x == y);
    });
}
