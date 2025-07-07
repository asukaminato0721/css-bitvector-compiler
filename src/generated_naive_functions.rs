use css_bitvector_compiler::{HtmlNode, SimpleSelector};
use std::collections::HashMap;

// === NAIVE CSS MATCHING FUNCTIONS ===
// These functions calculate layout from scratch without any caching

// Rule 0: CheckAndSetBit { selector: Class("H6sW5"), bit_pos: 0 }
pub fn matches_rule_0(node: &HtmlNode) -> bool {
    node.classes.contains("H6sW5")
}

// Rule 2: CheckAndSetBit { selector: Class("ds"), bit_pos: 2 }
pub fn matches_rule_2(node: &HtmlNode) -> bool {
    node.classes.contains("ds")
}

// Rule 4: CheckAndSetBit { selector: Class("gbg4a"), bit_pos: 4 }
pub fn matches_rule_4(node: &HtmlNode) -> bool {
    node.classes.contains("gbg4a")
}

// Rule 6: CheckAndSetBit { selector: Class("gbgt"), bit_pos: 6 }
pub fn matches_rule_6(node: &HtmlNode) -> bool {
    node.classes.contains("gbgt")
}

// Rule 8: CheckAndSetBit { selector: Class("gbi4p"), bit_pos: 8 }
pub fn matches_rule_8(node: &HtmlNode) -> bool {
    node.classes.contains("gbi4p")
}

// Rule 10: CheckAndSetBit { selector: Class("gbm"), bit_pos: 10 }
pub fn matches_rule_10(node: &HtmlNode) -> bool {
    node.classes.contains("gbm")
}

// Rule 12: CheckAndSetBit { selector: Class("gbma"), bit_pos: 12 }
pub fn matches_rule_12(node: &HtmlNode) -> bool {
    node.classes.contains("gbma")
}

// Rule 14: CheckAndSetBit { selector: Class("gbmab"), bit_pos: 14 }
pub fn matches_rule_14(node: &HtmlNode) -> bool {
    node.classes.contains("gbmab")
}

// Rule 16: CheckAndSetBit { selector: Class("gbmac"), bit_pos: 16 }
pub fn matches_rule_16(node: &HtmlNode) -> bool {
    node.classes.contains("gbmac")
}

// Rule 18: CheckAndSetBit { selector: Class("gbmc"), bit_pos: 18 }
pub fn matches_rule_18(node: &HtmlNode) -> bool {
    node.classes.contains("gbmc")
}

// Rule 20: CheckAndSetBit { selector: Class("gbmcc"), bit_pos: 20 }
pub fn matches_rule_20(node: &HtmlNode) -> bool {
    node.classes.contains("gbmcc")
}

// Rule 22: CheckAndSetBit { selector: Class("gbmh"), bit_pos: 22 }
pub fn matches_rule_22(node: &HtmlNode) -> bool {
    node.classes.contains("gbmh")
}

// Rule 24: CheckAndSetBit { selector: Class("gbmlbw"), bit_pos: 24 }
pub fn matches_rule_24(node: &HtmlNode) -> bool {
    node.classes.contains("gbmlbw")
}

// Rule 26: CheckAndSetBit { selector: Class("gbmpala"), bit_pos: 26 }
pub fn matches_rule_26(node: &HtmlNode) -> bool {
    node.classes.contains("gbmpala")
}

// Rule 28: CheckAndSetBit { selector: Class("gbmpalb"), bit_pos: 28 }
pub fn matches_rule_28(node: &HtmlNode) -> bool {
    node.classes.contains("gbmpalb")
}

// Rule 30: CheckAndSetBit { selector: Class("gbmpia"), bit_pos: 30 }
pub fn matches_rule_30(node: &HtmlNode) -> bool {
    node.classes.contains("gbmpia")
}

// Rule 32: CheckAndSetBit { selector: Class("gbmpiaa"), bit_pos: 32 }
pub fn matches_rule_32(node: &HtmlNode) -> bool {
    node.classes.contains("gbmpiaa")
}

// Rule 34: CheckAndSetBit { selector: Class("gbmpiaw"), bit_pos: 34 }
pub fn matches_rule_34(node: &HtmlNode) -> bool {
    node.classes.contains("gbmpiaw")
}

// Rule 36: CheckAndSetBit { selector: Class("gbmpnw"), bit_pos: 36 }
pub fn matches_rule_36(node: &HtmlNode) -> bool {
    node.classes.contains("gbmpnw")
}

// Rule 38: CheckAndSetBit { selector: Class("gbmt"), bit_pos: 38 }
pub fn matches_rule_38(node: &HtmlNode) -> bool {
    node.classes.contains("gbmt")
}

// Rule 40: CheckAndSetBit { selector: Class("gbmtc"), bit_pos: 40 }
pub fn matches_rule_40(node: &HtmlNode) -> bool {
    node.classes.contains("gbmtc")
}

// Rule 42: CheckAndSetBit { selector: Class("gbp0"), bit_pos: 42 }
pub fn matches_rule_42(node: &HtmlNode) -> bool {
    node.classes.contains("gbp0")
}

