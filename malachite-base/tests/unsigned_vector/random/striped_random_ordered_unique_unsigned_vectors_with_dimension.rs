// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_vector::random::*;

#[track_caller]
fn striped_random_ordered_unique_unsigned_vectors_with_dimension_helper<T: PrimitiveUnsigned>(
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_ordered_unique_unsigned_vectors_with_dimension::<T>(
            EXAMPLE_SEED,
            dimension,
            mean_stripe_numerator,
            mean_stripe_denominator
        )
        .take(20)
        .inspect(|v| assert!(v.elements.is_sorted_by(|x, y| x < y)))
        .map(|v| v.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_ordered_unique_unsigned_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    striped_random_ordered_unique_unsigned_vectors_with_dimension_helper::<u8>(
        0,
        4,
        1,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, u8 elements, mean stripe length = 4
    striped_random_ordered_unique_unsigned_vectors_with_dimension_helper::<u8>(
        1,
        4,
        1,
        &[
            "(1)", "(76)", "(127)", "(195)", "(0)", "(128)", "(15)", "(118)", "(0)", "(248)",
            "(255)", "(253)", "(121)", "(0)", "(240)", "(3)", "(0)", "(127)", "(1)", "(0)",
        ],
    );
    // dimension 3, u8 elements, mean stripe length = 16
    striped_random_ordered_unique_unsigned_vectors_with_dimension_helper::<u8>(
        3,
        16,
        1,
        &[
            "(0, 15, 252)",
            "(0, 254, 255)",
            "(127, 192, 255)",
            "(0, 15, 224)",
            "(0, 127, 255)",
            "(0, 63, 255)",
            "(15, 192, 255)",
            "(0, 15, 255)",
            "(1, 254, 255)",
            "(3, 224, 255)",
            "(0, 1, 255)",
            "(0, 248, 255)",
            "(0, 192, 255)",
            "(0, 224, 248)",
            "(0, 252, 255)",
            "(0, 192, 255)",
            "(7, 127, 248)",
            "(0, 63, 255)",
            "(0, 240, 255)",
            "(0, 7, 255)",
        ],
    );
    // dimension 3, u64 elements, mean stripe length = 4
    striped_random_ordered_unique_unsigned_vectors_with_dimension_helper::<u64>(
        3,
        4,
        1,
        &[
            "(100770103514905344, 143917548967825279, 576447764355809407)",
            "(518975997870076, 17329914038243544163, 18440266239814141817)",
            "(1738283919425159392, 4475456554103348762, 4579918139278066687)",
            "(14411520744717549504, 16145088524465474047, 18438281957344051495)",
            "(558184670035966008, 3999211896140980480, 17555765880769536000)",
            "(1148964929193703408, 3449761712612049351, 8143070487853989372)",
            "(2323296167166803456, 4540967569388715647, 13835027167756887004)",
            "(284598139280699776, 1418673196588593151, 18374689220160116832)",
            "(145100358520401983, 8925853127677050639, 17344591479198745568)",
            "(4620262210298576910, 9494594860043174343, 18443091499890769919)",
            "(157903067109394368, 7459065977070714368, 10663573613658505216)",
            "(71889248365559832, 13866832549808963328, 18194498552293491139)",
            "(6917529220650238735, 9231954825577726207, 9515842446442101760)",
            "(315771010281899504, 15636392232318091263, 16084892320150848400)",
            "(540492980224014336, 11545195345973207042, 16289089251260038975)",
            "(226980840619904, 16282333380578766854, 16436176982300457021)",
            "(16429096256508264472, 17580927045864489396, 18385387133092491758)",
            "(216419107061368801, 13902541745507516478, 14852871562482163696)",
            "(143409181326573567, 10950792339802955772, 18414084188377526267)",
            "(4891476543417614307, 9834769839778972302, 18444549299249381376)",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_unsigned_vectors_with_dimension_fail_1() {
    // mean_stripe_denominator is zero
    let _ =
        striped_random_ordered_unique_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, 3, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_unsigned_vectors_with_dimension_fail_2() {
    // mean_stripe_numerator < mean_stripe_denominator
    let _ =
        striped_random_ordered_unique_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, 3, 1, 2);
}

#[test]
#[should_panic]
fn striped_random_ordered_unique_unsigned_vectors_with_dimension_fail_3() {
    // dimension is greater than the number of u8s
    let _ = striped_random_ordered_unique_unsigned_vectors_with_dimension::<u8>(
        EXAMPLE_SEED,
        257,
        4,
        1,
    );
}
