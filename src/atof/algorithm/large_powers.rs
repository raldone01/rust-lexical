//! Precalculated large powers for prime numbers for `b^2^i`.
//!
//! We only need powers such that `b^n <= 2^1075`.

// PRIME (EXCEPT 2)

/// Large powers (&[u32]) for base3 operations.
const POW3: [&'static [u32]; 10] = [
    &[3],
    &[9],
    &[81],
    &[6561],
    &[43046721],
    &[3793632897, 431439],
    &[2038349057, 2033330060, 1456779151, 43],
    &[2324068865, 2151814273, 4219200356, 3831009148, 1515007727, 1223069914, 1878],
    &[120648705, 3524288303, 123305297, 1973552606, 1582677123, 4275192449, 223346561, 3350064824, 1500483036, 1257605964, 1367913622, 2878850248, 3527953],
    &[1995565057, 493227282, 1124501772, 1987702555, 1293151165, 901580639, 3173678675, 2491643407, 2858789928, 4190666723, 3234280160, 1552091534, 1098766192, 535263123, 671427889, 2428827667, 442154954, 3812608783, 2997209821, 882392610, 726842283, 3784254415, 1868908668, 1167284160, 3936843162, 2897],
];

/// Large powers (&[u32]) for base5 operations.
const POW5: [&'static [u32]; 9] = [
    &[5],
    &[25],
    &[625],
    &[390625],
    &[2264035265, 35],
    &[2242703233, 762134875, 1262],
    &[3211403009, 1849224548, 3668416493, 3913284084, 1593091],
    &[781532673, 64985353, 253049085, 594863151, 3553621484, 3288652808, 3167596762, 2788392729, 3911132675, 590],
    &[2553183233, 3201533787, 3638140786, 303378311, 1809731782, 3477761648, 3583367183, 649228654, 2915460784, 487929380, 1011012442, 1677677582, 3428152256, 1710878487, 1438394610, 2161952759, 4100910556, 1608314830, 349175],
];

/// Large powers (&[u32]) for base7 operations.
const POW7: [&'static [u32]; 9] = [
    &[7],
    &[49],
    &[2401],
    &[5764801],
    &[2768600449, 7737],
    &[1855011585, 809251672, 59871144],
    &[3233510913, 429135816, 2162242590, 3594221799, 4265959880, 834593],
    &[467332097, 140397015, 2040215064, 1542555902, 642371654, 3366106563, 831699702, 4090248544, 1331205625, 2632189658, 762431610, 162],
    &[21354497, 135586989, 282407512, 217755548, 3801957565, 1769047343, 3062175267, 1076844155, 1124873580, 2147641037, 1086167164, 3185325477, 3247742386, 2077448856, 1351003107, 2494468834, 1425599169, 1702421937, 627210051, 1827625163, 1157572577, 2350050876, 26301],
];

/// Large powers (&[u32]) for base11 operations.
const POW11: [&'static [u32]; 9] = [
    &[11],
    &[121],
    &[14641],
    &[214358881],
    &[772479681, 10698505],
    &[2875311489, 3456163242, 1429612321, 26649],
    &[2585905921, 2073480325, 1636806860, 3370723472, 3947938656, 3233553771, 710186941],
    &[725390849, 983511997, 1546466724, 1452173552, 4110885543, 3594965768, 3109748563, 3754293795, 4268788147, 2994526333, 3490277564, 3382469739, 833985310, 117431742],
    &[4267518977, 3418704948, 4145983365, 2633889206, 1401015784, 2168955970, 4023491585, 2524575715, 1049283211, 445723862, 2434794608, 580179552, 2985579862, 2834866824, 480627468, 2191904507, 996937640, 2935962584, 1843145488, 1510931281, 327439189, 27267795, 129172641, 339183348, 3871693391, 3654642246, 1800239665, 3210784],
];

/// Large powers (&[u32]) for base13 operations.
const POW13: [&'static [u32]; 9] = [
    &[13],
    &[169],
    &[28561],
    &[815730721],
    &[1778525249, 154929377],
    &[1780897921, 912625740, 57455785, 5588660],
    &[954528001, 2552460193, 811089956, 1065775915, 3255164516, 1007972746, 118568612, 7272],
    &[2628444673, 1667825520, 3480482745, 1792807966, 2982160738, 2216443239, 1414760918, 1741036037, 3705043424, 136135250, 395110073, 1084641480, 2452973265, 2183283898, 52882385],
    &[685360129, 1249145954, 645630652, 1995019793, 3253088488, 1804068787, 392320294, 1343267562, 507087513, 2330257028, 3658736453, 3072903141, 2377702882, 4240373633, 2610933614, 2563321149, 1240024358, 1174015769, 312909694, 3764772638, 510500093, 1167371064, 563930725, 3086997765, 1545500121, 2429233725, 68804066, 1863990200, 3296313385, 651121],
];

