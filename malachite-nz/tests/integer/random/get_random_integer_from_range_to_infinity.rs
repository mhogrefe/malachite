// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::random::{random_primitive_ints, VariableRangeGenerator};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::random::get_random_integer_from_range_to_infinity;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn get_random_integer_from_range_to_infinity_helper(
    a: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    out: &str,
) {
    let mut xs = random_primitive_ints(EXAMPLE_SEED.fork("ints"));
    let mut vrg = VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr"));
    let xs = (0..10)
        .map(|_| {
            get_random_integer_from_range_to_infinity(
                &mut xs,
                &mut vrg,
                Integer::from_str(a).unwrap(),
                mean_bits_numerator,
                mean_bits_denominator,
            )
        })
        .collect_vec();
    assert_eq!(xs.to_debug_string(), out);
}

#[test]
fn test_get_random_integer_from_range_to_infinity() {
    get_random_integer_from_range_to_infinity_helper("0", 1, 1, "[0, 1, 4, 1, 2, 1, 1, 0, 0, 0]");
    get_random_integer_from_range_to_infinity_helper(
        "0",
        10,
        1,
        "[7, 7816, 428, 130, 1, 141, 10, 0, 4, 4483]",
    );
    get_random_integer_from_range_to_infinity_helper(
        "0",
        100,
        1,
        "[5101205056696451696397798478058511, 1562796, 8799850658374624318722, \
        432133157539661383965541544934515144954635954990115469923269847259651409024994917000655083\
        9187394388518593842616549212512013, \
        279353891976332938189472063076409154515, 1660357170525, \
        143642188899218739960634489126387586224289351782452807884934768151051511265288490384892849\
        22660727526851378407, 86075361492, 353552745516847393429177033516378899307448925328642, \
        577340679116474858586805525866181088123189468507069123812855481357566943854]",
    );
    get_random_integer_from_range_to_infinity_helper(
        "1000",
        11,
        1,
        "[1015, 1672, 6316, 1282, 3037, 1805, 1122, 1003, 1014, 1019]",
    );
    get_random_integer_from_range_to_infinity_helper(
        "1000",
        100,
        1,
        "[1206982412795330974999926231143439, 457693356, 169360311075942561584386, \
        156864198081133600182484993110222733524619588763603298779764468364382591713280608608755884\
        131404116709444244598775565, 66677412650746398524862933431554022355, 26949124609373, \
        102746416386110194533593072869947149634954555999655687951279670456046799992002349096588693\
        3126118631, 22626063730900, 2792352557430693060292673664470974590025115014402, \
        68488724460300171666815845652220555564874106206890863873832895385292448366]",
    );
    get_random_integer_from_range_to_infinity_helper(
        "-1000",
        1,
        1,
        "[-3, -2, -1, 1, 29, -3, 0, 0, -1, 0]",
    );
    get_random_integer_from_range_to_infinity_helper(
        "-1000",
        11,
        1,
        "[-3, 136, -1, 2, -3, -731, 981996642, 764, 0, 1411]",
    );
    get_random_integer_from_range_to_infinity_helper(
        "-1000",
        100,
        1,
        "[70671, 33609936868504473224, 3330, 6514514285313835997, \
        141387787476503121093422704441276431644102874620098798311586658867138567258580573643023899\
        16124485683507351766006393560845, 250798235515229707219, 136491265145933085529437, \
        303813780, 816375814318068602464139315741117, -237]",
    );
}

#[test]
#[should_panic]
fn get_random_integer_from_range_to_infinity_fail_1() {
    get_random_integer_from_range_to_infinity(
        &mut random_primitive_ints(EXAMPLE_SEED.fork("ints")),
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        Integer::ZERO,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn get_random_integer_from_range_to_infinity_fail_2() {
    get_random_integer_from_range_to_infinity(
        &mut random_primitive_ints(EXAMPLE_SEED.fork("ints")),
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        Integer::ZERO,
        2,
        0,
    );
}

#[test]
#[should_panic]
fn get_random_integer_from_range_to_infinity_fail_3() {
    get_random_integer_from_range_to_infinity(
        &mut random_primitive_ints(EXAMPLE_SEED.fork("ints")),
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        Integer::from(10u32).pow(100),
        10,
        1,
    );
}
