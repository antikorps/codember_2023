use regex::Regex;

fn contar_coincidencias(caracter: String, cadena: String) -> i64 {
    let mut t = 0;
    let c = caracter.chars().nth(0).unwrap();
    for cc in cadena.chars() {
        if c == cc {
            t += 1
        }
    }
    return t;
}

fn cumple_condicion(min: i64, max: i64, coincidencias: i64) -> bool {
    if coincidencias >= min && coincidencias <= max {
        return true;
    }
    return false;
}

struct InformacionLinea {
    minimo: i64,
    maximo: i64,
    caracter: String,
    password: String,
}

fn capturar_grupos_linea(linea: &str, indice: i64) -> Result<InformacionLinea, String> {
    let exp_reg = Regex::new(r"^(\d*?)-(\d*?)\s([a-zA-Z]):\s([a-zA-Z]*)")
        .expect("La expresión regular ha fallado");
    let grupos_capturados;
    match exp_reg.captures(linea) {
        None => {
            let mensaje_error = format!(
                "ERROR: la expresión regular no ha capturado ningún grupo en {} (línea {})",
                linea, indice
            );
            return Err(mensaje_error);
        }
        Some(ok) => grupos_capturados = ok,
    }
    let total_grupos_capturados = grupos_capturados.iter().count();

    if grupos_capturados.iter().count() != 5 {
        let mensaje_error = format!(
            "ERROR: la expresión regular ha capturado {} grupos en vez de 5 en {} (línea {})",
            total_grupos_capturados, linea, indice
        );
        return Err(mensaje_error);
    }

    let minimo = grupos_capturados.get(1).unwrap().as_str();
    let min;
    match minimo.parse::<i64>() {
        Ok(ok) => min = ok,
        Err(_) => {
            let mensaje_error = format!(
                "ERROR: no ha podido parsearse como int64 el mínimo en {} (línea {})",
                linea, indice
            );
            return Err(mensaje_error);
        }
    }

    let maximo = grupos_capturados.get(2).unwrap().as_str();
    let max;
    match maximo.parse::<i64>() {
        Ok(ok) => max = ok,
        Err(_) => {
            let mensaje_error = format!(
                "ERROR: no ha podido parsearse como int64 el maximo en {} (línea {})",
                linea, indice
            );
            return Err(mensaje_error);
        }
    }
    let caracter = grupos_capturados.get(3).unwrap().as_str().to_string();
    let password = grupos_capturados.get(4).unwrap().as_str().to_string();

    return Ok(InformacionLinea {
        minimo: min,
        maximo: max,
        caracter,
        password,
    });
}

fn primera_solucion(entrada: &str) {
    let lineas = entrada.split("\n");
    let mut claves_invalidas = Vec::new();
    for (indice, linea) in lineas.enumerate() {
        let info_linea;
        match capturar_grupos_linea(linea, (indice + 1).try_into().unwrap()) {
            Err(error) => {
                println!("{error}");
                continue;
            }
            Ok(ok) => {
                info_linea = ok;
            }
        }

        let coincidencias = contar_coincidencias(info_linea.caracter, info_linea.password.clone());
        if cumple_condicion(info_linea.minimo, info_linea.maximo, coincidencias) {
            continue;
        }
        claves_invalidas.push(info_linea.password);
    }
    println!("{}", claves_invalidas[41])
}