// Rule 44: CheckAndSetBit { selector: Class("gbpmc"), bit_pos: 44 }
pub fn matches_rule_44(node: &HtmlNode) -> bool {
    node.classes.contains("gbpmc")
}

// Rule 46: CheckAndSetBit { selector: Class("gbpms2"), bit_pos: 46 }
pub fn matches_rule_46(node: &HtmlNode) -> bool {
    node.classes.contains("gbpms2")
}

// Rule 48: CheckAndSetBit { selector: Class("gbprcb"), bit_pos: 48 }
pub fn matches_rule_48(node: &HtmlNode) -> bool {
    node.classes.contains("gbprcb")
}

// Rule 50: CheckAndSetBit { selector: Class("gbprcd"), bit_pos: 50 }
pub fn matches_rule_50(node: &HtmlNode) -> bool {
    node.classes.contains("gbprcd")
}

// Rule 52: CheckAndSetBit { selector: Class("gbprci"), bit_pos: 52 }
pub fn matches_rule_52(node: &HtmlNode) -> bool {
    node.classes.contains("gbprci")
}

// Rule 54: CheckAndSetBit { selector: Class("gbprct"), bit_pos: 54 }
pub fn matches_rule_54(node: &HtmlNode) -> bool {
    node.classes.contains("gbprct")
}

// Rule 56: CheckAndSetBit { selector: Class("gbps2"), bit_pos: 56 }
pub fn matches_rule_56(node: &HtmlNode) -> bool {
    node.classes.contains("gbps2")
}

// Rule 58: CheckAndSetBit { selector: Class("gbqfb"), bit_pos: 58 }
pub fn matches_rule_58(node: &HtmlNode) -> bool {
    node.classes.contains("gbqfb")
}

// Rule 60: CheckAndSetBit { selector: Class("gbqfb-hvr"), bit_pos: 60 }
pub fn matches_rule_60(node: &HtmlNode) -> bool {
    node.classes.contains("gbqfb-hvr")
}

// Rule 62: CheckAndSetBit { selector: Class("gbqfba"), bit_pos: 62 }
pub fn matches_rule_62(node: &HtmlNode) -> bool {
    node.classes.contains("gbqfba")
}

// Rule 64: CheckAndSetBit { selector: Class("gbqfbb"), bit_pos: 64 }
pub fn matches_rule_64(node: &HtmlNode) -> bool {
    node.classes.contains("gbqfbb")
}

// Rule 66: CheckAndSetBit { selector: Class("gbqfbb-hvr"), bit_pos: 66 }
pub fn matches_rule_66(node: &HtmlNode) -> bool {
    node.classes.contains("gbqfbb-hvr")
}

// Rule 68: CheckAndSetBit { selector: Class("gbsb"), bit_pos: 68 }
pub fn matches_rule_68(node: &HtmlNode) -> bool {
    node.classes.contains("gbsb")
}

// Rule 70: CheckAndSetBit { selector: Class("gbsbic"), bit_pos: 70 }
pub fn matches_rule_70(node: &HtmlNode) -> bool {
    node.classes.contains("gbsbic")
}

// Rule 72: CheckAndSetBit { selector: Class("gbt"), bit_pos: 72 }
pub fn matches_rule_72(node: &HtmlNode) -> bool {
    node.classes.contains("gbt")
}

// Rule 74: CheckAndSetBit { selector: Class("gbtb2"), bit_pos: 74 }
pub fn matches_rule_74(node: &HtmlNode) -> bool {
    node.classes.contains("gbtb2")
}

// Rule 76: CheckAndSetBit { selector: Class("gbtcb"), bit_pos: 76 }
pub fn matches_rule_76(node: &HtmlNode) -> bool {
    node.classes.contains("gbtcb")
}

// Rule 78: CheckAndSetBit { selector: Class("gbto"), bit_pos: 78 }
pub fn matches_rule_78(node: &HtmlNode) -> bool {
    node.classes.contains("gbto")
}

// Rule 80: CheckAndSetBit { selector: Class("gbts"), bit_pos: 80 }
pub fn matches_rule_80(node: &HtmlNode) -> bool {
    node.classes.contains("gbts")
}

// Rule 82: CheckAndSetBit { selector: Class("gbtsa"), bit_pos: 82 }
pub fn matches_rule_82(node: &HtmlNode) -> bool {
    node.classes.contains("gbtsa")
}

// Rule 84: CheckAndSetBit { selector: Class("gbxms"), bit_pos: 84 }
pub fn matches_rule_84(node: &HtmlNode) -> bool {
    node.classes.contains("gbxms")
}

// Rule 86: CheckAndSetBit { selector: Class("gbxo"), bit_pos: 86 }
pub fn matches_rule_86(node: &HtmlNode) -> bool {
    node.classes.contains("gbxo")
}

// Rule 88: CheckAndSetBit { selector: Class("gbxv"), bit_pos: 88 }
pub fn matches_rule_88(node: &HtmlNode) -> bool {
    node.classes.contains("gbxv")
}

// Rule 90: CheckAndSetBit { selector: Class("gbxx"), bit_pos: 90 }
pub fn matches_rule_90(node: &HtmlNode) -> bool {
    node.classes.contains("gbxx")
}

