// These code are generated, dont edit by hand
        use crate::{BitVector, HtmlNode, IState, SimpleSelector};
        use std::collections::HashMap;
        use std::sync::OnceLock;

pub const BITVECTOR_CAPACITY: usize = 218;

/// generate_string_interning_code
// String interning for optimized selector matching
static STRING_TO_ID: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();

fn get_string_to_id_map() -> &'static HashMap<&'static str, u32> {
    STRING_TO_ID.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("gbpms2", 23);
        map.insert("gbmh", 11);
        map.insert("gbprct", 27);
        map.insert("gbmt", 19);
        map.insert("gbmtc", 20);
        map.insert("gbqfbb-hvr", 33);
        map.insert("gb", 75);
        map.insert("gbx4", 99);
        map.insert("gsls_a", 50);
        map.insert("gbprcd", 25);
        map.insert("gbmab", 7);
        map.insert("gbmpas", 87);
        map.insert("gbmpdv", 88);
        map.insert("gspqs_b", 55);
        map.insert("gbpmc", 22);
        map.insert("gbsbic", 35);
        map.insert("gbps2", 28);
        map.insert("gbtcb", 38);
        map.insert("gbmcc", 10);
        map.insert("gbm", 5);
        map.insert("gbmpnw", 18);
        map.insert("gbts", 40);
        map.insert("gsn_a", 52);
        map.insert("gbp0", 21);
        map.insert("gsib_b", 49);
        map.insert("gbxo", 43);
        map.insert("gsn_c", 54);
        map.insert("lsb", 69);
        map.insert("gbi4p", 4);
        map.insert("lsbb", 70);
        map.insert("sblc", 72);
        map.insert("gbqfba", 31);
        map.insert("gsfs", 47);
        map.insert("gbi4s1", 82);
        map.insert("gsib_a", 48);
        map.insert("gbmpia", 15);
        map.insert("gbmpal", 86);
        map.insert("gbmpiw", 90);
        map.insert("gbmps", 91);
        map.insert("gbx3", 98);
        map.insert("gbprci", 26);
        map.insert("gbmm", 85);
        map.insert("gsmq_a", 51);
        map.insert("SIvCob", 74);
        map.insert("gbqfbw", 96);
        map.insert("gbpms", 93);
        map.insert("z4hgWe", 73);
        map.insert("gbmac", 8);
        map.insert("gbmpid", 89);
        map.insert("gbprca", 94);
        map.insert("gws-output-pages-elements-homepage_additional_languages__als", 103);
        map.insert("input", 106);
        map.insert("gbmpalb", 14);
        map.insert("gbsb", 34);
        map.insert("gbi4id", 81);
        map.insert("gbmpala", 13);
        map.insert("gbprcb", 24);
        map.insert("gsn_b", 53);
        map.insert("gssb_g", 62);
        map.insert("gssb_b", 102);
        map.insert("gsq_a", 56);
        map.insert("gss_ifl", 57);
        map.insert("h", 68);
        map.insert("gbs", 97);
        map.insert("gssb_l", 66);
        map.insert("H6sW5", 0);
        map.insert("gbprcs", 95);
        map.insert("gbmpiaa", 16);
        map.insert("gbqfb", 29);
        map.insert("gbi5", 84);
        map.insert("gbg", 78);
        map.insert("gbxms", 42);
        map.insert("gbto", 39);
        map.insert("gssb_h", 63);
        map.insert("gbtb2", 37);
        map.insert("gssb_e", 60);
        map.insert("gbgs5", 80);
        map.insert("gssb_k", 65);
        map.insert("gbma", 6);
        map.insert("gssb_a", 58);
        map.insert("gbxx", 45);
        map.insert("gssb_c", 59);
        map.insert("gbi4t", 83);
        map.insert("ds", 1);
        map.insert("gssb_i", 64);
        map.insert("gssb_m", 67);
        map.insert("gbmpiaw", 17);
        map.insert("gsdd_a", 46);
        map.insert("lst", 71);
        map.insert("gbg5", 79);
        map.insert("gbz", 100);
        map.insert("gbqfbb", 32);
        map.insert("a", 104);
        map.insert("gbb", 76);
        map.insert("body", 105);
        map.insert("gbmc", 9);
        map.insert("gbgt", 3);
        map.insert("gbtsa", 41);
        map.insert("gssb_f", 61);
        map.insert("gbbw", 77);
        map.insert("gbmlbw", 12);
        map.insert("gbxv", 44);
        map.insert("gbt", 36);
        map.insert("gbpm", 92);
        map.insert("gog", 101);
        map.insert("gbg4a", 2);
        map.insert("td", 107);
        map.insert("gbqfb-hvr", 30);
        map
    })
}

// Fast selector matching using integer IDs and switch
        #[inline]
        fn get_node_tag_id(node: &HtmlNode) -> Option<u32> {
            get_string_to_id_map().get(node.tag_name.as_str()).copied()
        }
        #[inline]
        fn get_node_id_id(node: &HtmlNode) -> Option<u32> {
            node.id.as_ref().and_then(|id| get_string_to_id_map().get(id.as_str()).copied())
        }
        #[inline]
        fn node_has_class_id(node: &HtmlNode, class_id: u32) -> bool {
            let string_map = get_string_to_id_map();
            for class in &node.classes {
                if let Some(id) = string_map.get(class.as_str()) {
                    if *id == class_id {
                        return true;
                    }
                }
            }
            false
        }