/// Large powers (&[u32]) for base17 operations.
const POW17: [&'static [u32]; 9] = [
    &[17],
    &[289],
    &[83521],
    &[2680790145, 1],
    &[1614772481, 2739882033, 2],
    &[2276454913, 2771029215, 73658097, 4117442370, 6],
    &[3492013057, 3480731623, 2144081192, 1009276412, 4254173166, 4151923560, 452949210, 1816956013, 48],
    &[3277309953, 1864847191, 707834086, 1365492312, 4096642650, 2628241885, 1040897072, 3647618881, 4287772355, 126927969, 3695747218, 1041208948, 524092261, 1484945811, 3697469546, 3397736009, 2344],
    &[317689857, 1450665849, 1775742626, 792817246, 735751321, 1553574078, 160284371, 1873256746, 2085456896, 2906891001, 2367433931, 2570134778, 3520334249, 99950267, 475720041, 4178498415, 925636968, 3880190760, 4090350540, 1913195868, 737980718, 2149816287, 3313879437, 1462712522, 3066369386, 2488096894, 2055534827, 1929403434, 1042156826, 164256614, 685827684, 1240652339, 5498045],
];

/// Large powers (&[u32]) for base19 operations.
const POW19: [&'static [u32]; 8] = [
    &[19],
    &[361],
    &[130321],
    &[4098661153, 3],
    &[1637415489, 2733490537, 15],
    &[1674773633, 2120389729, 2125575713, 2140041202, 244],
    &[851069185, 953120046, 3422911417, 3249671984, 1253122465, 387558350, 2121285446, 1729366164, 59779],
    &[2386924033, 190345838, 2985903449, 2575403751, 3663280926, 3225853746, 4150572491, 603254694, 1674172707, 3673007190, 1176051141, 3802513312, 1962388533, 482203478, 2403994182, 530593434, 3573576981],
];

/// Large powers (&[u32]) for base23 operations.
const POW23: [&'static [u32]; 8] = [
    &[23],
    &[529],
    &[279841],
    &[1001573953, 18],
    &[1854975105, 1930488089, 332],
    &[1633523969, 2552572211, 2550303091, 2811546753, 110522],
    &[1210855937, 1436506257, 788091685, 4255817435, 2500410572, 2361723765, 2618086504, 4203283319, 3625322590, 2],
    &[3390858241, 538851, 3291369078, 492442738, 2850638734, 3029803664, 2378638741, 755966606, 185069185, 2748924587, 3547456468, 2115104919, 3398114034, 3022323801, 3693445824, 1646113784, 2709808682, 381505921, 8],
];

/// Large powers (&[u32]) for base29 operations.
const POW29: [&'static [u32]; 8] = [
    &[29],
    &[841],
    &[707281],
    &[2030206625, 116],
    &[1797710145, 3816168866, 13565],
    &[4219425409, 2265142903, 4119564573, 1570454940, 184033331],
    &[3969180929, 1340492553, 1739183727, 105569977, 1767257366, 1739519195, 3874617455, 3771111571, 1793220428, 7885570],
    &[1845676545, 3608513765, 1294341313, 344489966, 566774291, 427680924, 163628113, 422450508, 3222465417, 3727450692, 2861254789, 883344622, 1689193932, 1672377999, 1874044950, 3963222593, 1712211922, 4194848827, 3979265421, 14477],
];

/// Large powers (&[u32]) for base31 operations.
const POW31: [&'static [u32]; 8] = [
    &[31],
    &[961],
    &[923521],
    &[2487512833, 198],
    &[1353309697, 2948261936, 39433],
    &[2111290369, 854830039, 1377820608, 3005187674, 1555015626],
    &[1304393729, 558465812, 2768012290, 3935646446, 3376640041, 1419477874, 1557778152, 3873581017, 2245632988, 563001632],
    &[3820941313, 3840574348, 1970562388, 1279978999, 91057081, 172159608, 7026786, 3609778129, 320477644, 2720004857, 2683868502, 3872769187, 1705702550, 1250974998, 1821061582, 2142610495, 2457779444, 131107681, 1215733575, 73800524],
];

// HELPER

/// Get the correct large power from the base.
#[allow(dead_code)]
pub(in atof::algorithm) fn get_large_powers(base: u32)
    -> &'static [&'static [u32]]
{
    match base {
        3  => &POW3,
        5  => &POW5,
        7  => &POW7,
        11  => &POW11,
        13  => &POW13,
        17  => &POW17,
        19  => &POW19,
        23  => &POW23,
        29  => &POW29,
        31  => &POW31,
        _  => unreachable!(),
    }
}
