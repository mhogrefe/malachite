use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds_var_2;

fn next_power_of_two_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds_var_2::<T>, |&n| {
        let p = n.next_power_of_two();
        assert!(p >= n);
        assert!(p >> 1 <= n);
        assert!(p.is_power_of_two());

        let mut n = n;
        n.next_power_of_two_assign();
        assert_eq!(n, p);
    });
}

#[test]
fn next_power_of_two_properties() {
    next_power_of_two_properties_helper::<u8>();
    next_power_of_two_properties_helper::<u16>();
    next_power_of_two_properties_helper::<u32>();
    next_power_of_two_properties_helper::<u64>();
    next_power_of_two_properties_helper::<usize>();
}
