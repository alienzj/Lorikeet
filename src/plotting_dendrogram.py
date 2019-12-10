import scipy.cluster.hierarchy as hierarchy
import numpy as np
import matplotlib.pyplot as plt

test = np.array([[38, 115, 0.00042393410852713254, 2],
[22, 131, 0.022143515079482978, 2],
[72, 90, 0.02631578947368421, 2],
[43, 74, 0.02884468288256925, 2],
[84, 133, 0.030588687956906502, 2],
[44, 85, 0.039210657828980255, 2],
[4, 88, 0.03987431063229446, 2],
[17, 57, 0.04036264282165919, 2],
[81, 138, 0.04312147200480149, 3],
[2, 26, 0.04590395480225992, 2],
[19, 111, 0.0459733893557423, 2],
[42, 127, 0.04663083918974782, 2],
[79, 83, 0.0504207650273224, 2],
[16, 92, 0.05252266062383365, 2],
[93, 118, 0.05390624999999999, 2],
[32, 56, 0.0544908635425877, 2],
[40, 119, 0.05603046690940244, 2],
[48, 146, 0.059663316257169044, 4],
[0, 132, 0.06137984496124027, 2],
[58, 155, 0.06145006122204153, 5],
[35, 112, 0.06175522062680305, 2],
[46, 54, 0.061881650848432584, 2],
[11, 121, 0.06285883836510646, 2],
[98, 135, 0.06571162342951647, 2],
[31, 53, 0.06608454275222941, 2],
[102, 139, 0.06655089452856462, 3],
[18, 157, 0.0691491597298422, 6],
[15, 164, 0.07163336891495263, 7],
[136, 144, 0.0786919394556925, 3],
[27, 69, 0.07930320150659137, 2],
[59, 106, 0.07961309523809523, 2],
[109, 134, 0.08585132575757576, 2],
[24, 67, 0.09309256844850067, 2],
[123, 147, 0.09533340725101337, 3],
[71, 103, 0.09839562509054034, 2],
[29, 150, 0.10439896830492934, 3],
[77, 126, 0.1066290510477634, 2],
[66, 116, 0.11260769324111153, 2],
[100, 165, 0.11318741110654386, 8],
[101, 176, 0.11582358131306417, 9],
[21, 113, 0.11975896828097676, 2],
[6, 171, 0.12123302796470267, 4],
[114, 125, 0.12138408999598232, 2],
[76, 95, 0.1236354385099297, 2],
[149, 152, 0.1277579028118155, 4],
[1, 128, 0.13088871912401323, 2],
[9, 148, 0.1316146229933582, 3],
[49, 63, 0.13872938249727373, 2],
[87, 130, 0.14090486807878116, 2],
[65, 158, 0.1410738366693498, 3],
[160, 177, 0.1413615531229218, 11],
[82, 117, 0.15202390745932876, 2],
[70, 163, 0.1587892832384251, 4],
[8, 110, 0.1691565959336442, 2],
[45, 47, 0.17499147194269143, 2],
[12, 41, 0.17562499999999998, 2],
[122, 162, 0.19652003811292795, 3],
[61, 159, 0.19828404341920258, 3],
[13, 175, 0.22208519142840719, 3],
[73, 188, 0.2286180250923905, 12],
[99, 180, 0.2338887198759468, 3],
[52, 120, 0.23908492281390584, 2],
[80, 166, 0.23991816879907277, 4],
[86, 142, 0.24224913539331822, 3],
[107, 185, 0.24617794974634577, 3],
[104, 154, 0.24838861385383018, 3],
[97, 187, 0.25854888820013455, 4],
[10, 23, 0.25949367088607594, 2],
[36, 60, 0.2706578699713712, 2],
[108, 174, 0.2850904510292617, 3],
[64, 129, 0.28646113412300467, 2],
[30, 169, 0.2867833033091165, 3],
[170, 173, 0.28848103634572053, 5],
[145, 191, 0.2891685982656345, 4],
[182, 189, 0.31017187516698236, 6],
[7, 14, 0.31311951691922063, 2],
[183, 186, 0.3191904752938033, 4],
[20, 39, 0.3286832670410311, 2],
[105, 151, 0.35826048221356227, 3],
[141, 195, 0.3700870655394531, 5],
[91, 193, 0.3746046124255774, 3],
[28, 167, 0.38791338625143507, 3],
[94, 200, 0.40044408092484196, 5],
[68, 209, 0.4008430979785149, 4],
[25, 198, 0.42151709648582536, 4],
[62, 96, 0.430073706951328, 2],
[37, 216, 0.43206062143384144, 4],
[137, 143, 0.4374180898355313, 3],
[75, 218, 0.4748449748540898, 4],
[172, 184, 0.4850828853052319, 5],
[5, 215, 0.4852616849424041, 3],
[124, 219, 0.49731897784388385, 4],
[34, 221, 0.5000216882923072, 5],
[168, 196, 0.5024575197527028, 5],
[33, 51, 0.5043182961596512, 2],
[3, 55, 0.5052618990672089, 2],
[50, 203, 0.5222041713278149, 4],
[223, 232, 0.5354477644960007, 4],
[156, 234, 0.5626729264343313, 6],
[89, 161, 0.5825633884450769, 3],
[206, 224, 0.5906328852492184, 6],
[208, 228, 0.5968404895883906, 5],
[207, 210, 0.6019929571300933, 8],
[202, 213, 0.604190666084453, 5],
[181, 233, 0.6190259474879044, 4],
[192, 236, 0.6318899619436354, 8],
[205, 238, 0.6503313956188055, 8],
[78, 235, 0.6714956009757598, 5],
[225, 229, 0.6718665119963749, 7],
[199, 237, 0.6990972510667962, 5],
[178, 241, 0.700669168798661, 7],
[140, 211, 0.7062816567843251, 6],
[214, 222, 0.7137726723173455, 8],
[153, 244, 0.7612544221403444, 10],
[201, 204, 0.7668883895562939, 7],
[226, 247, 0.7782937350866314, 9],
[194, 253, 0.8212396727195436, 12],
[220, 245, 0.8614023434197848, 10],
[242, 254, 0.8687287131673241, 16],
[246, 251, 0.8816898694187592, 17],
[230, 256, 0.883035274695416, 21],
[227, 249, 0.9169806650072405, 11],
[179, 258, 0.9832415326886207, 25],
[231, 239, 0.9837464379956913, 10],
[243, 257, 0.9886734570497856, 25],
[190, 252, 1.0396000324260999, 11],
[217, 260, 1.043373153846693, 30],
[248, 262, 1.0547898714015105, 32],
[259, 264, 1.0639190534519314, 41],
[263, 266, 1.090031685893147, 52],
[212, 255, 1.1077857638500568, 16],
[250, 267, 1.1338620387470342, 60],
[240, 268, 1.1645396957301377, 24],
[261, 265, 1.2127793473972952, 42],
[197, 270, 1.874354162639448, 36],
[269, 272, 2.505034812649333, 96],
[271, 273, 4.107016543362054, 138]])