// --- Incremental Processing Functions ---
        pub fn process_node_generated_incremental(
            node: &mut HtmlNode,
            parent_state: &BitVector,
        ) -> BitVector { // returns child_states
            // Check if we need to recompute
            if !node.needs_any_recomputation(parent_state) {
                // Return cached result - entire subtree can be skipped
                return node.cached_child_states.clone().unwrap();
            }
            // Recompute node intrinsic matches if needed
            if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        /// generate_intrinsic_checks_code
let mut intrinsic_matches = BitVector::with_capacity(BITVECTOR_CAPACITY);
match get_node_tag_id(node) {
// Instruction 210: CheckAndSetBit { selector: Type("a"), bit_pos: 210 }
 Some(104)  => {
            intrinsic_matches.set_bit(210); // match_Type("a")
        }

// Instruction 212: CheckAndSetBit { selector: Type("body"), bit_pos: 212 }
 Some(105)  => {
            intrinsic_matches.set_bit(212); // match_Type("body")
        }

// Instruction 214: CheckAndSetBit { selector: Type("input"), bit_pos: 214 }
 Some(106)  => {
            intrinsic_matches.set_bit(214); // match_Type("input")
        }

// Instruction 216: CheckAndSetBit { selector: Type("td"), bit_pos: 216 }
 Some(107)  => {
            intrinsic_matches.set_bit(216); // match_Type("td")
        }

_ => {}}
        // Instruction 0: CheckAndSetBit { selector: Class("H6sW5"), bit_pos: 0 }
        if node_has_class_id(node, 0) {
            intrinsic_matches.set_bit(0); // match_Class("H6sW5")
        }

        // Instruction 2: CheckAndSetBit { selector: Class("ds"), bit_pos: 2 }
        if node_has_class_id(node, 1) {
            intrinsic_matches.set_bit(2); // match_Class("ds")
        }

        // Instruction 4: CheckAndSetBit { selector: Class("gbg4a"), bit_pos: 4 }
        if node_has_class_id(node, 2) {
            intrinsic_matches.set_bit(4); // match_Class("gbg4a")
        }

        // Instruction 6: CheckAndSetBit { selector: Class("gbgt"), bit_pos: 6 }
        if node_has_class_id(node, 3) {
            intrinsic_matches.set_bit(6); // match_Class("gbgt")
        }

        // Instruction 8: CheckAndSetBit { selector: Class("gbi4p"), bit_pos: 8 }
        if node_has_class_id(node, 4) {
            intrinsic_matches.set_bit(8); // match_Class("gbi4p")
        }

        // Instruction 10: CheckAndSetBit { selector: Class("gbm"), bit_pos: 10 }
        if node_has_class_id(node, 5) {
            intrinsic_matches.set_bit(10); // match_Class("gbm")
        }

        // Instruction 12: CheckAndSetBit { selector: Class("gbma"), bit_pos: 12 }
        if node_has_class_id(node, 6) {
            intrinsic_matches.set_bit(12); // match_Class("gbma")
        }

        // Instruction 14: CheckAndSetBit { selector: Class("gbmab"), bit_pos: 14 }
        if node_has_class_id(node, 7) {
            intrinsic_matches.set_bit(14); // match_Class("gbmab")
        }

        // Instruction 16: CheckAndSetBit { selector: Class("gbmac"), bit_pos: 16 }
        if node_has_class_id(node, 8) {
            intrinsic_matches.set_bit(16); // match_Class("gbmac")
        }

        // Instruction 18: CheckAndSetBit { selector: Class("gbmc"), bit_pos: 18 }
        if node_has_class_id(node, 9) {
            intrinsic_matches.set_bit(18); // match_Class("gbmc")
        }

        // Instruction 20: CheckAndSetBit { selector: Class("gbmcc"), bit_pos: 20 }
        if node_has_class_id(node, 10) {
            intrinsic_matches.set_bit(20); // match_Class("gbmcc")
        }

        // Instruction 22: CheckAndSetBit { selector: Class("gbmh"), bit_pos: 22 }
        if node_has_class_id(node, 11) {
            intrinsic_matches.set_bit(22); // match_Class("gbmh")
        }

        // Instruction 24: CheckAndSetBit { selector: Class("gbmlbw"), bit_pos: 24 }
        if node_has_class_id(node, 12) {
            intrinsic_matches.set_bit(24); // match_Class("gbmlbw")
        }

        // Instruction 26: CheckAndSetBit { selector: Class("gbmpala"), bit_pos: 26 }
        if node_has_class_id(node, 13) {
            intrinsic_matches.set_bit(26); // match_Class("gbmpala")
        }

        // Instruction 28: CheckAndSetBit { selector: Class("gbmpalb"), bit_pos: 28 }
        if node_has_class_id(node, 14) {
            intrinsic_matches.set_bit(28); // match_Class("gbmpalb")
        }

        // Instruction 30: CheckAndSetBit { selector: Class("gbmpia"), bit_pos: 30 }
        if node_has_class_id(node, 15) {
            intrinsic_matches.set_bit(30); // match_Class("gbmpia")
        }

        // Instruction 32: CheckAndSetBit { selector: Class("gbmpiaa"), bit_pos: 32 }
        if node_has_class_id(node, 16) {
            intrinsic_matches.set_bit(32); // match_Class("gbmpiaa")
        }

        // Instruction 34: CheckAndSetBit { selector: Class("gbmpiaw"), bit_pos: 34 }
        if node_has_class_id(node, 17) {
            intrinsic_matches.set_bit(34); // match_Class("gbmpiaw")
        }

        // Instruction 36: CheckAndSetBit { selector: Class("gbmpnw"), bit_pos: 36 }
        if node_has_class_id(node, 18) {
            intrinsic_matches.set_bit(36); // match_Class("gbmpnw")
        }

        // Instruction 38: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 38 }
        if node_has_class_id(node, 19) {
            intrinsic_matches.set_bit(38); // match_Class("gbmt")
        }

        // Instruction 40: CheckAndSetBit { selector: Class("gbmtc"), bit_pos: 40 }
        if node_has_class_id(node, 20) {
            intrinsic_matches.set_bit(40); // match_Class("gbmtc")
        }

        // Instruction 42: CheckAndSetBit { selector: Class("gbp0"), bit_pos: 42 }
        if node_has_class_id(node, 21) {
            intrinsic_matches.set_bit(42); // match_Class("gbp0")
        }

        // Instruction 44: CheckAndSetBit { selector: Class("gbpmc"), bit_pos: 44 }
        if node_has_class_id(node, 22) {
            intrinsic_matches.set_bit(44); // match_Class("gbpmc")
        }

        // Instruction 46: CheckAndSetBit { selector: Class("gbpms2"), bit_pos: 46 }
        if node_has_class_id(node, 23) {
            intrinsic_matches.set_bit(46); // match_Class("gbpms2")
        }

        // Instruction 48: CheckAndSetBit { selector: Class("gbprcb"), bit_pos: 48 }
        if node_has_class_id(node, 24) {
            intrinsic_matches.set_bit(48); // match_Class("gbprcb")
        }

        // Instruction 50: CheckAndSetBit { selector: Class("gbprcd"), bit_pos: 50 }
        if node_has_class_id(node, 25) {
            intrinsic_matches.set_bit(50); // match_Class("gbprcd")
        }

        // Instruction 52: CheckAndSetBit { selector: Class("gbprci"), bit_pos: 52 }
        if node_has_class_id(node, 26) {
            intrinsic_matches.set_bit(52); // match_Class("gbprci")
        }

        // Instruction 54: CheckAndSetBit { selector: Class("gbprct"), bit_pos: 54 }
        if node_has_class_id(node, 27) {
            intrinsic_matches.set_bit(54); // match_Class("gbprct")
        }

        // Instruction 56: CheckAndSetBit { selector: Class("gbps2"), bit_pos: 56 }
        if node_has_class_id(node, 28) {
            intrinsic_matches.set_bit(56); // match_Class("gbps2")
        }

        // Instruction 58: CheckAndSetBit { selector: Class("gbqfb"), bit_pos: 58 }
        if node_has_class_id(node, 29) {
            intrinsic_matches.set_bit(58); // match_Class("gbqfb")
        }

        // Instruction 60: CheckAndSetBit { selector: Class("gbqfb-hvr"), bit_pos: 60 }
        if node_has_class_id(node, 30) {
            intrinsic_matches.set_bit(60); // match_Class("gbqfb-hvr")
        }

        // Instruction 62: CheckAndSetBit { selector: Class("gbqfba"), bit_pos: 62 }
        if node_has_class_id(node, 31) {
            intrinsic_matches.set_bit(62); // match_Class("gbqfba")
        }

        // Instruction 64: CheckAndSetBit { selector: Class("gbqfbb"), bit_pos: 64 }
        if node_has_class_id(node, 32) {
            intrinsic_matches.set_bit(64); // match_Class("gbqfbb")
        }

        // Instruction 66: CheckAndSetBit { selector: Class("gbqfbb-hvr"), bit_pos: 66 }
        if node_has_class_id(node, 33) {
            intrinsic_matches.set_bit(66); // match_Class("gbqfbb-hvr")
        }

        // Instruction 68: CheckAndSetBit { selector: Class("gbsb"), bit_pos: 68 }
        if node_has_class_id(node, 34) {
            intrinsic_matches.set_bit(68); // match_Class("gbsb")
        }

        // Instruction 70: CheckAndSetBit { selector: Class("gbsbic"), bit_pos: 70 }
        if node_has_class_id(node, 35) {
            intrinsic_matches.set_bit(70); // match_Class("gbsbic")
        }

        // Instruction 72: CheckAndSetBit { selector: Class("gbt"), bit_pos: 72 }
        if node_has_class_id(node, 36) {
            intrinsic_matches.set_bit(72); // match_Class("gbt")
        }

        // Instruction 74: CheckAndSetBit { selector: Class("gbtb2"), bit_pos: 74 }
        if node_has_class_id(node, 37) {
            intrinsic_matches.set_bit(74); // match_Class("gbtb2")
        }

        // Instruction 76: CheckAndSetBit { selector: Class("gbtcb"), bit_pos: 76 }
        if node_has_class_id(node, 38) {
            intrinsic_matches.set_bit(76); // match_Class("gbtcb")
        }

        // Instruction 78: CheckAndSetBit { selector: Class("gbto"), bit_pos: 78 }
        if node_has_class_id(node, 39) {
            intrinsic_matches.set_bit(78); // match_Class("gbto")
        }

        // Instruction 80: CheckAndSetBit { selector: Class("gbts"), bit_pos: 80 }
        if node_has_class_id(node, 40) {
            intrinsic_matches.set_bit(80); // match_Class("gbts")
        }

        // Instruction 82: CheckAndSetBit { selector: Class("gbtsa"), bit_pos: 82 }
        if node_has_class_id(node, 41) {
            intrinsic_matches.set_bit(82); // match_Class("gbtsa")
        }

        // Instruction 84: CheckAndSetBit { selector: Class("gbxms"), bit_pos: 84 }
        if node_has_class_id(node, 42) {
            intrinsic_matches.set_bit(84); // match_Class("gbxms")
        }

        // Instruction 86: CheckAndSetBit { selector: Class("gbxo"), bit_pos: 86 }
        if node_has_class_id(node, 43) {
            intrinsic_matches.set_bit(86); // match_Class("gbxo")
        }

        // Instruction 88: CheckAndSetBit { selector: Class("gbxv"), bit_pos: 88 }
        if node_has_class_id(node, 44) {
            intrinsic_matches.set_bit(88); // match_Class("gbxv")
        }

        // Instruction 90: CheckAndSetBit { selector: Class("gbxx"), bit_pos: 90 }
        if node_has_class_id(node, 45) {
            intrinsic_matches.set_bit(90); // match_Class("gbxx")
        }

        // Instruction 92: CheckAndSetBit { selector: Class("gsdd_a"), bit_pos: 92 }
        if node_has_class_id(node, 46) {
            intrinsic_matches.set_bit(92); // match_Class("gsdd_a")
        }

        // Instruction 94: CheckAndSetBit { selector: Class("gsfs"), bit_pos: 94 }
        if node_has_class_id(node, 47) {
            intrinsic_matches.set_bit(94); // match_Class("gsfs")
        }

        // Instruction 96: CheckAndSetBit { selector: Class("gsib_a"), bit_pos: 96 }
        if node_has_class_id(node, 48) {
            intrinsic_matches.set_bit(96); // match_Class("gsib_a")
        }

        // Instruction 98: CheckAndSetBit { selector: Class("gsib_b"), bit_pos: 98 }
        if node_has_class_id(node, 49) {
            intrinsic_matches.set_bit(98); // match_Class("gsib_b")
        }

        // Instruction 100: CheckAndSetBit { selector: Class("gsls_a"), bit_pos: 100 }
        if node_has_class_id(node, 50) {
            intrinsic_matches.set_bit(100); // match_Class("gsls_a")
        }

        // Instruction 102: CheckAndSetBit { selector: Class("gsmq_a"), bit_pos: 102 }
        if node_has_class_id(node, 51) {
            intrinsic_matches.set_bit(102); // match_Class("gsmq_a")
        }

        // Instruction 104: CheckAndSetBit { selector: Class("gsn_a"), bit_pos: 104 }
        if node_has_class_id(node, 52) {
            intrinsic_matches.set_bit(104); // match_Class("gsn_a")
        }

        // Instruction 106: CheckAndSetBit { selector: Class("gsn_b"), bit_pos: 106 }
        if node_has_class_id(node, 53) {
            intrinsic_matches.set_bit(106); // match_Class("gsn_b")
        }

        // Instruction 108: CheckAndSetBit { selector: Class("gsn_c"), bit_pos: 108 }
        if node_has_class_id(node, 54) {
            intrinsic_matches.set_bit(108); // match_Class("gsn_c")
        }

        // Instruction 110: CheckAndSetBit { selector: Class("gspqs_b"), bit_pos: 110 }
        if node_has_class_id(node, 55) {
            intrinsic_matches.set_bit(110); // match_Class("gspqs_b")
        }

        // Instruction 112: CheckAndSetBit { selector: Class("gsq_a"), bit_pos: 112 }
        if node_has_class_id(node, 56) {
            intrinsic_matches.set_bit(112); // match_Class("gsq_a")
        }

        // Instruction 114: CheckAndSetBit { selector: Class("gss_ifl"), bit_pos: 114 }
        if node_has_class_id(node, 57) {
            intrinsic_matches.set_bit(114); // match_Class("gss_ifl")
        }

        // Instruction 116: CheckAndSetBit { selector: Class("gssb_a"), bit_pos: 116 }
        if node_has_class_id(node, 58) {
            intrinsic_matches.set_bit(116); // match_Class("gssb_a")
        }

        // Instruction 118: CheckAndSetBit { selector: Class("gssb_c"), bit_pos: 118 }
        if node_has_class_id(node, 59) {
            intrinsic_matches.set_bit(118); // match_Class("gssb_c")
        }

        // Instruction 120: CheckAndSetBit { selector: Class("gssb_e"), bit_pos: 120 }
        if node_has_class_id(node, 60) {
            intrinsic_matches.set_bit(120); // match_Class("gssb_e")
        }

        // Instruction 122: CheckAndSetBit { selector: Class("gssb_f"), bit_pos: 122 }
        if node_has_class_id(node, 61) {
            intrinsic_matches.set_bit(122); // match_Class("gssb_f")
        }

        // Instruction 124: CheckAndSetBit { selector: Class("gssb_g"), bit_pos: 124 }
        if node_has_class_id(node, 62) {
            intrinsic_matches.set_bit(124); // match_Class("gssb_g")
        }

        // Instruction 126: CheckAndSetBit { selector: Class("gssb_h"), bit_pos: 126 }
        if node_has_class_id(node, 63) {
            intrinsic_matches.set_bit(126); // match_Class("gssb_h")
        }

        // Instruction 128: CheckAndSetBit { selector: Class("gssb_i"), bit_pos: 128 }
        if node_has_class_id(node, 64) {
            intrinsic_matches.set_bit(128); // match_Class("gssb_i")
        }

        // Instruction 130: CheckAndSetBit { selector: Class("gssb_k"), bit_pos: 130 }
        if node_has_class_id(node, 65) {
            intrinsic_matches.set_bit(130); // match_Class("gssb_k")
        }

        // Instruction 132: CheckAndSetBit { selector: Class("gssb_l"), bit_pos: 132 }
        if node_has_class_id(node, 66) {
            intrinsic_matches.set_bit(132); // match_Class("gssb_l")
        }

        // Instruction 134: CheckAndSetBit { selector: Class("gssb_m"), bit_pos: 134 }
        if node_has_class_id(node, 67) {
            intrinsic_matches.set_bit(134); // match_Class("gssb_m")
        }

        // Instruction 136: CheckAndSetBit { selector: Class("h"), bit_pos: 136 }
        if node_has_class_id(node, 68) {
            intrinsic_matches.set_bit(136); // match_Class("h")
        }

        // Instruction 138: CheckAndSetBit { selector: Class("lsb"), bit_pos: 138 }
        if node_has_class_id(node, 69) {
            intrinsic_matches.set_bit(138); // match_Class("lsb")
        }

        // Instruction 140: CheckAndSetBit { selector: Class("lsbb"), bit_pos: 140 }
        if node_has_class_id(node, 70) {
            intrinsic_matches.set_bit(140); // match_Class("lsbb")
        }

        // Instruction 142: CheckAndSetBit { selector: Class("lst"), bit_pos: 142 }
        if node_has_class_id(node, 71) {
            intrinsic_matches.set_bit(142); // match_Class("lst")
        }

        // Instruction 144: CheckAndSetBit { selector: Class("sblc"), bit_pos: 144 }
        if node_has_class_id(node, 72) {
            intrinsic_matches.set_bit(144); // match_Class("sblc")
        }

        // Instruction 146: CheckAndSetBit { selector: Class("z4hgWe"), bit_pos: 146 }
        if node_has_class_id(node, 73) {
            intrinsic_matches.set_bit(146); // match_Class("z4hgWe")
        }

match get_node_id_id(node) {
        // Instruction 148: CheckAndSetBit { selector: Id("SIvCob"), bit_pos: 148 }
        Some(74) => {
            intrinsic_matches.set_bit(148); // match_Id("SIvCob")
        }
        // Instruction 150: CheckAndSetBit { selector: Id("gb"), bit_pos: 150 }
        Some(75) => {
            intrinsic_matches.set_bit(150); // match_Id("gb")
        }
        // Instruction 152: CheckAndSetBit { selector: Id("gbb"), bit_pos: 152 }
        Some(76) => {
            intrinsic_matches.set_bit(152); // match_Id("gbb")
        }
        // Instruction 154: CheckAndSetBit { selector: Id("gbbw"), bit_pos: 154 }
        Some(77) => {
            intrinsic_matches.set_bit(154); // match_Id("gbbw")
        }
        // Instruction 156: CheckAndSetBit { selector: Id("gbg"), bit_pos: 156 }
        Some(78) => {
            intrinsic_matches.set_bit(156); // match_Id("gbg")
        }
        // Instruction 158: CheckAndSetBit { selector: Id("gbg5"), bit_pos: 158 }
        Some(79) => {
            intrinsic_matches.set_bit(158); // match_Id("gbg5")
        }
        // Instruction 160: CheckAndSetBit { selector: Id("gbgs5"), bit_pos: 160 }
        Some(80) => {
            intrinsic_matches.set_bit(160); // match_Id("gbgs5")
        }
        // Instruction 162: CheckAndSetBit { selector: Id("gbi4id"), bit_pos: 162 }
        Some(81) => {
            intrinsic_matches.set_bit(162); // match_Id("gbi4id")
        }
        // Instruction 164: CheckAndSetBit { selector: Id("gbi4s1"), bit_pos: 164 }
        Some(82) => {
            intrinsic_matches.set_bit(164); // match_Id("gbi4s1")
        }
        // Instruction 166: CheckAndSetBit { selector: Id("gbi4t"), bit_pos: 166 }
        Some(83) => {
            intrinsic_matches.set_bit(166); // match_Id("gbi4t")
        }
        // Instruction 168: CheckAndSetBit { selector: Id("gbi5"), bit_pos: 168 }
        Some(84) => {
            intrinsic_matches.set_bit(168); // match_Id("gbi5")
        }
        // Instruction 170: CheckAndSetBit { selector: Id("gbmm"), bit_pos: 170 }
        Some(85) => {
            intrinsic_matches.set_bit(170); // match_Id("gbmm")
        }
        // Instruction 172: CheckAndSetBit { selector: Id("gbmpal"), bit_pos: 172 }
        Some(86) => {
            intrinsic_matches.set_bit(172); // match_Id("gbmpal")
        }
        // Instruction 174: CheckAndSetBit { selector: Id("gbmpas"), bit_pos: 174 }
        Some(87) => {
            intrinsic_matches.set_bit(174); // match_Id("gbmpas")
        }
        // Instruction 176: CheckAndSetBit { selector: Id("gbmpdv"), bit_pos: 176 }
        Some(88) => {
            intrinsic_matches.set_bit(176); // match_Id("gbmpdv")
        }
        // Instruction 178: CheckAndSetBit { selector: Id("gbmpid"), bit_pos: 178 }
        Some(89) => {
            intrinsic_matches.set_bit(178); // match_Id("gbmpid")
        }
        // Instruction 180: CheckAndSetBit { selector: Id("gbmpiw"), bit_pos: 180 }
        Some(90) => {
            intrinsic_matches.set_bit(180); // match_Id("gbmpiw")
        }
        // Instruction 182: CheckAndSetBit { selector: Id("gbmps"), bit_pos: 182 }
        Some(91) => {
            intrinsic_matches.set_bit(182); // match_Id("gbmps")
        }
        // Instruction 184: CheckAndSetBit { selector: Id("gbpm"), bit_pos: 184 }
        Some(92) => {
            intrinsic_matches.set_bit(184); // match_Id("gbpm")
        }
        // Instruction 186: CheckAndSetBit { selector: Id("gbpms"), bit_pos: 186 }
        Some(93) => {
            intrinsic_matches.set_bit(186); // match_Id("gbpms")
        }
        // Instruction 188: CheckAndSetBit { selector: Id("gbprca"), bit_pos: 188 }
        Some(94) => {
            intrinsic_matches.set_bit(188); // match_Id("gbprca")
        }
        // Instruction 190: CheckAndSetBit { selector: Id("gbprcs"), bit_pos: 190 }
        Some(95) => {
            intrinsic_matches.set_bit(190); // match_Id("gbprcs")
        }
        // Instruction 192: CheckAndSetBit { selector: Id("gbqfb"), bit_pos: 192 }
        Some(29) => {
            intrinsic_matches.set_bit(192); // match_Id("gbqfb")
        }
        // Instruction 194: CheckAndSetBit { selector: Id("gbqfbw"), bit_pos: 194 }
        Some(96) => {
            intrinsic_matches.set_bit(194); // match_Id("gbqfbw")
        }
        // Instruction 196: CheckAndSetBit { selector: Id("gbs"), bit_pos: 196 }
        Some(97) => {
            intrinsic_matches.set_bit(196); // match_Id("gbs")
        }
        // Instruction 198: CheckAndSetBit { selector: Id("gbx3"), bit_pos: 198 }
        Some(98) => {
            intrinsic_matches.set_bit(198); // match_Id("gbx3")
        }
        // Instruction 200: CheckAndSetBit { selector: Id("gbx4"), bit_pos: 200 }
        Some(99) => {
            intrinsic_matches.set_bit(200); // match_Id("gbx4")
        }
        // Instruction 202: CheckAndSetBit { selector: Id("gbz"), bit_pos: 202 }
        Some(100) => {
            intrinsic_matches.set_bit(202); // match_Id("gbz")
        }
        // Instruction 204: CheckAndSetBit { selector: Id("gog"), bit_pos: 204 }
        Some(101) => {
            intrinsic_matches.set_bit(204); // match_Id("gog")
        }
        // Instruction 206: CheckAndSetBit { selector: Id("gssb_b"), bit_pos: 206 }
        Some(102) => {
            intrinsic_matches.set_bit(206); // match_Id("gssb_b")
        }
        // Instruction 208: CheckAndSetBit { selector: Id("gws-output-pages-elements-homepage_additional_languages__als"), bit_pos: 208 }
        Some(103) => {
            intrinsic_matches.set_bit(208); // match_Id("gws-output-pages-elements-homepage_additional_languages__als")
        }
_ => {}}
        node.cached_node_intrinsic = Some(intrinsic_matches);
    }

    let mut current_matches = node.cached_node_intrinsic.clone().unwrap();
    
    // Track which parent state bits we actually use
    let mut parent_usage_tracker = vec![IState::IUnused; parent_state.capacity];
