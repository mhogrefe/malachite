use std::str::FromStr;

use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use num::{BigInt, Integer as NumInteger, Zero as NumZero};

use malachite_test::common::test_properties;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer_var_1,
    pairs_of_integer_and_nonzero_integer_var_2, pairs_of_integers,
};

fn num_divisible_by(x: BigInt, y: BigInt) -> bool {
    x == BigInt::zero() || y != BigInt::zero() && x.is_multiple_of(&y)
}

#[test]
fn test_divisible_by() {
    let test = |u, v, divisible| {
        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .divisible_by(Integer::from_str(v).unwrap()),
            divisible
        );
        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .divisible_by(&Integer::from_str(v).unwrap()),
            divisible
        );
        assert_eq!(
            (&Integer::from_str(u).unwrap()).divisible_by(Integer::from_str(v).unwrap()),
            divisible
        );
        assert_eq!(
            (&Integer::from_str(u).unwrap()).divisible_by(&Integer::from_str(v).unwrap()),
            divisible
        );

        let x = Integer::from_str(u).unwrap();
        let y = Integer::from_str(v).unwrap();
        assert_eq!(
            x == Integer::ZERO || y != Integer::ZERO && x % y == Integer::ZERO,
            divisible
        );

        assert_eq!(
            num_divisible_by(BigInt::from_str(u).unwrap(), BigInt::from_str(v).unwrap()),
            divisible
        );
        assert_eq!(
            rug::Integer::from_str(u)
                .unwrap()
                .is_divisible(&rug::Integer::from_str(v).unwrap()),
            divisible
        );
    };
    test("0", "0", true);
    test("1", "0", false);
    test("1000000000000", "0", false);
    test("0", "1", true);
    test("0", "123", true);
    test("1", "1", true);
    test("123", "1", true);
    test("123", "123", true);
    test("123", "456", false);
    test("456", "123", false);
    test("369", "123", true);
    test("4294967295", "1", true);
    test("4294967295", "4294967295", true);
    test("1000000000000", "1", true);
    test("1000000000000", "3", false);
    test("1000000000002", "3", true);
    test("1000000000000", "123", false);
    test("1000000000000", "4294967295", false);
    test("1000000000000000000000000", "1", true);
    test("1000000000000000000000000", "3", false);
    test("1000000000000000000000002", "3", true);
    test("1000000000000000000000000", "123", false);
    test("1000000000000000000000000", "4294967295", false);
    test("1000000000000000000000000", "1000000000000", true);
    test("1000000000000000000000000", "1000000000001", false);
    test(
        "851673906388325341550957943071111911557800036845129556099360938813259608650267079456739978\
        1156959952275409185911771336067392377245918291754269000751186715279414560474882570499082990\
        4913122978897463970860833616251189242098804876664368441608727895141238953179204529256780277\
        5978105200286025161944212712977056343127682601975191673217459602567633602198262568921008081\
        9448556670912575287371251190800855926311768876808375177446530243635212748346921654224589861\
        0625170426812525829689862407515510419445335472631905610235915226032848323874067128872385291\
        3730739275467227364692195226129501338887049710586931141309357190341064532366013123280106098\
        6468151628797945455179649866890394481799639832540978091736379482964522229064478167730317490\
        8194108506704480750395054067032502530392147690725919399930683143510771646869931527123340650\
        0547649792331568913460415939722111305270588701531404490040034302102101083691706550376288655\
        2667382899390792494118931379237432071316543313379792218794371176529684614085109418328963817\
        0601432767270419229719490809539776535671938041618536196941370647945336401901450921413823163\
        4059991707077834107830876756821880651429748186401020760113859498185638133726165286481741014\
        9079906337286599226335508424466369316294442004040440528589582239717042654541745348050157252\
        3448224036804997350851153108395928780441635856",
        "147502279655636565600250358452694051893980186696958535174009956523855720107322638159749368\
        0808217479494744305876890972595771484769733857514529616096199394092858302265998260483416016\
        5763904522044264005938281072568140883513713255548643044250086110483617215935636533809248102\
        6926590789142079805638445494760177551776636747830014495012489743990407355232286842071418922\
        9921358409573480901624487977319782755422730834468673438076805532952821406024399006814390166\
        6949827530796971086011267864607814906313334525518102221919643040440267323688341889035864376\
        1377246644579088153222669672271414315240318439843720039808993886410874969340996645010795670\
        2133518716987668865936529827437388042190084309005369564717390726257594902619365180097509576\
        6240189037770619308206906414128686856349950952623970023039440323701643457411485666776354448\
        186307133288106956593939073729500658176632828099789",
        true
    );
    test(
        "851673906388325341550957943071111911557800036845129556099360938813259608650267079456739978\
        1156959952275409185911771336067392377245918291754269000751186715279414560474882570499082990\
        4913122978897463970860833616251189242098804876664368441608727895141238953179204529256780277\
        5978105200286025161944212712977056343127682601975191673217459602567633602198262568921008081\
        9448556670912575287371251190800855926311768876808375177446530243635212748346921654224589861\
        0625170426812525829689862407515510419445335472631905610235915226032848323874067128872385291\
        3730739275467227364692195226129501338887049710586931141309357190341064532366013123280106098\
        6468151628797945455179649866890394481799639832540978091736379482964522229064478167730317490\
        8194108506704480750395054067032502530392147690725919399930683143510771646869931527123340650\
        0547649792331568913460415939722111305270588701531404490040034302102101083691706550376288655\
        2667382899390792494118931379237432071316543313379792218794371176529684614085109418328963817\
        0601432767270419229719490809539776535671938041618536196941370647945336401901450921413823163\
        4059991707077834107830876756821880651429748186401020760113859498185638133726165286481741014\
        9079906337286599226335508424466369316294442004040440528589582239717042654541745348050157252\
        3448224036804997350851153108395928780441635856",
        "147502279655636565600250358452694051893980186696958535174009956523855720107322638159749368\
        0808217479494744305876890972595771484769733857514529616096199394092858302265998260483416016\
        5763904522044264005938281072568140883513713255548643044250086110483617215935636533809248102\
        6926590789142079805638445494760177551776636747830014495012489743990407355232286842071418922\
        9921358409573480901624487977319782755422730834468673438076805532952821406024399006814390166\
        6949827530796971086011267864607814906313334525518102221919643040440267323688341889035864376\
        1377246644579088153222669672271414315240318439843720039808993886410874969340996645010795670\
        2133518716987668865936529827437388042190084309005369564717390726257594902619365180097509576\
        6240189037770619308206906414128686856349950952623970023039440323701643457411485666776354448\
        186307133288106956593939073729500658176632828099788",
        false
    );

    test("0", "-1", true);
    test("0", "-123", true);
    test("1", "-1", true);
    test("123", "-1", true);
    test("123", "-123", true);
    test("123", "-456", false);
    test("456", "-123", false);
    test("369", "-123", true);
    test("4294967295", "-1", true);
    test("4294967295", "-4294967295", true);
    test("1000000000000", "-1", true);
    test("1000000000000", "-3", false);
    test("1000000000002", "-3", true);
    test("1000000000000", "-123", false);
    test("1000000000000", "-4294967295", false);
    test("1000000000000000000000000", "-1", true);
    test("1000000000000000000000000", "-3", false);
    test("1000000000000000000000002", "-3", true);
    test("1000000000000000000000000", "-123", false);
    test("1000000000000000000000000", "-4294967295", false);
    test("1000000000000000000000000", "-1000000000000", true);
    test("1000000000000000000000000", "-1000000000001", false);
    test(
        "851673906388325341550957943071111911557800036845129556099360938813259608650267079456739978\
        1156959952275409185911771336067392377245918291754269000751186715279414560474882570499082990\
        4913122978897463970860833616251189242098804876664368441608727895141238953179204529256780277\
        5978105200286025161944212712977056343127682601975191673217459602567633602198262568921008081\
        9448556670912575287371251190800855926311768876808375177446530243635212748346921654224589861\
        0625170426812525829689862407515510419445335472631905610235915226032848323874067128872385291\
        3730739275467227364692195226129501338887049710586931141309357190341064532366013123280106098\
        6468151628797945455179649866890394481799639832540978091736379482964522229064478167730317490\
        8194108506704480750395054067032502530392147690725919399930683143510771646869931527123340650\
        0547649792331568913460415939722111305270588701531404490040034302102101083691706550376288655\
        2667382899390792494118931379237432071316543313379792218794371176529684614085109418328963817\
        0601432767270419229719490809539776535671938041618536196941370647945336401901450921413823163\
        4059991707077834107830876756821880651429748186401020760113859498185638133726165286481741014\
        9079906337286599226335508424466369316294442004040440528589582239717042654541745348050157252\
        3448224036804997350851153108395928780441635856",
        "-14750227965563656560025035845269405189398018669695853517400995652385572010732263815974936\
        8080821747949474430587689097259577148476973385751452961609619939409285830226599826048341601\
        6576390452204426400593828107256814088351371325554864304425008611048361721593563653380924810\
        2692659078914207980563844549476017755177663674783001449501248974399040735523228684207141892\
        2992135840957348090162448797731978275542273083446867343807680553295282140602439900681439016\
        6694982753079697108601126786460781490631333452551810222191964304044026732368834188903586437\
        6137724664457908815322266967227141431524031843984372003980899388641087496934099664501079567\
        0213351871698766886593652982743738804219008430900536956471739072625759490261936518009750957\
        6624018903777061930820690641412868685634995095262397002303944032370164345741148566677635444\
        8186307133288106956593939073729500658176632828099789",
        true
    );
    test(
        "851673906388325341550957943071111911557800036845129556099360938813259608650267079456739978\
        1156959952275409185911771336067392377245918291754269000751186715279414560474882570499082990\
        4913122978897463970860833616251189242098804876664368441608727895141238953179204529256780277\
        5978105200286025161944212712977056343127682601975191673217459602567633602198262568921008081\
        9448556670912575287371251190800855926311768876808375177446530243635212748346921654224589861\
        0625170426812525829689862407515510419445335472631905610235915226032848323874067128872385291\
        3730739275467227364692195226129501338887049710586931141309357190341064532366013123280106098\
        6468151628797945455179649866890394481799639832540978091736379482964522229064478167730317490\
        8194108506704480750395054067032502530392147690725919399930683143510771646869931527123340650\
        0547649792331568913460415939722111305270588701531404490040034302102101083691706550376288655\
        2667382899390792494118931379237432071316543313379792218794371176529684614085109418328963817\
        0601432767270419229719490809539776535671938041618536196941370647945336401901450921413823163\
        4059991707077834107830876756821880651429748186401020760113859498185638133726165286481741014\
        9079906337286599226335508424466369316294442004040440528589582239717042654541745348050157252\
        3448224036804997350851153108395928780441635856",
        "-14750227965563656560025035845269405189398018669695853517400995652385572010732263815974936\
        8080821747949474430587689097259577148476973385751452961609619939409285830226599826048341601\
        6576390452204426400593828107256814088351371325554864304425008611048361721593563653380924810\
        2692659078914207980563844549476017755177663674783001449501248974399040735523228684207141892\
        2992135840957348090162448797731978275542273083446867343807680553295282140602439900681439016\
        6694982753079697108601126786460781490631333452551810222191964304044026732368834188903586437\
        6137724664457908815322266967227141431524031843984372003980899388641087496934099664501079567\
        0213351871698766886593652982743738804219008430900536956471739072625759490261936518009750957\
        6624018903777061930820690641412868685634995095262397002303944032370164345741148566677635444\
        8186307133288106956593939073729500658176632828099788",
        false
    );

    test("-1", "0", false);
    test("-1000000000000", "0", false);
    test("-1", "1", true);
    test("-123", "1", true);
    test("-123", "123", true);
    test("-123", "456", false);
    test("-456", "123", false);
    test("-369", "123", true);
    test("-4294967295", "1", true);
    test("-4294967295", "4294967295", true);
    test("-1000000000000", "1", true);
    test("-1000000000000", "3", false);
    test("-1000000000002", "3", true);
    test("-1000000000000", "123", false);
    test("-1000000000000", "4294967295", false);
    test("-1000000000000000000000000", "1", true);
    test("-1000000000000000000000000", "3", false);
    test("-1000000000000000000000002", "3", true);
    test("-1000000000000000000000000", "123", false);
    test("-1000000000000000000000000", "4294967295", false);
    test("-1000000000000000000000000", "1000000000000", true);
    test("-1000000000000000000000000", "1000000000001", false);
    test(
        "-85167390638832534155095794307111191155780003684512955609936093881325960865026707945673997\
        8115695995227540918591177133606739237724591829175426900075118671527941456047488257049908299\
        0491312297889746397086083361625118924209880487666436844160872789514123895317920452925678027\
        7597810520028602516194421271297705634312768260197519167321745960256763360219826256892100808\
        1944855667091257528737125119080085592631176887680837517744653024363521274834692165422458986\
        1062517042681252582968986240751551041944533547263190561023591522603284832387406712887238529\
        1373073927546722736469219522612950133888704971058693114130935719034106453236601312328010609\
        8646815162879794545517964986689039448179963983254097809173637948296452222906447816773031749\
        0819410850670448075039505406703250253039214769072591939993068314351077164686993152712334065\
        0054764979233156891346041593972211130527058870153140449004003430210210108369170655037628865\
        5266738289939079249411893137923743207131654331337979221879437117652968461408510941832896381\
        7060143276727041922971949080953977653567193804161853619694137064794533640190145092141382316\
        3405999170707783410783087675682188065142974818640102076011385949818563813372616528648174101\
        4907990633728659922633550842446636931629444200404044052858958223971704265454174534805015725\
        23448224036804997350851153108395928780441635856",
        "147502279655636565600250358452694051893980186696958535174009956523855720107322638159749368\
        0808217479494744305876890972595771484769733857514529616096199394092858302265998260483416016\
        5763904522044264005938281072568140883513713255548643044250086110483617215935636533809248102\
        6926590789142079805638445494760177551776636747830014495012489743990407355232286842071418922\
        9921358409573480901624487977319782755422730834468673438076805532952821406024399006814390166\
        6949827530796971086011267864607814906313334525518102221919643040440267323688341889035864376\
        1377246644579088153222669672271414315240318439843720039808993886410874969340996645010795670\
        2133518716987668865936529827437388042190084309005369564717390726257594902619365180097509576\
        6240189037770619308206906414128686856349950952623970023039440323701643457411485666776354448\
        186307133288106956593939073729500658176632828099789",
        true
    );
    test(
        "-85167390638832534155095794307111191155780003684512955609936093881325960865026707945673997\
        8115695995227540918591177133606739237724591829175426900075118671527941456047488257049908299\
        0491312297889746397086083361625118924209880487666436844160872789514123895317920452925678027\
        7597810520028602516194421271297705634312768260197519167321745960256763360219826256892100808\
        1944855667091257528737125119080085592631176887680837517744653024363521274834692165422458986\
        1062517042681252582968986240751551041944533547263190561023591522603284832387406712887238529\
        1373073927546722736469219522612950133888704971058693114130935719034106453236601312328010609\
        8646815162879794545517964986689039448179963983254097809173637948296452222906447816773031749\
        0819410850670448075039505406703250253039214769072591939993068314351077164686993152712334065\
        0054764979233156891346041593972211130527058870153140449004003430210210108369170655037628865\
        5266738289939079249411893137923743207131654331337979221879437117652968461408510941832896381\
        7060143276727041922971949080953977653567193804161853619694137064794533640190145092141382316\
        3405999170707783410783087675682188065142974818640102076011385949818563813372616528648174101\
        4907990633728659922633550842446636931629444200404044052858958223971704265454174534805015725\
        23448224036804997350851153108395928780441635856",
        "147502279655636565600250358452694051893980186696958535174009956523855720107322638159749368\
        0808217479494744305876890972595771484769733857514529616096199394092858302265998260483416016\
        5763904522044264005938281072568140883513713255548643044250086110483617215935636533809248102\
        6926590789142079805638445494760177551776636747830014495012489743990407355232286842071418922\
        9921358409573480901624487977319782755422730834468673438076805532952821406024399006814390166\
        6949827530796971086011267864607814906313334525518102221919643040440267323688341889035864376\
        1377246644579088153222669672271414315240318439843720039808993886410874969340996645010795670\
        2133518716987668865936529827437388042190084309005369564717390726257594902619365180097509576\
        6240189037770619308206906414128686856349950952623970023039440323701643457411485666776354448\
        186307133288106956593939073729500658176632828099788",
        false
    );

    test("-1", "-1", true);
    test("-123", "-1", true);
    test("-123", "-123", true);
    test("-123", "-456", false);
    test("-456", "-123", false);
    test("-369", "-123", true);
    test("-4294967295", "-1", true);
    test("-4294967295", "-4294967295", true);
    test("-1000000000000", "-1", true);
    test("-1000000000000", "-3", false);
    test("-1000000000002", "-3", true);
    test("-1000000000000", "-123", false);
    test("-1000000000000", "-4294967295", false);
    test("-1000000000000000000000000", "-1", true);
    test("-1000000000000000000000000", "-3", false);
    test("-1000000000000000000000002", "-3", true);
    test("-1000000000000000000000000", "-123", false);
    test("-1000000000000000000000000", "-4294967295", false);
    test("-1000000000000000000000000", "-1000000000000", true);
    test("-1000000000000000000000000", "-1000000000001", false);
    test(
        "-85167390638832534155095794307111191155780003684512955609936093881325960865026707945673997\
        8115695995227540918591177133606739237724591829175426900075118671527941456047488257049908299\
        0491312297889746397086083361625118924209880487666436844160872789514123895317920452925678027\
        7597810520028602516194421271297705634312768260197519167321745960256763360219826256892100808\
        1944855667091257528737125119080085592631176887680837517744653024363521274834692165422458986\
        1062517042681252582968986240751551041944533547263190561023591522603284832387406712887238529\
        1373073927546722736469219522612950133888704971058693114130935719034106453236601312328010609\
        8646815162879794545517964986689039448179963983254097809173637948296452222906447816773031749\
        0819410850670448075039505406703250253039214769072591939993068314351077164686993152712334065\
        0054764979233156891346041593972211130527058870153140449004003430210210108369170655037628865\
        5266738289939079249411893137923743207131654331337979221879437117652968461408510941832896381\
        7060143276727041922971949080953977653567193804161853619694137064794533640190145092141382316\
        3405999170707783410783087675682188065142974818640102076011385949818563813372616528648174101\
        4907990633728659922633550842446636931629444200404044052858958223971704265454174534805015725\
        23448224036804997350851153108395928780441635856",
        "-14750227965563656560025035845269405189398018669695853517400995652385572010732263815974936\
        8080821747949474430587689097259577148476973385751452961609619939409285830226599826048341601\
        6576390452204426400593828107256814088351371325554864304425008611048361721593563653380924810\
        2692659078914207980563844549476017755177663674783001449501248974399040735523228684207141892\
        2992135840957348090162448797731978275542273083446867343807680553295282140602439900681439016\
        6694982753079697108601126786460781490631333452551810222191964304044026732368834188903586437\
        6137724664457908815322266967227141431524031843984372003980899388641087496934099664501079567\
        0213351871698766886593652982743738804219008430900536956471739072625759490261936518009750957\
        6624018903777061930820690641412868685634995095262397002303944032370164345741148566677635444\
        8186307133288106956593939073729500658176632828099789",
        true
    );
    test(
        "-85167390638832534155095794307111191155780003684512955609936093881325960865026707945673997\
        8115695995227540918591177133606739237724591829175426900075118671527941456047488257049908299\
        0491312297889746397086083361625118924209880487666436844160872789514123895317920452925678027\
        7597810520028602516194421271297705634312768260197519167321745960256763360219826256892100808\
        1944855667091257528737125119080085592631176887680837517744653024363521274834692165422458986\
        1062517042681252582968986240751551041944533547263190561023591522603284832387406712887238529\
        1373073927546722736469219522612950133888704971058693114130935719034106453236601312328010609\
        8646815162879794545517964986689039448179963983254097809173637948296452222906447816773031749\
        0819410850670448075039505406703250253039214769072591939993068314351077164686993152712334065\
        0054764979233156891346041593972211130527058870153140449004003430210210108369170655037628865\
        5266738289939079249411893137923743207131654331337979221879437117652968461408510941832896381\
        7060143276727041922971949080953977653567193804161853619694137064794533640190145092141382316\
        3405999170707783410783087675682188065142974818640102076011385949818563813372616528648174101\
        4907990633728659922633550842446636931629444200404044052858958223971704265454174534805015725\
        23448224036804997350851153108395928780441635856",
        "-14750227965563656560025035845269405189398018669695853517400995652385572010732263815974936\
        8080821747949474430587689097259577148476973385751452961609619939409285830226599826048341601\
        6576390452204426400593828107256814088351371325554864304425008611048361721593563653380924810\
        2692659078914207980563844549476017755177663674783001449501248974399040735523228684207141892\
        2992135840957348090162448797731978275542273083446867343807680553295282140602439900681439016\
        6694982753079697108601126786460781490631333452551810222191964304044026732368834188903586437\
        6137724664457908815322266967227141431524031843984372003980899388641087496934099664501079567\
        0213351871698766886593652982743738804219008430900536956471739072625759490261936518009750957\
        6624018903777061930820690641412868685634995095262397002303944032370164345741148566677635444\
        8186307133288106956593939073729500658176632828099788",
        false
    );
}

