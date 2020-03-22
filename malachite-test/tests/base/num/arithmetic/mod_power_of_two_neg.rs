use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::pairs_of_unsigned_and_small_u64_var_2;

fn mod_power_of_two_neg_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties_no_special(pairs_of_unsigned_and_small_u64_var_2::<T>, |&(n, pow)| {
        assert!(n.mod_power_of_two_is_reduced(pow));
        let neg = n.mod_power_of_two_neg(pow);
        assert!(neg.mod_power_of_two_is_reduced(pow));

        let mut n_alt = n;
        n_alt.mod_power_of_two_neg_assign(pow);
        assert_eq!(n_alt, neg);

        assert_eq!(neg, n.wrapping_neg().mod_power_of_two(pow));
        assert_eq!(neg.mod_power_of_two_neg(pow), n);
        //TODO use mod_add
        assert!(n.wrapping_add(neg).divisible_by_power_of_two(pow));
        assert_eq!(n == neg, n == T::ZERO || n == T::ONE << (pow - 1));
    });
}

#[test]
fn mod_power_of_two_neg_properties() {
    mod_power_of_two_neg_properties_helper::<u8>();
    mod_power_of_two_neg_properties_helper::<u16>();
    mod_power_of_two_neg_properties_helper::<u32>();
    mod_power_of_two_neg_properties_helper::<u64>();
    mod_power_of_two_neg_properties_helper::<usize>();
}
