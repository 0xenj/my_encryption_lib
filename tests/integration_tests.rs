#[cfg(test)]
mod integration_tests {
    use my_encryption_lib::{mel_presets, mel_912, mel_mdp};

    #[test]
    fn test_mel_1() {
        assert_eq!(mel_presets::mel_1("abc", true).unwrap(), "bcd");
        assert_eq!(mel_presets::mel_1("bcd", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_1("à mangé forêt !? ë", true).unwrap(), "b nbohf gpsfu !? f");
        assert_eq!(mel_presets::mel_1("b nbohf gpsfu !? f", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_1("", true).is_err());
        assert!(mel_presets::mel_1("", false).is_err());
    }

    #[test]
    fn test_mel_2() {
        assert_eq!(mel_presets::mel_2("abc", true).unwrap(), "cde");
        assert_eq!(mel_presets::mel_2("cde", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_2("à mangé forêt !? ë", true).unwrap(), "c ocpig hqtgv !? g");
        assert_eq!(mel_presets::mel_2("c ocpig hqtgv !? g", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_2("", true).is_err());
        assert!(mel_presets::mel_2("", false).is_err());
    }

    #[test]
    fn test_mel_3() {
        assert_eq!(mel_presets::mel_3("abc", true).unwrap(), "def");
        assert_eq!(mel_presets::mel_3("def", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_3("à mangé forêt !? ë", true).unwrap(), "d pdqjh iruhw !? h");
        assert_eq!(mel_presets::mel_3("d pdqjh iruhw !? h", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_3("", true).is_err());
        assert!(mel_presets::mel_3("", false).is_err());
    }

    #[test]
    fn test_mel_4() {
        assert_eq!(mel_presets::mel_4("abc", true).unwrap(), "efg");
        assert_eq!(mel_presets::mel_4("efg", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_4("à mangé forêt !? ë", true).unwrap(), "e qerki jsvix !? i");
        assert_eq!(mel_presets::mel_4("e qerki jsvix !? i", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_4("", true).is_err());
        assert!(mel_presets::mel_4("", false).is_err());
    }

    #[test]
    fn test_mel_5() {
        assert_eq!(mel_presets::mel_5("abc", true).unwrap(), "fgh");
        assert_eq!(mel_presets::mel_5("fgh", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_5("à mangé forêt !? ë", true).unwrap(), "f rfslj ktwjy !? j");
        assert_eq!(mel_presets::mel_5("f rfslj ktwjy !? j", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_5("", true).is_err());
        assert!(mel_presets::mel_5("", false).is_err());
    }

    #[test]
    fn test_mel_6() {
        assert_eq!(mel_presets::mel_6("abc", true).unwrap(), "ghi");
        assert_eq!(mel_presets::mel_6("ghi", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_6("à mangé forêt !? ë", true).unwrap(), "g sgtmk luxkz !? k");
        assert_eq!(mel_presets::mel_6("g sgtmk luxkz !? k", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_6("", true).is_err());
        assert!(mel_presets::mel_6("", false).is_err());
    }

    #[test]
    fn test_mel_7() {
        assert_eq!(mel_presets::mel_7("abc", true).unwrap(), "hij");
        assert_eq!(mel_presets::mel_7("hij", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_7("à mangé forêt !? ë", true).unwrap(), "h thunl mvyla !? l");
        assert_eq!(mel_presets::mel_7("h thunl mvyla !? l", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_7("", true).is_err());
        assert!(mel_presets::mel_7("", false).is_err());
    }

    #[test]
    fn test_mel_8() {
        assert_eq!(mel_presets::mel_8("abc", true).unwrap(), "ijk");
        assert_eq!(mel_presets::mel_8("ijk", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_8("à mangé forêt !? ë", true).unwrap(), "i uivom nwzmb !? m");
        assert_eq!(mel_presets::mel_8("i uivom nwzmb !? m", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_8("", true).is_err());
        assert!(mel_presets::mel_8("", false).is_err());
    }

    #[test]
    fn test_mel_9() {
        assert_eq!(mel_presets::mel_9("abc", true).unwrap(), "jkl");
        assert_eq!(mel_presets::mel_9("jkl", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_9("à mangé forêt !? ë", true).unwrap(), "j vjwpn oxanc !? n");
        assert_eq!(mel_presets::mel_9("j vjwpn oxanc !? n", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_9("", true).is_err());
        assert!(mel_presets::mel_9("", false).is_err());
    }

    #[test]
    fn test_mel_10() {
        assert_eq!(mel_presets::mel_10("abc", true).unwrap(), "klm");
        assert_eq!(mel_presets::mel_10("klm", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_10("à mangé forêt !? ë", true).unwrap(), "k wkxqo pybod !? o");
        assert_eq!(mel_presets::mel_10("k wkxqo pybod !? o", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_10("", true).is_err());
        assert!(mel_presets::mel_10("", false).is_err());
    }

    #[test]
    fn test_mel_11() {
        assert_eq!(mel_presets::mel_11("abc", true).unwrap(), "lmn");
        assert_eq!(mel_presets::mel_11("lmn", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_11("à mangé forêt !? ë", true).unwrap(), "l xlyrp qzcpe !? p");
        assert_eq!(mel_presets::mel_11("l xlyrp qzcpe !? p", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_11("", true).is_err());
        assert!(mel_presets::mel_11("", false).is_err());
    }

    #[test]
    fn test_mel_12() {
        assert_eq!(mel_presets::mel_12("abc", true).unwrap(), "mno");
        assert_eq!(mel_presets::mel_12("mno", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_12("à mangé forêt !? ë", true).unwrap(), "m ymzsq radqf !? q");
        assert_eq!(mel_presets::mel_12("m ymzsq radqf !? q", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_12("", true).is_err());
        assert!(mel_presets::mel_12("", false).is_err());
    }

    #[test]
    fn test_mel_13() {
        assert_eq!(mel_presets::mel_13("abc", true).unwrap(), "nop");
        assert_eq!(mel_presets::mel_13("nop", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_13("à mangé forêt !? ë", true).unwrap(), "n znatr sberg !? r");
        assert_eq!(mel_presets::mel_13("n znatr sberg !? r", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_13("", true).is_err());
        assert!(mel_presets::mel_13("", false).is_err());
    }

    #[test]
    fn test_mel_14() {
        assert_eq!(mel_presets::mel_14("abc", true).unwrap(), "opq");
        assert_eq!(mel_presets::mel_14("opq", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_14("à mangé forêt !? ë", true).unwrap(), "o aobus tcfsh !? s");
        assert_eq!(mel_presets::mel_14("o aobus tcfsh !? s", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_14("", true).is_err());
        assert!(mel_presets::mel_14("", false).is_err());
    }

    #[test]
    fn test_mel_15() {
        assert_eq!(mel_presets::mel_15("abc", true).unwrap(), "pqr");
        assert_eq!(mel_presets::mel_15("pqr", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_15("à mangé forêt !? ë", true).unwrap(), "p bpcvt udgti !? t");
        assert_eq!(mel_presets::mel_15("p bpcvt udgti !? t", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_15("", true).is_err());
        assert!(mel_presets::mel_15("", false).is_err());
    }

    #[test]
    fn test_mel_16() {
        assert_eq!(mel_presets::mel_16("abc", true).unwrap(), "qrs");
        assert_eq!(mel_presets::mel_16("qrs", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_16("à mangé forêt !? ë", true).unwrap(), "q cqdwu vehuj !? u");
        assert_eq!(mel_presets::mel_16("q cqdwu vehuj !? u", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_16("", true).is_err());
        assert!(mel_presets::mel_16("", false).is_err());
    }

    #[test]
    fn test_mel_17() {
        assert_eq!(mel_presets::mel_17("abc", true).unwrap(), "rst");
        assert_eq!(mel_presets::mel_17("rst", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_17("à mangé forêt !? ë", true).unwrap(), "r drexv wfivk !? v");
        assert_eq!(mel_presets::mel_17("r drexv wfivk !? v", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_17("", true).is_err());
        assert!(mel_presets::mel_17("", false).is_err());
    }

    #[test]
    fn test_mel_18() {
        assert_eq!(mel_presets::mel_18("abc", true).unwrap(), "stu");
        assert_eq!(mel_presets::mel_18("stu", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_18("à mangé forêt !? ë", true).unwrap(), "s esfyw xgjwl !? w");
        assert_eq!(mel_presets::mel_18("s esfyw xgjwl !? w", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_18("", true).is_err());
        assert!(mel_presets::mel_18("", false).is_err());
    }

    #[test]
    fn test_mel_19() {
        assert_eq!(mel_presets::mel_19("abc", true).unwrap(), "tuv");
        assert_eq!(mel_presets::mel_19("tuv", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_19("à mangé forêt !? ë", true).unwrap(), "t ftgzx yhkxm !? x");
        assert_eq!(mel_presets::mel_19("t ftgzx yhkxm !? x", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_19("", true).is_err());
        assert!(mel_presets::mel_19("", false).is_err());
    }

    #[test]
    fn test_mel_20() {
        assert_eq!(mel_presets::mel_20("abc", true).unwrap(), "uvw");
        assert_eq!(mel_presets::mel_20("uvw", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_20("à mangé forêt !? ë", true).unwrap(), "u guhay zilyn !? y");
        assert_eq!(mel_presets::mel_20("u guhay zilyn !? y", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_20("", true).is_err());
        assert!(mel_presets::mel_20("", false).is_err());
    }

    #[test]
    fn test_mel_21() {
        assert_eq!(mel_presets::mel_21("abc", true).unwrap(), "vwx");
        assert_eq!(mel_presets::mel_21("vwx", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_21("à mangé forêt !? ë", true).unwrap(), "v hvibz ajmzo !? z");
        assert_eq!(mel_presets::mel_21("v hvibz ajmzo !? z", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_21("", true).is_err());
        assert!(mel_presets::mel_21("", false).is_err());
    }

    #[test]
    fn test_mel_22() {
        assert_eq!(mel_presets::mel_22("abc", true).unwrap(), "wxy");
        assert_eq!(mel_presets::mel_22("wxy", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_22("à mangé forêt !? ë", true).unwrap(), "w iwjca bknap !? a");
        assert_eq!(mel_presets::mel_22("w iwjca bknap !? a", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_22("", true).is_err());
        assert!(mel_presets::mel_22("", false).is_err());
    }

    #[test]
    fn test_mel_23() {
        assert_eq!(mel_presets::mel_23("abc", true).unwrap(), "xyz");
        assert_eq!(mel_presets::mel_23("xyz", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_23("à mangé forêt !? ë", true).unwrap(), "x jxkdb clobq !? b");
        assert_eq!(mel_presets::mel_23("x jxkdb clobq !? b", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_23("", true).is_err());
        assert!(mel_presets::mel_23("", false).is_err());
    }

    #[test]
    fn test_mel_24() {
        assert_eq!(mel_presets::mel_24("abc", true).unwrap(), "yza");
        assert_eq!(mel_presets::mel_24("yza", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_24("à mangé forêt !? ë", true).unwrap(), "y kylec dmpcr !? c");
        assert_eq!(mel_presets::mel_24("y kylec dmpcr !? c", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_24("", true).is_err());
        assert!(mel_presets::mel_24("", false).is_err());
    }

    #[test]
    fn test_mel_25() {
        assert_eq!(mel_presets::mel_25("abc", true).unwrap(), "zab");
        assert_eq!(mel_presets::mel_25("zab", false).unwrap(), "abc");
        assert_eq!(mel_presets::mel_25("à mangé forêt !? ë", true).unwrap(), "z lzmfd enqds !? d");
        assert_eq!(mel_presets::mel_25("z lzmfd enqds !? d", false).unwrap(), "a mange foret !? e");
        assert!(mel_presets::mel_25("", true).is_err());
        assert!(mel_presets::mel_25("", false).is_err());
    }

    #[test]
    fn test_mel_912() {
        assert_eq!(mel_912::mel_912("Héllo World!", true).unwrap(), "Qmsrt Arcvm!");
        assert_eq!(mel_912::mel_912("Qmsrt Arcvm!", false).unwrap(), "Hello World!");
        assert!(mel_912::mel_912("", true).is_err());
    }

    #[test]
    fn test_mel_mdp() {
        assert_eq!(mel_mdp::mel_mdp("Hello World", "key", true).unwrap(), "Rijvs Uyvjn");
        assert!(mel_mdp::mel_mdp("", "key", true).is_err());
        assert!(mel_mdp::mel_mdp("Hello World", "k3y", true).is_err());
    }
}