/// generate_parent_dependent_rules_code
// match get_node_tag_id(node) {
            // match get_node_id_id(node) {
                let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);
/// generate_propagation_rules_code
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Class("H6sW5")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("ds")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("gbg4a")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Class("gbgt")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("gbi4p")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("gbm")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("gbma")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Class("gbmab")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Class("gbmac")
    }
    if current_matches.is_bit_set(18) {
        child_states.set_bit(19); // active_Class("gbmc")
    }
    if current_matches.is_bit_set(20) {
        child_states.set_bit(21); // active_Class("gbmcc")
    }
    if current_matches.is_bit_set(22) {
        child_states.set_bit(23); // active_Class("gbmh")
    }
    if current_matches.is_bit_set(24) {
        child_states.set_bit(25); // active_Class("gbmlbw")
    }
    if current_matches.is_bit_set(26) {
        child_states.set_bit(27); // active_Class("gbmpala")
    }
    if current_matches.is_bit_set(28) {
        child_states.set_bit(29); // active_Class("gbmpalb")
    }
    if current_matches.is_bit_set(30) {
        child_states.set_bit(31); // active_Class("gbmpia")
    }
    if current_matches.is_bit_set(32) {
        child_states.set_bit(33); // active_Class("gbmpiaa")
    }
    if current_matches.is_bit_set(34) {
        child_states.set_bit(35); // active_Class("gbmpiaw")
    }
    if current_matches.is_bit_set(36) {
        child_states.set_bit(37); // active_Class("gbmpnw")
    }
    if current_matches.is_bit_set(38) {
        child_states.set_bit(39); // active_Class("gbmt")
    }
    if current_matches.is_bit_set(40) {
        child_states.set_bit(41); // active_Class("gbmtc")
    }
    if current_matches.is_bit_set(42) {
        child_states.set_bit(43); // active_Class("gbp0")
    }
    if current_matches.is_bit_set(44) {
        child_states.set_bit(45); // active_Class("gbpmc")
    }
    if current_matches.is_bit_set(46) {
        child_states.set_bit(47); // active_Class("gbpms2")
    }
    if current_matches.is_bit_set(48) {
        child_states.set_bit(49); // active_Class("gbprcb")
    }
    if current_matches.is_bit_set(50) {
        child_states.set_bit(51); // active_Class("gbprcd")
    }
    if current_matches.is_bit_set(52) {
        child_states.set_bit(53); // active_Class("gbprci")
    }
    if current_matches.is_bit_set(54) {
        child_states.set_bit(55); // active_Class("gbprct")
    }
    if current_matches.is_bit_set(56) {
        child_states.set_bit(57); // active_Class("gbps2")
    }
    if current_matches.is_bit_set(58) {
        child_states.set_bit(59); // active_Class("gbqfb")
    }
    if current_matches.is_bit_set(60) {
        child_states.set_bit(61); // active_Class("gbqfb-hvr")
    }
    if current_matches.is_bit_set(62) {
        child_states.set_bit(63); // active_Class("gbqfba")
    }
    if current_matches.is_bit_set(64) {
        child_states.set_bit(65); // active_Class("gbqfbb")
    }
    if current_matches.is_bit_set(66) {
        child_states.set_bit(67); // active_Class("gbqfbb-hvr")
    }
    if current_matches.is_bit_set(68) {
        child_states.set_bit(69); // active_Class("gbsb")
    }
    if current_matches.is_bit_set(70) {
        child_states.set_bit(71); // active_Class("gbsbic")
    }
    if current_matches.is_bit_set(72) {
        child_states.set_bit(73); // active_Class("gbt")
    }
    if current_matches.is_bit_set(74) {
        child_states.set_bit(75); // active_Class("gbtb2")
    }
    if current_matches.is_bit_set(76) {
        child_states.set_bit(77); // active_Class("gbtcb")
    }
    if current_matches.is_bit_set(78) {
        child_states.set_bit(79); // active_Class("gbto")
    }
    if current_matches.is_bit_set(80) {
        child_states.set_bit(81); // active_Class("gbts")
    }
    if current_matches.is_bit_set(82) {
        child_states.set_bit(83); // active_Class("gbtsa")
    }
    if current_matches.is_bit_set(84) {
        child_states.set_bit(85); // active_Class("gbxms")
    }
    if current_matches.is_bit_set(86) {
        child_states.set_bit(87); // active_Class("gbxo")
    }
    if current_matches.is_bit_set(88) {
        child_states.set_bit(89); // active_Class("gbxv")
    }
    if current_matches.is_bit_set(90) {
        child_states.set_bit(91); // active_Class("gbxx")
    }
    if current_matches.is_bit_set(92) {
        child_states.set_bit(93); // active_Class("gsdd_a")
    }
    if current_matches.is_bit_set(94) {
        child_states.set_bit(95); // active_Class("gsfs")
    }
    if current_matches.is_bit_set(96) {
        child_states.set_bit(97); // active_Class("gsib_a")
    }
    if current_matches.is_bit_set(98) {
        child_states.set_bit(99); // active_Class("gsib_b")
    }
    if current_matches.is_bit_set(100) {
        child_states.set_bit(101); // active_Class("gsls_a")
    }
    if current_matches.is_bit_set(102) {
        child_states.set_bit(103); // active_Class("gsmq_a")
    }
    if current_matches.is_bit_set(104) {
        child_states.set_bit(105); // active_Class("gsn_a")
    }
    if current_matches.is_bit_set(106) {
        child_states.set_bit(107); // active_Class("gsn_b")
    }
    if current_matches.is_bit_set(108) {
        child_states.set_bit(109); // active_Class("gsn_c")
    }
    if current_matches.is_bit_set(110) {
        child_states.set_bit(111); // active_Class("gspqs_b")
    }
    if current_matches.is_bit_set(112) {
        child_states.set_bit(113); // active_Class("gsq_a")
    }
    if current_matches.is_bit_set(114) {
        child_states.set_bit(115); // active_Class("gss_ifl")
    }
    if current_matches.is_bit_set(116) {
        child_states.set_bit(117); // active_Class("gssb_a")
    }
    if current_matches.is_bit_set(118) {
        child_states.set_bit(119); // active_Class("gssb_c")
    }
    if current_matches.is_bit_set(120) {
        child_states.set_bit(121); // active_Class("gssb_e")
    }
    if current_matches.is_bit_set(122) {
        child_states.set_bit(123); // active_Class("gssb_f")
    }
    if current_matches.is_bit_set(124) {
        child_states.set_bit(125); // active_Class("gssb_g")
    }
    if current_matches.is_bit_set(126) {
        child_states.set_bit(127); // active_Class("gssb_h")
    }
    if current_matches.is_bit_set(128) {
        child_states.set_bit(129); // active_Class("gssb_i")
    }
    if current_matches.is_bit_set(130) {
        child_states.set_bit(131); // active_Class("gssb_k")
    }
    if current_matches.is_bit_set(132) {
        child_states.set_bit(133); // active_Class("gssb_l")
    }
    if current_matches.is_bit_set(134) {
        child_states.set_bit(135); // active_Class("gssb_m")
    }
    if current_matches.is_bit_set(136) {
        child_states.set_bit(137); // active_Class("h")
    }
    if current_matches.is_bit_set(138) {
        child_states.set_bit(139); // active_Class("lsb")
    }
    if current_matches.is_bit_set(140) {
        child_states.set_bit(141); // active_Class("lsbb")
    }
    if current_matches.is_bit_set(142) {
        child_states.set_bit(143); // active_Class("lst")
    }
    if current_matches.is_bit_set(144) {
        child_states.set_bit(145); // active_Class("sblc")
    }
    if current_matches.is_bit_set(146) {
        child_states.set_bit(147); // active_Class("z4hgWe")
    }
    if current_matches.is_bit_set(148) {
        child_states.set_bit(149); // active_Id("SIvCob")
    }
    if current_matches.is_bit_set(150) {
        child_states.set_bit(151); // active_Id("gb")
    }
    if current_matches.is_bit_set(152) {
        child_states.set_bit(153); // active_Id("gbb")
    }
    if current_matches.is_bit_set(154) {
        child_states.set_bit(155); // active_Id("gbbw")
    }
    if current_matches.is_bit_set(156) {
        child_states.set_bit(157); // active_Id("gbg")
    }
    if current_matches.is_bit_set(158) {
        child_states.set_bit(159); // active_Id("gbg5")
    }
    if current_matches.is_bit_set(160) {
        child_states.set_bit(161); // active_Id("gbgs5")
    }
    if current_matches.is_bit_set(162) {
        child_states.set_bit(163); // active_Id("gbi4id")
    }
    if current_matches.is_bit_set(164) {
        child_states.set_bit(165); // active_Id("gbi4s1")
    }
    if current_matches.is_bit_set(166) {
        child_states.set_bit(167); // active_Id("gbi4t")
    }
    if current_matches.is_bit_set(168) {
        child_states.set_bit(169); // active_Id("gbi5")
    }
    if current_matches.is_bit_set(170) {
        child_states.set_bit(171); // active_Id("gbmm")
    }
    if current_matches.is_bit_set(172) {
        child_states.set_bit(173); // active_Id("gbmpal")
    }
    if current_matches.is_bit_set(174) {
        child_states.set_bit(175); // active_Id("gbmpas")
    }
    if current_matches.is_bit_set(176) {
        child_states.set_bit(177); // active_Id("gbmpdv")
    }
    if current_matches.is_bit_set(178) {
        child_states.set_bit(179); // active_Id("gbmpid")
    }
    if current_matches.is_bit_set(180) {
        child_states.set_bit(181); // active_Id("gbmpiw")
    }
    if current_matches.is_bit_set(182) {
        child_states.set_bit(183); // active_Id("gbmps")
    }
    if current_matches.is_bit_set(184) {
        child_states.set_bit(185); // active_Id("gbpm")
    }
    if current_matches.is_bit_set(186) {
        child_states.set_bit(187); // active_Id("gbpms")
    }
    if current_matches.is_bit_set(188) {
        child_states.set_bit(189); // active_Id("gbprca")
    }
    if current_matches.is_bit_set(190) {
        child_states.set_bit(191); // active_Id("gbprcs")
    }
    if current_matches.is_bit_set(192) {
        child_states.set_bit(193); // active_Id("gbqfb")
    }
    if current_matches.is_bit_set(194) {
        child_states.set_bit(195); // active_Id("gbqfbw")
    }
    if current_matches.is_bit_set(196) {
        child_states.set_bit(197); // active_Id("gbs")
    }
    if current_matches.is_bit_set(198) {
        child_states.set_bit(199); // active_Id("gbx3")
    }
    if current_matches.is_bit_set(200) {
        child_states.set_bit(201); // active_Id("gbx4")
    }
    if current_matches.is_bit_set(202) {
        child_states.set_bit(203); // active_Id("gbz")
    }
    if current_matches.is_bit_set(204) {
        child_states.set_bit(205); // active_Id("gog")
    }
    if current_matches.is_bit_set(206) {
        child_states.set_bit(207); // active_Id("gssb_b")
    }
    if current_matches.is_bit_set(208) {
        child_states.set_bit(209); // active_Id("gws-output-pages-elements-homepage_additional_languages__als")
    }
    if current_matches.is_bit_set(210) {
        child_states.set_bit(211); // active_Type("a")
    }
    if current_matches.is_bit_set(212) {
        child_states.set_bit(213); // active_Type("body")
    }
    if current_matches.is_bit_set(214) {
        child_states.set_bit(215); // active_Type("input")
    }
    if current_matches.is_bit_set(216) {
        child_states.set_bit(217); // active_Type("td")
    }
    node.css_match_bitvector = current_matches;
            node.cached_parent_state = Some(parent_usage_tracker);
            node.cached_child_states = Some(child_states.clone());
            node.mark_clean();
            child_states
        }
/// Incremental processing driver with statistics tracking
pub fn process_tree_incremental_with_stats(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive_incremental(root, &initial_state, &mut total_nodes, &mut cache_hits, &mut cache_misses);
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_incremental(node: &mut HtmlNode, parent_state: &BitVector,
                                    total: &mut usize, hits: &mut usize, misses: &mut usize) {
    *total += 1;
    
    // Logic 1: Check if node itself needs recomputation
    let child_states = if node.needs_self_recomputation(parent_state) {
        *misses += 1;
        // Recompute node and get fresh child_states
        process_node_generated_incremental(node, parent_state)
    } else {
        *hits += 1;
        // Use cached child_states - major optimization for internal nodes!
        node.cached_child_states.clone().unwrap_or_else(|| BitVector::with_capacity(BITVECTOR_CAPACITY))
    };
    
    // Logic 2: Check if we need to recurse (only if there are dirty descendants)
    if node.has_dirty_descendant {
        // Recurse into children only if there are dirty descendants
        for child in node.children.iter_mut() {
            process_tree_recursive_incremental(child, &child_states, total, hits, misses);
        }
    }
    // If no dirty descendants, skip entire subtree recursion - major optimization!
}
