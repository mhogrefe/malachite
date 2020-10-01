use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_5;

fn mod_neg_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(pairs_of_unsigneds_var_5::<T>, |&(n, m)| {
        assert!(n.mod_is_reduced(&m));
        let neg = n.mod_neg(m);
        assert!(neg.mod_is_reduced(&m));

        let mut n_alt = n;
        n_alt.mod_neg_assign(m);
        assert_eq!(n_alt, neg);

        assert_eq!(neg.mod_neg(m), n);
        assert_eq!(n.mod_add(neg, m), T::ZERO);
        assert_eq!(n == neg, n == T::ZERO || m.even() && n == m >> 1);
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
