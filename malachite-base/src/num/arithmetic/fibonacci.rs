// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    CheckedFibonacci, CheckedLucasNumber, Fibonacci, LucasNumber,
};

const FIBONACCIS_U8: [u8; 14] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];

const LUCAS_NUMBERS_U8: [u8; 12] = [2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199];

const FIBONACCIS_U16: [u16; 25] = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
    10946, 17711, 28657, 46368,
];

const LUCAS_NUMBERS_U16: [u16; 24] = [
    2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521, 843, 1364, 2207, 3571, 5778, 9349,
    15127, 24476, 39603, 64079,
];

const FIBONACCIS_U32: [u32; 48] = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
    10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309,
    3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141,
    267914296, 433494437, 701408733, 1134903170, 1836311903, 2971215073,
];

const LUCAS_NUMBERS_U32: [u32; 47] = [
    2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521, 843, 1364, 2207, 3571, 5778, 9349,
    15127, 24476, 39603, 64079, 103682, 167761, 271443, 439204, 710647, 1149851, 1860498, 3010349,
    4870847, 7881196, 12752043, 20633239, 33385282, 54018521, 87403803, 141422324, 228826127,
    370248451, 599074578, 969323029, 1568397607, 2537720636, 4106118243,
];

const FIBONACCIS_U64: [u64; 94] = [
    0,
    1,
    1,
    2,
    3,
    5,
    8,
    13,
    21,
    34,
    55,
    89,
    144,
    233,
    377,
    610,
    987,
    1597,
    2584,
    4181,
    6765,
    10946,
    17711,
    28657,
    46368,
    75025,
    121393,
    196418,
    317811,
    514229,
    832040,
    1346269,
    2178309,
    3524578,
    5702887,
    9227465,
    14930352,
    24157817,
    39088169,
    63245986,
    102334155,
    165580141,
    267914296,
    433494437,
    701408733,
    1134903170,
    1836311903,
    2971215073,
    4807526976,
    7778742049,
    12586269025,
    20365011074,
    32951280099,
    53316291173,
    86267571272,
    139583862445,
    225851433717,
    365435296162,
    591286729879,
    956722026041,
    1548008755920,
    2504730781961,
    4052739537881,
    6557470319842,
    10610209857723,
    17167680177565,
    27777890035288,
    44945570212853,
    72723460248141,
    117669030460994,
    190392490709135,
    308061521170129,
    498454011879264,
    806515533049393,
    1304969544928657,
    2111485077978050,
    3416454622906707,
    5527939700884757,
    8944394323791464,
    14472334024676221,
    23416728348467685,
    37889062373143906,
    61305790721611591,
    99194853094755497,
    160500643816367088,
    259695496911122585,
    420196140727489673,
    679891637638612258,
    1100087778366101931,
    1779979416004714189,
    2880067194370816120,
    4660046610375530309,
    7540113804746346429,
    12200160415121876738,
];

const LUCAS_NUMBERS_U64: [u64; 93] = [
    2,
    1,
    3,
    4,
    7,
    11,
    18,
    29,
    47,
    76,
    123,
    199,
    322,
    521,
    843,
    1364,
    2207,
    3571,
    5778,
    9349,
    15127,
    24476,
    39603,
    64079,
    103682,
    167761,
    271443,
    439204,
    710647,
    1149851,
    1860498,
    3010349,
    4870847,
    7881196,
    12752043,
    20633239,
    33385282,
    54018521,
    87403803,
    141422324,
    228826127,
    370248451,
    599074578,
    969323029,
    1568397607,
    2537720636,
    4106118243,
    6643838879,
    10749957122,
    17393796001,
    28143753123,
    45537549124,
    73681302247,
    119218851371,
    192900153618,
    312119004989,
    505019158607,
    817138163596,
    1322157322203,
    2139295485799,
    3461452808002,
    5600748293801,
    9062201101803,
    14662949395604,
    23725150497407,
    38388099893011,
    62113250390418,
    100501350283429,
    162614600673847,
    263115950957276,
    425730551631123,
    688846502588399,
    1114577054219522,
    1803423556807921,
    2918000611027443,
    4721424167835364,
    7639424778862807,
    12360848946698171,
    20000273725560978,
    32361122672259149,
    52361396397820127,
    84722519070079276,
    137083915467899403,
    221806434537978679,
    358890350005878082,
    580696784543856761,
    939587134549734843,
    1520283919093591604,
    2459871053643326447,
    3980154972736918051,
    6440026026380244498,
    10420180999117162549,
    16860207025497407047,
];