fn recopilar_info_linea(linea: &str, indice: i64) -> Result<InformacionLinea, String> {
    let politica_clave: Vec<&str> = linea.split(": ").collect();
    if politica_clave.len() != 2 {
        let mensaje_error = format!("ERROR: formato de política y clave incorrecto en {} (línea {})",linea, indice);
        return Err(mensaje_error)
    }
    let password = politica_clave[1].to_string();
    let min_max_caracter: Vec<&str> = politica_clave[0].split(" ").collect();
    if min_max_caracter.len() != 2 {
        let mensaje_error = format!("ERROR: formato de mínimo, máximo y caracter incorrecto en {} (línea {})",linea, indice);
        return Err(mensaje_error)
    }
    let caracter = min_max_caracter[1].to_string();
    let min_max: Vec<&str> = min_max_caracter[0].split("-").collect();
    if min_max.len() != 2 {
        let mensaje_error = format!("ERROR: formato de mínimo y máximo incorrecto en {} (línea {})",linea, indice);
        return Err(mensaje_error)
    }
    let min;
    match min_max[0].parse::<i64>() {
        Ok(ok) => min = ok,
        Err(_) => {
            let mensaje_error = format!(
                "ERROR: no ha podido parsearse como int64 el mínimo en {} (línea {})",
                linea, indice
            );
            return Err(mensaje_error);
        }
    }

    let max;
    match min_max[1].parse::<i64>() {
        Ok(ok) => max = ok,
        Err(_) => {
            let mensaje_error = format!(
                "ERROR: no ha podido parsearse como int64 el maximo en {} (línea {})",
                linea, indice
            );
            return Err(mensaje_error);
        }
    }
    return Ok(InformacionLinea { minimo: min, maximo: max, caracter,  password })


}

fn segunda_opcion(entrada: &str) {
    let lineas = entrada.split("\n");
    let mut claves_invalidas = Vec::new();
    for (indice, linea) in lineas.enumerate() {
        let info_linea;
        match recopilar_info_linea(linea, (indice + 1).try_into().unwrap()) {
            Err(error) => {
                println!("{error}");
                continue;
            }
            Ok(ok) => {
                info_linea = ok;
            }
        }

        let coincidencias = contar_coincidencias(info_linea.caracter, info_linea.password.clone());
        if cumple_condicion(info_linea.minimo, info_linea.maximo, coincidencias) {
            continue;
        }
        claves_invalidas.push(info_linea.password);
    }
    println!("{}", claves_invalidas[41])
}

