use malachite_base::num::logic::traits::CountZeros;
use malachite_nz::integer::logic::checked_count_zeros::limbs_count_zeros_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::integer::logic::checked_count_zeros::{
    integer_checked_count_zeros_alt_1, integer_checked_count_zeros_alt_2,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    negative_signeds, nonempty_vecs_of_unsigned, vecs_of_unsigned_var_3,
};
use malachite_test::inputs::integer::integers;

#[test]
fn limbs_count_zeros_neg_properties() {
    test_properties(nonempty_vecs_of_unsigned, |limbs| {
        limbs_count_zeros_neg(limbs);
    });

    test_properties(vecs_of_unsigned_var_3, |limbs| {
        assert_eq!(
            Some(limbs_count_zeros_neg(limbs)),
            (-Natural::from_limbs_asc(limbs)).checked_count_zeros()
        );
    });
}

#[test]
fn checked_count_zeros_properties() {
    test_properties(integers, |x| {
        let zeros = x.checked_count_zeros();
        assert_eq!(integer_checked_count_zeros_alt_1(x), zeros);
        assert_eq!(integer_checked_count_zeros_alt_2(x), zeros);
        assert_eq!((!x).checked_count_ones(), zeros);
    });

    test_properties(negative_signeds::<SignedLimb>, |&i| {
        assert_eq!(
            Integer::from(i).checked_count_zeros(),
            Some(CountZeros::count_zeros(i))
        );
    });
}