// Rule 92: CheckAndSetBit { selector: Class("gsdd_a"), bit_pos: 92 }
pub fn matches_rule_92(node: &HtmlNode) -> bool {
    node.classes.contains("gsdd_a")
}

// Rule 94: CheckAndSetBit { selector: Class("gsfs"), bit_pos: 94 }
pub fn matches_rule_94(node: &HtmlNode) -> bool {
    node.classes.contains("gsfs")
}

// Rule 96: CheckAndSetBit { selector: Class("gsib_a"), bit_pos: 96 }
pub fn matches_rule_96(node: &HtmlNode) -> bool {
    node.classes.contains("gsib_a")
}

// Rule 98: CheckAndSetBit { selector: Class("gsib_b"), bit_pos: 98 }
pub fn matches_rule_98(node: &HtmlNode) -> bool {
    node.classes.contains("gsib_b")
}

// Rule 100: CheckAndSetBit { selector: Class("gsls_a"), bit_pos: 100 }
pub fn matches_rule_100(node: &HtmlNode) -> bool {
    node.classes.contains("gsls_a")
}

// Rule 102: CheckAndSetBit { selector: Class("gsmq_a"), bit_pos: 102 }
pub fn matches_rule_102(node: &HtmlNode) -> bool {
    node.classes.contains("gsmq_a")
}

// Rule 104: CheckAndSetBit { selector: Class("gsn_a"), bit_pos: 104 }
pub fn matches_rule_104(node: &HtmlNode) -> bool {
    node.classes.contains("gsn_a")
}

// Rule 106: CheckAndSetBit { selector: Class("gsn_b"), bit_pos: 106 }
pub fn matches_rule_106(node: &HtmlNode) -> bool {
    node.classes.contains("gsn_b")
}

// Rule 108: CheckAndSetBit { selector: Class("gsn_c"), bit_pos: 108 }
pub fn matches_rule_108(node: &HtmlNode) -> bool {
    node.classes.contains("gsn_c")
}

// Rule 110: CheckAndSetBit { selector: Class("gspqs_b"), bit_pos: 110 }
pub fn matches_rule_110(node: &HtmlNode) -> bool {
    node.classes.contains("gspqs_b")
}

// Rule 112: CheckAndSetBit { selector: Class("gsq_a"), bit_pos: 112 }
pub fn matches_rule_112(node: &HtmlNode) -> bool {
    node.classes.contains("gsq_a")
}

// Rule 114: CheckAndSetBit { selector: Class("gss_ifl"), bit_pos: 114 }
pub fn matches_rule_114(node: &HtmlNode) -> bool {
    node.classes.contains("gss_ifl")
}

// Rule 116: CheckAndSetBit { selector: Class("gssb_a"), bit_pos: 116 }
pub fn matches_rule_116(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_a")
}

// Rule 118: CheckAndSetBit { selector: Class("gssb_c"), bit_pos: 118 }
pub fn matches_rule_118(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_c")
}

// Rule 120: CheckAndSetBit { selector: Class("gssb_e"), bit_pos: 120 }
pub fn matches_rule_120(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_e")
}

// Rule 122: CheckAndSetBit { selector: Class("gssb_f"), bit_pos: 122 }
pub fn matches_rule_122(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_f")
}

// Rule 124: CheckAndSetBit { selector: Class("gssb_g"), bit_pos: 124 }
pub fn matches_rule_124(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_g")
}

// Rule 126: CheckAndSetBit { selector: Class("gssb_h"), bit_pos: 126 }
pub fn matches_rule_126(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_h")
}

// Rule 128: CheckAndSetBit { selector: Class("gssb_i"), bit_pos: 128 }
pub fn matches_rule_128(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_i")
}

// Rule 130: CheckAndSetBit { selector: Class("gssb_k"), bit_pos: 130 }
pub fn matches_rule_130(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_k")
}

// Rule 132: CheckAndSetBit { selector: Class("gssb_l"), bit_pos: 132 }
pub fn matches_rule_132(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_l")
}

// Rule 134: CheckAndSetBit { selector: Class("gssb_m"), bit_pos: 134 }
pub fn matches_rule_134(node: &HtmlNode) -> bool {
    node.classes.contains("gssb_m")
}

// Rule 136: CheckAndSetBit { selector: Class("h"), bit_pos: 136 }
pub fn matches_rule_136(node: &HtmlNode) -> bool {
    node.classes.contains("h")
}

// Rule 138: CheckAndSetBit { selector: Class("lsb"), bit_pos: 138 }
pub fn matches_rule_138(node: &HtmlNode) -> bool {
    node.classes.contains("lsb")
}

// Rule 140: CheckAndSetBit { selector: Class("lsbb"), bit_pos: 140 }
pub fn matches_rule_140(node: &HtmlNode) -> bool {
    node.classes.contains("lsbb")
}

// Rule 142: CheckAndSetBit { selector: Class("lst"), bit_pos: 142 }
pub fn matches_rule_142(node: &HtmlNode) -> bool {
    node.classes.contains("lst")
}

// Rule 144: CheckAndSetBit { selector: Class("sblc"), bit_pos: 144 }
pub fn matches_rule_144(node: &HtmlNode) -> bool {
    node.classes.contains("sblc")
}