fn main() {
    let entrada = "8-10 r: ozrcdfnug
9-10 q: hvsazxrigf
1-9 j: bbqonxjt
6-8 e: pzqcywelwiogwt
4-7 t: vvxirpoid
10-10 g: gifzgmpab
7-8 w: lfpveulq
8-10 f: xlcgglmllky
7-9 w: siridrjzxqpngwr
2-3 n: fkrmnniuxeboq
8-10 g: zlbevnqppg
1-6 a: yqvwieerrklo
1-9 d: vxannmz
8-10 u: hhjyvkckfxo
7-10 i: nljzuyfzb
9-10 c: imgjjpzdbmrw
9-9 v: hysjskvlajhxpd
10-10 w: oojxss
1-10 s: yysatgxngpxwoem
9-9 i: uhwcmhcdby
3-6 x: tygtgmzqgaoa
6-9 q: ivrvhtzzer
6-10 g: fwgegzkdv
6-7 l: wcjluyprnxu
9-10 a: fyefdpsdzaebc
7-8 y: kwxuffrrneojaco
6-10 p: dwgackmgbsus
8-8 o: dtiyaqmqndsp
2-8 i: gmafxjksdtxfjt
2-4 l: xwjuygyfnqgsz
1-1 b: kodslr
9-9 g: pdnxrrfxhjhi
4-9 t: ohmczh
4-5 d: qeavdrtphhyqrvb
1-3 i: qnuel
7-10 l: wbwdnzvqku
10-10 n: jjwhjrrysbpg
9-10 b: kovvlhsvdqfg
1-1 g: xolpkt
5-8 g: qubspcaytneg
2-8 k: fmptvvd
10-10 x: hzlaflrshptiy
4-10 i: dywatt
6-6 x: eflrp
9-10 d: bgamidqewtbus
3-10 n: llysdv
2-3 k: bvcbvo
8-9 a: iyljc
10-10 f: ndyfogk
1-9 z: nylbijvgysm
7-8 i: bjqgqt
5-7 u: tdtcwitrr
8-9 x: spwuw
8-10 y: rpzcs
2-7 i: gcysmom
1-9 n: jymsnungqygcq
4-4 a: kcrohkxqg
10-10 t: mvebpf
4-7 r: vfhunjqziv
10-10 k: mtmdjbv
5-6 c: sowigndyyai
7-7 b: cifknttgrpn
3-4 q: zrubvrlegv
8-10 r: ivczqqrois
8-10 z: ykqosdbk
9-9 h: urjpxaz
1-8 w: pxhfxubuyvo
4-6 a: tumdapgxenvr
2-7 f: yaeyvbkrdras
5-8 s: paqxymwsbah
8-8 x: xtxpnhgbofdrs
7-10 r: iqfpszsx
8-10 c: cdfsfnyiqwtcssy
5-10 z: suvqelmvkmvcy
5-7 b: hiodfdwq
4-5 u: zqghtqmw
8-10 h: liqwd
1-7 o: qhrlwdoltv
3-7 w: tcqcq
6-6 o: bfjaddqxqgssq
4-7 j: gkbzxsffzrnckzj
4-6 d: ogvdasrmribaqps
9-10 m: jrgezkxytjuf
2-9 g: htyjxmzulz
3-5 b: gxgvzkkx
6-8 g: iwgomwhwpjspj
2-7 b: usdmm
4-9 z: bowpthcs
5-8 v: ahrljnttfukcrw
1-7 d: rnwrji
8-9 q: zwtylxkv
5-9 b: ivxeagzx
7-9 o: wjtncvtwwlpv
4-4 m: xbpafqjzwdepp
7-8 x: znlfxvtbraikxjf
5-9 j: hybkaj
5-6 g: xjducbyhdrst
2-3 e: rupyvlo
1-1 y: yrruzwhbfkk
5-10 c: dxkhzipe
7-10 z: outpx
6-10 t: nlygjceachhhqij
5-10 f: fpuvb
10-10 c: laftqdzlcao
4-7 s: xotywhvh
1-8 w: qapwcw
5-10 g: zoiosnanbuugsbr
3-4 k: zuotdwegpfaut
1-3 y: kjjkhsr
8-8 n: vwipg
5-7 i: ofowebiqsuec
4-9 a: ixyewqvinq
1-3 v: cmqrlibqkpe
2-10 l: gtwnznkpm
6-7 y: eqhasppna
2-5 s: nhwfgrtsuoz
2-6 z: eigodwfjbwmsf
7-10 b: kvlif
1-3 i: moiiqwwbh
3-3 f: wvgyabdhgbjgoj
5-8 x: lyvgrb
5-5 x: afwocoxvzujg
6-10 o: egmuvuyxc
5-5 o: qomkkznp
6-10 q: isbdoqwkuozz
4-4 w: gcvtasxufjbgh
10-10 y: mimjvbwedh
4-10 x: kmfwtqmg
5-7 v: fhqjmstxuq
3-6 l: ggzcxkrzc
1-5 x: ktnfvz
7-7 d: uzthjbbxb
8-8 k: rpcuickgpcqp
9-9 i: jhmxls
6-6 j: pdvdgvodybqoty
7-7 e: zsmlnjjppwenmfg
9-10 d: drtnbb
2-9 t: dtdjusghxfjnk
5-9 u: njmijoctlludoa
4-9 n: rrkellcjp
2-8 s: eebsu
6-6 y: yjiptrs
2-9 c: ubggdune
4-8 n: ddcljhnptqrl
5-9 j: qnunzylw
1-7 f: ycqtdtcafiulm
9-9 g: gntyxnlb
1-5 i: mrlqt
9-10 q: gnwgseil
10-10 y: zojixlcsjd
1-4 w: iabesorarfowunh
9-10 i: glsptiqjt
7-9 p: vuxjj
10-10 d: edzbjzrrtnn
7-7 m: iqxrctdn
8-8 a: yudznhiy
4-6 e: rtqkztjjkyt
6-7 h: tchcuyunzxngh
5-10 q: pwtosnyorwqtt
8-10 k: arjuubxwfhirj
4-7 l: ydsanrbcxxnws
9-9 p: upvxqygmz
2-6 i: zduzras
4-9 o: danygeslyzelv
8-8 w: kkisegumx
3-8 t: wyjobisjh
8-9 d: etdze
2-4 q: hztgwbtku
10-10 y: qigddpj
3-10 u: aekdbznktxhhctu
2-3 i: oacbzkc
1-8 h: azyhyxnj
8-10 g: dkwkplqdfhry
6-7 h: nkfrol
4-7 o: htcuracoi
4-7 c: qiapikaiji
1-3 d: wxrvykfewrcr
2-7 d: aiwyytzuuq
7-10 l: mayhendroaj
10-10 w: cztbyaaoubvtja
8-9 a: ghvjtd
10-10 q: rlsqnhyoqsqd
7-9 a: xnfkibaldshxws
4-4 s: ounylctxyd
5-6 n: kzhmoagidyle
6-10 s: mswyvytow
6-9 u: suwlhjcnkpgrtv
4-4 r: btwxrkimhnffwr
4-5 u: ernqjnscqvrjzbd
2-2 r: yhkdxjkuy
1-3 p: pfmduv
10-10 k: cmknadgfxgiiw
2-2 o: acqvlqcnqtsem
4-7 c: dbvvlsydhbepk
1-5 t: akysdiarek
1-4 q: nwovcsvdpuma
9-10 b: xlbmykuusdann
2-9 y: yuvyayectdxj
2-9 t: qvdnfcxdoqwxjw
6-9 t: icxpei
8-10 n: euynwapumiic
3-9 v: ilupkmmhzbjvhu
1-5 z: ymorcaxogkiim
3-10 d: tpzcheysueiqas
7-10 b: weauuhosrmgzvv
2-7 c: asyue
10-10 s: gufkolsevebvpru
9-9 t: ghgxzpollnee
8-10 p: rwmajwbootmj
1-7 r: vojubpncicmoyxq
3-8 g: zwtclkhux
6-8 r: ztlkfxoujd
10-10 t: mkxiyfjbqnlii
2-5 i: oxvalyfmpsahgf
9-9 j: xwbpwgzo
7-8 o: tizkcoxwpqyryf
1-3 w: qkmisjvfatywi
5-8 r: jozangypzlrf
8-8 c: zoeowdk
1-2 w: wdfflckcjnceb
10-10 b: cyhdd
5-7 d: amhepcrktiuvvk
1-9 p: xdkctmvtoa
6-6 p: hsydtlxjftab
3-9 x: vmfstlen
2-5 j: qefmk
10-10 w: firbfo
9-9 t: vtvorreuq
3-9 q: vbnvf
5-7 d: rzfdnmwlehnjxt
9-9 f: esloelgkzgihp
8-8 r: sbnhygrguo
6-9 d: llybjbajpibr
2-4 q: itikoiexmtxd
6-6 n: uxblhtl
9-9 y: obwyforcrabfuuy
8-10 x: tekkudropxxt
1-1 r: pzwws
9-9 h: udbzxiifn
4-8 r: guiclzojblg
2-8 s: zuhmhvlktvh
4-7 g: mjmflepbhrlj
10-10 n: ucdnanjjsftty
8-10 v: wvfqhpm
2-8 t: wzchieehwfgohcq
4-10 n: lsxfrgz
9-9 u: paotl
7-10 s: mcylon
3-10 f: kzqjrcljohyjat
8-9 e: yabwyivmlanv
9-9 a: kjwqzfohq
2-7 x: lqfxyzzzcivl
6-10 v: lmjypukcyulsst
2-10 z: ggelhwocnk
5-7 t: xenffooe
4-6 g: ixkls
10-10 n: ofyfipxn
6-6 s: fnnufwp
7-7 r: expdcfdauczler
9-9 g: uowurzo
4-6 v: iivuf
6-9 k: hnshun
7-8 h: zkjlyxqza
1-7 e: jupqyycbsfl
6-6 k: xstrunykiycl
1-10 b: rttuvbrairceca
7-7 p: xgpoomivc
3-5 i: suaoe
1-6 t: xiowndnccojbdxl
10-10 d: ctvlplseppjnjf
6-9 f: irsdx
2-6 a: fpmnrmubfrh
4-8 r: atybqpoeswbjdr
9-10 k: fhnbe
3-5 q: vodpuhs
3-6 h: lbmaiw
4-4 s: joicgouzefuzff
5-6 o: nkritzr
5-7 t: dczzuusprvhjwel
8-10 z: ngwapbjhghi
2-2 u: omwqjh
5-6 f: uhmzjatqg
5-10 p: edjqbociloc
8-9 u: zrataihprppked
4-6 z: aqwfrmvhtlpq
3-9 l: wplbshoy
1-9 e: wminrrpgfr
5-6 k: wdvqngtmmtbzxjk
2-7 c: ptharwgn
4-6 s: sytmluweomy
8-9 n: tteqngrfyfxhxj
8-8 u: vadfiv
1-9 u: qzrzg
5-5 j: dhrvz
8-9 p: swtedii
7-9 z: vigiuogz
4-4 h: mrzself
3-6 k: suxmhlbu
1-4 i: ywfufvc
8-9 j: tycjdb
3-4 e: zjnzeuxzanrbe
6-6 b: zsibwipjzyglzc
1-2 e: dhorfrnqu
2-4 t: dusjlqayivtqp
4-4 s: ijnemgdildkh
3-6 j: mfswx
10-10 c: izeaxcasxorpapq
3-3 c: uxrumikdtpptqu
2-10 o: kqqukevdapxxij
9-10 q: lwmzwfjxk
9-9 q: gwodurvzho
5-8 r: xhudg
6-7 g: jydiibn
4-4 n: dxjrr
8-8 q: uusvflpccyigs
2-8 b: mfehit
6-6 f: mvjifcvkxujm
9-10 j: rrhjwxntalgd
9-9 s: ycyeesbpnnab
4-8 z: niniu
8-9 s: cxtowkhzftx
5-7 o: iaappteglayzfnf
9-10 i: psmlcr
9-10 g: vpjtomscyjfly
8-10 w: pbkazzbbqh
5-8 h: ucfuqkbtgr
2-8 s: eikmjel
9-9 t: etrchyqtfaf
8-8 b: asqnolcrcixx
8-8 x: kvput
4-9 t: mmnnlxyh
8-8 g: xtikclzwkhbbtt
3-5 v: cpwwjskvxlaydz
10-10 h: wxaxipezp
6-9 o: pctkaig
10-10 x: hljrt
8-8 h: kdmvpmrwb
4-7 v: hqmcasokrblpn
4-8 u: phjtrtzme
1-5 b: sqvalxokxru
8-10 w: epmytpuekvsghf
4-9 h: zyoajhsagyxhjxv
5-7 i: fhvarohjorejh
1-4 q: lkzfhpjzjuu
3-6 x: fjygmzcvv
9-10 n: peaylec
1-6 m: xhtylvol
8-8 i: otsloauykoggx
1-3 d: aqzlmscof
7-10 r: idluoaxvwgml
7-7 r: wixcx
8-9 q: dnvufcbs
3-3 q: yrsix
4-4 n: chusuenbyvzb
1-5 d: klfkkzssfmiytuo
4-7 b: llraeqstpsvkw
4-10 e: ejrmjjgqarrlv
7-9 z: hzvjicgnlmlfbzv
9-10 v: uggnlqlepn
8-9 o: wfrvj
2-2 m: gxtagtxjjr
4-10 b: dyamkdlvpsig
3-3 l: gafcejaaqd
2-6 z: qorxzmietsd
5-8 h: pzuvezzztlqbfg
5-8 b: tamiy
3-10 q: pydyuosinnaowkp
1-5 f: rgsxlgi
1-10 m: dtexfrs
5-6 p: wgcbvyeijz
4-6 m: uarixhtrn
1-9 d: ynhdgpcd
4-10 q: wfqmkogakchbre
4-8 b: gokfk
5-8 m: efjqrseq
7-10 g: pcloef
1-3 k: ltntpcvr
5-6 r: onjyyyean
1-8 d: htdivwdhphb
5-7 r: koqomqryg
4-6 x: phlnfjxk
10-10 h: aofesjqaxgq
9-10 c: qikwma
6-7 x: zieefaazjukv
9-10 n: plhmdmhofuggdu
8-9 x: qckskyoz
1-9 e: ubfjleihbtk
5-7 c: bpjmez
9-10 u: bsjfrnxygeh
3-6 e: djszyjr
10-10 l: atkgyunujvehsl
9-9 y: crbnqpsfku
8-9 p: vwllerkgxnkdc
5-5 t: fdnuitsxu
5-8 u: agcijvi
1-6 g: jytdtomtwu
5-5 p: gyuglpnlqez
10-10 w: zrfarqfevrug
4-6 i: tjmjnnngksk
4-8 x: meroctyegccrvg
7-7 s: uholyuccn
4-7 q: nnvlmakatgegw
4-10 p: zmwrc
4-4 n: amvpkjrqem
2-5 r: wqknpkmsacm
9-9 z: wdkyprmgrizjvs
9-9 d: nxgqvpq
7-8 j: qteqcp
8-10 k: gnmbbgnucbkgh
1-3 m: phkzqy
6-9 j: tbnkkpolmfryvx
4-10 n: jtdrdtrdhbd
7-7 f: ldrma
2-3 c: djfnnjdoyl
9-10 k: ocvoxxo
2-7 g: qsjriarzvvj
3-8 t: jhuvulkuggom
1-3 u: kabcesqnsrwof
3-9 k: lqgrpbcva
1-10 b: ywwkpngsmc
2-9 v: ytrdfzgqvimdq
6-9 g: qtrlxusknb
6-9 x: oauaaxazegfa
7-7 h: idnxrjc
8-9 v: ecxzvxboqdeybm
7-7 u: oiejrdegda
1-8 w: bbprsaizr
5-7 t: hkohqiouhity
10-10 t: plvtvusdlfy
3-7 y: nsglxvchbqtj
9-9 r: mcplogwkkgp
4-5 r: vogmgk
1-1 o: rvdfttcjvjp
10-10 i: bvlfpbedaudydw
2-7 l: yyezoage
1-1 t: vnoffsvgilwed
2-9 q: nuelf
7-7 o: dmanukcgrhnmz
1-7 i: spnykaq
9-10 f: zrbbttpk
9-10 c: wsibsdudywm
2-3 m: anxofhkbnudkzsl
1-5 s: meynkgbxhrmlpov
5-7 p: jmbbiafl
4-5 a: ldoqbdoibxtkwlm
1-7 d: zjbomuwqto
4-10 h: heydkxk
8-9 q: ftxtkezitffc
3-10 t: jszdxfa
8-8 p: swjluowwhypqo
7-8 y: oxgesctzrfauc
7-7 u: qlyavsyral
4-5 o: gbbncuewwj
7-7 g: ctgxrcdtfktmx
4-9 p: gaoyvwoaobtjaw
7-8 b: agkrpstygucmj
7-7 q: srsdxqxshwewx
8-8 f: cmddmfppantb
4-4 q: fnwxty
3-8 b: ujyxfgkiky
1-10 m: etmjjmqymmmql
9-9 u: epmclhulhhefo
3-4 z: cvwwxjd
3-8 b: xyvxfxpnftuixim
3-10 c: npkoiusttjv
8-8 k: ymndpkvzbjdsg
6-9 w: hyjzohfoe
7-7 z: hmgykqscpgnn
3-10 c: xhxbncgofcnk
2-7 m: wbspedadumoqfq
10-10 z: czxiw
1-8 h: mphku
4-8 d: ccixnnxe
7-9 r: hhsszicrttwizk
8-9 i: fxrmyxtynfggmxd
9-9 f: abccn
4-10 y: rcgpazaegcxnc
2-2 q: jncwibzyrqi
7-10 f: dqmirwccscer
7-7 w: ehzfswcd
7-9 x: nsius
9-10 h: doczjpxgpc
8-10 m: epkupgbmyjfpxv
8-8 f: eaagsnfzy
10-10 b: peosgoupjx
3-5 k: ckwqpiaw
3-10 q: ohrthtsmsyavc
2-2 d: wfrorjahsjlk
8-10 d: ljobzldfml
5-5 a: kthyhpiweldzp
7-10 a: bqgkqodsnerjv
10-10 z: ywljccdd
2-7 s: rfwzubzuieotaov
7-9 e: syqvrazzzjm
6-7 s: afcsxrib
4-8 o: cjgmhv
5-9 l: sqivoindk
5-10 c: eyrlqezulasez
10-10 p: spxyowwaxfdlj
4-9 e: wlepyecbd";

    primera_solucion(entrada);
    segunda_opcion(entrada);
}