const FIBONACCIS_U128: [u128; 187] = [
    0,
    1,
    1,
    2,
    3,
    5,
    8,
    13,
    21,
    34,
    55,
    89,
    144,
    233,
    377,
    610,
    987,
    1597,
    2584,
    4181,
    6765,
    10946,
    17711,
    28657,
    46368,
    75025,
    121393,
    196418,
    317811,
    514229,
    832040,
    1346269,
    2178309,
    3524578,
    5702887,
    9227465,
    14930352,
    24157817,
    39088169,
    63245986,
    102334155,
    165580141,
    267914296,
    433494437,
    701408733,
    1134903170,
    1836311903,
    2971215073,
    4807526976,
    7778742049,
    12586269025,
    20365011074,
    32951280099,
    53316291173,
    86267571272,
    139583862445,
    225851433717,
    365435296162,
    591286729879,
    956722026041,
    1548008755920,
    2504730781961,
    4052739537881,
    6557470319842,
    10610209857723,
    17167680177565,
    27777890035288,
    44945570212853,
    72723460248141,
    117669030460994,
    190392490709135,
    308061521170129,
    498454011879264,
    806515533049393,
    1304969544928657,
    2111485077978050,
    3416454622906707,
    5527939700884757,
    8944394323791464,
    14472334024676221,
    23416728348467685,
    37889062373143906,
    61305790721611591,
    99194853094755497,
    160500643816367088,
    259695496911122585,
    420196140727489673,
    679891637638612258,
    1100087778366101931,
    1779979416004714189,
    2880067194370816120,
    4660046610375530309,
    7540113804746346429,
    12200160415121876738,
    19740274219868223167,
    31940434634990099905,
    51680708854858323072,
    83621143489848422977,
    135301852344706746049,
    218922995834555169026,
    354224848179261915075,
    573147844013817084101,
    927372692193078999176,
    1500520536206896083277,
    2427893228399975082453,
    3928413764606871165730,
    6356306993006846248183,
    10284720757613717413913,
    16641027750620563662096,
    26925748508234281076009,
    43566776258854844738105,
    70492524767089125814114,
    114059301025943970552219,
    184551825793033096366333,
    298611126818977066918552,
    483162952612010163284885,
    781774079430987230203437,
    1264937032042997393488322,
    2046711111473984623691759,
    3311648143516982017180081,
    5358359254990966640871840,
    8670007398507948658051921,
    14028366653498915298923761,
    22698374052006863956975682,
    36726740705505779255899443,
    59425114757512643212875125,
    96151855463018422468774568,
    155576970220531065681649693,
    251728825683549488150424261,
    407305795904080553832073954,
    659034621587630041982498215,
    1066340417491710595814572169,
    1725375039079340637797070384,
    2791715456571051233611642553,
    4517090495650391871408712937,
    7308805952221443105020355490,
    11825896447871834976429068427,
    19134702400093278081449423917,
    30960598847965113057878492344,
    50095301248058391139327916261,
    81055900096023504197206408605,
    131151201344081895336534324866,
    212207101440105399533740733471,
    343358302784187294870275058337,
    555565404224292694404015791808,
    898923707008479989274290850145,
    1454489111232772683678306641953,
    2353412818241252672952597492098,
    3807901929474025356630904134051,
    6161314747715278029583501626149,
    9969216677189303386214405760200,
    16130531424904581415797907386349,
    26099748102093884802012313146549,
    42230279526998466217810220532898,
    68330027629092351019822533679447,
    110560307156090817237632754212345,
    178890334785183168257455287891792,
    289450641941273985495088042104137,
    468340976726457153752543329995929,
    757791618667731139247631372100066,
    1226132595394188293000174702095995,
    1983924214061919432247806074196061,
    3210056809456107725247980776292056,
    5193981023518027157495786850488117,
    8404037832974134882743767626780173,
    13598018856492162040239554477268290,
    22002056689466296922983322104048463,
    35600075545958458963222876581316753,
    57602132235424755886206198685365216,
    93202207781383214849429075266681969,
    150804340016807970735635273952047185,
    244006547798191185585064349218729154,
    394810887814999156320699623170776339,
    638817435613190341905763972389505493,
    1033628323428189498226463595560281832,
    1672445759041379840132227567949787325,
    2706074082469569338358691163510069157,
    4378519841510949178490918731459856482,
    7084593923980518516849609894969925639,
    11463113765491467695340528626429782121,
    18547707689471986212190138521399707760,
    30010821454963453907530667147829489881,
    48558529144435440119720805669229197641,
    78569350599398894027251472817058687522,
    127127879743834334146972278486287885163,
    205697230343233228174223751303346572685,
    332825110087067562321196029789634457848,
];

