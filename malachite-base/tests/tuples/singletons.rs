use itertools::Itertools;
use malachite_base::nevers::Never;
use malachite_base::tuples::singletons;
use std::fmt::Debug;

fn singletons_helper<T: Clone + Debug + Eq>(xs: &[T], out: &[(T,)]) {
    assert_eq!(singletons(xs.iter().cloned()).collect_vec().as_slice(), out);
}

#[test]
fn test_singletons() {
    singletons_helper::<Never>(&[], &[]);
    singletons_helper(&[5], &[(5,)]);
    singletons_helper(&[1, 2, 3], &[(1,), (2,), (3,)]);
    singletons_helper(&[(2,), (1,), (5,)], &[((2,),), ((1,),), ((5,),)]);
}