// Rule 146: CheckAndSetBit { selector: Class("z4hgWe"), bit_pos: 146 }
pub fn matches_rule_146(node: &HtmlNode) -> bool {
    node.classes.contains("z4hgWe")
}

// Rule 148: CheckAndSetBit { selector: Id("SIvCob"), bit_pos: 148 }
pub fn matches_rule_148(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"SIvCob".to_string())
}

// Rule 150: CheckAndSetBit { selector: Id("gb"), bit_pos: 150 }
pub fn matches_rule_150(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gb".to_string())
}

// Rule 152: CheckAndSetBit { selector: Id("gbb"), bit_pos: 152 }
pub fn matches_rule_152(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbb".to_string())
}

// Rule 154: CheckAndSetBit { selector: Id("gbbw"), bit_pos: 154 }
pub fn matches_rule_154(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbbw".to_string())
}

// Rule 156: CheckAndSetBit { selector: Id("gbg"), bit_pos: 156 }
pub fn matches_rule_156(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbg".to_string())
}

// Rule 158: CheckAndSetBit { selector: Id("gbg5"), bit_pos: 158 }
pub fn matches_rule_158(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbg5".to_string())
}

// Rule 160: CheckAndSetBit { selector: Id("gbgs5"), bit_pos: 160 }
pub fn matches_rule_160(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbgs5".to_string())
}

// Rule 162: CheckAndSetBit { selector: Id("gbi4id"), bit_pos: 162 }
pub fn matches_rule_162(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbi4id".to_string())
}

// Rule 164: CheckAndSetBit { selector: Id("gbi4s1"), bit_pos: 164 }
pub fn matches_rule_164(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbi4s1".to_string())
}

// Rule 166: CheckAndSetBit { selector: Id("gbi4t"), bit_pos: 166 }
pub fn matches_rule_166(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbi4t".to_string())
}

// Rule 168: CheckAndSetBit { selector: Id("gbi5"), bit_pos: 168 }
pub fn matches_rule_168(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbi5".to_string())
}

// Rule 170: CheckAndSetBit { selector: Id("gbmm"), bit_pos: 170 }
pub fn matches_rule_170(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbmm".to_string())
}

// Rule 172: CheckAndSetBit { selector: Id("gbmpal"), bit_pos: 172 }
pub fn matches_rule_172(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbmpal".to_string())
}

// Rule 174: CheckAndSetBit { selector: Id("gbmpas"), bit_pos: 174 }
pub fn matches_rule_174(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbmpas".to_string())
}

// Rule 176: CheckAndSetBit { selector: Id("gbmpdv"), bit_pos: 176 }
pub fn matches_rule_176(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbmpdv".to_string())
}

// Rule 178: CheckAndSetBit { selector: Id("gbmpid"), bit_pos: 178 }
pub fn matches_rule_178(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbmpid".to_string())
}

// Rule 180: CheckAndSetBit { selector: Id("gbmpiw"), bit_pos: 180 }
pub fn matches_rule_180(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbmpiw".to_string())
}

// Rule 182: CheckAndSetBit { selector: Id("gbmps"), bit_pos: 182 }
pub fn matches_rule_182(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbmps".to_string())
}

// Rule 184: CheckAndSetBit { selector: Id("gbpm"), bit_pos: 184 }
pub fn matches_rule_184(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbpm".to_string())
}

// Rule 186: CheckAndSetBit { selector: Id("gbpms"), bit_pos: 186 }
pub fn matches_rule_186(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbpms".to_string())
}

// Rule 188: CheckAndSetBit { selector: Id("gbprca"), bit_pos: 188 }
pub fn matches_rule_188(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbprca".to_string())
}

// Rule 190: CheckAndSetBit { selector: Id("gbprcs"), bit_pos: 190 }
pub fn matches_rule_190(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbprcs".to_string())
}

// Rule 192: CheckAndSetBit { selector: Id("gbqfb"), bit_pos: 192 }
pub fn matches_rule_192(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbqfb".to_string())
}

// Rule 194: CheckAndSetBit { selector: Id("gbqfbw"), bit_pos: 194 }
pub fn matches_rule_194(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbqfbw".to_string())
}

// Rule 196: CheckAndSetBit { selector: Id("gbs"), bit_pos: 196 }
pub fn matches_rule_196(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbs".to_string())
}

// Rule 198: CheckAndSetBit { selector: Id("gbx3"), bit_pos: 198 }
pub fn matches_rule_198(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbx3".to_string())
}

// Rule 200: CheckAndSetBit { selector: Id("gbx4"), bit_pos: 200 }
pub fn matches_rule_200(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbx4".to_string())
}

// Rule 202: CheckAndSetBit { selector: Id("gbz"), bit_pos: 202 }
pub fn matches_rule_202(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gbz".to_string())
}

// Rule 204: CheckAndSetBit { selector: Id("gog"), bit_pos: 204 }
pub fn matches_rule_204(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gog".to_string())
}

// Rule 206: CheckAndSetBit { selector: Id("gssb_b"), bit_pos: 206 }
pub fn matches_rule_206(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gssb_b".to_string())
}

// Rule 208: CheckAndSetBit { selector: Id("gws-output-pages-elements-homepage_additional_languages__als"), bit_pos: 208 }
pub fn matches_rule_208(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"gws-output-pages-elements-homepage_additional_languages__als".to_string())
}