const LUCAS_NUMBERS_U128: [u128; 185] = [
    2,
    1,
    3,
    4,
    7,
    11,
    18,
    29,
    47,
    76,
    123,
    199,
    322,
    521,
    843,
    1364,
    2207,
    3571,
    5778,
    9349,
    15127,
    24476,
    39603,
    64079,
    103682,
    167761,
    271443,
    439204,
    710647,
    1149851,
    1860498,
    3010349,
    4870847,
    7881196,
    12752043,
    20633239,
    33385282,
    54018521,
    87403803,
    141422324,
    228826127,
    370248451,
    599074578,
    969323029,
    1568397607,
    2537720636,
    4106118243,
    6643838879,
    10749957122,
    17393796001,
    28143753123,
    45537549124,
    73681302247,
    119218851371,
    192900153618,
    312119004989,
    505019158607,
    817138163596,
    1322157322203,
    2139295485799,
    3461452808002,
    5600748293801,
    9062201101803,
    14662949395604,
    23725150497407,
    38388099893011,
    62113250390418,
    100501350283429,
    162614600673847,
    263115950957276,
    425730551631123,
    688846502588399,
    1114577054219522,
    1803423556807921,
    2918000611027443,
    4721424167835364,
    7639424778862807,
    12360848946698171,
    20000273725560978,
    32361122672259149,
    52361396397820127,
    84722519070079276,
    137083915467899403,
    221806434537978679,
    358890350005878082,
    580696784543856761,
    939587134549734843,
    1520283919093591604,
    2459871053643326447,
    3980154972736918051,
    6440026026380244498,
    10420180999117162549,
    16860207025497407047,
    27280388024614569596,
    44140595050111976643,
    71420983074726546239,
    115561578124838522882,
    186982561199565069121,
    302544139324403592003,
    489526700523968661124,
    792070839848372253127,
    1281597540372340914251,
    2073668380220713167378,
    3355265920593054081629,
    5428934300813767249007,
    8784200221406821330636,
    14213134522220588579643,
    22997334743627409910279,
    37210469265847998489922,
    60207804009475408400201,
    97418273275323406890123,
    157626077284798815290324,
    255044350560122222180447,
    412670427844921037470771,
    667714778405043259651218,
    1080385206249964297121989,
    1748099984655007556773207,
    2828485190904971853895196,
    4576585175559979410668403,
    7405070366464951264563599,
    11981655542024930675232002,
    19386725908489881939795601,
    31368381450514812615027603,
    50755107359004694554823204,
    82123488809519507169850807,
    132878596168524201724674011,
    215002084978043708894524818,
    347880681146567910619198829,
    562882766124611619513723647,
    910763447271179530132922476,
    1473646213395791149646646123,
    2384409660666970679779568599,
    3858055874062761829426214722,
    6242465534729732509205783321,
    10100521408792494338631998043,
    16342986943522226847837781364,
    26443508352314721186469779407,
    42786495295836948034307560771,
    69230003648151669220777340178,
    112016498943988617255084900949,
    181246502592140286475862241127,
    293263001536128903730947142076,
    474509504128269190206809383203,
    767772505664398093937756525279,
    1242282009792667284144565908482,
    2010054515457065378082322433761,
    3252336525249732662226888342243,
    5262391040706798040309210776004,
    8514727565956530702536099118247,
    13777118606663328742845309894251,
    22291846172619859445381409012498,
    36068964779283188188226718906749,
    58360810951903047633608127919247,
    94429775731186235821834846825996,
    152790586683089283455442974745243,
    247220362414275519277277821571239,
    400010949097364802732720796316482,
    647231311511640322009998617887721,
    1047242260609005124742719414204203,
    1694473572120645446752718032091924,
    2741715832729650571495437446296127,
    4436189404850296018248155478388051,
    7177905237579946589743592924684178,
    11614094642430242607991748403072229,
    18791999880010189197735341327756407,
    30406094522440431805727089730828636,
    49198094402450621003462431058585043,
    79604188924891052809189520789413679,
    128802283327341673812651951847998722,
    208406472252232726621841472637412401,
    337208755579574400434493424485411123,
    545615227831807127056334897122823524,
    882823983411381527490828321608234647,
    1428439211243188654547163218731058171,
    2311263194654570182037991540339292818,
    3739702405897758836585154759070350989,
    6050965600552329018623146299409643807,
    9790668006450087855208301058479994796,
    15841633607002416873831447357889638603,
    25632301613452504729039748416369633399,
    41473935220454921602871195774259272002,
    67106236833907426331910944190628905401,
    108580172054362347934782139964888177403,
    175686408888269774266693084155517082804,
    284266580942632122201475224120405260207,
];

