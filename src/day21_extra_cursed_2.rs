pub fn part2_single_lookup(num: usize) -> u32 {
    const TABLE2: [u32; 1000] = {
        let mut c_table = [0u32; 1000];
        c_table[0] = 0;
        c_table[1] = 66;
        c_table[2] = 104;
        c_table[3] = 162;
        c_table[4] = 272;
        c_table[5] = 270;
        c_table[6] = 336;
        c_table[7] = 497;
        c_table[8] = 424;
        c_table[9] = 531;
        c_table[10] = 740;
        c_table[11] = 726;
        c_table[12] = 888;
        c_table[13] = 910;
        c_table[14] = 1092;
        c_table[15] = 1260;
        c_table[16] = 1280;
        c_table[17] = 1360;
        c_table[18] = 1476;
        c_table[19] = 1558;
        c_table[20] = 1120;
        c_table[21] = 1470;
        c_table[22] = 1144;
        c_table[23] = 1288;
        c_table[24] = 1872;
        c_table[25] = 1600;
        c_table[26] = 1716;
        c_table[27] = 2160;
        c_table[28] = 1736;
        c_table[29] = 1972;
        c_table[30] = 2160;
        c_table[31] = 2418;
        c_table[32] = 2432;
        c_table[33] = 1782;
        c_table[34] = 2788;
        c_table[35] = 2940;
        c_table[36] = 2376;
        c_table[37] = 3108;
        c_table[38] = 3116;
        c_table[39] = 2652;
        c_table[40] = 3040;
        c_table[41] = 3362;
        c_table[42] = 3612;
        c_table[43] = 3526;
        c_table[44] = 2992;
        c_table[45] = 3420;
        c_table[46] = 3312;
        c_table[47] = 3760;
        c_table[48] = 3936;
        c_table[49] = 4018;
        c_table[50] = 2900;
        c_table[51] = 3978;
        c_table[52] = 3536;
        c_table[53] = 3604;
        c_table[54] = 3888;
        c_table[55] = 2970;
        c_table[56] = 3248;
        c_table[57] = 4560;
        c_table[58] = 3596;
        c_table[59] = 4012;
        c_table[60] = 4440;
        c_table[61] = 5246;
        c_table[62] = 5208;
        c_table[63] = 4410;
        c_table[64] = 5120;
        c_table[65] = 5070;
        c_table[66] = 3696;
        c_table[67] = 5896;
        c_table[68] = 5576;
        c_table[69] = 4692;
        c_table[70] = 5530;
        c_table[71] = 6035;
        c_table[72] = 6408;
        c_table[73] = 6205;
        c_table[74] = 6290;
        c_table[75] = 6450;
        c_table[76] = 6460;
        c_table[77] = 5467;
        c_table[78] = 5850;
        c_table[79] = 5925;
        c_table[80] = 4880;
        c_table[81] = 6561;
        c_table[82] = 5822;
        c_table[83] = 5893;
        c_table[84] = 6720;
        c_table[85] = 6035;
        c_table[86] = 6106;
        c_table[87] = 6525;
        c_table[88] = 4664;
        c_table[89] = 5429;
        c_table[90] = 6930;
        c_table[91] = 8099;
        c_table[92] = 8004;
        c_table[93] = 6789;
        c_table[94] = 8366;
        c_table[95] = 8265;
        c_table[96] = 7008;
        c_table[97] = 8051;
        c_table[98] = 7546;
        c_table[99] = 5841;
        c_table[100] = 5800;
        c_table[101] = 9494;
        c_table[102] = 8160;
        c_table[103] = 8446;
        c_table[104] = 9984;
        c_table[105] = 8610;
        c_table[106] = 8904;
        c_table[107] = 10593;
        c_table[108] = 8748;
        c_table[109] = 9483;
        c_table[110] = 6380;
        c_table[111] = 5550;
        c_table[112] = 6496;
        c_table[113] = 6102;
        c_table[114] = 7068;
        c_table[115] = 7820;
        c_table[116] = 7424;
        c_table[117] = 7488;
        c_table[118] = 7788;
        c_table[119] = 7854;
        c_table[120] = 7440;
        c_table[121] = 9196;
        c_table[122] = 7076;
        c_table[123] = 7626;
        c_table[124] = 10416;
        c_table[125] = 8750;
        c_table[126] = 9072;
        c_table[127] = 10922;
        c_table[128] = 8704;
        c_table[129] = 9546;
        c_table[130] = 9360;
        c_table[131] = 10218;
        c_table[132] = 10032;
        c_table[133] = 7182;
        c_table[134] = 10988;
        c_table[135] = 11340;
        c_table[136] = 8976;
        c_table[137] = 11508;
        c_table[138] = 11316;
        c_table[139] = 9452;
        c_table[140] = 9800;
        c_table[141] = 10716;
        c_table[142] = 11360;
        c_table[143] = 10868;
        c_table[144] = 8928;
        c_table[145] = 10150;
        c_table[146] = 9636;
        c_table[147] = 10878;
        c_table[148] = 11248;
        c_table[149] = 11324;
        c_table[150] = 10800;
        c_table[151] = 13892;
        c_table[152] = 12464;
        c_table[153] = 12546;
        c_table[154] = 13244;
        c_table[155] = 10540;
        c_table[156] = 11232;
        c_table[157] = 14758;
        c_table[158] = 12008;
        c_table[159] = 13038;
        c_table[160] = 13120;
        c_table[161] = 15134;
        c_table[162] = 14904;
        c_table[163] = 12714;
        c_table[164] = 14432;
        c_table[165] = 14190;
        c_table[166] = 10624;
        c_table[167] = 16032;
        c_table[168] = 15120;
        c_table[169] = 12844;
        c_table[170] = 12240;
        c_table[171] = 13338;
        c_table[172] = 14104;
        c_table[173] = 13494;
        c_table[174] = 13572;
        c_table[175] = 13825;
        c_table[176] = 13728;
        c_table[177] = 11328;
        c_table[178] = 12104;
        c_table[179] = 12172;
        c_table[180] = 13320;
        c_table[181] = 17014;
        c_table[182] = 15288;
        c_table[183] = 15372;
        c_table[184] = 17112;
        c_table[185] = 15540;
        c_table[186] = 15624;
        c_table[187] = 16456;
        c_table[188] = 12408;
        c_table[189] = 13986;
        c_table[190] = 15960;
        c_table[191] = 18336;
        c_table[192] = 18048;
        c_table[193] = 15440;
        c_table[194] = 18624;
        c_table[195] = 18330;
        c_table[196] = 15680;
        c_table[197] = 17730;
        c_table[198] = 16632;
        c_table[199] = 13134;
        c_table[200] = 10400;
        c_table[201] = 17688;
        c_table[202] = 14948;
        c_table[203] = 15428;
        c_table[204] = 18360;
        c_table[205] = 15580;
        c_table[206] = 16068;
        c_table[207] = 19251;
        c_table[208] = 15600;
        c_table[209] = 16929;
        c_table[210] = 15540;
        c_table[211] = 13926;
        c_table[212] = 15688;
        c_table[213] = 14910;
        c_table[214] = 16692;
        c_table[215] = 18060;
        c_table[216] = 17280;
        c_table[217] = 17360;
        c_table[218] = 17876;
        c_table[219] = 17958;
        c_table[220] = 11440;
        c_table[221] = 14586;
        c_table[222] = 10656;
        c_table[223] = 11596;
        c_table[224] = 16576;
        c_table[225] = 13500;
        c_table[226] = 14012;
        c_table[227] = 17252;
        c_table[228] = 13224;
        c_table[229] = 14656;
        c_table[230] = 16100;
        c_table[231] = 17556;
        c_table[232] = 17168;
        c_table[233] = 12116;
        c_table[234] = 18720;
        c_table[235] = 19270;
        c_table[236] = 15104;
        c_table[237] = 19434;
        c_table[238] = 19040;
        c_table[239] = 15774;
        c_table[240] = 19680;
        c_table[241] = 21208;
        c_table[242] = 22264;
        c_table[243] = 21384;
        c_table[244] = 18056;
        c_table[245] = 20090;
        c_table[246] = 19188;
        c_table[247] = 21242;
        c_table[248] = 21824;
        c_table[249] = 21912;
        c_table[250] = 16000;
        c_table[251] = 21084;
        c_table[252] = 18648;
        c_table[253] = 18722;
        c_table[254] = 19812;
        c_table[255] = 15300;
        c_table[256] = 16384;
        c_table[257] = 22102;
        c_table[258] = 17544;
        c_table[259] = 19166;
        c_table[260] = 20800;
        c_table[261] = 24012;
        c_table[262] = 23580;
        c_table[263] = 19988;
        c_table[264] = 22704;
        c_table[265] = 22260;
        c_table[266] = 16492;
        c_table[267] = 25098;
        c_table[268] = 23584;
        c_table[269] = 19906;
        c_table[270] = 22680;
        c_table[271] = 24390;
        c_table[272] = 25568;
        c_table[273] = 24570;
        c_table[274] = 24660;
        c_table[275] = 25025;
        c_table[276] = 24840;
        c_table[277] = 21052;
        c_table[278] = 22240;
        c_table[279] = 22320;
        c_table[280] = 18480;
        c_table[281] = 24166;
        c_table[282] = 21432;
        c_table[283] = 21508;
        c_table[284] = 24140;
        c_table[285] = 21660;
        c_table[286] = 21736;
        c_table[287] = 22960;
        c_table[288] = 16704;
        c_table[289] = 19074;
        c_table[290] = 23780;
        c_table[291] = 27354;
        c_table[292] = 26864;
        c_table[293] = 22854;
        c_table[294] = 27636;
        c_table[295] = 27140;
        c_table[296] = 23088;
        c_table[297] = 26136;
        c_table[298] = 24436;
        c_table[299] = 19136;
        c_table[300] = 14400;
        c_table[301] = 25284;
        c_table[302] = 21140;
        c_table[303] = 21816;
        c_table[304] = 26144;
        c_table[305] = 21960;
        c_table[306] = 22644;
        c_table[307] = 27323;
        c_table[308] = 21868;
        c_table[309] = 23793;
        c_table[310] = 19220;
        c_table[311] = 16794;
        c_table[312] = 19344;
        c_table[313] = 18154;
        c_table[314] = 20724;
        c_table[315] = 22680;
        c_table[316] = 21488;
        c_table[317] = 21556;
        c_table[318] = 22260;
        c_table[319] = 22330;
        c_table[320] = 17920;
        c_table[321] = 22470;
        c_table[322] = 16744;
        c_table[323] = 18088;
        c_table[324] = 25272;
        c_table[325] = 20800;
        c_table[326] = 21516;
        c_table[327] = 26160;
        c_table[328] = 20336;
        c_table[329] = 22372;
        c_table[330] = 15840;
        c_table[331] = 17874;
        c_table[332] = 17264;
        c_table[333] = 9990;
        c_table[334] = 19372;
        c_table[335] = 20100;
        c_table[336] = 14112;
        c_table[337] = 20220;
        c_table[338] = 19604;
        c_table[339] = 14916;
        c_table[340] = 22440;
        c_table[341] = 24552;
        c_table[342] = 25992;
        c_table[343] = 24696;
        c_table[344] = 19952;
        c_table[345] = 22770;
        c_table[346] = 21452;
        c_table[347] = 24290;
        c_table[348] = 25056;
        c_table[349] = 25128;
        c_table[350] = 22400;
        c_table[351] = 29484;
        c_table[352] = 26048;
        c_table[353] = 26122;
        c_table[354] = 27612;
        c_table[355] = 21300;
        c_table[356] = 22784;
        c_table[357] = 30702;
        c_table[358] = 24344;
        c_table[359] = 26566;
        c_table[360] = 21600;
        c_table[361] = 25992;
        c_table[362] = 25340;
        c_table[363] = 20328;
        c_table[364] = 24024;
        c_table[365] = 23360;
        c_table[366] = 15372;
        c_table[367] = 27158;
        c_table[368] = 25024;
        c_table[369] = 19926;
        c_table[370] = 25160;
        c_table[371] = 27454;
        c_table[372] = 29016;
        c_table[373] = 27602;
        c_table[374] = 27676;
        c_table[375] = 28125;
        c_table[376] = 27824;
        c_table[377] = 22620;
        c_table[378] = 24192;
        c_table[379] = 24256;
        c_table[380] = 25080;
        c_table[381] = 32766;
        c_table[382] = 29032;
        c_table[383] = 29108;
        c_table[384] = 32640;
        c_table[385] = 29260;
        c_table[386] = 29336;
        c_table[387] = 30960;
        c_table[388] = 22504;
        c_table[389] = 25674;
        c_table[390] = 24180;
        c_table[391] = 28934;
        c_table[392] = 28224;
        c_table[393] = 22794;
        c_table[394] = 29156;
        c_table[395] = 28440;
        c_table[396] = 22968;
        c_table[397] = 26996;
        c_table[398] = 24676;
        c_table[399] = 17556;
        c_table[400] = 24000;
        c_table[401] = 38496;
        c_table[402] = 32964;
        c_table[403] = 33852;
        c_table[404] = 39592;
        c_table[405] = 34020;
        c_table[406] = 34916;
        c_table[407] = 41107;
        c_table[408] = 33864;
        c_table[409] = 36401;
        c_table[410] = 30340;
        c_table[411] = 27126;
        c_table[412] = 30488;
        c_table[413] = 28910;
        c_table[414] = 32292;
        c_table[415] = 34860;
        c_table[416] = 33280;
        c_table[417] = 33360;
        c_table[418] = 34276;
        c_table[419] = 34358;
        c_table[420] = 31080;
        c_table[421] = 37048;
        c_table[422] = 29540;
        c_table[423] = 31302;
        c_table[424] = 40704;
        c_table[425] = 34850;
        c_table[426] = 35784;
        c_table[427] = 41846;
        c_table[428] = 34240;
        c_table[429] = 36894;
        c_table[430] = 36120;
        c_table[431] = 38790;
        c_table[432] = 38016;
        c_table[433] = 28578;
        c_table[434] = 40796;
        c_table[435] = 41760;
        c_table[436] = 34008;
        c_table[437] = 41952;
        c_table[438] = 41172;
        c_table[439] = 35120;
        c_table[440] = 26400;
        c_table[441] = 29106;
        c_table[442] = 30940;
        c_table[443] = 29238;
        c_table[444] = 23088;
        c_table[445] = 26700;
        c_table[446] = 24976;
        c_table[447] = 28608;
        c_table[448] = 29568;
        c_table[449] = 29634;
        c_table[450] = 28800;
        c_table[451] = 37884;
        c_table[452] = 33448;
        c_table[453] = 33522;
        c_table[454] = 35412;
        c_table[455] = 27300;
        c_table[456] = 29184;
        c_table[457] = 39302;
        c_table[458] = 31144;
        c_table[459] = 33966;
        c_table[460] = 34040;
        c_table[461] = 39646;
        c_table[462] = 38808;
        c_table[463] = 32410;
        c_table[464] = 37120;
        c_table[465] = 36270;
        c_table[466] = 26096;
        c_table[467] = 41096;
        c_table[468] = 38376;
        c_table[469] = 31892;
        c_table[470] = 33840;
        c_table[471] = 36738;
        c_table[472] = 38704;
        c_table[473] = 36894;
        c_table[474] = 36972;
        c_table[475] = 37525;
        c_table[476] = 37128;
        c_table[477] = 30528;
        c_table[478] = 32504;
        c_table[479] = 32572;
        c_table[480] = 35520;
        c_table[481] = 45214;
        c_table[482] = 40488;
        c_table[483] = 40572;
        c_table[484] = 45012;
        c_table[485] = 40740;
        c_table[486] = 40824;
        c_table[487] = 42856;
        c_table[488] = 32208;
        c_table[489] = 36186;
        c_table[490] = 41160;
        c_table[491] = 47136;
        c_table[492] = 46248;
        c_table[493] = 39440;
        c_table[494] = 47424;
        c_table[495] = 46530;
        c_table[496] = 39680;
        c_table[497] = 44730;
        c_table[498] = 41832;
        c_table[499] = 32934;
        c_table[500] = 25000;
        c_table[501] = 43086;
        c_table[502] = 36144;
        c_table[503] = 37222;
        c_table[504] = 44352;
        c_table[505] = 37370;
        c_table[506] = 38456;
        c_table[507] = 46137;
        c_table[508] = 37084;
        c_table[509] = 40211;
        c_table[510] = 39780;
        c_table[511] = 35770;
        c_table[512] = 39936;
        c_table[513] = 37962;
        c_table[514] = 42148;
        c_table[515] = 45320;
        c_table[516] = 43344;
        c_table[517] = 43428;
        c_table[518] = 44548;
        c_table[519] = 44634;
        c_table[520] = 33280;
        c_table[521] = 40638;
        c_table[522] = 31320;
        c_table[523] = 33472;
        c_table[524] = 45064;
        c_table[525] = 37800;
        c_table[526] = 38924;
        c_table[527] = 46376;
        c_table[528] = 36960;
        c_table[529] = 40204;
        c_table[530] = 41340;
        c_table[531] = 44604;
        c_table[532] = 43624;
        c_table[533] = 31980;
        c_table[534] = 46992;
        c_table[535] = 48150;
        c_table[536] = 38592;
        c_table[537] = 48330;
        c_table[538] = 47344;
        c_table[539] = 39886;
        c_table[540] = 38880;
        c_table[541] = 42198;
        c_table[542] = 44444;
        c_table[543] = 42354;
        c_table[544] = 34816;
        c_table[545] = 39240;
        c_table[546] = 37128;
        c_table[547] = 41572;
        c_table[548] = 42744;
        c_table[549] = 42822;
        c_table[550] = 27500;
        c_table[551] = 38570;
        c_table[552] = 33120;
        c_table[553] = 33180;
        c_table[554] = 35456;
        c_table[555] = 25530;
        c_table[556] = 27800;
        c_table[557] = 40104;
        c_table[558] = 30132;
        c_table[559] = 33540;
        c_table[560] = 38080;
        c_table[561] = 44880;
        c_table[562] = 43836;
        c_table[563] = 36032;
        c_table[564] = 41736;
        c_table[565] = 40680;
        c_table[566] = 28300;
        c_table[567] = 46494;
        c_table[568] = 43168;
        c_table[569] = 35278;
        c_table[570] = 45600;
        c_table[571] = 49106;
        c_table[572] = 51480;
        c_table[573] = 49278;
        c_table[574] = 49364;
        c_table[575] = 50025;
        c_table[576] = 49536;
        c_table[577] = 41544;
        c_table[578] = 43928;
        c_table[579] = 44004;
        c_table[580] = 35960;
        c_table[581] = 47642;
        c_table[582] = 41904;
        c_table[583] = 41976;
        c_table[584] = 47304;
        c_table[585] = 42120;
        c_table[586] = 42192;
        c_table[587] = 44612;
        c_table[588] = 31752;
        c_table[589] = 36518;
        c_table[590] = 46020;
        c_table[591] = 53190;
        c_table[592] = 52096;
        c_table[593] = 43882;
        c_table[594] = 53460;
        c_table[595] = 52360;
        c_table[596] = 44104;
        c_table[597] = 50148;
        c_table[598] = 46644;
        c_table[599] = 35940;
        c_table[600] = 30000;
        c_table[601] = 51686;
        c_table[602] = 43344;
        c_table[603] = 44622;
        c_table[604] = 53152;
        c_table[605] = 44770;
        c_table[606] = 46056;
        c_table[607] = 55237;
        c_table[608] = 44384;
        c_table[609] = 48111;
        c_table[610] = 42700;
        c_table[611] = 37882;
        c_table[612] = 42840;
        c_table[613] = 40458;
        c_table[614] = 45436;
        c_table[615] = 49200;
        c_table[616] = 46816;
        c_table[617] = 46892;
        c_table[618] = 48204;
        c_table[619] = 48282;
        c_table[620] = 39680;
        c_table[621] = 48438;
        c_table[622] = 37320;
        c_table[623] = 39872;
        c_table[624] = 53664;
        c_table[625] = 45000;
        c_table[626] = 46324;
        c_table[627] = 55176;
        c_table[628] = 43960;
        c_table[629] = 47804;
        c_table[630] = 40320;
        c_table[631] = 44170;
        c_table[632] = 42976;
        c_table[633] = 29118;
        c_table[634] = 46916;
        c_table[635] = 48260;
        c_table[636] = 36888;
        c_table[637] = 48412;
        c_table[638] = 47212;
        c_table[639] = 38340;
        c_table[640] = 40960;
        c_table[641] = 44870;
        c_table[642] = 47508;
        c_table[643] = 45010;
        c_table[644] = 36064;
        c_table[645] = 41280;
        c_table[646] = 38760;
        c_table[647] = 43996;
        c_table[648] = 45360;
        c_table[649] = 45430;
        c_table[650] = 37700;
        c_table[651] = 50778;
        c_table[652] = 44336;
        c_table[653] = 44404;
        c_table[654] = 47088;
        c_table[655] = 35370;
        c_table[656] = 38048;
        c_table[657] = 52560;
        c_table[658] = 40796;
        c_table[659] = 44812;
        c_table[660] = 33000;
        c_table[661] = 40982;
        c_table[662] = 39720;
        c_table[663] = 30498;
        c_table[664] = 37184;
        c_table[665] = 35910;
        c_table[666] = 21312;
        c_table[667] = 42688;
        c_table[668] = 38744;
        c_table[669] = 29436;
        c_table[670] = 48240;
        c_table[671] = 52338;
        c_table[672] = 55104;
        c_table[673] = 52494;
        c_table[674] = 52572;
        c_table[675] = 53325;
        c_table[676] = 52728;
        c_table[677] = 43328;
        c_table[678] = 46104;
        c_table[679] = 46172;
        c_table[680] = 44880;
        c_table[681] = 58566;
        c_table[682] = 51832;
        c_table[683] = 51908;
        c_table[684] = 58140;
        c_table[685] = 52060;
        c_table[686] = 52136;
        c_table[687] = 54960;
        c_table[688] = 39904;
        c_table[689] = 45474;
        c_table[690] = 42780;
        c_table[691] = 51134;
        c_table[692] = 49824;
        c_table[693] = 40194;
        c_table[694] = 51356;
        c_table[695] = 50040;
        c_table[696] = 40368;
        c_table[697] = 47396;
        c_table[698] = 43276;
        c_table[699] = 30756;
        c_table[700] = 43400;
        c_table[701] = 68698;
        c_table[702] = 58968;
        c_table[703] = 60458;
        c_table[704] = 70400;
        c_table[705] = 60630;
        c_table[706] = 62128;
        c_table[707] = 72821;
        c_table[708] = 60180;
        c_table[709] = 64519;
        c_table[710] = 53960;
        c_table[711] = 48348;
        c_table[712] = 54112;
        c_table[713] = 51336;
        c_table[714] = 57120;
        c_table[715] = 61490;
        c_table[716] = 58712;
        c_table[717] = 58794;
        c_table[718] = 60312;
        c_table[719] = 60396;
        c_table[720] = 54720;
        c_table[721] = 64890;
        c_table[722] = 51984;
        c_table[723] = 54948;
        c_table[724] = 70952;
        c_table[725] = 60900;
        c_table[726] = 62436;
        c_table[727] = 72700;
        c_table[728] = 59696;
        c_table[729] = 64152;
        c_table[730] = 62780;
        c_table[731] = 67252;
        c_table[732] = 65880;
        c_table[733] = 49844;
        c_table[734] = 70464;
        c_table[735] = 72030;
        c_table[736] = 58880;
        c_table[737] = 72226;
        c_table[738] = 70848;
        c_table[739] = 60598;
        c_table[740] = 56240;
        c_table[741] = 60762;
        c_table[742] = 63812;
        c_table[743] = 60926;
        c_table[744] = 50592;
        c_table[745] = 56620;
        c_table[746] = 53712;
        c_table[747] = 59760;
        c_table[748] = 61336;
        c_table[749] = 61418;
        c_table[750] = 54750;
        c_table[751] = 69843;
        c_table[752] = 62416;
        c_table[753] = 62499;
        c_table[754] = 65598;
        c_table[755] = 52095;
        c_table[756] = 55188;
        c_table[757] = 71915;
        c_table[758] = 58366;
        c_table[759] = 62997;
        c_table[760] = 65360;
        c_table[761] = 74578;
        c_table[762] = 73152;
        c_table[763] = 62566;
        c_table[764] = 70288;
        c_table[765] = 68850;
        c_table[766] = 52088;
        c_table[767] = 76700;
        c_table[768] = 72192;
        c_table[769] = 61520;
        c_table[770] = 47740;
        c_table[771] = 52428;
        c_table[772] = 55584;
        c_table[773] = 52564;
        c_table[774] = 52632;
        c_table[775] = 53475;
        c_table[776] = 52768;
        c_table[777] = 41958;
        c_table[778] = 45124;
        c_table[779] = 45182;
        c_table[780] = 51480;
        c_table[781] = 67166;
        c_table[782] = 59432;
        c_table[783] = 59508;
        c_table[784] = 66640;
        c_table[785] = 59660;
        c_table[786] = 59736;
        c_table[787] = 62960;
        c_table[788] = 45704;
        c_table[789] = 52074;
        c_table[790] = 60040;
        c_table[791] = 69608;
        c_table[792] = 68112;
        c_table[793] = 57096;
        c_table[794] = 69872;
        c_table[795] = 68370;
        c_table[796] = 57312;
        c_table[797] = 65354;
        c_table[798] = 60648;
        c_table[799] = 46342;
        c_table[800] = 44800;
        c_table[801] = 73692;
        c_table[802] = 62556;
        c_table[803] = 64240;
        c_table[804] = 75576;
        c_table[805] = 64400;
        c_table[806] = 66092;
        c_table[807] = 78279;
        c_table[808] = 63832;
        c_table[809] = 68765;
        c_table[810] = 68040;
        c_table[811] = 61636;
        c_table[812] = 68208;
        c_table[813] = 65040;
        c_table[814] = 71632;
        c_table[815] = 76610;
        c_table[816] = 73440;
        c_table[817] = 73530;
        c_table[818] = 75256;
        c_table[819] = 75348;
        c_table[820] = 57400;
        c_table[821] = 68964;
        c_table[822] = 54252;
        c_table[823] = 57610;
        c_table[824] = 75808;
        c_table[825] = 64350;
        c_table[826] = 66080;
        c_table[827] = 77738;
        c_table[828] = 62928;
        c_table[829] = 67978;
        c_table[830] = 69720;
        c_table[831] = 74790;
        c_table[832] = 73216;
        c_table[833] = 54978;
        c_table[834] = 78396;
        c_table[835] = 80160;
        c_table[836] = 65208;
        c_table[837] = 80352;
        c_table[838] = 78772;
        c_table[839] = 67120;
        c_table[840] = 69720;
        c_table[841] = 74849;
        c_table[842] = 78306;
        c_table[843] = 75027;
        c_table[844] = 63300;
        c_table[845] = 70135;
        c_table[846] = 66834;
        c_table[847] = 73689;
        c_table[848] = 75472;
        c_table[849] = 75561;
        c_table[850] = 59500;
        c_table[851] = 76590;
        c_table[852] = 68160;
        c_table[853] = 68240;
        c_table[854] = 71736;
        c_table[855] = 56430;
        c_table[856] = 59920;
        c_table[857] = 78844;
        c_table[858] = 63492;
        c_table[859] = 68720;
        c_table[860] = 72240;
        c_table[861] = 82656;
        c_table[862] = 81028;
        c_table[863] = 69040;
        c_table[864] = 77760;
        c_table[865] = 76120;
        c_table[866] = 57156;
        c_table[867] = 84966;
        c_table[868] = 79856;
        c_table[869] = 67782;
        c_table[870] = 67860;
        c_table[871] = 73164;
        c_table[872] = 76736;
        c_table[873] = 73332;
        c_table[874] = 73416;
        c_table[875] = 74375;
        c_table[876] = 73584;
        c_table[877] = 61390;
        c_table[878] = 64972;
        c_table[879] = 65046;
        c_table[880] = 49280;
        c_table[881] = 66956;
        c_table[882] = 58212;
        c_table[883] = 58278;
        c_table[884] = 66300;
        c_table[885] = 58410;
        c_table[886] = 58476;
        c_table[887] = 62090;
        c_table[888] = 42624;
        c_table[889] = 49784;
        c_table[890] = 65860;
        c_table[891] = 76626;
        c_table[892] = 74928;
        c_table[893] = 62510;
        c_table[894] = 76884;
        c_table[895] = 75180;
        c_table[896] = 62720;
        c_table[897] = 71760;
        c_table[898] = 66452;
        c_table[899] = 50344;
        c_table[900] = 46800;
        c_table[901] = 79288;
        c_table[902] = 66748;
        c_table[903] = 68628;
        c_table[904] = 81360;
        c_table[905] = 68780;
        c_table[906] = 70668;
        c_table[907] = 84351;
        c_table[908] = 68100;
        c_table[909] = 73629;
        c_table[910] = 65520;
        c_table[911] = 58304;
        c_table[912] = 65664;
        c_table[913] = 62084;
        c_table[914] = 69464;
        c_table[915] = 75030;
        c_table[916] = 71448;
        c_table[917] = 71526;
        c_table[918] = 73440;
        c_table[919] = 73520;
        c_table[920] = 60720;
        c_table[921] = 73680;
        c_table[922] = 57164;
        c_table[923] = 60918;
        c_table[924] = 81312;
        c_table[925] = 68450;
        c_table[926] = 70376;
        c_table[927] = 83430;
        c_table[928] = 66816;
        c_table[929] = 72462;
        c_table[930] = 61380;
        c_table[931] = 67032;
        c_table[932] = 65240;
        c_table[933] = 44784;
        c_table[934] = 70984;
        c_table[935] = 72930;
        c_table[936] = 56160;
        c_table[937] = 73086;
        c_table[938] = 71288;
        c_table[939] = 58218;
        c_table[940] = 67680;
        c_table[941] = 73398;
        c_table[942] = 77244;
        c_table[943] = 73554;
        c_table[944] = 60416;
        c_table[945] = 68040;
        c_table[946] = 64328;
        c_table[947] = 71972;
        c_table[948] = 73944;
        c_table[949] = 74022;
        c_table[950] = 62700;
        c_table[951] = 81786;
        c_table[952] = 72352;
        c_table[953] = 72428;
        c_table[954] = 76320;
        c_table[955] = 59210;
        c_table[956] = 63096;
        c_table[957] = 84216;
        c_table[958] = 67060;
        c_table[959] = 72884;
        c_table[960] = 63360;
        c_table[961] = 74958;
        c_table[962] = 73112;
        c_table[963] = 59706;
        c_table[964] = 69408;
        c_table[965] = 67550;
        c_table[966] = 46368;
        c_table[967] = 77360;
        c_table[968] = 71632;
        c_table[969] = 58140;
        c_table[970] = 64020;
        c_table[971] = 69912;
        c_table[972] = 73872;
        c_table[973] = 70056;
        c_table[974] = 70128;
        c_table[975] = 71175;
        c_table[976] = 70272;
        c_table[977] = 56666;
        c_table[978] = 60636;
        c_table[979] = 60698;
        c_table[980] = 58800;
        c_table[981] = 78480;
        c_table[982] = 68740;
        c_table[983] = 68810;
        c_table[984] = 77736;
        c_table[985] = 68950;
        c_table[986] = 69020;
        c_table[987] = 73038;
        c_table[988] = 51376;
        c_table[989] = 59340;
        c_table[990] = 51480;
        c_table[991] = 63424;
        c_table[992] = 61504;
        c_table[993] = 47664;
        c_table[994] = 63616;
        c_table[995] = 61690;
        c_table[996] = 47808;
        c_table[997] = 57826;
        c_table[998] = 51896;
        c_table[999] = 33966;
        c_table
    };
    TABLE2[num]
}
