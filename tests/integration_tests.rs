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
        assert_eq!(mel_presets::mel_7("abc", true), "def");
        assert_eq!(mel_presets::mel_7("def", false), "abc");
    }

    #[test]
    fn test_mel_8() {
        assert_eq!(mel_presets::mel_8("abc", true), "def");
        assert_eq!(mel_presets::mel_8("def", false), "abc");
    }

    #[test]
    fn test_mel_9() {
        assert_eq!(mel_presets::mel_9("abc", true), "def");
        assert_eq!(mel_presets::mel_9("def", false), "abc");
    }

    #[test]
    fn test_mel_10() {
        assert_eq!(mel_presets::mel_10("abc", true), "def");
        assert_eq!(mel_presets::mel_10("def", false), "abc");
    }

    #[test]
    fn test_mel_11() {
        assert_eq!(mel_presets::mel_11("abc", true), "def");
        assert_eq!(mel_presets::mel_11("def", false), "abc");
    }

    #[test]
    fn test_mel_12() {
        assert_eq!(mel_presets::mel_12("abc", true), "def");
        assert_eq!(mel_presets::mel_12("def", false), "abc");
    }

    #[test]
    fn test_mel_13() {
        assert_eq!(mel_presets::mel_13("abc", true), "def");
        assert_eq!(mel_presets::mel_13("def", false), "abc");
    }

    #[test]
    fn test_mel_14() {
        assert_eq!(mel_presets::mel_14("abc", true), "def");
        assert_eq!(mel_presets::mel_14("def", false), "abc");
    }

    #[test]
    fn test_mel_15() {
        assert_eq!(mel_presets::mel_15("abc", true), "def");
        assert_eq!(mel_presets::mel_15("def", false), "abc");
    }

    #[test]
    fn test_mel_16() {
        assert_eq!(mel_presets::mel_16("abc", true), "def");
        assert_eq!(mel_presets::mel_16("def", false), "abc");
    }

    #[test]
    fn test_mel_17() {
        assert_eq!(mel_presets::mel_17("abc", true), "def");
        assert_eq!(mel_presets::mel_17("def", false), "abc");
    }

    #[test]
    fn test_mel_18() {
        assert_eq!(mel_presets::mel_18("abc", true), "def");
        assert_eq!(mel_presets::mel_18("def", false), "abc");
    }

    #[test]
    fn test_mel_19() {
        assert_eq!(mel_presets::mel_19("abc", true), "def");
        assert_eq!(mel_presets::mel_19("def", false), "abc");
    }

    #[test]
    fn test_mel_20() {
        assert_eq!(mel_presets::mel_20("abc", true), "def");
        assert_eq!(mel_presets::mel_20("def", false), "abc");
    }

    #[test]
    fn test_mel_21() {
        assert_eq!(mel_presets::mel_21("abc", true), "def");
        assert_eq!(mel_presets::mel_21("def", false), "abc");
    }

    #[test]
    fn test_mel_22() {
        assert_eq!(mel_presets::mel_22("abc", true), "def");
        assert_eq!(mel_presets::mel_22("def", false), "abc");
    }

    #[test]
    fn test_mel_23() {
        assert_eq!(mel_presets::mel_23("abc", true), "def");
        assert_eq!(mel_presets::mel_23("def", false), "abc");
    }

    #[test]
    fn test_mel_24() {
        assert_eq!(mel_presets::mel_24("abc", true), "def");
        assert_eq!(mel_presets::mel_24("def", false), "abc");
    }

    #[test]
    fn test_mel_25() {
        assert_eq!(mel_presets::mel_25("abc", true), "def");
        assert_eq!(mel_presets::mel_25("def", false), "abc");
    }

    #[test]
    fn test_mel_912() {
        let input = "Héllo World!".to_string();
        let converted = "Qmsrt Arcvm".to_string();
        assert!(mel_912::mel_912(&input, true) == "Qmsrt Arcvm ");
        assert!(mel_912::mel_912(&converted, false) == "Hello World");
    }

    #[test]
    fn test_mel_mdp() {
        assert_eq!(mel_mdp::mel_mdp("SALUT", "HEY", true), "ZEJBX");
        assert_eq!(mel_mdp::mel_mdp("ZEJBX", "HEY", false), "SALUT");
    }
}