macro_rules! impl_fibonaccis_a {
    ($t:ident, $fs:ident, $ls:ident) => {
        impl CheckedFibonacci for $t {
            /// Computes the $n$th Fibonacci number.
            ///
            /// If the result is too large to be represented, the function returns `None`.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}(F(n)) & \text{if} \\quad F(n) < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad F(n) \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// $F(n) = O(\phi^n)$, where $\phi$ is the golden ratio.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::fibonacci#checked_fibonacci).
            #[inline]
            fn checked_fibonacci(n: u64) -> Option<$t> {
                $fs.get(usize::try_from(n).ok()?).copied()
            }

            /// Computes the $n$th Fibonacci number, paired with its predecessor: $(F(n), F(n-1))$.
            ///
            /// If either component is too large to be represented, the function returns `None`.
            /// Since $F(-1) = 1$, the pair is defined for $n = 0$ as well.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}((F(n), F(n-1))) & \text{if} \\quad F(n) < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad F(n) \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::fibonacci#checked_fibonacci_pair).
            #[inline]
            fn checked_fibonacci_pair(n: u64) -> Option<($t, $t)> {
                if n == 0 {
                    // F(-1) = 1
                    Some((0, 1))
                } else {
                    let i = usize::try_from(n).ok()?;
                    Some((*$fs.get(i)?, $fs[i - 1]))
                }
            }
        }

        impl CheckedLucasNumber for $t {
            /// Computes the $n$th Lucas number.
            ///
            /// If the result is too large to be represented, the function returns `None`.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}(L(n)) & \text{if} \\quad L(n) < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad L(n) \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// $L(n) = O(\phi^n)$, where $\phi$ is the golden ratio.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::fibonacci#checked_lucas_number).
            #[inline]
            fn checked_lucas_number(n: u64) -> Option<$t> {
                $ls.get(usize::try_from(n).ok()?).copied()
            }

            /// Computes the $n$th Lucas number, paired with its predecessor: $(L(n), L(n-1))$.
            ///
            /// If either component is too large to be represented, the function returns `None`. The
            /// function also returns `None` when $n = 0$, since $L(-1) = -1$ cannot be represented.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}((L(n), L(n-1))) &
            ///         \text{if} \\quad n > 0 \ \text{and} \ L(n) < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad n = 0 \ \text{or} \ L(n) \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::fibonacci#checked_lucas_number_pair).
            #[inline]
            fn checked_lucas_number_pair(n: u64) -> Option<($t, $t)> {
                if n == 0 {
                    // L(-1) = -1, which cannot be represented
                    None
                } else {
                    let i = usize::try_from(n).ok()?;
                    Some((*$ls.get(i)?, $ls[i - 1]))
                }
            }
        }
    };
}
impl_fibonaccis_a!(u8, FIBONACCIS_U8, LUCAS_NUMBERS_U8);
impl_fibonaccis_a!(u16, FIBONACCIS_U16, LUCAS_NUMBERS_U16);
impl_fibonaccis_a!(u32, FIBONACCIS_U32, LUCAS_NUMBERS_U32);
impl_fibonaccis_a!(u64, FIBONACCIS_U64, LUCAS_NUMBERS_U64);
impl_fibonaccis_a!(u128, FIBONACCIS_U128, LUCAS_NUMBERS_U128);

impl CheckedFibonacci for usize {
    /// Computes the $n$th Fibonacci number.
    ///
    /// If the result is too large to be represented, the function returns `None`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::fibonacci#checked_fibonacci).
    #[inline]
    fn checked_fibonacci(n: u64) -> Option<Self> {
        FIBONACCIS_U64
            .get(Self::try_from(n).ok()?)
            .and_then(|&f| Self::try_from(f).ok())
    }