test_dict = {5650: {"C": (2, 84)},
1192: {"A": (2, 53)},
1241: {"G": (2, 67)},
2784: {"G": (1, 20)},
1206: {"G": (1, 63)},
2773: {"G": (2, 123)},
4342: {"G": (2, 117)},
5479: {"C": (2, 29)},
2357: {"C": (2, 79)},
5509: {"A": (2, 38)},
4541: {"A": (3, 95), "TA": (5, 96)},
5387: {"A": (2, 118)},
3594: {"G": (2, 62)},
736: {"A": (2, 65)},
842: {"G": (2, 89)},
770: {"C": (2, 122)},
5159: {"T": (1, 135)},
789: {"CC": (2, 81)},
3085: {"C": (2, 78)},
5562: {"T": (2, 94)},
3161: {"C": (2, 41)},
480: {"G": (2, 40)},
4675: {"G": (2, 97)},
5508: {"A": (2, 72)},
3965: {"G": (1, 14)},
4685: {"G": (2, 35)},
2323: {"A": (2, 107)},
3953: {"C": (2, 131)},
4684: {"A": (4, 28)},
422: {"C": (2, 50)},
2523: {"A": (1, 134)},
725: {"A": (1, 127)},
2327: {"A": (2, 56)},
4588: {"G": (1, 23)},
2504: {"A": (2, 37)},
4692: {"C": (2, 27)},
4830: {"C": (1, 57)},
1233: {"G": (2, 49)},
5389: {"G": (2, 64)},
4801: {"C": (2, 126)},
3703: {"C": (2, 136)},
5604: {"A": (1, 76)},
4546: {"A": (3, 90)},
4552: {"T": (3, 106)},
4557: {"A": (3, 70)},
5575: {"G": (2, 130)},
2790: {"C": (2, 34)},
820: {"T": (2, 60)},
5626: {"T": (2, 73)},
4809: {"C": (2, 42)},
3707: {"G": (2, 101)},
4556: {"C": (3, 120)},
4603: {"G": (2, 39)},
5565: {"T": (3, 102)},
1173: {"C": (2, 103)},
3692: {"C": (1, 25)},
4782: {"C": (2, 44)},
3694: {"G": (2, 9)},
4543: {"G": (5, 87)},
4687: {"A": (2, 7)},
782: {"A": (2, 108)},
5500: {"T": (1, 68)},
799: {"T": (1, 8)},
5413: {"C": (2, 132)},
1188: {"T": (2, 26)},
4554: {"A": (3, 110)},
4764: {"A": (2, 91)},
3126: {"C": (2, 59)},
2527: {"CGTT": (2, 46)},
481: {"G": (2, 3)},
5435: {"T": (2, 6)},
4542: {"T": (3, 137)},
3600: {"A": (1, 43)},
4551: {"G": (3, 88)},
3595: {"A": (4, 99)},
4550: {"T": (3, 54)},
4812: {"A": (2, 98)},
4669: {"C": (2, 86)},
2428: {"C": (1, 24)},
804: {"C": (1, 13)},
4555: {"G": (3, 124)},
492: {"T": (2, 45)},
3713: {"T": (2, 58)},
4648: {"T": (2, 66)},
824: {"T": (2, 116)},
2360: {"C": (2, 32)},
792: {"C": (2, 69)},
5417: {"A": (1, 47)},
3087: {"G": (2, 85)},
3065: {"T": (2, 11)},
4545: {"G": (3, 16)},
2518: {"G": (2, 22)},
2555: {"T": (2, 128)},
3080: {"C": (2, 105)},
4638: {"C": (2, 17)},
5151: {"AATAT": (5, 75)},
1356: {"TN": (2, 109)},
5426: {"T": (2, 114)},
3114: {"A": (1, 12)},
2342: {"G": (2, 48)},
4606: {"C": (1, 112)},
463: {"G": (2, 1)},
2535: {"C": (2, 10)},
419: {"C": (1, 104)},
5415: {"C": (2, 31)},
5379: {"C": (2, 71)},
2497: {"C": (2, 5)},
4835: {"C": (1, 30)},
4562: {"G": (1, 0)},
4633: {"G": (2, 19)},
2502: {"G": (2, 51)},
858: {"T": (1, 133)},
4147: {"A": (2, 93)},
2525: {"C": (1, 33)},
2776: {"G": (1, 111)},
2892: {"A": (2, 2)},
3174: {"C": (2, 92)},
5598: {"G": (1, 119)},
2860: {"CTG": (1, 125)},
4320: {"A": (4, 129)},
4160: {"G": (2, 61)},
1191: {"G": (2, 115)},
3952: {"C": (1, 52)},
5529: {"G": (2, 121)},
4548: {"C": (3, 77)},
1230: {"T": (2, 83)},
5537: {"GCC": (1, 55)},
4742: {"A": (1, 100)},
3657: {"CC": (1, 4)},
438: {"A": (2, 18)},
2424: {"C": (2, 21)},
479: {"G": (1, 80)},
1165: {"C": (1, 82)},
5503: {"C": (1, 15)},
4752: {"G": (4, 113)},
4805: {"A": (2, 74)},
3665: {"AAG": (5, 36)}}


test_clust = {}

for (pos, var_map) in test_dict.items():
    for (var, tup) in var_map.items():
        test_clust[tup[1]] = (tup[0], pos)



plt.figure(figsize=[12,10])

# hierarchy.dendrogram(test, leaf_label_func=lambda id: test_clust[id], color_threshold=0.4*max(test[:,2]))
hierarchy.dendrogram(test, color_threshold=0.7*max(test[:,2]))

plt.axis('off')
plt.gca().set_position([0, 0, 1, 1])
plt.savefig("test_dendro.svg")
