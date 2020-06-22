use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{signeds_var_2, unsigneds_var_8};

fn unsigned_square_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties_no_special(unsigneds_var_8::<T>, |&x| {
        let mut square = x;
        square.square_assign();
        assert_eq!(square, x.square());
        assert_eq!(square, x.pow(2));
    });
}

fn signed_square_properties_helper<T: PrimitiveSigned + Rand + SampleRange>() {
    test_properties_no_special(signeds_var_2::<T>, |&x| {
        let mut square = x;
        square.square_assign();
        assert_eq!(square, x.square());
        assert_eq!(square, x.pow(2));
    });
}

#[test]
fn square_properties() {
    unsigned_square_properties_helper::<u8>();
    unsigned_square_properties_helper::<u16>();
    unsigned_square_properties_helper::<u32>();
    unsigned_square_properties_helper::<u64>();
    unsigned_square_properties_helper::<usize>();

    signed_square_properties_helper::<i8>();
    signed_square_properties_helper::<i16>();
    signed_square_properties_helper::<i32>();
    signed_square_properties_helper::<i64>();
    signed_square_properties_helper::<isize>();
}