    /// Computes the $n$th Fibonacci number, paired with its predecessor: $(F(n), F(n-1))$.
    ///
    /// If either component is too large to be represented, the function returns `None`. Since
    /// $F(-1) = 1$, the pair is defined for $n = 0$ as well.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::fibonacci#checked_fibonacci_pair).
    #[inline]
    fn checked_fibonacci_pair(n: u64) -> Option<(Self, Self)> {
        if n == 0 {
            // F(-1) = 1
            Some((0, 1))
        } else {
            let i = Self::try_from(n).ok()?;
            let f = Self::try_from(*FIBONACCIS_U64.get(i)?).ok()?;
            Some((f, Self::try_from(FIBONACCIS_U64[i - 1]).ok()?))
        }
    }
}

impl CheckedLucasNumber for usize {
    /// Computes the $n$th Lucas number.
    ///
    /// If the result is too large to be represented, the function returns `None`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::fibonacci#checked_lucas_number).
    #[inline]
    fn checked_lucas_number(n: u64) -> Option<Self> {
        LUCAS_NUMBERS_U64
            .get(Self::try_from(n).ok()?)
            .and_then(|&x| Self::try_from(x).ok())
    }

    /// Computes the $n$th Lucas number, paired with its predecessor: $(L(n), L(n-1))$.
    ///
    /// If either component is too large to be represented, the function returns `None`. The
    /// function also returns `None` when $n = 0$, since $L(-1) = -1$ cannot be represented.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::fibonacci#checked_lucas_number_pair).
    #[inline]
    fn checked_lucas_number_pair(n: u64) -> Option<(Self, Self)> {
        if n == 0 {
            // L(-1) = -1, which cannot be represented
            None
        } else {
            let i = Self::try_from(n).ok()?;
            let x = Self::try_from(*LUCAS_NUMBERS_U64.get(i)?).ok()?;
            Some((x, Self::try_from(LUCAS_NUMBERS_U64[i - 1]).ok()?))
        }
    }
}

macro_rules! impl_fibonaccis_b {
    ($t:ident) => {
        impl Fibonacci for $t {
            /// Computes the $n$th Fibonacci number.
            ///
            /// If the result is too large to be represented, the function panics. For a function
            /// that returns `None` instead, try
            /// [`checked_fibonacci`](CheckedFibonacci::checked_fibonacci).
            ///
            /// $$
            /// f(n) = F(n) = F(n - 1) + F(n - 2), \\quad F(0) = 0, \ F(1) = 1.
            /// $$
            ///
            /// $F(n) = O(\phi^n)$, where $\phi$ is the golden ratio.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::fibonacci#fibonacci).
            #[inline]
            fn fibonacci(n: u64) -> $t {
                $t::checked_fibonacci(n).unwrap()
            }

            /// Computes the $n$th Fibonacci number, paired with its predecessor: $(F(n), F(n-1))$.
            ///
            /// If either component is too large to be represented, the function panics. For a
            /// function that returns `None` instead, try
            /// [`checked_fibonacci_pair`](CheckedFibonacci::checked_fibonacci_pair).
            ///
            /// Since $F(-1) = 1$, the pair is defined for $n = 0$ as well.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::fibonacci#fibonacci_pair).
            #[inline]
            fn fibonacci_pair(n: u64) -> ($t, $t) {
                $t::checked_fibonacci_pair(n).unwrap()
            }
        }

        impl LucasNumber for $t {
            /// Computes the $n$th Lucas number.
            ///
            /// If the result is too large to be represented, the function panics. For a function
            /// that returns `None` instead, try
            /// [`checked_lucas_number`](CheckedLucasNumber::checked_lucas_number).
            ///
            /// $$
            /// f(n) = L(n) = L(n - 1) + L(n - 2), \\quad L(0) = 2, \ L(1) = 1.
            /// $$
            ///
            /// $L(n) = O(\phi^n)$, where $\phi$ is the golden ratio.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::fibonacci#lucas_number).
            #[inline]
            fn lucas_number(n: u64) -> $t {
                $t::checked_lucas_number(n).unwrap()
            }

            /// Computes the $n$th Lucas number, paired with its predecessor: $(L(n), L(n-1))$.
            ///
            /// If either component is too large to be represented, the function panics. It also
            /// panics when $n = 0$, since $L(-1) = -1$ cannot be represented. For a function that
            /// returns `None` instead, try
            /// [`checked_lucas_number_pair`](CheckedLucasNumber::checked_lucas_number_pair).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `n` is 0 or if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::fibonacci#lucas_number_pair).
            #[inline]
            fn lucas_number_pair(n: u64) -> ($t, $t) {
                $t::checked_lucas_number_pair(n).unwrap()
            }
        }
    };
}
apply_to_unsigneds!(impl_fibonaccis_b);
