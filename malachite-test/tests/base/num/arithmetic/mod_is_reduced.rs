use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_4;

fn mod_is_reduced_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigneds_var_4::<T>, |&(n, m)| {
        assert_eq!(n.mod_is_reduced(&m), n % m == n);
    });
}

#[test]
fn mod_is_reduced_properties() {
    mod_is_reduced_properties_helper::<u8>();
    mod_is_reduced_properties_helper::<u16>();
    mod_is_reduced_properties_helper::<u32>();
    mod_is_reduced_properties_helper::<u64>();
    mod_is_reduced_properties_helper::<usize>();
}