// Rule 210: CheckAndSetBit { selector: Type("a"), bit_pos: 210 }
pub fn matches_rule_210(node: &HtmlNode) -> bool {
    node.tag_name == "a"
}

// Rule 212: CheckAndSetBit { selector: Type("body"), bit_pos: 212 }
pub fn matches_rule_212(node: &HtmlNode) -> bool {
    node.tag_name == "body"
}

// Rule 214: CheckAndSetBit { selector: Type("div"), bit_pos: 214 }
pub fn matches_rule_214(node: &HtmlNode) -> bool {
    node.tag_name == "div"
}

// Rule 216: CheckAndSetBit { selector: Type("input"), bit_pos: 216 }
pub fn matches_rule_216(node: &HtmlNode) -> bool {
    node.tag_name == "input"
}

// Rule 218: CheckAndSetBit { selector: Type("span"), bit_pos: 218 }
pub fn matches_rule_218(node: &HtmlNode) -> bool {
    node.tag_name == "span"
}

// Rule 220: CheckAndSetBit { selector: Type("td"), bit_pos: 220 }
pub fn matches_rule_220(node: &HtmlNode) -> bool {
    node.tag_name == "td"
}

// === MAIN NAIVE PROCESSING FUNCTION ===
pub fn process_node_naive(node: &mut HtmlNode, parent_matches: &[bool]) -> Vec<bool> {
    let mut matches = vec![false; 222];
    
    // Check all simple selectors
    if matches_rule_0(node) {
        matches[0] = true;
    }
    if matches_rule_2(node) {
        matches[2] = true;
    }
    if matches_rule_4(node) {
        matches[4] = true;
    }
    if matches_rule_6(node) {
        matches[6] = true;
    }
    if matches_rule_8(node) {
        matches[8] = true;
    }
    if matches_rule_10(node) {
        matches[10] = true;
    }
    if matches_rule_12(node) {
        matches[12] = true;
    }
    if matches_rule_14(node) {
        matches[14] = true;
    }
    if matches_rule_16(node) {
        matches[16] = true;
    }
    if matches_rule_18(node) {
        matches[18] = true;
    }
    if matches_rule_20(node) {
        matches[20] = true;
    }
    if matches_rule_22(node) {
        matches[22] = true;
    }
    if matches_rule_24(node) {
        matches[24] = true;
    }
    if matches_rule_26(node) {
        matches[26] = true;
    }
    if matches_rule_28(node) {
        matches[28] = true;
    }
    if matches_rule_30(node) {
        matches[30] = true;
    }
    if matches_rule_32(node) {
        matches[32] = true;
    }
    if matches_rule_34(node) {
        matches[34] = true;
    }
    if matches_rule_36(node) {
        matches[36] = true;
    }
    if matches_rule_38(node) {
        matches[38] = true;
    }
    if matches_rule_40(node) {
        matches[40] = true;
    }
    if matches_rule_42(node) {
        matches[42] = true;
    }
    if matches_rule_44(node) {
        matches[44] = true;
    }
    if matches_rule_46(node) {
        matches[46] = true;
    }
    if matches_rule_48(node) {
        matches[48] = true;
    }
    if matches_rule_50(node) {
        matches[50] = true;
    }
    if matches_rule_52(node) {
        matches[52] = true;
    }
    if matches_rule_54(node) {
        matches[54] = true;
    }
    if matches_rule_56(node) {
        matches[56] = true;
    }
    if matches_rule_58(node) {
        matches[58] = true;
    }
    if matches_rule_60(node) {
        matches[60] = true;
    }
    if matches_rule_62(node) {
        matches[62] = true;
    }
    if matches_rule_64(node) {
        matches[64] = true;
    }
    if matches_rule_66(node) {
        matches[66] = true;
    }
    if matches_rule_68(node) {
        matches[68] = true;
    }
    if matches_rule_70(node) {
        matches[70] = true;
    }
    if matches_rule_72(node) {
        matches[72] = true;
    }
    if matches_rule_74(node) {
        matches[74] = true;
    }
    if matches_rule_76(node) {
        matches[76] = true;
    }
    if matches_rule_78(node) {
        matches[78] = true;
    }
    if matches_rule_80(node) {
        matches[80] = true;
    }
    if matches_rule_82(node) {
        matches[82] = true;
    }
    if matches_rule_84(node) {
        matches[84] = true;
    }
    if matches_rule_86(node) {
        matches[86] = true;
    }
    if matches_rule_88(node) {
        matches[88] = true;
    }
    if matches_rule_90(node) {
        matches[90] = true;
    }
    if matches_rule_92(node) {
        matches[92] = true;
    }
    if matches_rule_94(node) {
        matches[94] = true;
    }
    if matches_rule_96(node) {
        matches[96] = true;
    }
    if matches_rule_98(node) {
        matches[98] = true;
    }
    if matches_rule_100(node) {
        matches[100] = true;
    }
    if matches_rule_102(node) {
        matches[102] = true;
    }
    if matches_rule_104(node) {
        matches[104] = true;
    }
    if matches_rule_106(node) {
        matches[106] = true;
    }
    if matches_rule_108(node) {
        matches[108] = true;
    }
    if matches_rule_110(node) {
        matches[110] = true;
    }
    if matches_rule_112(node) {
        matches[112] = true;
    }
    if matches_rule_114(node) {
        matches[114] = true;
    }
    if matches_rule_116(node) {
        matches[116] = true;
    }
    if matches_rule_118(node) {
        matches[118] = true;
    }
    if matches_rule_120(node) {
        matches[120] = true;
    }
    if matches_rule_122(node) {
        matches[122] = true;
    }
    if matches_rule_124(node) {
        matches[124] = true;
    }
    if matches_rule_126(node) {
        matches[126] = true;
    }
    if matches_rule_128(node) {
        matches[128] = true;
    }
    if matches_rule_130(node) {
        matches[130] = true;
    }
    if matches_rule_132(node) {
        matches[132] = true;
    }
    if matches_rule_134(node) {
        matches[134] = true;
    }
    if matches_rule_136(node) {
        matches[136] = true;
    }
    if matches_rule_138(node) {
        matches[138] = true;
    }
    if matches_rule_140(node) {
        matches[140] = true;
    }
    if matches_rule_142(node) {
        matches[142] = true;
    }
    if matches_rule_144(node) {
        matches[144] = true;
    }
    if matches_rule_146(node) {
        matches[146] = true;
    }
    if matches_rule_148(node) {
        matches[148] = true;
    }
    if matches_rule_150(node) {
        matches[150] = true;
    }
    if matches_rule_152(node) {
        matches[152] = true;
    }
    if matches_rule_154(node) {
        matches[154] = true;
    }
    if matches_rule_156(node) {
        matches[156] = true;
    }
    if matches_rule_158(node) {
        matches[158] = true;
    }
    if matches_rule_160(node) {
        matches[160] = true;
    }
    if matches_rule_162(node) {
        matches[162] = true;
    }
    if matches_rule_164(node) {
        matches[164] = true;
    }
    if matches_rule_166(node) {
        matches[166] = true;
    }
    if matches_rule_168(node) {
        matches[168] = true;
    }
    if matches_rule_170(node) {
        matches[170] = true;
    }
    if matches_rule_172(node) {
        matches[172] = true;
    }
    if matches_rule_174(node) {
        matches[174] = true;
    }
    if matches_rule_176(node) {
        matches[176] = true;
    }
    if matches_rule_178(node) {
        matches[178] = true;
    }
    if matches_rule_180(node) {
        matches[180] = true;
    }
    if matches_rule_182(node) {
        matches[182] = true;
    }
    if matches_rule_184(node) {
        matches[184] = true;
    }
    if matches_rule_186(node) {
        matches[186] = true;
    }
    if matches_rule_188(node) {
        matches[188] = true;
    }
    if matches_rule_190(node) {
        matches[190] = true;
    }
    if matches_rule_192(node) {
        matches[192] = true;
    }
    if matches_rule_194(node) {
        matches[194] = true;
    }
    if matches_rule_196(node) {
        matches[196] = true;
    }
    if matches_rule_198(node) {
        matches[198] = true;
    }
    if matches_rule_200(node) {
        matches[200] = true;
    }
    if matches_rule_202(node) {
        matches[202] = true;
    }
    if matches_rule_204(node) {
        matches[204] = true;
    }
    if matches_rule_206(node) {
        matches[206] = true;
    }
    if matches_rule_208(node) {
        matches[208] = true;
    }
    if matches_rule_210(node) {
        matches[210] = true;
    }
    if matches_rule_212(node) {
        matches[212] = true;
    }
    if matches_rule_214(node) {
        matches[214] = true;
    }
    if matches_rule_216(node) {
        matches[216] = true;
    }
    if matches_rule_218(node) {
        matches[218] = true;
    }
    if matches_rule_220(node) {
        matches[220] = true;
    }
    
    // Check all parent-child rules
    // No parent-child rules to check
    let _ = parent_matches; // Suppress unused parameter warning
    
    matches
}