#[test]
fn divisible_by_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(x.divisible_by(y.clone()), divisible);
        assert_eq!(x.clone().divisible_by(y), divisible);
        assert_eq!(x.clone().divisible_by(y.clone()), divisible);

        assert_eq!(
            *x == Integer::ZERO || *y != Integer::ZERO && x % y == Integer::ZERO,
            divisible
        );
        assert_eq!(
            num_divisible_by(integer_to_bigint(x), integer_to_bigint(y)),
            divisible
        );
        assert_eq!(
            integer_to_rug_integer(x).is_divisible(&integer_to_rug_integer(y)),
            divisible
        );
    });

    test_properties(
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            assert!(x.divisible_by(y));
            assert!(*x == Integer::ZERO || *y != Integer::ZERO && x % y == Integer::ZERO);
            assert!(num_divisible_by(integer_to_bigint(x), integer_to_bigint(y)));
            assert!(integer_to_rug_integer(x).is_divisible(&integer_to_rug_integer(y)));
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_integer_var_2,
        |&(ref x, ref y)| {
            assert!(!x.divisible_by(y));
            assert!(*x != Integer::ZERO && (*y == Integer::ZERO || x * y != Integer::ZERO));
            assert!(!num_divisible_by(
                integer_to_bigint(x),
                integer_to_bigint(y)
            ));
            assert!(!integer_to_rug_integer(x).is_divisible(&integer_to_rug_integer(y)));
        },
    );

    test_properties(integers, |n| {
        assert!(n.divisible_by(Integer::ONE));
    });

    test_properties(nonzero_integers, |n| {
        assert!(!n.divisible_by(Integer::ZERO));
        assert!(Integer::ZERO.divisible_by(n));
        if *n > Integer::ONE {
            assert!(!Integer::ONE.divisible_by(n));
        }
        assert!(n.divisible_by(n));
    });
}
