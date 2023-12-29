#[cfg(test)]
mod integration_tests {
    use my_encryption_lib::{mel_presets, mel_912, mel_mdp};

    #[test]
    fn test_mel_1() {
        assert_eq!(mel_presets::mel_1("abc", true), "bcd");
        assert_eq!(mel_presets::mel_1("bcd", false), "abc");
        assert_eq!(mel_presets::mel_1("à mangé forêt !? ë", true), "b nbohf gpsfu !? f");
        assert_eq!(mel_presets::mel_1("b nbohf gpsfu !? f", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_2() {
        assert_eq!(mel_presets::mel_2("abc", true), "cde");
        assert_eq!(mel_presets::mel_2("cde", false), "abc");
        assert_eq!(mel_presets::mel_2("à mangé forêt !? ë", true), "c ocpig hqtgv !? g");
        assert_eq!(mel_presets::mel_2("c ocpig hqtgv !? g", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_3() {
        assert_eq!(mel_presets::mel_3("abc", true), "def");
        assert_eq!(mel_presets::mel_3("def", false), "abc");
        assert_eq!(mel_presets::mel_3("à mangé forêt !? ë", true), "d pdqjh iruhw !? h");
        assert_eq!(mel_presets::mel_3("d pdqjh iruhw !? h", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_4() {
        assert_eq!(mel_presets::mel_4("abc", true), "efg");
        assert_eq!(mel_presets::mel_4("efg", false), "abc");
        assert_eq!(mel_presets::mel_4("à mangé forêt !? ë", true), "e qerki jsvix !? i");
        assert_eq!(mel_presets::mel_4("e qerki jsvix !? i", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_5() {
        assert_eq!(mel_presets::mel_5("abc", true), "fgh");
        assert_eq!(mel_presets::mel_5("fgh", false), "abc");
        assert_eq!(mel_presets::mel_5("à mangé forêt !? ë", true), "f rfslj ktwjy !? j");
        assert_eq!(mel_presets::mel_5("f rfslj ktwjy !? j", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_6() {
        assert_eq!(mel_presets::mel_6("abc", true), "ghi");
        assert_eq!(mel_presets::mel_6("ghi", false), "abc");
        assert_eq!(mel_presets::mel_6("à mangé forêt !? ë", true), "g sgtmk luxkz !? k");
        assert_eq!(mel_presets::mel_6("g sgtmk luxkz !? k", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_7() {
        assert_eq!(mel_presets::mel_7("abc", true), "hij");
        assert_eq!(mel_presets::mel_7("hij", false), "abc");
        assert_eq!(mel_presets::mel_7("à mangé forêt !? ë", true), "h thunl mvyla !? l");
        assert_eq!(mel_presets::mel_7("h thunl mvyla !? l", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_8() {
        assert_eq!(mel_presets::mel_8("abc", true), "ijk");
        assert_eq!(mel_presets::mel_8("ijk", false), "abc");
        assert_eq!(mel_presets::mel_8("à mangé forêt !? ë", true), "i uivom nwzmb !? m");
        assert_eq!(mel_presets::mel_8("i uivom nwzmb !? m", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_9() {
        assert_eq!(mel_presets::mel_9("abc", true), "jkl");
        assert_eq!(mel_presets::mel_9("jkl", false), "abc");
        assert_eq!(mel_presets::mel_9("à mangé forêt !? ë", true), "j vjwpn oxanc !? n");
        assert_eq!(mel_presets::mel_9("j vjwpn oxanc !? n", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_10() {
        assert_eq!(mel_presets::mel_10("abc", true), "klm");
        assert_eq!(mel_presets::mel_10("klm", false), "abc");
        assert_eq!(mel_presets::mel_10("à mangé forêt !? ë", true), "k wkxqo pybod !? o");
        assert_eq!(mel_presets::mel_10("k wkxqo pybod !? o", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_11() {
        assert_eq!(mel_presets::mel_11("abc", true), "lmn");
        assert_eq!(mel_presets::mel_11("lmn", false), "abc");
        assert_eq!(mel_presets::mel_11("à mangé forêt !? ë", true), "l xlyrp qzcpe !? p");
        assert_eq!(mel_presets::mel_11("l xlyrp qzcpe !? p", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_12() {
        assert_eq!(mel_presets::mel_12("abc", true), "mno");
        assert_eq!(mel_presets::mel_12("mno", false), "abc");
        assert_eq!(mel_presets::mel_12("à mangé forêt !? ë", true), "m ymzsq radqf !? q");
        assert_eq!(mel_presets::mel_12("m ymzsq radqf !? q", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_13() {
        assert_eq!(mel_presets::mel_13("abc", true), "nop");
        assert_eq!(mel_presets::mel_13("nop", false), "abc");
        assert_eq!(mel_presets::mel_13("à mangé forêt !? ë", true), "n znatr sberg !? r");
        assert_eq!(mel_presets::mel_13("n znatr sberg !? r", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_14() {
        assert_eq!(mel_presets::mel_14("abc", true), "opq");
        assert_eq!(mel_presets::mel_14("opq", false), "abc");
        assert_eq!(mel_presets::mel_14("à mangé forêt !? ë", true), "o aobus tcfsh !? s");
        assert_eq!(mel_presets::mel_14("o aobus tcfsh !? s", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_15() {
        assert_eq!(mel_presets::mel_15("abc", true), "pqr");
        assert_eq!(mel_presets::mel_15("pqr", false), "abc");
        assert_eq!(mel_presets::mel_15("à mangé forêt !? ë", true), "p bpcvt udgti !? t");
        assert_eq!(mel_presets::mel_15("p bpcvt udgti !? t", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_16() {
        assert_eq!(mel_presets::mel_16("abc", true), "qrs");
        assert_eq!(mel_presets::mel_16("qrs", false), "abc");
        assert_eq!(mel_presets::mel_16("à mangé forêt !? ë", true), "q cqdwu vehuj !? u");
        assert_eq!(mel_presets::mel_16("q cqdwu vehuj !? u", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_17() {
        assert_eq!(mel_presets::mel_17("abc", true), "rst");
        assert_eq!(mel_presets::mel_17("rst", false), "abc");
        assert_eq!(mel_presets::mel_17("à mangé forêt !? ë", true), "r drexv wfivk !? v");
        assert_eq!(mel_presets::mel_17("r drexv wfivk !? v", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_18() {
        assert_eq!(mel_presets::mel_18("abc", true), "stu");
        assert_eq!(mel_presets::mel_18("stu", false), "abc");
        assert_eq!(mel_presets::mel_18("à mangé forêt !? ë", true), "s esfyw xgjwl !? w");
        assert_eq!(mel_presets::mel_18("s esfyw xgjwl !? w", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_19() {
        assert_eq!(mel_presets::mel_19("abc", true), "tuv");
        assert_eq!(mel_presets::mel_19("tuv", false), "abc");
        assert_eq!(mel_presets::mel_19("à mangé forêt !? ë", true), "t ftgzx yhkxm !? x");
        assert_eq!(mel_presets::mel_19("t ftgzx yhkxm !? x", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_20() {
        assert_eq!(mel_presets::mel_20("abc", true), "uvw");
        assert_eq!(mel_presets::mel_20("uvw", false), "abc");
        assert_eq!(mel_presets::mel_20("à mangé forêt !? ë", true), "u guhay zilyn !? y");
        assert_eq!(mel_presets::mel_20("u guhay zilyn !? y", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_21() {
        assert_eq!(mel_presets::mel_21("abc", true), "vwx");
        assert_eq!(mel_presets::mel_21("vwx", false), "abc");
        assert_eq!(mel_presets::mel_21("à mangé forêt !? ë", true), "v hvibz ajmzo !? z");
        assert_eq!(mel_presets::mel_21("v hvibz ajmzo !? z", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_22() {
        assert_eq!(mel_presets::mel_22("abc", true), "wxy");
        assert_eq!(mel_presets::mel_22("wxy", false), "abc");
        assert_eq!(mel_presets::mel_22("à mangé forêt !? ë", true), "w iwjca bknap !? a");
        assert_eq!(mel_presets::mel_22("w iwjca bknap !? a", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_23() {
        assert_eq!(mel_presets::mel_23("abc", true), "xyz");
        assert_eq!(mel_presets::mel_23("xyz", false), "abc");
        assert_eq!(mel_presets::mel_23("à mangé forêt !? ë", true), "x jxkdb clobq !? b");
        assert_eq!(mel_presets::mel_23("x jxkdb clobq !? b", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_24() {
        assert_eq!(mel_presets::mel_24("abc", true), "yza");
        assert_eq!(mel_presets::mel_24("yza", false), "abc");
        assert_eq!(mel_presets::mel_24("à mangé forêt !? ë", true), "y kylec dmpcr !? c");
        assert_eq!(mel_presets::mel_24("y kylec dmpcr !? c", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_25() {
        assert_eq!(mel_presets::mel_25("abc", true), "zab");
        assert_eq!(mel_presets::mel_25("zab", false), "abc");
        assert_eq!(mel_presets::mel_25("à mangé forêt !? ë", true), "z lzmfd enqds !? d");
        assert_eq!(mel_presets::mel_25("z lzmfd enqds !? d", false), "a mange foret !? e");
    }

    #[test]
    fn test_mel_912() {
        assert_eq!(mel_912::mel_912("Héllo World!", true), "Qmsrt Arcvm!");
        assert_eq!(mel_912::mel_912("Qmsrt Arcvm!", false), "Hello World!");
    }

    #[test]
    fn test_mel_mdp() {
        assert_eq!(mel_mdp::mel_mdp("SaLUt", "HEy", true), "ZeJBx");
        assert_eq!(mel_mdp::mel_mdp("ZeJBx", "HEy", false), "SaLUt");
    }
}