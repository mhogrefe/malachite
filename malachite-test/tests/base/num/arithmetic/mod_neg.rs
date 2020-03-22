use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_5;

fn mod_neg_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(pairs_of_unsigneds_var_5::<T>, |&(n, modulus)| {
        assert!(n.mod_is_reduced(&modulus));
        let neg = n.mod_neg(modulus);
        assert!(neg.mod_is_reduced(&modulus));

        let mut n_alt = n;
        n_alt.mod_neg_assign(modulus);
        assert_eq!(n_alt, neg);

        assert_eq!(neg.mod_neg(modulus), n);
        //TODO use mod_add
        assert_eq!(
            n == neg,
            n == T::ZERO || modulus.even() && n == modulus >> 1
        );
    });
}

#[test]
fn mod_neg_properties() {
    mod_neg_properties_helper::<u8>();
    mod_neg_properties_helper::<u16>();
    mod_neg_properties_helper::<u32>();
    mod_neg_properties_helper::<u64>();
    mod_neg_properties_helper::<usize>();
}
