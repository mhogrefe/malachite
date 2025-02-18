// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::char_is_graphic;
use malachite_base::chars::exhaustive::{ascii_chars_increasing, chars_increasing};
use malachite_base::iterators::matching_intervals_in_iterator;

#[test]
fn test_is_graphic() {
    let test = |c, out| {
        assert_eq!(char_is_graphic(c), out);
    };
    test(' ', true);
    test('a', true);
    test('A', true);
    test('0', true);
    test('!', true);
    test('ñ', true);
    test('\0', false);
    test('\n', false);
    test('\u{5f771}', false);

    assert_eq!(
        ascii_chars_increasing()
            .filter(|&c| char_is_graphic(c))
            .count(),
        95
    );
    assert_eq!(
        chars_increasing().filter(|&c| char_is_graphic(c)).count(),
        152714
    );
    assert_eq!(
        matching_intervals_in_iterator(chars_increasing(), |&c| { char_is_graphic(c) })
            .iter()
            .map(|i| (i.0 as u32, i.1 as u32))
            .collect_vec(),
        &[
            (32, 126),
            (161, 172),
            (174, 767),
            (880, 887),
            (890, 895),
            (900, 906),
            (908, 908),
            (910, 929),
            (931, 1154),
            (1162, 1327),
            (1329, 1366),
            (1369, 1418),
            (1421, 1423),
            (1470, 1470),
            (1472, 1472),
            (1475, 1475),
            (1478, 1478),
            (1488, 1514),
            (1519, 1524),
            (1542, 1551),
            (1563, 1563),
            (1565, 1610),
            (1632, 1647),
            (1649, 1749),
            (1758, 1758),
            (1765, 1766),
            (1769, 1769),
            (1774, 1805),
            (1808, 1808),
            (1810, 1839),
            (1869, 1957),
            (1969, 1969),
            (1984, 2026),
            (2036, 2042),
            (2046, 2069),
            (2074, 2074),
            (2084, 2084),
            (2088, 2088),
            (2096, 2110),
            (2112, 2136),
            (2142, 2142),
            (2144, 2154),
            (2160, 2190),
            (2208, 2249),
            (2307, 2361),
            (2363, 2363),
            (2365, 2368),
            (2377, 2380),
            (2382, 2384),
            (2392, 2401),
            (2404, 2432),
            (2434, 2435),
            (2437, 2444),
            (2447, 2448),
            (2451, 2472),
            (2474, 2480),
            (2482, 2482),
            (2486, 2489),
            (2493, 2493),
            (2495, 2496),
            (2503, 2504),
            (2507, 2508),
            (2510, 2510),
            (2524, 2525),
            (2527, 2529),
            (2534, 2557),
            (2563, 2563),
            (2565, 2570),
            (2575, 2576),
            (2579, 2600),
            (2602, 2608),
            (2610, 2611),
            (2613, 2614),
            (2616, 2617),
            (2622, 2624),
            (2649, 2652),
            (2654, 2654),
            (2662, 2671),
            (2674, 2676),
            (2678, 2678),
            (2691, 2691),
            (2693, 2701),
            (2703, 2705),
            (2707, 2728),
            (2730, 2736),
            (2738, 2739),
            (2741, 2745),
            (2749, 2752),
            (2761, 2761),
            (2763, 2764),
            (2768, 2768),
            (2784, 2785),
            (2790, 2801),
            (2809, 2809),
            (2818, 2819),
            (2821, 2828),
            (2831, 2832),
            (2835, 2856),
            (2858, 2864),
            (2866, 2867),
            (2869, 2873),
            (2877, 2877),
            (2880, 2880),
            (2887, 2888),
            (2891, 2892),
            (2908, 2909),
            (2911, 2913),
            (2918, 2935),
            (2947, 2947),
            (2949, 2954),
            (2958, 2960),
            (2962, 2965),
            (2969, 2970),
            (2972, 2972),
            (2974, 2975),
            (2979, 2980),
            (2984, 2986),
            (2990, 3001),
            (3007, 3007),
            (3009, 3010),
            (3014, 3016),
            (3018, 3020),
            (3024, 3024),
            (3046, 3066),
            (3073, 3075),
            (3077, 3084),
            (3086, 3088),
            (3090, 3112),
            (3114, 3129),
            (3133, 3133),
            (3137, 3140),
            (3160, 3162),
            (3165, 3165),
            (3168, 3169),
            (3174, 3183),
            (3191, 3200),
            (3202, 3212),
            (3214, 3216),
            (3218, 3240),
            (3242, 3251),
            (3253, 3257),
            (3261, 3262),
            (3265, 3265),
            (3267, 3268),
            (3293, 3294),
            (3296, 3297),
            (3302, 3311),
            (3313, 3315),
            (3330, 3340),
            (3342, 3344),
            (3346, 3386),
            (3389, 3389),
            (3391, 3392),
            (3398, 3400),
            (3402, 3404),
            (3406, 3407),
            (3412, 3414),
            (3416, 3425),
            (3430, 3455),
            (3458, 3459),
            (3461, 3478),
            (3482, 3505),
            (3507, 3515),
            (3517, 3517),
            (3520, 3526),
            (3536, 3537),
            (3544, 3550),
            (3558, 3567),
            (3570, 3572),
            (3585, 3632),
            (3634, 3635),
            (3647, 3654),
            (3663, 3675),
            (3713, 3714),
            (3716, 3716),
            (3718, 3722),
            (3724, 3747),
            (3749, 3749),
            (3751, 3760),
            (3762, 3763),
            (3773, 3773),
            (3776, 3780),
            (3782, 3782),
            (3792, 3801),
            (3804, 3807),
            (3840, 3863),
            (3866, 3892),
            (3894, 3894),
            (3896, 3896),
            (3898, 3911),
            (3913, 3948),
            (3967, 3967),
            (3973, 3973),
            (3976, 3980),
            (4030, 4037),
            (4039, 4044),
            (4046, 4058),
            (4096, 4140),
            (4145, 4145),
            (4152, 4152),
            (4155, 4156),
            (4159, 4183),
            (4186, 4189),
            (4193, 4208),
            (4213, 4225),
            (4227, 4228),
            (4231, 4236),
            (4238, 4252),
            (4254, 4293),
            (4295, 4295),
            (4301, 4301),
            (4304, 4680),
            (4682, 4685),
            (4688, 4694),
            (4696, 4696),
            (4698, 4701),
            (4704, 4744),
            (4746, 4749),
            (4752, 4784),
            (4786, 4789),
            (4792, 4798),
            (4800, 4800),
            (4802, 4805),
            (4808, 4822),
            (4824, 4880),
            (4882, 4885),
            (4888, 4954),
            (4960, 4988),
            (4992, 5017),
            (5024, 5109),
            (5112, 5117),
            (5120, 5759),
            (5761, 5788),
            (5792, 5880),
            (5888, 5905),
            (5919, 5937),
            (5941, 5942),
            (5952, 5969),
            (5984, 5996),
            (5998, 6000),
            (6016, 6067),
            (6070, 6070),
            (6078, 6085),
            (6087, 6088),
            (6100, 6108),
            (6112, 6121),
            (6128, 6137),
            (6144, 6154),
            (6160, 6169),
            (6176, 6264),
            (6272, 6276),
            (6279, 6312),
            (6314, 6314),
            (6320, 6389),
            (6400, 6430),
            (6435, 6438),
            (6441, 6443),
            (6448, 6449),
            (6451, 6456),
            (6464, 6464),
            (6468, 6509),
            (6512, 6516),
            (6528, 6571),
            (6576, 6601),
            (6608, 6618),
            (6622, 6678),
            (6681, 6682),
            (6686, 6741),
            (6743, 6743),
            (6753, 6753),
            (6755, 6756),
            (6765, 6770),
            (6784, 6793),
            (6800, 6809),
            (6816, 6829),
            (6916, 6963),
            (6974, 6977),
            (6981, 6988),
            (6990, 7018),
            (7028, 7039),
            (7042, 7073),
            (7078, 7079),
            (7086, 7141),
            (7143, 7143),
            (7146, 7148),
            (7150, 7150),
            (7164, 7211),
            (7220, 7221),
            (7227, 7241),
            (7245, 7306),
            (7312, 7354),
            (7357, 7367),
            (7379, 7379),
            (7393, 7393),
            (7401, 7404),
            (7406, 7411),
            (7413, 7415),
            (7418, 7418),
            (7424, 7615),
            (7680, 7957),
            (7960, 7965),
            (7968, 8005),
            (8008, 8013),
            (8016, 8023),
            (8025, 8025),
            (8027, 8027),
            (8029, 8029),
            (8031, 8061),
            (8064, 8116),
            (8118, 8132),
            (8134, 8147),
            (8150, 8155),
            (8157, 8175),
            (8178, 8180),
            (8182, 8190),
            (8208, 8231),
            (8240, 8286),
            (8304, 8305),
            (8308, 8334),
            (8336, 8348),
            (8352, 8384),
            (8448, 8587),
            (8592, 9257),
            (9280, 9290),
            (9312, 11123),
            (11126, 11157),
            (11159, 11502),
            (11506, 11507),
            (11513, 11557),
            (11559, 11559),
            (11565, 11565),
            (11568, 11623),
            (11631, 11632),
            (11648, 11670),
            (11680, 11686),
            (11688, 11694),
            (11696, 11702),
            (11704, 11710),
            (11712, 11718),
            (11720, 11726),
            (11728, 11734),
            (11736, 11742),
            (11776, 11869),
            (11904, 11929),
            (11931, 12019),
            (12032, 12245),
            (12272, 12287),
            (12289, 12329),
            (12336, 12351),
            (12353, 12438),
            (12443, 12543),
            (12549, 12591),
            (12593, 12686),
            (12688, 12773),
            (12783, 12830),
            (12832, 42124),
            (42128, 42182),
            (42192, 42539),
            (42560, 42606),
            (42611, 42611),
            (42622, 42653),
            (42656, 42735),
            (42738, 42743),
            (42752, 42957),
            (42960, 42961),
            (42963, 42963),
            (42965, 42972),
            (42994, 43009),
            (43011, 43013),
            (43015, 43018),
            (43020, 43044),
            (43047, 43051),
            (43056, 43065),
            (43072, 43127),
            (43136, 43203),
            (43214, 43225),
            (43250, 43262),
            (43264, 43301),
            (43310, 43334),
            (43346, 43346),
            (43359, 43388),
            (43395, 43442),
            (43444, 43445),
            (43450, 43451),
            (43454, 43455),
            (43457, 43469),
            (43471, 43481),
            (43486, 43492),
            (43494, 43518),
            (43520, 43560),
            (43567, 43568),
            (43571, 43572),
            (43584, 43586),
            (43588, 43595),
            (43597, 43597),
            (43600, 43609),
            (43612, 43643),
            (43645, 43695),
            (43697, 43697),
            (43701, 43702),
            (43705, 43709),
            (43712, 43712),
            (43714, 43714),
            (43739, 43755),
            (43758, 43765),
            (43777, 43782),
            (43785, 43790),
            (43793, 43798),
            (43808, 43814),
            (43816, 43822),
            (43824, 43883),
            (43888, 44004),
            (44006, 44007),
            (44009, 44012),
            (44016, 44025),
            (44032, 55203),
            (55216, 55238),
            (55243, 55291),
            (63744, 64109),
            (64112, 64217),
            (64256, 64262),
            (64275, 64279),
            (64285, 64285),
            (64287, 64310),
            (64312, 64316),
            (64318, 64318),
            (64320, 64321),
            (64323, 64324),
            (64326, 64450),
            (64467, 64911),
            (64914, 64967),
            (64975, 64975),
            (65008, 65023),
            (65040, 65049),
            (65072, 65106),
            (65108, 65126),
            (65128, 65131),
            (65136, 65140),
            (65142, 65276),
            (65281, 65437),
            (65440, 65470),
            (65474, 65479),
            (65482, 65487),
            (65490, 65495),
            (65498, 65500),
            (65504, 65510),
            (65512, 65518),
            (65532, 65533),
            (65536, 65547),
            (65549, 65574),
            (65576, 65594),
            (65596, 65597),
            (65599, 65613),
            (65616, 65629),
            (65664, 65786),
            (65792, 65794),
            (65799, 65843),
            (65847, 65934),
            (65936, 65948),
            (65952, 65952),
            (66000, 66044),
            (66176, 66204),
            (66208, 66256),
            (66273, 66299),
            (66304, 66339),
            (66349, 66378),
            (66384, 66421),
            (66432, 66461),
            (66463, 66499),
            (66504, 66517),
            (66560, 66717),
            (66720, 66729),
            (66736, 66771),
            (66776, 66811),
            (66816, 66855),
            (66864, 66915),
            (66927, 66938),
            (66940, 66954),
            (66956, 66962),
            (66964, 66965),
            (66967, 66977),
            (66979, 66993),
            (66995, 67001),
            (67003, 67004),
            (67008, 67059),
            (67072, 67382),
            (67392, 67413),
            (67424, 67431),
            (67456, 67461),
            (67463, 67504),
            (67506, 67514),
            (67584, 67589),
            (67592, 67592),
            (67594, 67637),
            (67639, 67640),
            (67644, 67644),
            (67647, 67669),
            (67671, 67742),
            (67751, 67759),
            (67808, 67826),
            (67828, 67829),
            (67835, 67867),
            (67871, 67897),
            (67903, 67903),
            (67968, 68023),
            (68028, 68047),
            (68050, 68096),
            (68112, 68115),
            (68117, 68119),
            (68121, 68149),
            (68160, 68168),
            (68176, 68184),
            (68192, 68255),
            (68288, 68324),
            (68331, 68342),
            (68352, 68405),
            (68409, 68437),
            (68440, 68466),
            (68472, 68497),
            (68505, 68508),
            (68521, 68527),
            (68608, 68680),
            (68736, 68786),
            (68800, 68850),
            (68858, 68899),
            (68912, 68921),
            (68928, 68965),
            (68974, 68997),
            (69006, 69007),
            (69216, 69246),
            (69248, 69289),
            (69293, 69293),
            (69296, 69297),
            (69314, 69316),
            (69376, 69415),
            (69424, 69445),
            (69457, 69465),
            (69488, 69505),
            (69510, 69513),
            (69552, 69579),
            (69600, 69622),
            (69632, 69632),
            (69634, 69687),
            (69703, 69709),
            (69714, 69743),
            (69745, 69746),
            (69749, 69749),
            (69762, 69810),
            (69815, 69816),
            (69819, 69820),
            (69822, 69825),
            (69840, 69864),
            (69872, 69881),
            (69891, 69926),
            (69932, 69932),
            (69942, 69959),
            (69968, 70002),
            (70004, 70006),
            (70018, 70069),
            (70079, 70079),
            (70081, 70088),
            (70093, 70094),
            (70096, 70111),
            (70113, 70132),
            (70144, 70161),
            (70163, 70190),
            (70194, 70195),
            (70200, 70205),
            (70207, 70208),
            (70272, 70278),
            (70280, 70280),
            (70282, 70285),
            (70287, 70301),
            (70303, 70313),
            (70320, 70366),
            (70368, 70370),
            (70384, 70393),
            (70402, 70403),
            (70405, 70412),
            (70415, 70416),
            (70419, 70440),
            (70442, 70448),
            (70450, 70451),
            (70453, 70457),
            (70461, 70461),
            (70463, 70463),
            (70465, 70468),
            (70471, 70472),
            (70475, 70476),
            (70480, 70480),
            (70493, 70499),
            (70528, 70537),
            (70539, 70539),
            (70542, 70542),
            (70544, 70581),
            (70583, 70583),
            (70585, 70586),
            (70602, 70602),
            (70604, 70605),
            (70609, 70609),
            (70611, 70613),
            (70615, 70616),
            (70656, 70711),
            (70720, 70721),
            (70725, 70725),
            (70727, 70747),
            (70749, 70749),
            (70751, 70753),
            (70784, 70831),
            (70833, 70834),
            (70841, 70841),
            (70843, 70844),
            (70846, 70846),
            (70849, 70849),
            (70852, 70855),
            (70864, 70873),
            (71040, 71086),
            (71088, 71089),
            (71096, 71099),
            (71102, 71102),
            (71105, 71131),
            (71168, 71218),
            (71227, 71228),
            (71230, 71230),
            (71233, 71236),
            (71248, 71257),
            (71264, 71276),
            (71296, 71338),
            (71340, 71340),
            (71342, 71343),
            (71352, 71353),
            (71360, 71369),
            (71376, 71395),
            (71424, 71450),
            (71454, 71454),
            (71456, 71457),
            (71462, 71462),
            (71472, 71494),
            (71680, 71726),
            (71736, 71736),
            (71739, 71739),
            (71840, 71922),
            (71935, 71942),
            (71945, 71945),
            (71948, 71955),
            (71957, 71958),
            (71960, 71983),
            (71985, 71989),
            (71991, 71992),
            (71999, 72002),
            (72004, 72006),
            (72016, 72025),
            (72096, 72103),
            (72106, 72147),
            (72156, 72159),
            (72161, 72164),
            (72192, 72192),
            (72203, 72242),
            (72249, 72250),
            (72255, 72262),
            (72272, 72272),
            (72279, 72280),
            (72284, 72329),
            (72343, 72343),
            (72346, 72354),
            (72368, 72440),
            (72448, 72457),
            (72640, 72673),
            (72688, 72697),
            (72704, 72712),
            (72714, 72751),
            (72766, 72766),
            (72768, 72773),
            (72784, 72812),
            (72816, 72847),
            (72873, 72873),
            (72881, 72881),
            (72884, 72884),
            (72960, 72966),
            (72968, 72969),
            (72971, 73008),
            (73030, 73030),
            (73040, 73049),
            (73056, 73061),
            (73063, 73064),
            (73066, 73102),
            (73107, 73108),
            (73110, 73110),
            (73112, 73112),
            (73120, 73129),
            (73440, 73458),
            (73461, 73464),
            (73474, 73488),
            (73490, 73525),
            (73534, 73535),
            (73539, 73561),
            (73648, 73648),
            (73664, 73713),
            (73727, 74649),
            (74752, 74862),
            (74864, 74868),
            (74880, 75075),
            (77712, 77810),
            (77824, 78895),
            (78913, 78918),
            (78944, 82938),
            (82944, 83526),
            (90368, 90397),
            (90410, 90412),
            (90416, 90425),
            (92160, 92728),
            (92736, 92766),
            (92768, 92777),
            (92782, 92862),
            (92864, 92873),
            (92880, 92909),
            (92917, 92917),
            (92928, 92975),
            (92983, 92997),
            (93008, 93017),
            (93019, 93025),
            (93027, 93047),
            (93053, 93071),
            (93504, 93561),
            (93760, 93850),
            (93952, 94026),
            (94032, 94087),
            (94099, 94111),
            (94176, 94179),
            (94208, 100343),
            (100352, 101589),
            (101631, 101640),
            (110576, 110579),
            (110581, 110587),
            (110589, 110590),
            (110592, 110882),
            (110898, 110898),
            (110928, 110930),
            (110933, 110933),
            (110948, 110951),
            (110960, 111355),
            (113664, 113770),
            (113776, 113788),
            (113792, 113800),
            (113808, 113817),
            (113820, 113820),
            (113823, 113823),
            (117760, 118009),
            (118016, 118451),
            (118608, 118723),
            (118784, 119029),
            (119040, 119078),
            (119081, 119140),
            (119146, 119148),
            (119171, 119172),
            (119180, 119209),
            (119214, 119274),
            (119296, 119361),
            (119365, 119365),
            (119488, 119507),
            (119520, 119539),
            (119552, 119638),
            (119648, 119672),
            (119808, 119892),
            (119894, 119964),
            (119966, 119967),
            (119970, 119970),
            (119973, 119974),
            (119977, 119980),
            (119982, 119993),
            (119995, 119995),
            (119997, 120003),
            (120005, 120069),
            (120071, 120074),
            (120077, 120084),
            (120086, 120092),
            (120094, 120121),
            (120123, 120126),
            (120128, 120132),
            (120134, 120134),
            (120138, 120144),
            (120146, 120485),
            (120488, 120779),
            (120782, 121343),
            (121399, 121402),
            (121453, 121460),
            (121462, 121475),
            (121477, 121483),
            (122624, 122654),
            (122661, 122666),
            (122928, 122989),
            (123136, 123180),
            (123191, 123197),
            (123200, 123209),
            (123214, 123215),
            (123536, 123565),
            (123584, 123627),
            (123632, 123641),
            (123647, 123647),
            (124112, 124139),
            (124144, 124153),
            (124368, 124397),
            (124400, 124410),
            (124415, 124415),
            (124896, 124902),
            (124904, 124907),
            (124909, 124910),
            (124912, 124926),
            (124928, 125124),
            (125127, 125135),
            (125184, 125251),
            (125259, 125259),
            (125264, 125273),
            (125278, 125279),
            (126065, 126132),
            (126209, 126269),
            (126464, 126467),
            (126469, 126495),
            (126497, 126498),
            (126500, 126500),
            (126503, 126503),
            (126505, 126514),
            (126516, 126519),
            (126521, 126521),
            (126523, 126523),
            (126530, 126530),
            (126535, 126535),
            (126537, 126537),
            (126539, 126539),
            (126541, 126543),
            (126545, 126546),
            (126548, 126548),
            (126551, 126551),
            (126553, 126553),
            (126555, 126555),
            (126557, 126557),
            (126559, 126559),
            (126561, 126562),
            (126564, 126564),
            (126567, 126570),
            (126572, 126578),
            (126580, 126583),
            (126585, 126588),
            (126590, 126590),
            (126592, 126601),
            (126603, 126619),
            (126625, 126627),
            (126629, 126633),
            (126635, 126651),
            (126704, 126705),
            (126976, 127019),
            (127024, 127123),
            (127136, 127150),
            (127153, 127167),
            (127169, 127183),
            (127185, 127221),
            (127232, 127405),
            (127462, 127490),
            (127504, 127547),
            (127552, 127560),
            (127568, 127569),
            (127584, 127589),
            (127744, 128727),
            (128732, 128748),
            (128752, 128764),
            (128768, 128886),
            (128891, 128985),
            (128992, 129003),
            (129008, 129008),
            (129024, 129035),
            (129040, 129095),
            (129104, 129113),
            (129120, 129159),
            (129168, 129197),
            (129200, 129211),
            (129216, 129217),
            (129280, 129619),
            (129632, 129645),
            (129648, 129660),
            (129664, 129673),
            (129679, 129734),
            (129742, 129756),
            (129759, 129769),
            (129776, 129784),
            (129792, 129938),
            (129940, 130041),
            (131072, 173791),
            (173824, 177977),
            (177984, 178205),
            (178208, 183969),
            (183984, 191456),
            (191472, 192093),
            (194560, 195101),
            (196608, 201546),
            (201552, 205743)
        ][..]
    );
}