// === NAIVE TREE TRAVERSAL ===
pub fn process_tree_naive(root: &mut HtmlNode) -> usize {
    let mut total_nodes = 0;
    let empty_parent = vec![false; 222];
    process_tree_recursive_naive(root, &empty_parent, &mut total_nodes);
    total_nodes
}

fn process_tree_recursive_naive(node: &mut HtmlNode, parent_matches: &[bool], total: &mut usize) {
    *total += 1;
    
    // Calculate matches for this node from scratch
    let node_matches = process_node_naive(node, parent_matches);
    
    // Process all children with this node's matches as their parent context
    for child in node.children.iter_mut() {
        process_tree_recursive_naive(child, &node_matches, total);
    }
}

// === HELPER FUNCTIONS ===
pub fn get_rule_name(rule_index: usize) -> String {
    format!("rule_{}", rule_index)
}

// Rule mapping:
// Rule 151: active_Id("gb")
// Rule 153: active_Id("gbb")
// Rule 99: active_Class("gsib_b")
// Rule 158: match_Id("gbg5")
// Rule 159: active_Id("gbg5")
// Rule 162: match_Id("gbi4id")
// Rule 103: active_Class("gsmq_a")
// Rule 163: active_Id("gbi4id")
// Rule 89: active_Class("gbxv")
// Rule 169: active_Id("gbi5")
// Rule 81: active_Class("gbts")
// Rule 115: active_Class("gss_ifl")
// Rule 15: active_Class("gbmab")
// Rule 131: active_Class("gssb_k")
// Rule 164: match_Id("gbi4s1")
// Rule 4: match_Class("gbg4a")
// Rule 106: match_Class("gsn_b")
// Rule 107: active_Class("gsn_b")
// Rule 64: match_Class("gbqfbb")
// Rule 52: match_Class("gbprci")
// Rule 109: active_Class("gsn_c")
// Rule 123: active_Class("gssb_f")
// Rule 20: match_Class("gbmcc")
// Rule 139: active_Class("lsb")
// Rule 170: match_Id("gbmm")
// Rule 186: match_Id("gbpms")
// Rule 202: match_Id("gbz")
// Rule 44: match_Class("gbpmc")
// Rule 10: match_Class("gbm")
// Rule 2: match_Class("ds")
// Rule 181: active_Id("gbmpiw")
// Rule 182: match_Id("gbmps")
// Rule 147: active_Class("z4hgWe")
// Rule 35: active_Class("gbmpiaw")
// Rule 37: active_Class("gbmpnw")
// Rule 16: match_Class("gbmac")
// Rule 60: match_Class("gbqfb-hvr")
// Rule 11: active_Class("gbm")
// Rule 61: active_Class("gbqfb-hvr")
// Rule 101: active_Class("gsls_a")
// Rule 114: match_Class("gss_ifl")
// Rule 78: match_Class("gbto")
// Rule 154: match_Id("gbbw")
// Rule 31: active_Class("gbmpia")
// Rule 62: match_Class("gbqfba")
// Rule 93: active_Class("gsdd_a")
// Rule 171: active_Id("gbmm")
// Rule 57: active_Class("gbps2")
// Rule 66: match_Class("gbqfbb-hvr")
// Rule 175: active_Id("gbmpas")
// Rule 79: active_Class("gbto")
// Rule 183: active_Id("gbmps")
// Rule 63: active_Class("gbqfba")
// Rule 189: active_Id("gbprca")
// Rule 48: match_Class("gbprcb")
// Rule 6: match_Class("gbgt")
// Rule 105: active_Class("gsn_a")
// Rule 25: active_Class("gbmlbw")
// Rule 206: match_Id("gssb_b")
// Rule 213: active_Type("body")
// Rule 214: match_Type("div")
// Rule 94: match_Class("gsfs")
// Rule 161: active_Id("gbgs5")
// Rule 75: active_Class("gbtb2")
// Rule 58: match_Class("gbqfb")
// Rule 179: active_Id("gbmpid")
// Rule 221: active_Type("td")
// Rule 29: active_Class("gbmpalb")
// Rule 55: active_Class("gbprct")
// Rule 82: match_Class("gbtsa")
// Rule 187: active_Id("gbpms")
// Rule 49: active_Class("gbprcb")
// Rule 119: active_Class("gssb_c")
// Rule 14: match_Class("gbmab")
// Rule 129: active_Class("gssb_i")
// Rule 200: match_Id("gbx4")
// Rule 209: active_Id("gws-output-pages-elements-homepage_additional_languages__als")
// Rule 210: match_Type("a")
// Rule 196: match_Id("gbs")
// Rule 203: active_Id("gbz")
// Rule 216: match_Type("input")
// Rule 41: active_Class("gbmtc")
// Rule 102: match_Class("gsmq_a")
// Rule 5: active_Class("gbg4a")
// Rule 138: match_Class("lsb")
// Rule 155: active_Id("gbbw")
// Rule 13: active_Class("gbma")
// Rule 205: active_Id("gog")
// Rule 67: active_Class("gbqfbb-hvr")
// Rule 143: active_Class("lst")
// Rule 56: match_Class("gbps2")
// Rule 157: active_Id("gbg")
// Rule 172: match_Id("gbmpal")
// Rule 65: active_Class("gbqfbb")
// Rule 173: active_Id("gbmpal")
// Rule 27: active_Class("gbmpala")
// Rule 34: match_Class("gbmpiaw")
// Rule 219: active_Type("span")
// Rule 220: match_Type("td")
// Rule 95: active_Class("gsfs")
// Rule 148: match_Id("SIvCob")
// Rule 91: active_Class("gbxx")
// Rule 132: match_Class("gssb_l")
// Rule 28: match_Class("gbmpalb")
// Rule 149: active_Id("SIvCob")
// Rule 167: active_Id("gbi4t")
// Rule 177: active_Id("gbmpdv")
// Rule 217: active_Type("input")
// Rule 133: active_Class("gssb_l")
// Rule 50: match_Class("gbprcd")
// Rule 130: match_Class("gssb_k")
// Rule 112: match_Class("gsq_a")
// Rule 176: match_Id("gbmpdv")
// Rule 9: active_Class("gbi4p")
// Rule 26: match_Class("gbmpala")
// Rule 1: active_Class("H6sW5")
// Rule 36: match_Class("gbmpnw")
// Rule 23: active_Class("gbmh")
// Rule 51: active_Class("gbprcd")
// Rule 137: active_Class("h")
// Rule 42: match_Class("gbp0")
// Rule 104: match_Class("gsn_a")
// Rule 197: active_Id("gbs")
// Rule 198: match_Id("gbx3")
// Rule 165: active_Id("gbi4s1")
// Rule 191: active_Id("gbprcs")
// Rule 125: active_Class("gssb_g")
// Rule 24: match_Class("gbmlbw")
// Rule 30: match_Class("gbmpia")
// Rule 21: active_Class("gbmcc")
// Rule 100: match_Class("gsls_a")
// Rule 166: match_Id("gbi4t")
// Rule 174: match_Id("gbmpas")
// Rule 188: match_Id("gbprca")
// Rule 92: match_Class("gsdd_a")
// Rule 121: active_Class("gssb_e")
// Rule 76: match_Class("gbtcb")
// Rule 98: match_Class("gsib_b")
// Rule 111: active_Class("gspqs_b")
// Rule 120: match_Class("gssb_e")
// Rule 86: match_Class("gbxo")
// Rule 136: match_Class("h")
// Rule 180: match_Id("gbmpiw")
// Rule 185: active_Id("gbpm")
// Rule 199: active_Id("gbx3")
// Rule 134: match_Class("gssb_m")
// Rule 145: active_Class("sblc")
// Rule 146: match_Class("z4hgWe")
// Rule 22: match_Class("gbmh")
// Rule 128: match_Class("gssb_i")
// Rule 113: active_Class("gsq_a")
// Rule 126: match_Class("gssb_h")
// Rule 40: match_Class("gbmtc")
// Rule 178: match_Id("gbmpid")
// Rule 39: active_Class("gbmt")
// Rule 43: active_Class("gbp0")
// Rule 70: match_Class("gbsbic")
// Rule 194: match_Id("gbqfbw")
// Rule 18: match_Class("gbmc")
// Rule 47: active_Class("gbpms2")
// Rule 192: match_Id("gbqfb")
// Rule 211: active_Type("a")
// Rule 72: match_Class("gbt")
// Rule 38: match_Class("gbmt")
// Rule 96: match_Class("gsib_a")
// Rule 142: match_Class("lst")
// Rule 3: active_Class("ds")
// Rule 156: match_Id("gbg")
// Rule 201: active_Id("gbx4")
// Rule 127: active_Class("gssb_h")
// Rule 19: active_Class("gbmc")
// Rule 71: active_Class("gbsbic")
// Rule 140: match_Class("lsbb")
// Rule 160: match_Id("gbgs5")
// Rule 195: active_Id("gbqfbw")
// Rule 207: active_Id("gssb_b")
// Rule 168: match_Id("gbi5")
// Rule 208: match_Id("gws-output-pages-elements-homepage_additional_languages__als")
// Rule 215: active_Type("div")
// Rule 218: match_Type("span")
// Rule 8: match_Class("gbi4p")
// Rule 73: active_Class("gbt")
// Rule 85: active_Class("gbxms")
// Rule 135: active_Class("gssb_m")
// Rule 46: match_Class("gbpms2")
// Rule 88: match_Class("gbxv")
// Rule 45: active_Class("gbpmc")
// Rule 17: active_Class("gbmac")
// Rule 53: active_Class("gbprci")
// Rule 190: match_Id("gbprcs")
// Rule 193: active_Id("gbqfb")
// Rule 212: match_Type("body")
// Rule 77: active_Class("gbtcb")
// Rule 110: match_Class("gspqs_b")
// Rule 117: active_Class("gssb_a")
// Rule 54: match_Class("gbprct")
// Rule 204: match_Id("gog")
// Rule 69: active_Class("gbsb")
// Rule 184: match_Id("gbpm")
// Rule 12: match_Class("gbma")
// Rule 68: match_Class("gbsb")
// Rule 108: match_Class("gsn_c")
// Rule 152: match_Id("gbb")
// Rule 87: active_Class("gbxo")
// Rule 32: match_Class("gbmpiaa")
// Rule 59: active_Class("gbqfb")
// Rule 80: match_Class("gbts")
// Rule 83: active_Class("gbtsa")
// Rule 84: match_Class("gbxms")
// Rule 118: match_Class("gssb_c")
// Rule 124: match_Class("gssb_g")
// Rule 33: active_Class("gbmpiaa")
// Rule 0: match_Class("H6sW5")
// Rule 97: active_Class("gsib_a")
// Rule 74: match_Class("gbtb2")
// Rule 90: match_Class("gbxx")
// Rule 7: active_Class("gbgt")
// Rule 122: match_Class("gssb_f")
// Rule 141: active_Class("lsbb")
// Rule 144: match_Class("sblc")
// Rule 116: match_Class("gssb_a")
// Rule 150: match_Id("gb")

pub fn print_node_matches(node: &HtmlNode, matches: &[bool]) {
    println!("Node '{}' matches:", node.tag_name);
    for (i, &matched) in matches.iter().enumerate() {
        if matched {
            println!("  Rule {}: {}", i, get_rule_name(i));
        }
    }
}

pub fn get_total_rules() -> usize {
    222 // Total number of CSS rules
}
