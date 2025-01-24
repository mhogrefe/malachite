// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::random::get_random_natural_less_than;
use malachite_nz::natural::Natural;
use std::str::FromStr;

fn get_random_natural_less_than_helper(limit: &str, out: &str) {
    let mut xs = random_primitive_ints(EXAMPLE_SEED);
    let xs = (0..10)
        .map(|_| get_random_natural_less_than(&mut xs, &Natural::from_str(limit).unwrap()))
        .collect_vec();
    assert_eq!(xs.to_debug_string(), out);
}

#[test]
fn test_get_random_natural_less_than() {
    get_random_natural_less_than_helper("1", "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]");
    get_random_natural_less_than_helper("10", "[1, 7, 5, 7, 9, 2, 8, 2, 4, 6]");
    get_random_natural_less_than_helper("100", "[87, 93, 7, 84, 27, 46, 93, 22, 86, 1]");
    get_random_natural_less_than_helper("1000", "[881, 87, 93, 629, 519, 626, 360, 242, 491, 84]");
    get_random_natural_less_than_helper(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000",
        "[6513696165206921204311261523739269601083760431420210295861564381205783648442563498105196\
        329975541617, \
        868400879467069938866982772480181142332366549281254507368280851531146621198387695461761814\
        2446877811, \
        454890116058063209121711848211479855802275293374032067837601343116564901989251772312271223\
        8643735453, \
        221953236727409585623978519825787712675415538807055958167887436471257145918881448806179877\
        2042912434, \
        156597062580980418852701766729202981599785596117178894442015894874216386954905163169649546\
        2984128417, \
        775809396529562140517641222093832263868653173407259880967133743504333426167140950839221651\
        5440206577, \
        194588045371630389938122892550032325836127322381120175893002672149553900804839171978848159\
        3900827375, \
        681710429818657256239982130308509711765427303233424706691965226432442255381734547395128615\
        696717955, \
        525885978270298534580002381561976404877189873292849476887431075456216366244793235854298324\
        5431956916, \
        314957843714266314102082575313210347468784454645937249947829551130804781109663224113983528\
        7147185026]",
    );
}

#[test]
#[should_panic]
fn get_random_natural_less_than_fail() {
    get_random_natural_less_than(&mut random_primitive_ints(EXAMPLE_SEED), &Natural::ZERO);
}
