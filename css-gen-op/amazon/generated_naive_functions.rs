use crate::{HtmlNode, SimpleSelector};
// === NAIVE CSS MATCHING FUNCTIONS ===
// These functions calculate layout from scratch without any caching

// Rule 0: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2"), bit_pos: 0 }
pub fn matches_rule_0(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
}

// Rule 2: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3"), bit_pos: 2 }
pub fn matches_rule_2(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
}

// Rule 4: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P"), bit_pos: 4 }
pub fn matches_rule_4(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
}

// Rule 6: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK"), bit_pos: 6 }
pub fn matches_rule_6(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
}

// Rule 8: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating__3_0eN"), bit_pos: 8 }
pub fn matches_rule_8(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
}

// Rule 10: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-shape__1IcJY"), bit_pos: 10 }
pub fn matches_rule_10(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
}

// Rule 12: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-standard__28gp8"), bit_pos: 12 }
pub fn matches_rule_12(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-standard__28gp8")
}

// Rule 14: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-container__1Pkva"), bit_pos: 14 }
pub fn matches_rule_14(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-container__1Pkva")
}

// Rule 16: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_image_asin-container-white-box__QwmgO"), bit_pos: 16 }
pub fn matches_rule_16(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_image_asin-container-white-box__QwmgO")
}

// Rule 18: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_image_asin-container__2jyCM"), bit_pos: 18 }
pub fn matches_rule_18(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_image_asin-container__2jyCM")
}

// Rule 20: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_image_round-corners__2y_fS"), bit_pos: 20 }
pub fn matches_rule_20(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_image_round-corners__2y_fS")
}

// Rule 22: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner-rtl__2BoOY"), bit_pos: 22 }
pub fn matches_rule_22(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner-rtl__2BoOY")
}

// Rule 24: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner__1nmZw"), bit_pos: 24 }
pub fn matches_rule_24(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner__1nmZw")
}

// Rule 26: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-primary-link__2bIZi"), bit_pos: 26 }
pub fn matches_rule_26(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_ad-feedback-primary-link__2bIZi")
}

// Rule 28: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-sprite-mobile__2_rj8"), bit_pos: 28 }
pub fn matches_rule_28(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_ad-feedback-sprite-mobile__2_rj8")
}

// Rule 30: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-sprite__28uwB"), bit_pos: 30 }
pub fn matches_rule_30(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_ad-feedback-sprite__28uwB")
}

// Rule 32: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-text-desktop__q3xp_"), bit_pos: 32 }
pub fn matches_rule_32(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_ad-feedback-text-desktop__q3xp_")
}

// Rule 34: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-text__2HjQ9"), bit_pos: 34 }
pub fn matches_rule_34(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_ad-feedback-text__2HjQ9")
}

// Rule 36: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_apexBadgeLabel__2-Vye"), bit_pos: 36 }
pub fn matches_rule_36(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_apexBadgeLabel__2-Vye")
}

// Rule 38: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_apexBadgeMessage__1tHvd"), bit_pos: 38 }
pub fn matches_rule_38(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_apexBadgeMessage__1tHvd")
}

// Rule 40: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-button-group__1LqUG"), bit_pos: 40 }
pub fn matches_rule_40(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-button-group__1LqUG")
}

// Rule 42: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-button__7cH_E"), bit_pos: 42 }
pub fn matches_rule_42(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-button__7cH_E")
}

// Rule 44: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-1236x1080__3aEzl"), bit_pos: 44 }
pub fn matches_rule_44(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-ratio-1236x1080__3aEzl")
}

// Rule 46: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-15x3__1h649"), bit_pos: 46 }
pub fn matches_rule_46(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-ratio-15x3__1h649")
}

// Rule 48: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-16x9__cBPv8"), bit_pos: 48 }
pub fn matches_rule_48(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-ratio-16x9__cBPv8")
}

// Rule 50: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-4x3__3BewI"), bit_pos: 50 }
pub fn matches_rule_50(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-ratio-4x3__3BewI")
}

// Rule 52: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-5x8__2IaNz"), bit_pos: 52 }
pub fn matches_rule_52(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-ratio-5x8__2IaNz")
}

// Rule 54: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-dynamic-60vh__3N5g_"), bit_pos: 54 }
pub fn matches_rule_54(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-ratio-dynamic-60vh__3N5g_")
}

// Rule 56: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-fill__2Zjfb"), bit_pos: 56 }
pub fn matches_rule_56(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-ratio-fill__2Zjfb")
}

// Rule 58: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-text__S4PU1"), bit_pos: 58 }
pub fn matches_rule_58(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_aspect-text__S4PU1")
}

// Rule 60: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_autoplay-span__2CMfc"), bit_pos: 60 }
pub fn matches_rule_60(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_autoplay-span__2CMfc")
}

// Rule 62: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_badge-container__20aJ2"), bit_pos: 62 }
pub fn matches_rule_62(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_badge-container__20aJ2")
}

// Rule 64: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_badgeLabel__pJ5rc"), bit_pos: 64 }
pub fn matches_rule_64(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_badgeLabel__pJ5rc")
}

// Rule 66: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_badgeMessage__2Dtw7"), bit_pos: 66 }
pub fn matches_rule_66(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_badgeMessage__2Dtw7")
}

// Rule 68: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_carouselContainer__3N7M1"), bit_pos: 68 }
pub fn matches_rule_68(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_carouselContainer__3N7M1")
}

// Rule 70: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_close-black-icon__3hkbe"), bit_pos: 70 }
pub fn matches_rule_70(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_close-black-icon__3hkbe")
}

// Rule 72: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_close-icon-wrapper__1zvdC"), bit_pos: 72 }
pub fn matches_rule_72(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_close-icon-wrapper__1zvdC")
}

// Rule 74: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_close-icon__2RJs3"), bit_pos: 74 }
pub fn matches_rule_74(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_close-icon__2RJs3")
}

// Rule 76: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_close-text__2-gwn"), bit_pos: 76 }
pub fn matches_rule_76(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_close-text__2-gwn")
}

// Rule 78: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_cover-portrait-image__2lhzL"), bit_pos: 78 }
pub fn matches_rule_78(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_cover-portrait-image__2lhzL")
}

// Rule 80: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_cta-link__2xo74"), bit_pos: 80 }
pub fn matches_rule_80(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_cta-link__2xo74")
}

// Rule 82: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_desktop-close-button__1iL_P"), bit_pos: 82 }
pub fn matches_rule_82(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_desktop-close-button__1iL_P")
}

// Rule 84: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_displayCount__1MVut"), bit_pos: 84 }
pub fn matches_rule_84(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_displayCount__1MVut")
}

// Rule 86: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_dynamic-portrait-image__1Wrzd"), bit_pos: 86 }
pub fn matches_rule_86(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_dynamic-portrait-image__1Wrzd")
}

// Rule 88: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_empty-footer__2d59h"), bit_pos: 88 }
pub fn matches_rule_88(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_empty-footer__2d59h")
}

// Rule 90: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_five-pack__1-Tql"), bit_pos: 90 }
pub fn matches_rule_90(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_five-pack__1-Tql")
}

// Rule 92: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_fluid-landscape-image__TE6PT"), bit_pos: 92 }
pub fn matches_rule_92(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_fluid-landscape-image__TE6PT")
}

// Rule 94: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_four-pack__1ufgr"), bit_pos: 94 }
pub fn matches_rule_94(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_four-pack__1ufgr")
}

// Rule 96: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_gw-hero-close-button__3svyZ"), bit_pos: 96 }
pub fn matches_rule_96(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_gw-hero-close-button__3svyZ")
}

// Rule 98: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_gwm-link-footer__3OF47"), bit_pos: 98 }
pub fn matches_rule_98(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_gwm-link-footer__3OF47")
}

// Rule 100: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_haulRibbon__3VZNi"), bit_pos: 100 }
pub fn matches_rule_100(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_haulRibbon__3VZNi")
}

// Rule 102: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_header-icon__2cuVV"), bit_pos: 102 }
pub fn matches_rule_102(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_header-icon__2cuVV")
}

// Rule 104: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_header-link__cUhOK"), bit_pos: 104 }
pub fn matches_rule_104(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_header-link__cUhOK")
}

// Rule 106: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_header__1vGdj"), bit_pos: 106 }
pub fn matches_rule_106(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_header__1vGdj")
}

// Rule 108: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_image-container__2OiZA"), bit_pos: 108 }
pub fn matches_rule_108(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_image-container__2OiZA")
}

// Rule 110: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_inlineErrorDetails__1NBx-"), bit_pos: 110 }
pub fn matches_rule_110(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_inlineErrorDetails__1NBx-")
}

// Rule 112: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logoGap__nKNZ9"), bit_pos: 112 }
pub fn matches_rule_112(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_logoGap__nKNZ9")
}

// Rule 114: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logoRectangle__1VJwu"), bit_pos: 114 }
pub fn matches_rule_114(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_logoRectangle__1VJwu")
}

// Rule 116: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logoSquareContainer__3Paoc"), bit_pos: 116 }
pub fn matches_rule_116(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_logoSquareContainer__3Paoc")
}

// Rule 118: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logoSquare__3NZyi"), bit_pos: 118 }
pub fn matches_rule_118(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_logoSquare__3NZyi")
}

// Rule 120: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logo__2ZQ-N"), bit_pos: 120 }
pub fn matches_rule_120(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_logo__2ZQ-N")
}

// Rule 122: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_mixed-button__2og-m"), bit_pos: 122 }
pub fn matches_rule_122(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_mixed-button__2og-m")
}

// Rule 124: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_mobile-close-button__3PB07"), bit_pos: 124 }
pub fn matches_rule_124(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_mobile-close-button__3PB07")
}

// Rule 126: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_mosaic-card-body__1HmTs"), bit_pos: 126 }
pub fn matches_rule_126(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_mosaic-card-body__1HmTs")
}

// Rule 128: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_mosaic-card__1C-_R"), bit_pos: 128 }
pub fn matches_rule_128(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_mosaic-card__1C-_R")
}

// Rule 130: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_negative-button__1Dvqz"), bit_pos: 130 }
pub fn matches_rule_130(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_negative-button__1Dvqz")
}

// Rule 132: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_negativeMarginAdjust__1nqu9"), bit_pos: 132 }
pub fn matches_rule_132(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_negativeMarginAdjust__1nqu9")
}

// Rule 134: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_oneLineTruncation__2WWse"), bit_pos: 134 }
pub fn matches_rule_134(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_oneLineTruncation__2WWse")
}

// Rule 136: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_overlay__3Sx3u"), bit_pos: 136 }
pub fn matches_rule_136(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_overlay__3Sx3u")
}

// Rule 138: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_positive-button__3UOC3"), bit_pos: 138 }
pub fn matches_rule_138(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_positive-button__3UOC3")
}

// Rule 140: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_poster-image__1W0yA"), bit_pos: 140 }
pub fn matches_rule_140(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_poster-image__1W0yA")
}

// Rule 142: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_smartText__ubpEw"), bit_pos: 142 }
pub fn matches_rule_142(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_smartText__ubpEw")
}

// Rule 144: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_spCSRFTreatment__-hwVO"), bit_pos: 144 }
pub fn matches_rule_144(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_spCSRFTreatment__-hwVO")
}

// Rule 146: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_spacer__7Pyg3"), bit_pos: 146 }
pub fn matches_rule_146(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_spacer__7Pyg3")
}

// Rule 148: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_stacking-context__3PbQE"), bit_pos: 148 }
pub fn matches_rule_148(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_stacking-context__3PbQE")
}

// Rule 150: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_theming-background-override__1HfzJ"), bit_pos: 150 }
pub fn matches_rule_150(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_theming-background-override__1HfzJ")
}

// Rule 152: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_themingTextColorWhite__1zryO"), bit_pos: 152 }
pub fn matches_rule_152(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_themingTextColorWhite__1zryO")
}

// Rule 154: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_themingTextColor__1oQsI"), bit_pos: 154 }
pub fn matches_rule_154(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_themingTextColor__1oQsI")
}

// Rule 156: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_three-pack__5s3hP"), bit_pos: 156 }
pub fn matches_rule_156(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_three-pack__5s3hP")
}

// Rule 158: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_threeLineTruncation__UkUjj"), bit_pos: 158 }
pub fn matches_rule_158(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_threeLineTruncation__UkUjj")
}

// Rule 160: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_tile-container__1QgAV"), bit_pos: 160 }
pub fn matches_rule_160(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_tile-container__1QgAV")
}

// Rule 162: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_tile-grid__QMxNY"), bit_pos: 162 }
pub fn matches_rule_162(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_tile-grid__QMxNY")
}

// Rule 164: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_tile-link__38lTa"), bit_pos: 164 }
pub fn matches_rule_164(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_tile-link__38lTa")
}

// Rule 166: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_tile-theming__3eeyj"), bit_pos: 166 }
pub fn matches_rule_166(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_tile-theming__3eeyj")
}

// Rule 168: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_truncation__x9-69"), bit_pos: 168 }
pub fn matches_rule_168(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_truncation__x9-69")
}

// Rule 170: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_twoLineTruncation__16TLV"), bit_pos: 170 }
pub fn matches_rule_170(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_twoLineTruncation__16TLV")
}

// Rule 172: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_video-container__1hKS1"), bit_pos: 172 }
pub fn matches_rule_172(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_video-container__1hKS1")
}

// Rule 174: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_wd-backdrop-data__1znxG"), bit_pos: 174 }
pub fn matches_rule_174(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_wd-backdrop-data__1znxG")
}

// Rule 176: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_wdHeader__Edrev"), bit_pos: 176 }
pub fn matches_rule_176(node: &HtmlNode) -> bool {
    node.classes.contains("_ameyal-product-shoveler_style_wdHeader__Edrev")
}

// Rule 178: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2"), bit_pos: 178 }
pub fn matches_rule_178(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
}

// Rule 180: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3"), bit_pos: 180 }
pub fn matches_rule_180(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
}

// Rule 182: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P"), bit_pos: 182 }
pub fn matches_rule_182(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
}

// Rule 184: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK"), bit_pos: 184 }
pub fn matches_rule_184(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
}

// Rule 186: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating__3_0eN"), bit_pos: 186 }
pub fn matches_rule_186(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
}

// Rule 188: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-shape__1IcJY"), bit_pos: 188 }
pub fn matches_rule_188(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
}

// Rule 190: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-standard__28gp8"), bit_pos: 190 }
pub fn matches_rule_190(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_energy-efficiency_energy-efficiency-badge-standard__28gp8")
}

// Rule 192: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-container__1Pkva"), bit_pos: 192 }
pub fn matches_rule_192(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_energy-efficiency_energy-efficiency-container__1Pkva")
}

// Rule 194: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container-full-height__MOKlF"), bit_pos: 194 }
pub fn matches_rule_194(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_image_asin-container-full-height__MOKlF")
}

// Rule 196: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container-white-box__3Stwp"), bit_pos: 196 }
pub fn matches_rule_196(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_image_asin-container-white-box__3Stwp")
}

// Rule 198: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container-white-box__QwmgO"), bit_pos: 198 }
pub fn matches_rule_198(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_image_asin-container-white-box__QwmgO")
}

// Rule 200: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container__2jyCM"), bit_pos: 200 }
pub fn matches_rule_200(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_image_asin-container__2jyCM")
}

// Rule 202: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container__LRY5p"), bit_pos: 202 }
pub fn matches_rule_202(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_image_asin-container__LRY5p")
}

// Rule 204: CheckAndSetBit { selector: Class("_cropped-image-link_image_round-corners__22iOW"), bit_pos: 204 }
pub fn matches_rule_204(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_image_round-corners__22iOW")
}

// Rule 206: CheckAndSetBit { selector: Class("_cropped-image-link_image_round-corners__2y_fS"), bit_pos: 206 }
pub fn matches_rule_206(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_image_round-corners__2y_fS")
}

// Rule 208: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-loading-spinnner-rtl__2BoOY"), bit_pos: 208 }
pub fn matches_rule_208(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_ad-feedback-loading-spinnner-rtl__2BoOY")
}

// Rule 210: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-loading-spinnner__1nmZw"), bit_pos: 210 }
pub fn matches_rule_210(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_ad-feedback-loading-spinnner__1nmZw")
}

// Rule 212: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-primary-link__2bIZi"), bit_pos: 212 }
pub fn matches_rule_212(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_ad-feedback-primary-link__2bIZi")
}

// Rule 214: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-sprite-mobile__2_rj8"), bit_pos: 214 }
pub fn matches_rule_214(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_ad-feedback-sprite-mobile__2_rj8")
}

// Rule 216: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-sprite__28uwB"), bit_pos: 216 }
pub fn matches_rule_216(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_ad-feedback-sprite__28uwB")
}

// Rule 218: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-text-desktop__q3xp_"), bit_pos: 218 }
pub fn matches_rule_218(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_ad-feedback-text-desktop__q3xp_")
}

// Rule 220: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-text__2HjQ9"), bit_pos: 220 }
pub fn matches_rule_220(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_ad-feedback-text__2HjQ9")
}

// Rule 222: CheckAndSetBit { selector: Class("_cropped-image-link_style_apexBadgeLabel__2-Vye"), bit_pos: 222 }
pub fn matches_rule_222(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_apexBadgeLabel__2-Vye")
}

// Rule 224: CheckAndSetBit { selector: Class("_cropped-image-link_style_apexBadgeMessage__1tHvd"), bit_pos: 224 }
pub fn matches_rule_224(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_apexBadgeMessage__1tHvd")
}

// Rule 226: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-button-group__1LqUG"), bit_pos: 226 }
pub fn matches_rule_226(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-button-group__1LqUG")
}

// Rule 228: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-button__7cH_E"), bit_pos: 228 }
pub fn matches_rule_228(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-button__7cH_E")
}

// Rule 230: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-1236x1080__3aEzl"), bit_pos: 230 }
pub fn matches_rule_230(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-ratio-1236x1080__3aEzl")
}

// Rule 232: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-15x3__1h649"), bit_pos: 232 }
pub fn matches_rule_232(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-ratio-15x3__1h649")
}

// Rule 234: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-16x9__cBPv8"), bit_pos: 234 }
pub fn matches_rule_234(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-ratio-16x9__cBPv8")
}

// Rule 236: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-4x3__3BewI"), bit_pos: 236 }
pub fn matches_rule_236(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-ratio-4x3__3BewI")
}

// Rule 238: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-5x8__2IaNz"), bit_pos: 238 }
pub fn matches_rule_238(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-ratio-5x8__2IaNz")
}

// Rule 240: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-dynamic-60vh__3N5g_"), bit_pos: 240 }
pub fn matches_rule_240(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-ratio-dynamic-60vh__3N5g_")
}

// Rule 242: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-fill__2Zjfb"), bit_pos: 242 }
pub fn matches_rule_242(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-ratio-fill__2Zjfb")
}

// Rule 244: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-text__S4PU1"), bit_pos: 244 }
pub fn matches_rule_244(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_aspect-text__S4PU1")
}

// Rule 246: CheckAndSetBit { selector: Class("_cropped-image-link_style_autoplay-span__2CMfc"), bit_pos: 246 }
pub fn matches_rule_246(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_autoplay-span__2CMfc")
}

// Rule 248: CheckAndSetBit { selector: Class("_cropped-image-link_style_badge-container__20aJ2"), bit_pos: 248 }
pub fn matches_rule_248(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_badge-container__20aJ2")
}

// Rule 250: CheckAndSetBit { selector: Class("_cropped-image-link_style_badgeLabel__pJ5rc"), bit_pos: 250 }
pub fn matches_rule_250(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_badgeLabel__pJ5rc")
}

// Rule 252: CheckAndSetBit { selector: Class("_cropped-image-link_style_badgeMessage__2Dtw7"), bit_pos: 252 }
pub fn matches_rule_252(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_badgeMessage__2Dtw7")
}

// Rule 254: CheckAndSetBit { selector: Class("_cropped-image-link_style_carouselContainer__3N7M1"), bit_pos: 254 }
pub fn matches_rule_254(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_carouselContainer__3N7M1")
}

// Rule 256: CheckAndSetBit { selector: Class("_cropped-image-link_style_centerImage__1rzYI"), bit_pos: 256 }
pub fn matches_rule_256(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_centerImage__1rzYI")
}

// Rule 258: CheckAndSetBit { selector: Class("_cropped-image-link_style_close-black-icon__3hkbe"), bit_pos: 258 }
pub fn matches_rule_258(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_close-black-icon__3hkbe")
}

// Rule 260: CheckAndSetBit { selector: Class("_cropped-image-link_style_close-icon-wrapper__1zvdC"), bit_pos: 260 }
pub fn matches_rule_260(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_close-icon-wrapper__1zvdC")
}

// Rule 262: CheckAndSetBit { selector: Class("_cropped-image-link_style_close-icon__2RJs3"), bit_pos: 262 }
pub fn matches_rule_262(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_close-icon__2RJs3")
}

// Rule 264: CheckAndSetBit { selector: Class("_cropped-image-link_style_close-text__2-gwn"), bit_pos: 264 }
pub fn matches_rule_264(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_close-text__2-gwn")
}

// Rule 266: CheckAndSetBit { selector: Class("_cropped-image-link_style_cover-portrait-image__2lhzL"), bit_pos: 266 }
pub fn matches_rule_266(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_cover-portrait-image__2lhzL")
}

// Rule 268: CheckAndSetBit { selector: Class("_cropped-image-link_style_cropped-image-link__3winf"), bit_pos: 268 }
pub fn matches_rule_268(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_cropped-image-link__3winf")
}

// Rule 270: CheckAndSetBit { selector: Class("_cropped-image-link_style_cta-link__2xo74"), bit_pos: 270 }
pub fn matches_rule_270(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_cta-link__2xo74")
}

// Rule 272: CheckAndSetBit { selector: Class("_cropped-image-link_style_desktop-close-button__1iL_P"), bit_pos: 272 }
pub fn matches_rule_272(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_desktop-close-button__1iL_P")
}

// Rule 274: CheckAndSetBit { selector: Class("_cropped-image-link_style_displayCount__1MVut"), bit_pos: 274 }
pub fn matches_rule_274(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_displayCount__1MVut")
}

// Rule 276: CheckAndSetBit { selector: Class("_cropped-image-link_style_dt-TextContainer__3nbU9"), bit_pos: 276 }
pub fn matches_rule_276(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_dt-TextContainer__3nbU9")
}

// Rule 278: CheckAndSetBit { selector: Class("_cropped-image-link_style_dynamic-portrait-image__1Wrzd"), bit_pos: 278 }
pub fn matches_rule_278(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_dynamic-portrait-image__1Wrzd")
}

// Rule 280: CheckAndSetBit { selector: Class("_cropped-image-link_style_empty-footer__2d59h"), bit_pos: 280 }
pub fn matches_rule_280(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_empty-footer__2d59h")
}

// Rule 282: CheckAndSetBit { selector: Class("_cropped-image-link_style_five-pack__1-Tql"), bit_pos: 282 }
pub fn matches_rule_282(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_five-pack__1-Tql")
}

// Rule 284: CheckAndSetBit { selector: Class("_cropped-image-link_style_fluid-landscape-image__TE6PT"), bit_pos: 284 }
pub fn matches_rule_284(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_fluid-landscape-image__TE6PT")
}

// Rule 286: CheckAndSetBit { selector: Class("_cropped-image-link_style_fluidImageContainer__2jd50"), bit_pos: 286 }
pub fn matches_rule_286(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_fluidImageContainer__2jd50")
}

// Rule 288: CheckAndSetBit { selector: Class("_cropped-image-link_style_fluidLandscapeImage__3eTVC"), bit_pos: 288 }
pub fn matches_rule_288(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_fluidLandscapeImage__3eTVC")
}

// Rule 290: CheckAndSetBit { selector: Class("_cropped-image-link_style_fluidPortraitImage__3yQ-X"), bit_pos: 290 }
pub fn matches_rule_290(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_fluidPortraitImage__3yQ-X")
}

// Rule 292: CheckAndSetBit { selector: Class("_cropped-image-link_style_four-pack__1ufgr"), bit_pos: 292 }
pub fn matches_rule_292(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_four-pack__1ufgr")
}

// Rule 294: CheckAndSetBit { selector: Class("_cropped-image-link_style_gw-hero-close-button__3svyZ"), bit_pos: 294 }
pub fn matches_rule_294(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_gw-hero-close-button__3svyZ")
}

// Rule 296: CheckAndSetBit { selector: Class("_cropped-image-link_style_gwm-link-footer__3OF47"), bit_pos: 296 }
pub fn matches_rule_296(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_gwm-link-footer__3OF47")
}

// Rule 298: CheckAndSetBit { selector: Class("_cropped-image-link_style_haulRibbon__3VZNi"), bit_pos: 298 }
pub fn matches_rule_298(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_haulRibbon__3VZNi")
}

// Rule 300: CheckAndSetBit { selector: Class("_cropped-image-link_style_header-icon__2cuVV"), bit_pos: 300 }
pub fn matches_rule_300(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_header-icon__2cuVV")
}

// Rule 302: CheckAndSetBit { selector: Class("_cropped-image-link_style_header-link__cUhOK"), bit_pos: 302 }
pub fn matches_rule_302(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_header-link__cUhOK")
}

// Rule 304: CheckAndSetBit { selector: Class("_cropped-image-link_style_header__1vGdj"), bit_pos: 304 }
pub fn matches_rule_304(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_header__1vGdj")
}

// Rule 306: CheckAndSetBit { selector: Class("_cropped-image-link_style_image-container__2OiZA"), bit_pos: 306 }
pub fn matches_rule_306(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_image-container__2OiZA")
}

// Rule 308: CheckAndSetBit { selector: Class("_cropped-image-link_style_logoGap__nKNZ9"), bit_pos: 308 }
pub fn matches_rule_308(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_logoGap__nKNZ9")
}

// Rule 310: CheckAndSetBit { selector: Class("_cropped-image-link_style_logoRectangle__1VJwu"), bit_pos: 310 }
pub fn matches_rule_310(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_logoRectangle__1VJwu")
}

// Rule 312: CheckAndSetBit { selector: Class("_cropped-image-link_style_logoSquareContainer__3Paoc"), bit_pos: 312 }
pub fn matches_rule_312(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_logoSquareContainer__3Paoc")
}

// Rule 314: CheckAndSetBit { selector: Class("_cropped-image-link_style_logoSquare__3NZyi"), bit_pos: 314 }
pub fn matches_rule_314(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_logoSquare__3NZyi")
}

// Rule 316: CheckAndSetBit { selector: Class("_cropped-image-link_style_logo__2ZQ-N"), bit_pos: 316 }
pub fn matches_rule_316(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_logo__2ZQ-N")
}

// Rule 318: CheckAndSetBit { selector: Class("_cropped-image-link_style_mixed-button__2og-m"), bit_pos: 318 }
pub fn matches_rule_318(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_mixed-button__2og-m")
}

// Rule 320: CheckAndSetBit { selector: Class("_cropped-image-link_style_mobile-close-button__3PB07"), bit_pos: 320 }
pub fn matches_rule_320(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_mobile-close-button__3PB07")
}

// Rule 322: CheckAndSetBit { selector: Class("_cropped-image-link_style_mosaic-card-body__1HmTs"), bit_pos: 322 }
pub fn matches_rule_322(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_mosaic-card-body__1HmTs")
}

// Rule 324: CheckAndSetBit { selector: Class("_cropped-image-link_style_mosaic-card__1C-_R"), bit_pos: 324 }
pub fn matches_rule_324(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_mosaic-card__1C-_R")
}

// Rule 326: CheckAndSetBit { selector: Class("_cropped-image-link_style_negative-button__1Dvqz"), bit_pos: 326 }
pub fn matches_rule_326(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_negative-button__1Dvqz")
}

// Rule 328: CheckAndSetBit { selector: Class("_cropped-image-link_style_negativeMarginAdjust__1nqu9"), bit_pos: 328 }
pub fn matches_rule_328(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_negativeMarginAdjust__1nqu9")
}

// Rule 330: CheckAndSetBit { selector: Class("_cropped-image-link_style_oneLineTruncation__2WWse"), bit_pos: 330 }
pub fn matches_rule_330(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_oneLineTruncation__2WWse")
}

// Rule 332: CheckAndSetBit { selector: Class("_cropped-image-link_style_overlay__3Sx3u"), bit_pos: 332 }
pub fn matches_rule_332(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_overlay__3Sx3u")
}

// Rule 334: CheckAndSetBit { selector: Class("_cropped-image-link_style_positive-button__3UOC3"), bit_pos: 334 }
pub fn matches_rule_334(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_positive-button__3UOC3")
}

// Rule 336: CheckAndSetBit { selector: Class("_cropped-image-link_style_poster-image__1W0yA"), bit_pos: 336 }
pub fn matches_rule_336(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_poster-image__1W0yA")
}

// Rule 338: CheckAndSetBit { selector: Class("_cropped-image-link_style_smartText__ubpEw"), bit_pos: 338 }
pub fn matches_rule_338(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_smartText__ubpEw")
}

// Rule 340: CheckAndSetBit { selector: Class("_cropped-image-link_style_spacer__7Pyg3"), bit_pos: 340 }
pub fn matches_rule_340(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_spacer__7Pyg3")
}

// Rule 342: CheckAndSetBit { selector: Class("_cropped-image-link_style_stacking-context__3PbQE"), bit_pos: 342 }
pub fn matches_rule_342(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_stacking-context__3PbQE")
}

// Rule 344: CheckAndSetBit { selector: Class("_cropped-image-link_style_theming-background-override__1HfzJ"), bit_pos: 344 }
pub fn matches_rule_344(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_theming-background-override__1HfzJ")
}

// Rule 346: CheckAndSetBit { selector: Class("_cropped-image-link_style_themingTextColorWhite__1zryO"), bit_pos: 346 }
pub fn matches_rule_346(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_themingTextColorWhite__1zryO")
}

// Rule 348: CheckAndSetBit { selector: Class("_cropped-image-link_style_themingTextColor__1oQsI"), bit_pos: 348 }
pub fn matches_rule_348(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_themingTextColor__1oQsI")
}

// Rule 350: CheckAndSetBit { selector: Class("_cropped-image-link_style_three-pack__5s3hP"), bit_pos: 350 }
pub fn matches_rule_350(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_three-pack__5s3hP")
}

// Rule 352: CheckAndSetBit { selector: Class("_cropped-image-link_style_threeLineTruncation__UkUjj"), bit_pos: 352 }
pub fn matches_rule_352(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_threeLineTruncation__UkUjj")
}

// Rule 354: CheckAndSetBit { selector: Class("_cropped-image-link_style_tile-container__1QgAV"), bit_pos: 354 }
pub fn matches_rule_354(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_tile-container__1QgAV")
}

// Rule 356: CheckAndSetBit { selector: Class("_cropped-image-link_style_tile-grid__QMxNY"), bit_pos: 356 }
pub fn matches_rule_356(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_tile-grid__QMxNY")
}

// Rule 358: CheckAndSetBit { selector: Class("_cropped-image-link_style_tile-link__38lTa"), bit_pos: 358 }
pub fn matches_rule_358(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_tile-link__38lTa")
}

// Rule 360: CheckAndSetBit { selector: Class("_cropped-image-link_style_tile-theming__3eeyj"), bit_pos: 360 }
pub fn matches_rule_360(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_tile-theming__3eeyj")
}

// Rule 362: CheckAndSetBit { selector: Class("_cropped-image-link_style_truncation__x9-69"), bit_pos: 362 }
pub fn matches_rule_362(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_truncation__x9-69")
}

// Rule 364: CheckAndSetBit { selector: Class("_cropped-image-link_style_twoLineTruncation__16TLV"), bit_pos: 364 }
pub fn matches_rule_364(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_twoLineTruncation__16TLV")
}

// Rule 366: CheckAndSetBit { selector: Class("_cropped-image-link_style_video-container__1hKS1"), bit_pos: 366 }
pub fn matches_rule_366(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_video-container__1hKS1")
}

// Rule 368: CheckAndSetBit { selector: Class("_cropped-image-link_style_wd-backdrop-data__1znxG"), bit_pos: 368 }
pub fn matches_rule_368(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_wd-backdrop-data__1znxG")
}

// Rule 370: CheckAndSetBit { selector: Class("_cropped-image-link_style_wdHeader__Edrev"), bit_pos: 370 }
pub fn matches_rule_370(node: &HtmlNode) -> bool {
    node.classes.contains("_cropped-image-link_style_wdHeader__Edrev")
}

// Rule 372: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_bodyFooterStyle_cardBody__1YuQY"), bit_pos: 372 }
pub fn matches_rule_372(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_bodyFooterStyle_cardBody__1YuQY")
}

// Rule 374: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2"), bit_pos: 374 }
pub fn matches_rule_374(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
}

// Rule 376: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3"), bit_pos: 376 }
pub fn matches_rule_376(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
}

// Rule 378: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P"), bit_pos: 378 }
pub fn matches_rule_378(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
}

// Rule 380: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK"), bit_pos: 380 }
pub fn matches_rule_380(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
}

// Rule 382: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN"), bit_pos: 382 }
pub fn matches_rule_382(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
}

// Rule 384: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY"), bit_pos: 384 }
pub fn matches_rule_384(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
}

// Rule 386: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8"), bit_pos: 386 }
pub fn matches_rule_386(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
}

// Rule 388: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-container__1Pkva"), bit_pos: 388 }
pub fn matches_rule_388(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-container__1Pkva")
}

// Rule 390: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_image_asin-container-white-box__QwmgO"), bit_pos: 390 }
pub fn matches_rule_390(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_image_asin-container-white-box__QwmgO")
}

// Rule 392: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_image_asin-container__2jyCM"), bit_pos: 392 }
pub fn matches_rule_392(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_image_asin-container__2jyCM")
}

// Rule 394: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_image_round-corners__2y_fS"), bit_pos: 394 }
pub fn matches_rule_394(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_image_round-corners__2y_fS")
}

// Rule 396: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_singleLinkStyle_bodyFooterLink__9LvH0"), bit_pos: 396 }
pub fn matches_rule_396(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_singleLinkStyle_bodyFooterLink__9LvH0")
}

// Rule 398: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_singleLinkStyle_footer__2cH0y"), bit_pos: 398 }
pub fn matches_rule_398(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_singleLinkStyle_footer__2cH0y")
}

// Rule 400: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY"), bit_pos: 400 }
pub fn matches_rule_400(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
}

// Rule 402: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner__1nmZw"), bit_pos: 402 }
pub fn matches_rule_402(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner__1nmZw")
}

// Rule 404: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-primary-link__2bIZi"), bit_pos: 404 }
pub fn matches_rule_404(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_ad-feedback-primary-link__2bIZi")
}

// Rule 406: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite-mobile__2_rj8"), bit_pos: 406 }
pub fn matches_rule_406(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_ad-feedback-sprite-mobile__2_rj8")
}

// Rule 408: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite__28uwB"), bit_pos: 408 }
pub fn matches_rule_408(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_ad-feedback-sprite__28uwB")
}

// Rule 410: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-text-desktop__q3xp_"), bit_pos: 410 }
pub fn matches_rule_410(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_ad-feedback-text-desktop__q3xp_")
}

// Rule 412: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-text__2HjQ9"), bit_pos: 412 }
pub fn matches_rule_412(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_ad-feedback-text__2HjQ9")
}

// Rule 414: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_apexBadgeLabel__2-Vye"), bit_pos: 414 }
pub fn matches_rule_414(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_apexBadgeLabel__2-Vye")
}

// Rule 416: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_apexBadgeMessage__1tHvd"), bit_pos: 416 }
pub fn matches_rule_416(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_apexBadgeMessage__1tHvd")
}

// Rule 418: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-button-group__1LqUG"), bit_pos: 418 }
pub fn matches_rule_418(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-button-group__1LqUG")
}

// Rule 420: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-button__7cH_E"), bit_pos: 420 }
pub fn matches_rule_420(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-button__7cH_E")
}

// Rule 422: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-1236x1080__3aEzl"), bit_pos: 422 }
pub fn matches_rule_422(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-ratio-1236x1080__3aEzl")
}

// Rule 424: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-15x3__1h649"), bit_pos: 424 }
pub fn matches_rule_424(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-ratio-15x3__1h649")
}

// Rule 426: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-16x9__cBPv8"), bit_pos: 426 }
pub fn matches_rule_426(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-ratio-16x9__cBPv8")
}

// Rule 428: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-4x3__3BewI"), bit_pos: 428 }
pub fn matches_rule_428(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-ratio-4x3__3BewI")
}

// Rule 430: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-5x8__2IaNz"), bit_pos: 430 }
pub fn matches_rule_430(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-ratio-5x8__2IaNz")
}

// Rule 432: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-dynamic-60vh__3N5g_"), bit_pos: 432 }
pub fn matches_rule_432(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
}

// Rule 434: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-fill__2Zjfb"), bit_pos: 434 }
pub fn matches_rule_434(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-ratio-fill__2Zjfb")
}

// Rule 436: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-text__S4PU1"), bit_pos: 436 }
pub fn matches_rule_436(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_aspect-text__S4PU1")
}

// Rule 438: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_autoplay-span__2CMfc"), bit_pos: 438 }
pub fn matches_rule_438(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_autoplay-span__2CMfc")
}

// Rule 440: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_badge-container__20aJ2"), bit_pos: 440 }
pub fn matches_rule_440(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_badge-container__20aJ2")
}

// Rule 442: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_badgeLabel__pJ5rc"), bit_pos: 442 }
pub fn matches_rule_442(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_badgeLabel__pJ5rc")
}

// Rule 444: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_badgeMessage__2Dtw7"), bit_pos: 444 }
pub fn matches_rule_444(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_badgeMessage__2Dtw7")
}

// Rule 446: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_carouselContainer__3N7M1"), bit_pos: 446 }
pub fn matches_rule_446(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_carouselContainer__3N7M1")
}

// Rule 448: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_centerImage__30wh-"), bit_pos: 448 }
pub fn matches_rule_448(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_centerImage__30wh-")
}

// Rule 450: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_close-black-icon__3hkbe"), bit_pos: 450 }
pub fn matches_rule_450(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_close-black-icon__3hkbe")
}

// Rule 452: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_close-icon-wrapper__1zvdC"), bit_pos: 452 }
pub fn matches_rule_452(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_close-icon-wrapper__1zvdC")
}

// Rule 454: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_close-icon__2RJs3"), bit_pos: 454 }
pub fn matches_rule_454(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_close-icon__2RJs3")
}

// Rule 456: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_close-text__2-gwn"), bit_pos: 456 }
pub fn matches_rule_456(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_close-text__2-gwn")
}

// Rule 458: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_cover-portrait-image__2lhzL"), bit_pos: 458 }
pub fn matches_rule_458(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_cover-portrait-image__2lhzL")
}

// Rule 460: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_cta-link__2xo74"), bit_pos: 460 }
pub fn matches_rule_460(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_cta-link__2xo74")
}

// Rule 462: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_desktop-close-button__1iL_P"), bit_pos: 462 }
pub fn matches_rule_462(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_desktop-close-button__1iL_P")
}

// Rule 464: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_displayCount__1MVut"), bit_pos: 464 }
pub fn matches_rule_464(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_displayCount__1MVut")
}

// Rule 466: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_dynamic-portrait-image__1Wrzd"), bit_pos: 466 }
pub fn matches_rule_466(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_dynamic-portrait-image__1Wrzd")
}

// Rule 468: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_empty-footer__2d59h"), bit_pos: 468 }
pub fn matches_rule_468(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_empty-footer__2d59h")
}

// Rule 470: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_five-pack__1-Tql"), bit_pos: 470 }
pub fn matches_rule_470(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_five-pack__1-Tql")
}

// Rule 472: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluid-landscape-image__TE6PT"), bit_pos: 472 }
pub fn matches_rule_472(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_fluid-landscape-image__TE6PT")
}

// Rule 474: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidFatImageLinkBody__1LsOX"), bit_pos: 474 }
pub fn matches_rule_474(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_fluidFatImageLinkBody__1LsOX")
}

// Rule 476: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidFatImageLink__1nw4J"), bit_pos: 476 }
pub fn matches_rule_476(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_fluidFatImageLink__1nw4J")
}

// Rule 478: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2SOMr"), bit_pos: 478 }
pub fn matches_rule_478(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_fluidImageContainer__2SOMr")
}

// Rule 480: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2vGwp"), bit_pos: 480 }
pub fn matches_rule_480(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_fluidImageContainer__2vGwp")
}

// Rule 482: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidLandscapeImage__2euAK"), bit_pos: 482 }
pub fn matches_rule_482(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_fluidLandscapeImage__2euAK")
}

// Rule 484: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidPortraitImage__2SAYm"), bit_pos: 484 }
pub fn matches_rule_484(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_fluidPortraitImage__2SAYm")
}

// Rule 486: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_four-pack__1ufgr"), bit_pos: 486 }
pub fn matches_rule_486(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_four-pack__1ufgr")
}

// Rule 488: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_gw-hero-close-button__3svyZ"), bit_pos: 488 }
pub fn matches_rule_488(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_gw-hero-close-button__3svyZ")
}

// Rule 490: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_gwm-link-footer__3OF47"), bit_pos: 490 }
pub fn matches_rule_490(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_gwm-link-footer__3OF47")
}

// Rule 492: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_haulRibbon__3VZNi"), bit_pos: 492 }
pub fn matches_rule_492(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_haulRibbon__3VZNi")
}

// Rule 494: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_header-icon__2cuVV"), bit_pos: 494 }
pub fn matches_rule_494(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_header-icon__2cuVV")
}

// Rule 496: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_header-link__cUhOK"), bit_pos: 496 }
pub fn matches_rule_496(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_header-link__cUhOK")
}

// Rule 498: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_header__1vGdj"), bit_pos: 498 }
pub fn matches_rule_498(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_header__1vGdj")
}

// Rule 500: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_image-container__2OiZA"), bit_pos: 500 }
pub fn matches_rule_500(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_image-container__2OiZA")
}

// Rule 502: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_imageLabel__3ANSV"), bit_pos: 502 }
pub fn matches_rule_502(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_imageLabel__3ANSV")
}

// Rule 504: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_inlineErrorDetails__1NBx-"), bit_pos: 504 }
pub fn matches_rule_504(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_inlineErrorDetails__1NBx-")
}

// Rule 506: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logoGap__nKNZ9"), bit_pos: 506 }
pub fn matches_rule_506(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_logoGap__nKNZ9")
}

// Rule 508: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logoRectangle__1VJwu"), bit_pos: 508 }
pub fn matches_rule_508(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_logoRectangle__1VJwu")
}

// Rule 510: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logoSquareContainer__3Paoc"), bit_pos: 510 }
pub fn matches_rule_510(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_logoSquareContainer__3Paoc")
}

// Rule 512: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logoSquare__3NZyi"), bit_pos: 512 }
pub fn matches_rule_512(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_logoSquare__3NZyi")
}

// Rule 514: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logo__2ZQ-N"), bit_pos: 514 }
pub fn matches_rule_514(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_logo__2ZQ-N")
}

// Rule 516: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mergedLinksCta__3Npog"), bit_pos: 516 }
pub fn matches_rule_516(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_mergedLinksCta__3Npog")
}

// Rule 518: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mergedLinks__10JqZ"), bit_pos: 518 }
pub fn matches_rule_518(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_mergedLinks__10JqZ")
}

// Rule 520: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mixed-button__2og-m"), bit_pos: 520 }
pub fn matches_rule_520(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_mixed-button__2og-m")
}

// Rule 522: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mobile-close-button__3PB07"), bit_pos: 522 }
pub fn matches_rule_522(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_mobile-close-button__3PB07")
}

// Rule 524: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mosaic-card-body__1HmTs"), bit_pos: 524 }
pub fn matches_rule_524(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_mosaic-card-body__1HmTs")
}

// Rule 526: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mosaic-card__1C-_R"), bit_pos: 526 }
pub fn matches_rule_526(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_mosaic-card__1C-_R")
}

// Rule 528: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_negative-button__1Dvqz"), bit_pos: 528 }
pub fn matches_rule_528(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_negative-button__1Dvqz")
}

// Rule 530: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_negativeMarginAdjust__1nqu9"), bit_pos: 530 }
pub fn matches_rule_530(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_negativeMarginAdjust__1nqu9")
}

// Rule 532: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_oneLineTruncation__2WWse"), bit_pos: 532 }
pub fn matches_rule_532(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_oneLineTruncation__2WWse")
}

// Rule 534: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_overlay__3Sx3u"), bit_pos: 534 }
pub fn matches_rule_534(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_overlay__3Sx3u")
}

// Rule 536: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_positive-button__3UOC3"), bit_pos: 536 }
pub fn matches_rule_536(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_positive-button__3UOC3")
}

// Rule 538: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_poster-image__1W0yA"), bit_pos: 538 }
pub fn matches_rule_538(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_poster-image__1W0yA")
}

// Rule 540: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_smartText__ubpEw"), bit_pos: 540 }
pub fn matches_rule_540(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_smartText__ubpEw")
}

// Rule 542: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_spCSRFTreatment__-hwVO"), bit_pos: 542 }
pub fn matches_rule_542(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_spCSRFTreatment__-hwVO")
}

// Rule 544: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_spacer__7Pyg3"), bit_pos: 544 }
pub fn matches_rule_544(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_spacer__7Pyg3")
}

// Rule 546: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_stacking-context__3PbQE"), bit_pos: 546 }
pub fn matches_rule_546(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_stacking-context__3PbQE")
}

// Rule 548: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_theming-background-override__1HfzJ"), bit_pos: 548 }
pub fn matches_rule_548(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_theming-background-override__1HfzJ")
}

// Rule 550: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_themingTextColorWhite__1zryO"), bit_pos: 550 }
pub fn matches_rule_550(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_themingTextColorWhite__1zryO")
}

// Rule 552: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_themingTextColor__1oQsI"), bit_pos: 552 }
pub fn matches_rule_552(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_themingTextColor__1oQsI")
}

// Rule 554: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_three-pack__5s3hP"), bit_pos: 554 }
pub fn matches_rule_554(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_three-pack__5s3hP")
}

// Rule 556: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_threeLineTruncation__UkUjj"), bit_pos: 556 }
pub fn matches_rule_556(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_threeLineTruncation__UkUjj")
}

// Rule 558: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_tile-container__1QgAV"), bit_pos: 558 }
pub fn matches_rule_558(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_tile-container__1QgAV")
}

// Rule 560: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_tile-grid__QMxNY"), bit_pos: 560 }
pub fn matches_rule_560(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_tile-grid__QMxNY")
}

// Rule 562: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_tile-link__38lTa"), bit_pos: 562 }
pub fn matches_rule_562(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_tile-link__38lTa")
}

// Rule 564: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_tile-theming__3eeyj"), bit_pos: 564 }
pub fn matches_rule_564(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_tile-theming__3eeyj")
}

// Rule 566: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_truncation__x9-69"), bit_pos: 566 }
pub fn matches_rule_566(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_truncation__x9-69")
}

// Rule 568: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_twoLineTruncation__16TLV"), bit_pos: 568 }
pub fn matches_rule_568(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_twoLineTruncation__16TLV")
}

// Rule 570: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_video-container__1hKS1"), bit_pos: 570 }
pub fn matches_rule_570(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_video-container__1hKS1")
}

// Rule 572: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_wd-backdrop-data__1znxG"), bit_pos: 572 }
pub fn matches_rule_572(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_wd-backdrop-data__1znxG")
}

// Rule 574: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_wdHeader__Edrev"), bit_pos: 574 }
pub fn matches_rule_574(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-fat-image-link-v2_style_wdHeader__Edrev")
}

// Rule 576: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2"), bit_pos: 576 }
pub fn matches_rule_576(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
}

// Rule 578: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3"), bit_pos: 578 }
pub fn matches_rule_578(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
}

// Rule 580: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P"), bit_pos: 580 }
pub fn matches_rule_580(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
}

// Rule 582: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK"), bit_pos: 582 }
pub fn matches_rule_582(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
}

// Rule 584: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN"), bit_pos: 584 }
pub fn matches_rule_584(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
}

// Rule 586: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY"), bit_pos: 586 }
pub fn matches_rule_586(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
}

// Rule 588: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8"), bit_pos: 588 }
pub fn matches_rule_588(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
}

// Rule 590: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-container__1Pkva"), bit_pos: 590 }
pub fn matches_rule_590(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-container__1Pkva")
}

// Rule 592: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_image_asin-container-white-box__QwmgO"), bit_pos: 592 }
pub fn matches_rule_592(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_image_asin-container-white-box__QwmgO")
}

// Rule 594: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_image_asin-container__2jyCM"), bit_pos: 594 }
pub fn matches_rule_594(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_image_asin-container__2jyCM")
}

// Rule 596: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_image_round-corners__2y_fS"), bit_pos: 596 }
pub fn matches_rule_596(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_image_round-corners__2y_fS")
}

// Rule 598: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY"), bit_pos: 598 }
pub fn matches_rule_598(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
}

// Rule 600: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner__1nmZw"), bit_pos: 600 }
pub fn matches_rule_600(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner__1nmZw")
}

// Rule 602: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-primary-link__2bIZi"), bit_pos: 602 }
pub fn matches_rule_602(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_ad-feedback-primary-link__2bIZi")
}

// Rule 604: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite-mobile__2_rj8"), bit_pos: 604 }
pub fn matches_rule_604(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_ad-feedback-sprite-mobile__2_rj8")
}

// Rule 606: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite__28uwB"), bit_pos: 606 }
pub fn matches_rule_606(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_ad-feedback-sprite__28uwB")
}

// Rule 608: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-text-desktop__q3xp_"), bit_pos: 608 }
pub fn matches_rule_608(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_ad-feedback-text-desktop__q3xp_")
}

// Rule 610: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-text__2HjQ9"), bit_pos: 610 }
pub fn matches_rule_610(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_ad-feedback-text__2HjQ9")
}

// Rule 612: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_apexBadgeLabel__2-Vye"), bit_pos: 612 }
pub fn matches_rule_612(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_apexBadgeLabel__2-Vye")
}

// Rule 614: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_apexBadgeMessage__1tHvd"), bit_pos: 614 }
pub fn matches_rule_614(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_apexBadgeMessage__1tHvd")
}

// Rule 616: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-button-group__1LqUG"), bit_pos: 616 }
pub fn matches_rule_616(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-button-group__1LqUG")
}

// Rule 618: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-button__7cH_E"), bit_pos: 618 }
pub fn matches_rule_618(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-button__7cH_E")
}

// Rule 620: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-1236x1080__3aEzl"), bit_pos: 620 }
pub fn matches_rule_620(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-ratio-1236x1080__3aEzl")
}

// Rule 622: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-15x3__1h649"), bit_pos: 622 }
pub fn matches_rule_622(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-ratio-15x3__1h649")
}

// Rule 624: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-16x9__cBPv8"), bit_pos: 624 }
pub fn matches_rule_624(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-ratio-16x9__cBPv8")
}

// Rule 626: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-4x3__3BewI"), bit_pos: 626 }
pub fn matches_rule_626(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-ratio-4x3__3BewI")
}

// Rule 628: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-5x8__2IaNz"), bit_pos: 628 }
pub fn matches_rule_628(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-ratio-5x8__2IaNz")
}

// Rule 630: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-dynamic-60vh__3N5g_"), bit_pos: 630 }
pub fn matches_rule_630(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
}

// Rule 632: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-fill__2Zjfb"), bit_pos: 632 }
pub fn matches_rule_632(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-ratio-fill__2Zjfb")
}

// Rule 634: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-text__S4PU1"), bit_pos: 634 }
pub fn matches_rule_634(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_aspect-text__S4PU1")
}

// Rule 636: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_autoplay-span__2CMfc"), bit_pos: 636 }
pub fn matches_rule_636(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_autoplay-span__2CMfc")
}

// Rule 638: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_badge-container__20aJ2"), bit_pos: 638 }
pub fn matches_rule_638(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_badge-container__20aJ2")
}

// Rule 640: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_badgeLabel__pJ5rc"), bit_pos: 640 }
pub fn matches_rule_640(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_badgeLabel__pJ5rc")
}

// Rule 642: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_badgeMessage__2Dtw7"), bit_pos: 642 }
pub fn matches_rule_642(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_badgeMessage__2Dtw7")
}

// Rule 644: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_carouselContainer__3N7M1"), bit_pos: 644 }
pub fn matches_rule_644(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_carouselContainer__3N7M1")
}

// Rule 646: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_centerImage__30wh-"), bit_pos: 646 }
pub fn matches_rule_646(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_centerImage__30wh-")
}

// Rule 648: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_close-black-icon__3hkbe"), bit_pos: 648 }
pub fn matches_rule_648(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_close-black-icon__3hkbe")
}

// Rule 650: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_close-icon-wrapper__1zvdC"), bit_pos: 650 }
pub fn matches_rule_650(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_close-icon-wrapper__1zvdC")
}

// Rule 652: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_close-icon__2RJs3"), bit_pos: 652 }
pub fn matches_rule_652(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_close-icon__2RJs3")
}

// Rule 654: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_close-text__2-gwn"), bit_pos: 654 }
pub fn matches_rule_654(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_close-text__2-gwn")
}

// Rule 656: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_cover-portrait-image__2lhzL"), bit_pos: 656 }
pub fn matches_rule_656(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_cover-portrait-image__2lhzL")
}

// Rule 658: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_cta-link__2xo74"), bit_pos: 658 }
pub fn matches_rule_658(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_cta-link__2xo74")
}

// Rule 660: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_desktop-close-button__1iL_P"), bit_pos: 660 }
pub fn matches_rule_660(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_desktop-close-button__1iL_P")
}

// Rule 662: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_displayCount__1MVut"), bit_pos: 662 }
pub fn matches_rule_662(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_displayCount__1MVut")
}

// Rule 664: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_dynamic-portrait-image__1Wrzd"), bit_pos: 664 }
pub fn matches_rule_664(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_dynamic-portrait-image__1Wrzd")
}

// Rule 666: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_empty-footer__2d59h"), bit_pos: 666 }
pub fn matches_rule_666(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_empty-footer__2d59h")
}

// Rule 668: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_five-pack__1-Tql"), bit_pos: 668 }
pub fn matches_rule_668(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_five-pack__1-Tql")
}

// Rule 670: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluid-landscape-image__TE6PT"), bit_pos: 670 }
pub fn matches_rule_670(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_fluid-landscape-image__TE6PT")
}

// Rule 672: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidImageContainer__2SOMr"), bit_pos: 672 }
pub fn matches_rule_672(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_fluidImageContainer__2SOMr")
}

// Rule 674: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidLandscapeImage__2euAK"), bit_pos: 674 }
pub fn matches_rule_674(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_fluidLandscapeImage__2euAK")
}

// Rule 676: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidPortraitImage__2SAYm"), bit_pos: 676 }
pub fn matches_rule_676(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_fluidPortraitImage__2SAYm")
}

// Rule 678: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabelBody__3tld0"), bit_pos: 678 }
pub fn matches_rule_678(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_fluidQuadImageLabelBody__3tld0")
}

// Rule 680: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabel__3b-Iv"), bit_pos: 680 }
pub fn matches_rule_680(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_fluidQuadImageLabel__3b-Iv")
}

// Rule 682: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_four-pack__1ufgr"), bit_pos: 682 }
pub fn matches_rule_682(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_four-pack__1ufgr")
}

// Rule 684: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_gridRowOne__1t0zL"), bit_pos: 684 }
pub fn matches_rule_684(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_gridRowOne__1t0zL")
}

// Rule 686: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_gridRowTwo__15woW"), bit_pos: 686 }
pub fn matches_rule_686(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_gridRowTwo__15woW")
}

// Rule 688: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_gw-hero-close-button__3svyZ"), bit_pos: 688 }
pub fn matches_rule_688(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_gw-hero-close-button__3svyZ")
}

// Rule 690: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_gwm-link-footer__3OF47"), bit_pos: 690 }
pub fn matches_rule_690(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_gwm-link-footer__3OF47")
}

// Rule 692: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_haulRibbon__3VZNi"), bit_pos: 692 }
pub fn matches_rule_692(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_haulRibbon__3VZNi")
}

// Rule 694: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_header-icon__2cuVV"), bit_pos: 694 }
pub fn matches_rule_694(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_header-icon__2cuVV")
}

// Rule 696: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_header-link__cUhOK"), bit_pos: 696 }
pub fn matches_rule_696(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_header-link__cUhOK")
}

// Rule 698: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_header__1vGdj"), bit_pos: 698 }
pub fn matches_rule_698(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_header__1vGdj")
}

// Rule 700: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_image-container__2OiZA"), bit_pos: 700 }
pub fn matches_rule_700(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_image-container__2OiZA")
}

// Rule 702: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_imageLabel__3ANSV"), bit_pos: 702 }
pub fn matches_rule_702(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_imageLabel__3ANSV")
}

// Rule 704: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_inlineErrorDetails__1NBx-"), bit_pos: 704 }
pub fn matches_rule_704(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_inlineErrorDetails__1NBx-")
}

// Rule 706: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_leftQuadrant__21nVp"), bit_pos: 706 }
pub fn matches_rule_706(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_leftQuadrant__21nVp")
}

// Rule 708: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logoGap__nKNZ9"), bit_pos: 708 }
pub fn matches_rule_708(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_logoGap__nKNZ9")
}

// Rule 710: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logoRectangle__1VJwu"), bit_pos: 710 }
pub fn matches_rule_710(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_logoRectangle__1VJwu")
}

// Rule 712: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logoSquareContainer__3Paoc"), bit_pos: 712 }
pub fn matches_rule_712(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_logoSquareContainer__3Paoc")
}

// Rule 714: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logoSquare__3NZyi"), bit_pos: 714 }
pub fn matches_rule_714(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_logoSquare__3NZyi")
}

// Rule 716: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logo__2ZQ-N"), bit_pos: 716 }
pub fn matches_rule_716(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_logo__2ZQ-N")
}

// Rule 718: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_mixed-button__2og-m"), bit_pos: 718 }
pub fn matches_rule_718(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_mixed-button__2og-m")
}

// Rule 720: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_mobile-close-button__3PB07"), bit_pos: 720 }
pub fn matches_rule_720(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_mobile-close-button__3PB07")
}

// Rule 722: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_mosaic-card-body__1HmTs"), bit_pos: 722 }
pub fn matches_rule_722(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_mosaic-card-body__1HmTs")
}

// Rule 724: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_mosaic-card__1C-_R"), bit_pos: 724 }
pub fn matches_rule_724(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_mosaic-card__1C-_R")
}

// Rule 726: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_negative-button__1Dvqz"), bit_pos: 726 }
pub fn matches_rule_726(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_negative-button__1Dvqz")
}

// Rule 728: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_negativeMarginAdjust__1nqu9"), bit_pos: 728 }
pub fn matches_rule_728(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_negativeMarginAdjust__1nqu9")
}

// Rule 730: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_oneLineTruncation__2WWse"), bit_pos: 730 }
pub fn matches_rule_730(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_oneLineTruncation__2WWse")
}

// Rule 732: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_overlay__3Sx3u"), bit_pos: 732 }
pub fn matches_rule_732(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_overlay__3Sx3u")
}

// Rule 734: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_positive-button__3UOC3"), bit_pos: 734 }
pub fn matches_rule_734(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_positive-button__3UOC3")
}

// Rule 736: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_poster-image__1W0yA"), bit_pos: 736 }
pub fn matches_rule_736(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_poster-image__1W0yA")
}

// Rule 738: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_quadrantContainer__3TMqG"), bit_pos: 738 }
pub fn matches_rule_738(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_quadrantContainer__3TMqG")
}

// Rule 740: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_rightQuadrant__PI01n"), bit_pos: 740 }
pub fn matches_rule_740(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_rightQuadrant__PI01n")
}

// Rule 742: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_smartText__ubpEw"), bit_pos: 742 }
pub fn matches_rule_742(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_smartText__ubpEw")
}

// Rule 744: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_spCSRFTreatment__-hwVO"), bit_pos: 744 }
pub fn matches_rule_744(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_spCSRFTreatment__-hwVO")
}

// Rule 746: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_spacer__7Pyg3"), bit_pos: 746 }
pub fn matches_rule_746(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_spacer__7Pyg3")
}

// Rule 748: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_stacking-context__3PbQE"), bit_pos: 748 }
pub fn matches_rule_748(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_stacking-context__3PbQE")
}

// Rule 750: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_theming-background-override__1HfzJ"), bit_pos: 750 }
pub fn matches_rule_750(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_theming-background-override__1HfzJ")
}

// Rule 752: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_themingTextColorWhite__1zryO"), bit_pos: 752 }
pub fn matches_rule_752(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_themingTextColorWhite__1zryO")
}

// Rule 754: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_themingTextColor__1oQsI"), bit_pos: 754 }
pub fn matches_rule_754(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_themingTextColor__1oQsI")
}

// Rule 756: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_three-pack__5s3hP"), bit_pos: 756 }
pub fn matches_rule_756(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_three-pack__5s3hP")
}

// Rule 758: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_threeLineTruncation__UkUjj"), bit_pos: 758 }
pub fn matches_rule_758(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_threeLineTruncation__UkUjj")
}

// Rule 760: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_tile-container__1QgAV"), bit_pos: 760 }
pub fn matches_rule_760(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_tile-container__1QgAV")
}

// Rule 762: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_tile-grid__QMxNY"), bit_pos: 762 }
pub fn matches_rule_762(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_tile-grid__QMxNY")
}

// Rule 764: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_tile-link__38lTa"), bit_pos: 764 }
pub fn matches_rule_764(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_tile-link__38lTa")
}

// Rule 766: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_tile-theming__3eeyj"), bit_pos: 766 }
pub fn matches_rule_766(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_tile-theming__3eeyj")
}

// Rule 768: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_truncation__x9-69"), bit_pos: 768 }
pub fn matches_rule_768(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_truncation__x9-69")
}

// Rule 770: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_twoLineTruncation__16TLV"), bit_pos: 770 }
pub fn matches_rule_770(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_twoLineTruncation__16TLV")
}

// Rule 772: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_video-container__1hKS1"), bit_pos: 772 }
pub fn matches_rule_772(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_video-container__1hKS1")
}

// Rule 774: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_wd-backdrop-data__1znxG"), bit_pos: 774 }
pub fn matches_rule_774(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_wd-backdrop-data__1znxG")
}

// Rule 776: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_wdHeader__Edrev"), bit_pos: 776 }
pub fn matches_rule_776(node: &HtmlNode) -> bool {
    node.classes.contains("_fluid-quad-image-label-v2_style_wdHeader__Edrev")
}

// Rule 778: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_cardBody__3Rdh1"), bit_pos: 778 }
pub fn matches_rule_778(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_desktopStyle_cardBody__3Rdh1")
}

// Rule 780: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_categoryImage__35jKN"), bit_pos: 780 }
pub fn matches_rule_780(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_desktopStyle_categoryImage__35jKN")
}

// Rule 782: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_category__3flCQ"), bit_pos: 782 }
pub fn matches_rule_782(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_desktopStyle_category__3flCQ")
}

// Rule 784: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_heroCategory__3KS3k"), bit_pos: 784 }
pub fn matches_rule_784(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_desktopStyle_heroCategory__3KS3k")
}

// Rule 786: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_heroImage__2V8-9"), bit_pos: 786 }
pub fn matches_rule_786(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_desktopStyle_heroImage__2V8-9")
}

// Rule 788: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_heroLink__1EhW2"), bit_pos: 788 }
pub fn matches_rule_788(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_desktopStyle_heroLink__1EhW2")
}

// Rule 790: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_leftMost__1LmQB"), bit_pos: 790 }
pub fn matches_rule_790(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_desktopStyle_leftMost__1LmQB")
}

// Rule 792: CheckAndSetBit { selector: Class("_quad-category-card_fluid_fluidCardBody__3TzJ4"), bit_pos: 792 }
pub fn matches_rule_792(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_fluid_fluidCardBody__3TzJ4")
}

// Rule 794: CheckAndSetBit { selector: Class("_quad-category-card_fluid_fluidCard__3hmFA"), bit_pos: 794 }
pub fn matches_rule_794(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_fluid_fluidCard__3hmFA")
}

// Rule 796: CheckAndSetBit { selector: Class("_quad-category-card_image_asin-container-full-height__MOKlF"), bit_pos: 796 }
pub fn matches_rule_796(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_image_asin-container-full-height__MOKlF")
}

// Rule 798: CheckAndSetBit { selector: Class("_quad-category-card_image_asin-container-white-box__3Stwp"), bit_pos: 798 }
pub fn matches_rule_798(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_image_asin-container-white-box__3Stwp")
}

// Rule 800: CheckAndSetBit { selector: Class("_quad-category-card_image_asin-container__LRY5p"), bit_pos: 800 }
pub fn matches_rule_800(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_image_asin-container__LRY5p")
}

// Rule 802: CheckAndSetBit { selector: Class("_quad-category-card_image_round-corners__22iOW"), bit_pos: 802 }
pub fn matches_rule_802(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_image_round-corners__22iOW")
}

// Rule 804: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_cardBody__3ODbW"), bit_pos: 804 }
pub fn matches_rule_804(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_mobileStyle_cardBody__3ODbW")
}

// Rule 806: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_categoryContainer__2xY0I"), bit_pos: 806 }
pub fn matches_rule_806(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_mobileStyle_categoryContainer__2xY0I")
}

// Rule 808: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_categoryImage__3hSFw"), bit_pos: 808 }
pub fn matches_rule_808(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_mobileStyle_categoryImage__3hSFw")
}

// Rule 810: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_category__1amt4"), bit_pos: 810 }
pub fn matches_rule_810(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_mobileStyle_category__1amt4")
}

// Rule 812: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_heroImage__1SewP"), bit_pos: 812 }
pub fn matches_rule_812(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_mobileStyle_heroImage__1SewP")
}

// Rule 814: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_leftMost__3WtU6"), bit_pos: 814 }
pub fn matches_rule_814(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_mobileStyle_leftMost__3WtU6")
}

// Rule 816: CheckAndSetBit { selector: Class("_quad-category-card_style_dashboard-card-with-border__1e4z_"), bit_pos: 816 }
pub fn matches_rule_816(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_style_dashboard-card-with-border__1e4z_")
}

// Rule 818: CheckAndSetBit { selector: Class("_quad-category-card_style_fluidImageContainer__2jd50"), bit_pos: 818 }
pub fn matches_rule_818(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_style_fluidImageContainer__2jd50")
}

// Rule 820: CheckAndSetBit { selector: Class("_quad-category-card_style_fluidLandscapeImage__3eTVC"), bit_pos: 820 }
pub fn matches_rule_820(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_style_fluidLandscapeImage__3eTVC")
}

// Rule 822: CheckAndSetBit { selector: Class("_quad-category-card_style_fluidPortraitImage__3yQ-X"), bit_pos: 822 }
pub fn matches_rule_822(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_style_fluidPortraitImage__3yQ-X")
}

// Rule 824: CheckAndSetBit { selector: Class("_quad-category-card_style_gwm-link-footer__3EX7d"), bit_pos: 824 }
pub fn matches_rule_824(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_style_gwm-link-footer__3EX7d")
}

// Rule 826: CheckAndSetBit { selector: Class("_quad-category-card_style_heading__1mnEu"), bit_pos: 826 }
pub fn matches_rule_826(node: &HtmlNode) -> bool {
    node.classes.contains("_quad-category-card_style_heading__1mnEu")
}

// Rule 828: CheckAndSetBit { selector: Class("_text-link-stripe-v2_style_textlinkstripe__3aQhz"), bit_pos: 828 }
pub fn matches_rule_828(node: &HtmlNode) -> bool {
    node.classes.contains("_text-link-stripe-v2_style_textlinkstripe__3aQhz")
}

// Rule 830: CheckAndSetBit { selector: Class("a-cardui-body"), bit_pos: 830 }
pub fn matches_rule_830(node: &HtmlNode) -> bool {
    node.classes.contains("a-cardui-body")
}

// Rule 832: CheckAndSetBit { selector: Class("a-cardui-footer"), bit_pos: 832 }
pub fn matches_rule_832(node: &HtmlNode) -> bool {
    node.classes.contains("a-cardui-footer")
}

// Rule 834: CheckAndSetBit { selector: Class("a-cardui-header"), bit_pos: 834 }
pub fn matches_rule_834(node: &HtmlNode) -> bool {
    node.classes.contains("a-cardui-header")
}

// Rule 836: CheckAndSetBit { selector: Class("a-carousel-container"), bit_pos: 836 }
pub fn matches_rule_836(node: &HtmlNode) -> bool {
    node.classes.contains("a-carousel-container")
}

// Rule 838: CheckAndSetBit { selector: Class("a-carousel-controls"), bit_pos: 838 }
pub fn matches_rule_838(node: &HtmlNode) -> bool {
    node.classes.contains("a-carousel-controls")
}

// Rule 840: CheckAndSetBit { selector: Class("a-carousel-right"), bit_pos: 840 }
pub fn matches_rule_840(node: &HtmlNode) -> bool {
    node.classes.contains("a-carousel-right")
}

// Rule 842: CheckAndSetBit { selector: Class("a-carousel-viewport"), bit_pos: 842 }
pub fn matches_rule_842(node: &HtmlNode) -> bool {
    node.classes.contains("a-carousel-viewport")
}

// Rule 844: CheckAndSetBit { selector: Class("a-link-normal"), bit_pos: 844 }
pub fn matches_rule_844(node: &HtmlNode) -> bool {
    node.classes.contains("a-link-normal")
}

// Rule 846: CheckAndSetBit { selector: Class("card-flow-row-break"), bit_pos: 846 }
pub fn matches_rule_846(node: &HtmlNode) -> bool {
    node.classes.contains("card-flow-row-break")
}

// Rule 848: CheckAndSetBit { selector: Class("gw-auto-height"), bit_pos: 848 }
pub fn matches_rule_848(node: &HtmlNode) -> bool {
    node.classes.contains("gw-auto-height")
}

// Rule 850: CheckAndSetBit { selector: Class("gw-card-layout"), bit_pos: 850 }
pub fn matches_rule_850(node: &HtmlNode) -> bool {
    node.classes.contains("gw-card-layout")
}

// Rule 852: CheckAndSetBit { selector: Class("gw-col"), bit_pos: 852 }
pub fn matches_rule_852(node: &HtmlNode) -> bool {
    node.classes.contains("gw-col")
}

// Rule 854: CheckAndSetBit { selector: Class("gw-fixed-col"), bit_pos: 854 }
pub fn matches_rule_854(node: &HtmlNode) -> bool {
    node.classes.contains("gw-fixed-col")
}

// Rule 856: CheckAndSetBit { selector: Class("gw-media-card"), bit_pos: 856 }
pub fn matches_rule_856(node: &HtmlNode) -> bool {
    node.classes.contains("gw-media-card")
}

// Rule 858: CheckAndSetBit { selector: Class("gw-row"), bit_pos: 858 }
pub fn matches_rule_858(node: &HtmlNode) -> bool {
    node.classes.contains("gw-row")
}

// Rule 860: CheckAndSetBit { selector: Class("nav-focus"), bit_pos: 860 }
pub fn matches_rule_860(node: &HtmlNode) -> bool {
    node.classes.contains("nav-focus")
}

// Rule 862: CheckAndSetBit { selector: Class("nav-spinner"), bit_pos: 862 }
pub fn matches_rule_862(node: &HtmlNode) -> bool {
    node.classes.contains("nav-spinner")
}

// Rule 864: CheckAndSetBit { selector: Class("nav-timeline-prime-icon"), bit_pos: 864 }
pub fn matches_rule_864(node: &HtmlNode) -> bool {
    node.classes.contains("nav-timeline-prime-icon")
}

// Rule 866: CheckAndSetBit { selector: Class("single-slide-hero"), bit_pos: 866 }
pub fn matches_rule_866(node: &HtmlNode) -> bool {
    node.classes.contains("single-slide-hero")
}

// Rule 868: CheckAndSetBit { selector: Class("truncate-1line"), bit_pos: 868 }
pub fn matches_rule_868(node: &HtmlNode) -> bool {
    node.classes.contains("truncate-1line")
}

// Rule 870: CheckAndSetBit { selector: Class("truncate-2line"), bit_pos: 870 }
pub fn matches_rule_870(node: &HtmlNode) -> bool {
    node.classes.contains("truncate-2line")
}

// Rule 872: CheckAndSetBit { selector: Class("vjs-fluid"), bit_pos: 872 }
pub fn matches_rule_872(node: &HtmlNode) -> bool {
    node.classes.contains("vjs-fluid")
}

// Rule 874: CheckAndSetBit { selector: Id("icp-touch-link-cop"), bit_pos: 874 }
pub fn matches_rule_874(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"icp-touch-link-cop".to_string())
}

// Rule 876: CheckAndSetBit { selector: Id("icp-touch-link-country"), bit_pos: 876 }
pub fn matches_rule_876(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"icp-touch-link-country".to_string())
}

// Rule 878: CheckAndSetBit { selector: Id("icp-touch-link-language"), bit_pos: 878 }
pub fn matches_rule_878(node: &HtmlNode) -> bool {
    node.id.as_ref() == Some(&"icp-touch-link-language".to_string())
}

// Rule 880: CheckAndSetBit { selector: Type("h3"), bit_pos: 880 }
pub fn matches_rule_880(node: &HtmlNode) -> bool {
    node.tag_name == "h3"
}

// Rule 882: CheckAndSetBit { selector: Type("span"), bit_pos: 882 }
pub fn matches_rule_882(node: &HtmlNode) -> bool {
    node.tag_name == "span"
}

// === MAIN NAIVE PROCESSING FUNCTION ===
pub fn process_node_naive(node: &mut HtmlNode, parent_matches: &[bool]) -> Vec<bool> {
    let mut matches = vec![false; 884];
    
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
    if matches_rule_222(node) {
        matches[222] = true;
    }
    if matches_rule_224(node) {
        matches[224] = true;
    }
    if matches_rule_226(node) {
        matches[226] = true;
    }
    if matches_rule_228(node) {
        matches[228] = true;
    }
    if matches_rule_230(node) {
        matches[230] = true;
    }
    if matches_rule_232(node) {
        matches[232] = true;
    }
    if matches_rule_234(node) {
        matches[234] = true;
    }
    if matches_rule_236(node) {
        matches[236] = true;
    }
    if matches_rule_238(node) {
        matches[238] = true;
    }
    if matches_rule_240(node) {
        matches[240] = true;
    }
    if matches_rule_242(node) {
        matches[242] = true;
    }
    if matches_rule_244(node) {
        matches[244] = true;
    }
    if matches_rule_246(node) {
        matches[246] = true;
    }
    if matches_rule_248(node) {
        matches[248] = true;
    }
    if matches_rule_250(node) {
        matches[250] = true;
    }
    if matches_rule_252(node) {
        matches[252] = true;
    }
    if matches_rule_254(node) {
        matches[254] = true;
    }
    if matches_rule_256(node) {
        matches[256] = true;
    }
    if matches_rule_258(node) {
        matches[258] = true;
    }
    if matches_rule_260(node) {
        matches[260] = true;
    }
    if matches_rule_262(node) {
        matches[262] = true;
    }
    if matches_rule_264(node) {
        matches[264] = true;
    }
    if matches_rule_266(node) {
        matches[266] = true;
    }
    if matches_rule_268(node) {
        matches[268] = true;
    }
    if matches_rule_270(node) {
        matches[270] = true;
    }
    if matches_rule_272(node) {
        matches[272] = true;
    }
    if matches_rule_274(node) {
        matches[274] = true;
    }
    if matches_rule_276(node) {
        matches[276] = true;
    }
    if matches_rule_278(node) {
        matches[278] = true;
    }
    if matches_rule_280(node) {
        matches[280] = true;
    }
    if matches_rule_282(node) {
        matches[282] = true;
    }
    if matches_rule_284(node) {
        matches[284] = true;
    }
    if matches_rule_286(node) {
        matches[286] = true;
    }
    if matches_rule_288(node) {
        matches[288] = true;
    }
    if matches_rule_290(node) {
        matches[290] = true;
    }
    if matches_rule_292(node) {
        matches[292] = true;
    }
    if matches_rule_294(node) {
        matches[294] = true;
    }
    if matches_rule_296(node) {
        matches[296] = true;
    }
    if matches_rule_298(node) {
        matches[298] = true;
    }
    if matches_rule_300(node) {
        matches[300] = true;
    }
    if matches_rule_302(node) {
        matches[302] = true;
    }
    if matches_rule_304(node) {
        matches[304] = true;
    }
    if matches_rule_306(node) {
        matches[306] = true;
    }
    if matches_rule_308(node) {
        matches[308] = true;
    }
    if matches_rule_310(node) {
        matches[310] = true;
    }
    if matches_rule_312(node) {
        matches[312] = true;
    }
    if matches_rule_314(node) {
        matches[314] = true;
    }
    if matches_rule_316(node) {
        matches[316] = true;
    }
    if matches_rule_318(node) {
        matches[318] = true;
    }
    if matches_rule_320(node) {
        matches[320] = true;
    }
    if matches_rule_322(node) {
        matches[322] = true;
    }
    if matches_rule_324(node) {
        matches[324] = true;
    }
    if matches_rule_326(node) {
        matches[326] = true;
    }
    if matches_rule_328(node) {
        matches[328] = true;
    }
    if matches_rule_330(node) {
        matches[330] = true;
    }
    if matches_rule_332(node) {
        matches[332] = true;
    }
    if matches_rule_334(node) {
        matches[334] = true;
    }
    if matches_rule_336(node) {
        matches[336] = true;
    }
    if matches_rule_338(node) {
        matches[338] = true;
    }
    if matches_rule_340(node) {
        matches[340] = true;
    }
    if matches_rule_342(node) {
        matches[342] = true;
    }
    if matches_rule_344(node) {
        matches[344] = true;
    }
    if matches_rule_346(node) {
        matches[346] = true;
    }
    if matches_rule_348(node) {
        matches[348] = true;
    }
    if matches_rule_350(node) {
        matches[350] = true;
    }
    if matches_rule_352(node) {
        matches[352] = true;
    }
    if matches_rule_354(node) {
        matches[354] = true;
    }
    if matches_rule_356(node) {
        matches[356] = true;
    }
    if matches_rule_358(node) {
        matches[358] = true;
    }
    if matches_rule_360(node) {
        matches[360] = true;
    }
    if matches_rule_362(node) {
        matches[362] = true;
    }
    if matches_rule_364(node) {
        matches[364] = true;
    }
    if matches_rule_366(node) {
        matches[366] = true;
    }
    if matches_rule_368(node) {
        matches[368] = true;
    }
    if matches_rule_370(node) {
        matches[370] = true;
    }
    if matches_rule_372(node) {
        matches[372] = true;
    }
    if matches_rule_374(node) {
        matches[374] = true;
    }
    if matches_rule_376(node) {
        matches[376] = true;
    }
    if matches_rule_378(node) {
        matches[378] = true;
    }
    if matches_rule_380(node) {
        matches[380] = true;
    }
    if matches_rule_382(node) {
        matches[382] = true;
    }
    if matches_rule_384(node) {
        matches[384] = true;
    }
    if matches_rule_386(node) {
        matches[386] = true;
    }
    if matches_rule_388(node) {
        matches[388] = true;
    }
    if matches_rule_390(node) {
        matches[390] = true;
    }
    if matches_rule_392(node) {
        matches[392] = true;
    }
    if matches_rule_394(node) {
        matches[394] = true;
    }
    if matches_rule_396(node) {
        matches[396] = true;
    }
    if matches_rule_398(node) {
        matches[398] = true;
    }
    if matches_rule_400(node) {
        matches[400] = true;
    }
    if matches_rule_402(node) {
        matches[402] = true;
    }
    if matches_rule_404(node) {
        matches[404] = true;
    }
    if matches_rule_406(node) {
        matches[406] = true;
    }
    if matches_rule_408(node) {
        matches[408] = true;
    }
    if matches_rule_410(node) {
        matches[410] = true;
    }
    if matches_rule_412(node) {
        matches[412] = true;
    }
    if matches_rule_414(node) {
        matches[414] = true;
    }
    if matches_rule_416(node) {
        matches[416] = true;
    }
    if matches_rule_418(node) {
        matches[418] = true;
    }
    if matches_rule_420(node) {
        matches[420] = true;
    }
    if matches_rule_422(node) {
        matches[422] = true;
    }
    if matches_rule_424(node) {
        matches[424] = true;
    }
    if matches_rule_426(node) {
        matches[426] = true;
    }
    if matches_rule_428(node) {
        matches[428] = true;
    }
    if matches_rule_430(node) {
        matches[430] = true;
    }
    if matches_rule_432(node) {
        matches[432] = true;
    }
    if matches_rule_434(node) {
        matches[434] = true;
    }
    if matches_rule_436(node) {
        matches[436] = true;
    }
    if matches_rule_438(node) {
        matches[438] = true;
    }
    if matches_rule_440(node) {
        matches[440] = true;
    }
    if matches_rule_442(node) {
        matches[442] = true;
    }
    if matches_rule_444(node) {
        matches[444] = true;
    }
    if matches_rule_446(node) {
        matches[446] = true;
    }
    if matches_rule_448(node) {
        matches[448] = true;
    }
    if matches_rule_450(node) {
        matches[450] = true;
    }
    if matches_rule_452(node) {
        matches[452] = true;
    }
    if matches_rule_454(node) {
        matches[454] = true;
    }
    if matches_rule_456(node) {
        matches[456] = true;
    }
    if matches_rule_458(node) {
        matches[458] = true;
    }
    if matches_rule_460(node) {
        matches[460] = true;
    }
    if matches_rule_462(node) {
        matches[462] = true;
    }
    if matches_rule_464(node) {
        matches[464] = true;
    }
    if matches_rule_466(node) {
        matches[466] = true;
    }
    if matches_rule_468(node) {
        matches[468] = true;
    }
    if matches_rule_470(node) {
        matches[470] = true;
    }
    if matches_rule_472(node) {
        matches[472] = true;
    }
    if matches_rule_474(node) {
        matches[474] = true;
    }
    if matches_rule_476(node) {
        matches[476] = true;
    }
    if matches_rule_478(node) {
        matches[478] = true;
    }
    if matches_rule_480(node) {
        matches[480] = true;
    }
    if matches_rule_482(node) {
        matches[482] = true;
    }
    if matches_rule_484(node) {
        matches[484] = true;
    }
    if matches_rule_486(node) {
        matches[486] = true;
    }
    if matches_rule_488(node) {
        matches[488] = true;
    }
    if matches_rule_490(node) {
        matches[490] = true;
    }
    if matches_rule_492(node) {
        matches[492] = true;
    }
    if matches_rule_494(node) {
        matches[494] = true;
    }
    if matches_rule_496(node) {
        matches[496] = true;
    }
    if matches_rule_498(node) {
        matches[498] = true;
    }
    if matches_rule_500(node) {
        matches[500] = true;
    }
    if matches_rule_502(node) {
        matches[502] = true;
    }
    if matches_rule_504(node) {
        matches[504] = true;
    }
    if matches_rule_506(node) {
        matches[506] = true;
    }
    if matches_rule_508(node) {
        matches[508] = true;
    }
    if matches_rule_510(node) {
        matches[510] = true;
    }
    if matches_rule_512(node) {
        matches[512] = true;
    }
    if matches_rule_514(node) {
        matches[514] = true;
    }
    if matches_rule_516(node) {
        matches[516] = true;
    }
    if matches_rule_518(node) {
        matches[518] = true;
    }
    if matches_rule_520(node) {
        matches[520] = true;
    }
    if matches_rule_522(node) {
        matches[522] = true;
    }
    if matches_rule_524(node) {
        matches[524] = true;
    }
    if matches_rule_526(node) {
        matches[526] = true;
    }
    if matches_rule_528(node) {
        matches[528] = true;
    }
    if matches_rule_530(node) {
        matches[530] = true;
    }
    if matches_rule_532(node) {
        matches[532] = true;
    }
    if matches_rule_534(node) {
        matches[534] = true;
    }
    if matches_rule_536(node) {
        matches[536] = true;
    }
    if matches_rule_538(node) {
        matches[538] = true;
    }
    if matches_rule_540(node) {
        matches[540] = true;
    }
    if matches_rule_542(node) {
        matches[542] = true;
    }
    if matches_rule_544(node) {
        matches[544] = true;
    }
    if matches_rule_546(node) {
        matches[546] = true;
    }
    if matches_rule_548(node) {
        matches[548] = true;
    }
    if matches_rule_550(node) {
        matches[550] = true;
    }
    if matches_rule_552(node) {
        matches[552] = true;
    }
    if matches_rule_554(node) {
        matches[554] = true;
    }
    if matches_rule_556(node) {
        matches[556] = true;
    }
    if matches_rule_558(node) {
        matches[558] = true;
    }
    if matches_rule_560(node) {
        matches[560] = true;
    }
    if matches_rule_562(node) {
        matches[562] = true;
    }
    if matches_rule_564(node) {
        matches[564] = true;
    }
    if matches_rule_566(node) {
        matches[566] = true;
    }
    if matches_rule_568(node) {
        matches[568] = true;
    }
    if matches_rule_570(node) {
        matches[570] = true;
    }
    if matches_rule_572(node) {
        matches[572] = true;
    }
    if matches_rule_574(node) {
        matches[574] = true;
    }
    if matches_rule_576(node) {
        matches[576] = true;
    }
    if matches_rule_578(node) {
        matches[578] = true;
    }
    if matches_rule_580(node) {
        matches[580] = true;
    }
    if matches_rule_582(node) {
        matches[582] = true;
    }
    if matches_rule_584(node) {
        matches[584] = true;
    }
    if matches_rule_586(node) {
        matches[586] = true;
    }
    if matches_rule_588(node) {
        matches[588] = true;
    }
    if matches_rule_590(node) {
        matches[590] = true;
    }
    if matches_rule_592(node) {
        matches[592] = true;
    }
    if matches_rule_594(node) {
        matches[594] = true;
    }
    if matches_rule_596(node) {
        matches[596] = true;
    }
    if matches_rule_598(node) {
        matches[598] = true;
    }
    if matches_rule_600(node) {
        matches[600] = true;
    }
    if matches_rule_602(node) {
        matches[602] = true;
    }
    if matches_rule_604(node) {
        matches[604] = true;
    }
    if matches_rule_606(node) {
        matches[606] = true;
    }
    if matches_rule_608(node) {
        matches[608] = true;
    }
    if matches_rule_610(node) {
        matches[610] = true;
    }
    if matches_rule_612(node) {
        matches[612] = true;
    }
    if matches_rule_614(node) {
        matches[614] = true;
    }
    if matches_rule_616(node) {
        matches[616] = true;
    }
    if matches_rule_618(node) {
        matches[618] = true;
    }
    if matches_rule_620(node) {
        matches[620] = true;
    }
    if matches_rule_622(node) {
        matches[622] = true;
    }
    if matches_rule_624(node) {
        matches[624] = true;
    }
    if matches_rule_626(node) {
        matches[626] = true;
    }
    if matches_rule_628(node) {
        matches[628] = true;
    }
    if matches_rule_630(node) {
        matches[630] = true;
    }
    if matches_rule_632(node) {
        matches[632] = true;
    }
    if matches_rule_634(node) {
        matches[634] = true;
    }
    if matches_rule_636(node) {
        matches[636] = true;
    }
    if matches_rule_638(node) {
        matches[638] = true;
    }
    if matches_rule_640(node) {
        matches[640] = true;
    }
    if matches_rule_642(node) {
        matches[642] = true;
    }
    if matches_rule_644(node) {
        matches[644] = true;
    }
    if matches_rule_646(node) {
        matches[646] = true;
    }
    if matches_rule_648(node) {
        matches[648] = true;
    }
    if matches_rule_650(node) {
        matches[650] = true;
    }
    if matches_rule_652(node) {
        matches[652] = true;
    }
    if matches_rule_654(node) {
        matches[654] = true;
    }
    if matches_rule_656(node) {
        matches[656] = true;
    }
    if matches_rule_658(node) {
        matches[658] = true;
    }
    if matches_rule_660(node) {
        matches[660] = true;
    }
    if matches_rule_662(node) {
        matches[662] = true;
    }
    if matches_rule_664(node) {
        matches[664] = true;
    }
    if matches_rule_666(node) {
        matches[666] = true;
    }
    if matches_rule_668(node) {
        matches[668] = true;
    }
    if matches_rule_670(node) {
        matches[670] = true;
    }
    if matches_rule_672(node) {
        matches[672] = true;
    }
    if matches_rule_674(node) {
        matches[674] = true;
    }
    if matches_rule_676(node) {
        matches[676] = true;
    }
    if matches_rule_678(node) {
        matches[678] = true;
    }
    if matches_rule_680(node) {
        matches[680] = true;
    }
    if matches_rule_682(node) {
        matches[682] = true;
    }
    if matches_rule_684(node) {
        matches[684] = true;
    }
    if matches_rule_686(node) {
        matches[686] = true;
    }
    if matches_rule_688(node) {
        matches[688] = true;
    }
    if matches_rule_690(node) {
        matches[690] = true;
    }
    if matches_rule_692(node) {
        matches[692] = true;
    }
    if matches_rule_694(node) {
        matches[694] = true;
    }
    if matches_rule_696(node) {
        matches[696] = true;
    }
    if matches_rule_698(node) {
        matches[698] = true;
    }
    if matches_rule_700(node) {
        matches[700] = true;
    }
    if matches_rule_702(node) {
        matches[702] = true;
    }
    if matches_rule_704(node) {
        matches[704] = true;
    }
    if matches_rule_706(node) {
        matches[706] = true;
    }
    if matches_rule_708(node) {
        matches[708] = true;
    }
    if matches_rule_710(node) {
        matches[710] = true;
    }
    if matches_rule_712(node) {
        matches[712] = true;
    }
    if matches_rule_714(node) {
        matches[714] = true;
    }
    if matches_rule_716(node) {
        matches[716] = true;
    }
    if matches_rule_718(node) {
        matches[718] = true;
    }
    if matches_rule_720(node) {
        matches[720] = true;
    }
    if matches_rule_722(node) {
        matches[722] = true;
    }
    if matches_rule_724(node) {
        matches[724] = true;
    }
    if matches_rule_726(node) {
        matches[726] = true;
    }
    if matches_rule_728(node) {
        matches[728] = true;
    }
    if matches_rule_730(node) {
        matches[730] = true;
    }
    if matches_rule_732(node) {
        matches[732] = true;
    }
    if matches_rule_734(node) {
        matches[734] = true;
    }
    if matches_rule_736(node) {
        matches[736] = true;
    }
    if matches_rule_738(node) {
        matches[738] = true;
    }
    if matches_rule_740(node) {
        matches[740] = true;
    }
    if matches_rule_742(node) {
        matches[742] = true;
    }
    if matches_rule_744(node) {
        matches[744] = true;
    }
    if matches_rule_746(node) {
        matches[746] = true;
    }
    if matches_rule_748(node) {
        matches[748] = true;
    }
    if matches_rule_750(node) {
        matches[750] = true;
    }
    if matches_rule_752(node) {
        matches[752] = true;
    }
    if matches_rule_754(node) {
        matches[754] = true;
    }
    if matches_rule_756(node) {
        matches[756] = true;
    }
    if matches_rule_758(node) {
        matches[758] = true;
    }
    if matches_rule_760(node) {
        matches[760] = true;
    }
    if matches_rule_762(node) {
        matches[762] = true;
    }
    if matches_rule_764(node) {
        matches[764] = true;
    }
    if matches_rule_766(node) {
        matches[766] = true;
    }
    if matches_rule_768(node) {
        matches[768] = true;
    }
    if matches_rule_770(node) {
        matches[770] = true;
    }
    if matches_rule_772(node) {
        matches[772] = true;
    }
    if matches_rule_774(node) {
        matches[774] = true;
    }
    if matches_rule_776(node) {
        matches[776] = true;
    }
    if matches_rule_778(node) {
        matches[778] = true;
    }
    if matches_rule_780(node) {
        matches[780] = true;
    }
    if matches_rule_782(node) {
        matches[782] = true;
    }
    if matches_rule_784(node) {
        matches[784] = true;
    }
    if matches_rule_786(node) {
        matches[786] = true;
    }
    if matches_rule_788(node) {
        matches[788] = true;
    }
    if matches_rule_790(node) {
        matches[790] = true;
    }
    if matches_rule_792(node) {
        matches[792] = true;
    }
    if matches_rule_794(node) {
        matches[794] = true;
    }
    if matches_rule_796(node) {
        matches[796] = true;
    }
    if matches_rule_798(node) {
        matches[798] = true;
    }
    if matches_rule_800(node) {
        matches[800] = true;
    }
    if matches_rule_802(node) {
        matches[802] = true;
    }
    if matches_rule_804(node) {
        matches[804] = true;
    }
    if matches_rule_806(node) {
        matches[806] = true;
    }
    if matches_rule_808(node) {
        matches[808] = true;
    }
    if matches_rule_810(node) {
        matches[810] = true;
    }
    if matches_rule_812(node) {
        matches[812] = true;
    }
    if matches_rule_814(node) {
        matches[814] = true;
    }
    if matches_rule_816(node) {
        matches[816] = true;
    }
    if matches_rule_818(node) {
        matches[818] = true;
    }
    if matches_rule_820(node) {
        matches[820] = true;
    }
    if matches_rule_822(node) {
        matches[822] = true;
    }
    if matches_rule_824(node) {
        matches[824] = true;
    }
    if matches_rule_826(node) {
        matches[826] = true;
    }
    if matches_rule_828(node) {
        matches[828] = true;
    }
    if matches_rule_830(node) {
        matches[830] = true;
    }
    if matches_rule_832(node) {
        matches[832] = true;
    }
    if matches_rule_834(node) {
        matches[834] = true;
    }
    if matches_rule_836(node) {
        matches[836] = true;
    }
    if matches_rule_838(node) {
        matches[838] = true;
    }
    if matches_rule_840(node) {
        matches[840] = true;
    }
    if matches_rule_842(node) {
        matches[842] = true;
    }
    if matches_rule_844(node) {
        matches[844] = true;
    }
    if matches_rule_846(node) {
        matches[846] = true;
    }
    if matches_rule_848(node) {
        matches[848] = true;
    }
    if matches_rule_850(node) {
        matches[850] = true;
    }
    if matches_rule_852(node) {
        matches[852] = true;
    }
    if matches_rule_854(node) {
        matches[854] = true;
    }
    if matches_rule_856(node) {
        matches[856] = true;
    }
    if matches_rule_858(node) {
        matches[858] = true;
    }
    if matches_rule_860(node) {
        matches[860] = true;
    }
    if matches_rule_862(node) {
        matches[862] = true;
    }
    if matches_rule_864(node) {
        matches[864] = true;
    }
    if matches_rule_866(node) {
        matches[866] = true;
    }
    if matches_rule_868(node) {
        matches[868] = true;
    }
    if matches_rule_870(node) {
        matches[870] = true;
    }
    if matches_rule_872(node) {
        matches[872] = true;
    }
    if matches_rule_874(node) {
        matches[874] = true;
    }
    if matches_rule_876(node) {
        matches[876] = true;
    }
    if matches_rule_878(node) {
        matches[878] = true;
    }
    if matches_rule_880(node) {
        matches[880] = true;
    }
    if matches_rule_882(node) {
        matches[882] = true;
    }
    
    // Check all parent-child rules
    // No parent-child rules to check
    let _ = parent_matches; // Suppress unused parameter warning
    
    matches
}

// === NAIVE TREE TRAVERSAL ===
pub fn process_tree_naive(root: &mut HtmlNode) -> usize {
    let mut total_nodes = 0;
    let empty_parent = vec![false; 884];
    process_tree_recursive_naive(root, &empty_parent, &mut total_nodes);
            total_nodes
        }
        fn process_tree_recursive_naive(node: &mut HtmlNode, parent_matches: &[bool], total: &mut usize) {
            *total += 1;
            
            // Calculate matches for this node from scratch
            let node_matches = process_node_naive(node, parent_matches);
                // Process all children with this node's matches as their parent context,
            for child in node.children.iter_mut() {
                process_tree_recursive_naive(child, &node_matches, total);
            }
        }

        pub fn get_rule_name(rule_index: usize) -> String {
            format!("rule_{}", rule_index)
        }// Rule mapping:
// Rule 565: active_Class("_fluid-fat-image-link-v2_style_tile-theming__3eeyj")
// Rule 568: match_Class("_fluid-fat-image-link-v2_style_twoLineTruncation__16TLV")
// Rule 18: match_Class("_ameyal-product-shoveler_image_asin-container__2jyCM")
// Rule 643: active_Class("_fluid-quad-image-label-v2_style_badgeMessage__2Dtw7")
// Rule 56: match_Class("_ameyal-product-shoveler_style_aspect-ratio-fill__2Zjfb")
// Rule 342: match_Class("_cropped-image-link_style_stacking-context__3PbQE")
// Rule 510: match_Class("_fluid-fat-image-link-v2_style_logoSquareContainer__3Paoc")
// Rule 296: match_Class("_cropped-image-link_style_gwm-link-footer__3OF47")
// Rule 603: active_Class("_fluid-quad-image-label-v2_style_ad-feedback-primary-link__2bIZi")
// Rule 104: match_Class("_ameyal-product-shoveler_style_header-link__cUhOK")
// Rule 625: active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-16x9__cBPv8")
// Rule 198: match_Class("_cropped-image-link_image_asin-container-white-box__QwmgO")
// Rule 249: active_Class("_cropped-image-link_style_badge-container__20aJ2")
// Rule 657: active_Class("_fluid-quad-image-label-v2_style_cover-portrait-image__2lhzL")
// Rule 658: match_Class("_fluid-quad-image-label-v2_style_cta-link__2xo74")
// Rule 468: match_Class("_fluid-fat-image-link-v2_style_empty-footer__2d59h")
// Rule 17: active_Class("_ameyal-product-shoveler_image_asin-container-white-box__QwmgO")
// Rule 442: match_Class("_fluid-fat-image-link-v2_style_badgeLabel__pJ5rc")
// Rule 107: active_Class("_ameyal-product-shoveler_style_header__1vGdj")
// Rule 122: match_Class("_ameyal-product-shoveler_style_mixed-button__2og-m")
// Rule 557: active_Class("_fluid-fat-image-link-v2_style_threeLineTruncation__UkUjj")
// Rule 262: match_Class("_cropped-image-link_style_close-icon__2RJs3")
// Rule 179: active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
// Rule 623: active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-15x3__1h649")
// Rule 62: match_Class("_ameyal-product-shoveler_style_badge-container__20aJ2")
// Rule 293: active_Class("_cropped-image-link_style_four-pack__1ufgr")
// Rule 718: match_Class("_fluid-quad-image-label-v2_style_mixed-button__2og-m")
// Rule 437: active_Class("_fluid-fat-image-link-v2_style_aspect-text__S4PU1")
// Rule 771: active_Class("_fluid-quad-image-label-v2_style_twoLineTruncation__16TLV")
// Rule 809: active_Class("_quad-category-card_mobileStyle_categoryImage__3hSFw")
// Rule 812: match_Class("_quad-category-card_mobileStyle_heroImage__1SewP")
// Rule 655: active_Class("_fluid-quad-image-label-v2_style_close-text__2-gwn")
// Rule 110: match_Class("_ameyal-product-shoveler_style_inlineErrorDetails__1NBx-")
// Rule 142: match_Class("_ameyal-product-shoveler_style_smartText__ubpEw")
// Rule 790: match_Class("_quad-category-card_desktopStyle_leftMost__1LmQB")
// Rule 779: active_Class("_quad-category-card_desktopStyle_cardBody__3Rdh1")
// Rule 404: match_Class("_fluid-fat-image-link-v2_style_ad-feedback-primary-link__2bIZi")
// Rule 479: active_Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2SOMr")
// Rule 397: active_Class("_fluid-fat-image-link-v2_singleLinkStyle_bodyFooterLink__9LvH0")
// Rule 696: match_Class("_fluid-quad-image-label-v2_style_header-link__cUhOK")
// Rule 829: active_Class("_text-link-stripe-v2_style_textlinkstripe__3aQhz")
// Rule 247: active_Class("_cropped-image-link_style_autoplay-span__2CMfc")
// Rule 588: match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
// Rule 788: match_Class("_quad-category-card_desktopStyle_heroLink__1EhW2")
// Rule 766: match_Class("_fluid-quad-image-label-v2_style_tile-theming__3eeyj")
// Rule 854: match_Class("gw-fixed-col")
// Rule 189: active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
// Rule 312: match_Class("_cropped-image-link_style_logoSquareContainer__3Paoc")
// Rule 307: active_Class("_cropped-image-link_style_image-container__2OiZA")
// Rule 185: active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
// Rule 508: match_Class("_fluid-fat-image-link-v2_style_logoRectangle__1VJwu")
// Rule 255: active_Class("_cropped-image-link_style_carouselContainer__3N7M1")
// Rule 407: active_Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite-mobile__2_rj8")
// Rule 131: active_Class("_ameyal-product-shoveler_style_negative-button__1Dvqz")
// Rule 109: active_Class("_ameyal-product-shoveler_style_image-container__2OiZA")
// Rule 748: match_Class("_fluid-quad-image-label-v2_style_stacking-context__3PbQE")
// Rule 23: active_Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner-rtl__2BoOY")
// Rule 394: match_Class("_fluid-fat-image-link-v2_image_round-corners__2y_fS")
// Rule 538: match_Class("_fluid-fat-image-link-v2_style_poster-image__1W0yA")
// Rule 203: active_Class("_cropped-image-link_image_asin-container__LRY5p")
// Rule 253: active_Class("_cropped-image-link_style_badgeMessage__2Dtw7")
// Rule 501: active_Class("_fluid-fat-image-link-v2_style_image-container__2OiZA")
// Rule 585: active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
// Rule 633: active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-fill__2Zjfb")
// Rule 221: active_Class("_cropped-image-link_style_ad-feedback-text__2HjQ9")
// Rule 328: match_Class("_cropped-image-link_style_negativeMarginAdjust__1nqu9")
// Rule 567: active_Class("_fluid-fat-image-link-v2_style_truncation__x9-69")
// Rule 745: active_Class("_fluid-quad-image-label-v2_style_spCSRFTreatment__-hwVO")
// Rule 865: active_Class("nav-timeline-prime-icon")
// Rule 880: match_Type("h3")
// Rule 268: match_Class("_cropped-image-link_style_cropped-image-link__3winf")
// Rule 174: match_Class("_ameyal-product-shoveler_style_wd-backdrop-data__1znxG")
// Rule 265: active_Class("_cropped-image-link_style_close-text__2-gwn")
// Rule 188: match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
// Rule 191: active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-standard__28gp8")
// Rule 136: match_Class("_ameyal-product-shoveler_style_overlay__3Sx3u")
// Rule 315: active_Class("_cropped-image-link_style_logoSquare__3NZyi")
// Rule 348: match_Class("_cropped-image-link_style_themingTextColor__1oQsI")
// Rule 252: match_Class("_cropped-image-link_style_badgeMessage__2Dtw7")
// Rule 586: match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
// Rule 645: active_Class("_fluid-quad-image-label-v2_style_carouselContainer__3N7M1")
// Rule 363: active_Class("_cropped-image-link_style_truncation__x9-69")
// Rule 750: match_Class("_fluid-quad-image-label-v2_style_theming-background-override__1HfzJ")
// Rule 398: match_Class("_fluid-fat-image-link-v2_singleLinkStyle_footer__2cH0y")
// Rule 673: active_Class("_fluid-quad-image-label-v2_style_fluidImageContainer__2SOMr")
// Rule 214: match_Class("_cropped-image-link_style_ad-feedback-sprite-mobile__2_rj8")
// Rule 292: match_Class("_cropped-image-link_style_four-pack__1ufgr")
// Rule 656: match_Class("_fluid-quad-image-label-v2_style_cover-portrait-image__2lhzL")
// Rule 839: active_Class("a-carousel-controls")
// Rule 209: active_Class("_cropped-image-link_style_ad-feedback-loading-spinnner-rtl__2BoOY")
// Rule 742: match_Class("_fluid-quad-image-label-v2_style_smartText__ubpEw")
// Rule 217: active_Class("_cropped-image-link_style_ad-feedback-sprite__28uwB")
// Rule 486: match_Class("_fluid-fat-image-link-v2_style_four-pack__1ufgr")
// Rule 87: active_Class("_ameyal-product-shoveler_style_dynamic-portrait-image__1Wrzd")
// Rule 596: match_Class("_fluid-quad-image-label-v2_image_round-corners__2y_fS")
// Rule 15: active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-container__1Pkva")
// Rule 339: active_Class("_cropped-image-link_style_smartText__ubpEw")
// Rule 259: active_Class("_cropped-image-link_style_close-black-icon__3hkbe")
// Rule 402: match_Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner__1nmZw")
// Rule 632: match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-fill__2Zjfb")
// Rule 207: active_Class("_cropped-image-link_image_round-corners__2y_fS")
// Rule 251: active_Class("_cropped-image-link_style_badgeLabel__pJ5rc")
// Rule 502: match_Class("_fluid-fat-image-link-v2_style_imageLabel__3ANSV")
// Rule 148: match_Class("_ameyal-product-shoveler_style_stacking-context__3PbQE")
// Rule 638: match_Class("_fluid-quad-image-label-v2_style_badge-container__20aJ2")
// Rule 729: active_Class("_fluid-quad-image-label-v2_style_negativeMarginAdjust__1nqu9")
// Rule 400: match_Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
// Rule 285: active_Class("_cropped-image-link_style_fluid-landscape-image__TE6PT")
// Rule 432: match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
// Rule 789: active_Class("_quad-category-card_desktopStyle_heroLink__1EhW2")
// Rule 877: active_Id("icp-touch-link-country")
// Rule 689: active_Class("_fluid-quad-image-label-v2_style_gw-hero-close-button__3svyZ")
// Rule 683: active_Class("_fluid-quad-image-label-v2_style_four-pack__1ufgr")
// Rule 277: active_Class("_cropped-image-link_style_dt-TextContainer__3nbU9")
// Rule 58: match_Class("_ameyal-product-shoveler_style_aspect-text__S4PU1")
// Rule 282: match_Class("_cropped-image-link_style_five-pack__1-Tql")
// Rule 61: active_Class("_ameyal-product-shoveler_style_autoplay-span__2CMfc")
// Rule 171: active_Class("_ameyal-product-shoveler_style_twoLineTruncation__16TLV")
// Rule 378: match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
// Rule 635: active_Class("_fluid-quad-image-label-v2_style_aspect-text__S4PU1")
// Rule 85: active_Class("_ameyal-product-shoveler_style_displayCount__1MVut")
// Rule 430: match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-5x8__2IaNz")
// Rule 39: active_Class("_ameyal-product-shoveler_style_apexBadgeMessage__1tHvd")
// Rule 245: active_Class("_cropped-image-link_style_aspect-text__S4PU1")
// Rule 626: match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-4x3__3BewI")
// Rule 755: active_Class("_fluid-quad-image-label-v2_style_themingTextColor__1oQsI")
// Rule 157: active_Class("_ameyal-product-shoveler_style_three-pack__5s3hP")
// Rule 305: active_Class("_cropped-image-link_style_header__1vGdj")
// Rule 547: active_Class("_fluid-fat-image-link-v2_style_stacking-context__3PbQE")
// Rule 520: match_Class("_fluid-fat-image-link-v2_style_mixed-button__2og-m")
// Rule 162: match_Class("_ameyal-product-shoveler_style_tile-grid__QMxNY")
// Rule 347: active_Class("_cropped-image-link_style_themingTextColorWhite__1zryO")
// Rule 401: active_Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
// Rule 561: active_Class("_fluid-fat-image-link-v2_style_tile-grid__QMxNY")
// Rule 50: match_Class("_ameyal-product-shoveler_style_aspect-ratio-4x3__3BewI")
// Rule 775: active_Class("_fluid-quad-image-label-v2_style_wd-backdrop-data__1znxG")
// Rule 464: match_Class("_fluid-fat-image-link-v2_style_displayCount__1MVut")
// Rule 88: match_Class("_ameyal-product-shoveler_style_empty-footer__2d59h")
// Rule 337: active_Class("_cropped-image-link_style_poster-image__1W0yA")
// Rule 719: active_Class("_fluid-quad-image-label-v2_style_mixed-button__2og-m")
// Rule 173: active_Class("_ameyal-product-shoveler_style_video-container__1hKS1")
// Rule 219: active_Class("_cropped-image-link_style_ad-feedback-text-desktop__q3xp_")
// Rule 445: active_Class("_fluid-fat-image-link-v2_style_badgeMessage__2Dtw7")
// Rule 834: match_Class("a-cardui-header")
// Rule 160: match_Class("_ameyal-product-shoveler_style_tile-container__1QgAV")
// Rule 563: active_Class("_fluid-fat-image-link-v2_style_tile-link__38lTa")
// Rule 730: match_Class("_fluid-quad-image-label-v2_style_oneLineTruncation__2WWse")
// Rule 158: match_Class("_ameyal-product-shoveler_style_threeLineTruncation__UkUjj")
// Rule 6: match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
// Rule 702: match_Class("_fluid-quad-image-label-v2_style_imageLabel__3ANSV")
// Rule 111: active_Class("_ameyal-product-shoveler_style_inlineErrorDetails__1NBx-")
// Rule 433: active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
// Rule 630: match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
// Rule 237: active_Class("_cropped-image-link_style_aspect-ratio-4x3__3BewI")
// Rule 233: active_Class("_cropped-image-link_style_aspect-ratio-15x3__1h649")
// Rule 517: active_Class("_fluid-fat-image-link-v2_style_mergedLinksCta__3Npog")
// Rule 101: active_Class("_ameyal-product-shoveler_style_haulRibbon__3VZNi")
// Rule 682: match_Class("_fluid-quad-image-label-v2_style_four-pack__1ufgr")
// Rule 741: active_Class("_fluid-quad-image-label-v2_style_rightQuadrant__PI01n")
// Rule 598: match_Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
// Rule 155: active_Class("_ameyal-product-shoveler_style_themingTextColor__1oQsI")
// Rule 873: active_Class("vjs-fluid")
// Rule 879: active_Id("icp-touch-link-language")
// Rule 581: active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
// Rule 43: active_Class("_ameyal-product-shoveler_style_aspect-button__7cH_E")
// Rule 578: match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
// Rule 359: active_Class("_cropped-image-link_style_tile-link__38lTa")
// Rule 176: match_Class("_ameyal-product-shoveler_style_wdHeader__Edrev")
// Rule 213: active_Class("_cropped-image-link_style_ad-feedback-primary-link__2bIZi")
// Rule 29: active_Class("_ameyal-product-shoveler_style_ad-feedback-sprite-mobile__2_rj8")
// Rule 274: match_Class("_cropped-image-link_style_displayCount__1MVut")
// Rule 875: active_Id("icp-touch-link-cop")
// Rule 325: active_Class("_cropped-image-link_style_mosaic-card__1C-_R")
// Rule 75: active_Class("_ameyal-product-shoveler_style_close-icon__2RJs3")
// Rule 765: active_Class("_fluid-quad-image-label-v2_style_tile-link__38lTa")
// Rule 291: active_Class("_cropped-image-link_style_fluidPortraitImage__3yQ-X")
// Rule 144: match_Class("_ameyal-product-shoveler_style_spCSRFTreatment__-hwVO")
// Rule 196: match_Class("_cropped-image-link_image_asin-container-white-box__3Stwp")
// Rule 594: match_Class("_fluid-quad-image-label-v2_image_asin-container__2jyCM")
// Rule 423: active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-1236x1080__3aEzl")
// Rule 754: match_Class("_fluid-quad-image-label-v2_style_themingTextColor__1oQsI")
// Rule 543: active_Class("_fluid-fat-image-link-v2_style_spCSRFTreatment__-hwVO")
// Rule 419: active_Class("_fluid-fat-image-link-v2_style_aspect-button-group__1LqUG")
// Rule 395: active_Class("_fluid-fat-image-link-v2_image_round-corners__2y_fS")
// Rule 275: active_Class("_cropped-image-link_style_displayCount__1MVut")
// Rule 808: match_Class("_quad-category-card_mobileStyle_categoryImage__3hSFw")
// Rule 866: match_Class("single-slide-hero")
// Rule 667: active_Class("_fluid-quad-image-label-v2_style_empty-footer__2d59h")
// Rule 341: active_Class("_cropped-image-link_style_spacer__7Pyg3")
// Rule 25: active_Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner__1nmZw")
// Rule 133: active_Class("_ameyal-product-shoveler_style_negativeMarginAdjust__1nqu9")
// Rule 370: match_Class("_cropped-image-link_style_wdHeader__Edrev")
// Rule 687: active_Class("_fluid-quad-image-label-v2_style_gridRowTwo__15woW")
// Rule 859: active_Class("gw-row")
// Rule 455: active_Class("_fluid-fat-image-link-v2_style_close-icon__2RJs3")
// Rule 793: active_Class("_quad-category-card_fluid_fluidCardBody__3TzJ4")
// Rule 48: match_Class("_ameyal-product-shoveler_style_aspect-ratio-16x9__cBPv8")
// Rule 709: active_Class("_fluid-quad-image-label-v2_style_logoGap__nKNZ9")
// Rule 849: active_Class("gw-auto-height")
// Rule 45: active_Class("_ameyal-product-shoveler_style_aspect-ratio-1236x1080__3aEzl")
// Rule 297: active_Class("_cropped-image-link_style_gwm-link-footer__3OF47")
// Rule 470: match_Class("_fluid-fat-image-link-v2_style_five-pack__1-Tql")
// Rule 695: active_Class("_fluid-quad-image-label-v2_style_header-icon__2cuVV")
// Rule 622: match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-15x3__1h649")
// Rule 302: match_Class("_cropped-image-link_style_header-link__cUhOK")
// Rule 34: match_Class("_ameyal-product-shoveler_style_ad-feedback-text__2HjQ9")
// Rule 16: match_Class("_ameyal-product-shoveler_image_asin-container-white-box__QwmgO")
// Rule 345: active_Class("_cropped-image-link_style_theming-background-override__1HfzJ")
// Rule 668: match_Class("_fluid-quad-image-label-v2_style_five-pack__1-Tql")
// Rule 814: match_Class("_quad-category-card_mobileStyle_leftMost__3WtU6")
// Rule 843: active_Class("a-carousel-viewport")
// Rule 169: active_Class("_ameyal-product-shoveler_style_truncation__x9-69")
// Rule 138: match_Class("_ameyal-product-shoveler_style_positive-button__3UOC3")
// Rule 186: match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
// Rule 474: match_Class("_fluid-fat-image-link-v2_style_fluidFatImageLinkBody__1LsOX")
// Rule 78: match_Class("_ameyal-product-shoveler_style_cover-portrait-image__2lhzL")
// Rule 269: active_Class("_cropped-image-link_style_cropped-image-link__3winf")
// Rule 44: match_Class("_ameyal-product-shoveler_style_aspect-ratio-1236x1080__3aEzl")
// Rule 429: active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-4x3__3BewI")
// Rule 478: match_Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2SOMr")
// Rule 146: match_Class("_ameyal-product-shoveler_style_spacer__7Pyg3")
// Rule 609: active_Class("_fluid-quad-image-label-v2_style_ad-feedback-text-desktop__q3xp_")
// Rule 116: match_Class("_ameyal-product-shoveler_style_logoSquareContainer__3Paoc")
// Rule 621: active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-1236x1080__3aEzl")
// Rule 65: active_Class("_ameyal-product-shoveler_style_badgeLabel__pJ5rc")
// Rule 631: active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
// Rule 484: match_Class("_fluid-fat-image-link-v2_style_fluidPortraitImage__2SAYm")
// Rule 661: active_Class("_fluid-quad-image-label-v2_style_desktop-close-button__1iL_P")
// Rule 680: match_Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabel__3b-Iv")
// Rule 756: match_Class("_fluid-quad-image-label-v2_style_three-pack__5s3hP")
// Rule 143: active_Class("_ameyal-product-shoveler_style_smartText__ubpEw")
// Rule 260: match_Class("_cropped-image-link_style_close-icon-wrapper__1zvdC")
// Rule 267: active_Class("_cropped-image-link_style_cover-portrait-image__2lhzL")
// Rule 55: active_Class("_ameyal-product-shoveler_style_aspect-ratio-dynamic-60vh__3N5g_")
// Rule 336: match_Class("_cropped-image-link_style_poster-image__1W0yA")
// Rule 358: match_Class("_cropped-image-link_style_tile-link__38lTa")
// Rule 263: active_Class("_cropped-image-link_style_close-icon__2RJs3")
// Rule 549: active_Class("_fluid-fat-image-link-v2_style_theming-background-override__1HfzJ")
// Rule 605: active_Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite-mobile__2_rj8")
// Rule 513: active_Class("_fluid-fat-image-link-v2_style_logoSquare__3NZyi")
// Rule 778: match_Class("_quad-category-card_desktopStyle_cardBody__3Rdh1")
// Rule 781: active_Class("_quad-category-card_desktopStyle_categoryImage__35jKN")
// Rule 826: match_Class("_quad-category-card_style_heading__1mnEu")
// Rule 74: match_Class("_ameyal-product-shoveler_style_close-icon__2RJs3")
// Rule 446: match_Class("_fluid-fat-image-link-v2_style_carouselContainer__3N7M1")
// Rule 447: active_Class("_fluid-fat-image-link-v2_style_carouselContainer__3N7M1")
// Rule 140: match_Class("_ameyal-product-shoveler_style_poster-image__1W0yA")
// Rule 537: active_Class("_fluid-fat-image-link-v2_style_positive-button__3UOC3")
// Rule 650: match_Class("_fluid-quad-image-label-v2_style_close-icon-wrapper__1zvdC")
// Rule 435: active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-fill__2Zjfb")
// Rule 340: match_Class("_cropped-image-link_style_spacer__7Pyg3")
// Rule 772: match_Class("_fluid-quad-image-label-v2_style_video-container__1hKS1")
// Rule 112: match_Class("_ameyal-product-shoveler_style_logoGap__nKNZ9")
// Rule 556: match_Class("_fluid-fat-image-link-v2_style_threeLineTruncation__UkUjj")
// Rule 59: active_Class("_ameyal-product-shoveler_style_aspect-text__S4PU1")
// Rule 531: active_Class("_fluid-fat-image-link-v2_style_negativeMarginAdjust__1nqu9")
// Rule 119: active_Class("_ameyal-product-shoveler_style_logoSquare__3NZyi")
// Rule 375: active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
// Rule 246: match_Class("_cropped-image-link_style_autoplay-span__2CMfc")
// Rule 414: match_Class("_fluid-fat-image-link-v2_style_apexBadgeLabel__2-Vye")
// Rule 482: match_Class("_fluid-fat-image-link-v2_style_fluidLandscapeImage__2euAK")
// Rule 163: active_Class("_ameyal-product-shoveler_style_tile-grid__QMxNY")
// Rule 320: match_Class("_cropped-image-link_style_mobile-close-button__3PB07")
// Rule 165: active_Class("_ameyal-product-shoveler_style_tile-link__38lTa")
// Rule 653: active_Class("_fluid-quad-image-label-v2_style_close-icon__2RJs3")
// Rule 691: active_Class("_fluid-quad-image-label-v2_style_gwm-link-footer__3OF47")
// Rule 698: match_Class("_fluid-quad-image-label-v2_style_header__1vGdj")
// Rule 532: match_Class("_fluid-fat-image-link-v2_style_oneLineTruncation__2WWse")
// Rule 580: match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
// Rule 619: active_Class("_fluid-quad-image-label-v2_style_aspect-button__7cH_E")
// Rule 573: active_Class("_fluid-fat-image-link-v2_style_wd-backdrop-data__1znxG")
// Rule 785: active_Class("_quad-category-card_desktopStyle_heroCategory__3KS3k")
// Rule 792: match_Class("_quad-category-card_fluid_fluidCardBody__3TzJ4")
// Rule 804: match_Class("_quad-category-card_mobileStyle_cardBody__3ODbW")
// Rule 833: active_Class("a-cardui-footer")
// Rule 271: active_Class("_cropped-image-link_style_cta-link__2xo74")
// Rule 318: match_Class("_cropped-image-link_style_mixed-button__2og-m")
// Rule 152: match_Class("_ameyal-product-shoveler_style_themingTextColorWhite__1zryO")
// Rule 509: active_Class("_fluid-fat-image-link-v2_style_logoRectangle__1VJwu")
// Rule 555: active_Class("_fluid-fat-image-link-v2_style_three-pack__5s3hP")
// Rule 377: active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
// Rule 570: match_Class("_fluid-fat-image-link-v2_style_video-container__1hKS1")
// Rule 589: active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
// Rule 649: active_Class("_fluid-quad-image-label-v2_style_close-black-icon__3hkbe")
// Rule 516: match_Class("_fluid-fat-image-link-v2_style_mergedLinksCta__3Npog")
// Rule 787: active_Class("_quad-category-card_desktopStyle_heroImage__2V8-9")
// Rule 816: match_Class("_quad-category-card_style_dashboard-card-with-border__1e4z_")
// Rule 871: active_Class("truncate-2line")
// Rule 130: match_Class("_ameyal-product-shoveler_style_negative-button__1Dvqz")
// Rule 300: match_Class("_cropped-image-link_style_header-icon__2cuVV")
// Rule 4: match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
// Rule 92: match_Class("_ameyal-product-shoveler_style_fluid-landscape-image__TE6PT")
// Rule 192: match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-container__1Pkva")
// Rule 323: active_Class("_cropped-image-link_style_mosaic-card-body__1HmTs")
// Rule 326: match_Class("_cropped-image-link_style_negative-button__1Dvqz")
// Rule 350: match_Class("_cropped-image-link_style_three-pack__5s3hP")
// Rule 504: match_Class("_fluid-fat-image-link-v2_style_inlineErrorDetails__1NBx-")
// Rule 693: active_Class("_fluid-quad-image-label-v2_style_haulRibbon__3VZNi")
// Rule 354: match_Class("_cropped-image-link_style_tile-container__1QgAV")
// Rule 390: match_Class("_fluid-fat-image-link-v2_image_asin-container-white-box__QwmgO")
// Rule 180: match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
// Rule 413: active_Class("_fluid-fat-image-link-v2_style_ad-feedback-text__2HjQ9")
// Rule 270: match_Class("_cropped-image-link_style_cta-link__2xo74")
// Rule 334: match_Class("_cropped-image-link_style_positive-button__3UOC3")
// Rule 376: match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
// Rule 313: active_Class("_cropped-image-link_style_logoSquareContainer__3Paoc")
// Rule 560: match_Class("_fluid-fat-image-link-v2_style_tile-grid__QMxNY")
// Rule 22: match_Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner-rtl__2BoOY")
// Rule 469: active_Class("_fluid-fat-image-link-v2_style_empty-footer__2d59h")
// Rule 344: match_Class("_cropped-image-link_style_theming-background-override__1HfzJ")
// Rule 31: active_Class("_ameyal-product-shoveler_style_ad-feedback-sprite__28uwB")
// Rule 629: active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-5x8__2IaNz")
// Rule 636: match_Class("_fluid-quad-image-label-v2_style_autoplay-span__2CMfc")
// Rule 365: active_Class("_cropped-image-link_style_twoLineTruncation__16TLV")
// Rule 651: active_Class("_fluid-quad-image-label-v2_style_close-icon-wrapper__1zvdC")
// Rule 663: active_Class("_fluid-quad-image-label-v2_style_displayCount__1MVut")
// Rule 678: match_Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabelBody__3tld0")
// Rule 261: active_Class("_cropped-image-link_style_close-icon-wrapper__1zvdC")
// Rule 708: match_Class("_fluid-quad-image-label-v2_style_logoGap__nKNZ9")
// Rule 195: active_Class("_cropped-image-link_image_asin-container-full-height__MOKlF")
// Rule 449: active_Class("_fluid-fat-image-link-v2_style_centerImage__30wh-")
// Rule 715: active_Class("_fluid-quad-image-label-v2_style_logoSquare__3NZyi")
// Rule 137: active_Class("_ameyal-product-shoveler_style_overlay__3Sx3u")
// Rule 164: match_Class("_ameyal-product-shoveler_style_tile-link__38lTa")
// Rule 309: active_Class("_cropped-image-link_style_logoGap__nKNZ9")
// Rule 248: match_Class("_cropped-image-link_style_badge-container__20aJ2")
// Rule 409: active_Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite__28uwB")
// Rule 294: match_Class("_cropped-image-link_style_gw-hero-close-button__3svyZ")
// Rule 364: match_Class("_cropped-image-link_style_twoLineTruncation__16TLV")
// Rule 438: match_Class("_fluid-fat-image-link-v2_style_autoplay-span__2CMfc")
// Rule 417: active_Class("_fluid-fat-image-link-v2_style_apexBadgeMessage__1tHvd")
// Rule 473: active_Class("_fluid-fat-image-link-v2_style_fluid-landscape-image__TE6PT")
// Rule 145: active_Class("_ameyal-product-shoveler_style_spCSRFTreatment__-hwVO")
// Rule 710: match_Class("_fluid-quad-image-label-v2_style_logoRectangle__1VJwu")
// Rule 367: active_Class("_cropped-image-link_style_video-container__1hKS1")
// Rule 167: active_Class("_ameyal-product-shoveler_style_tile-theming__3eeyj")
// Rule 712: match_Class("_fluid-quad-image-label-v2_style_logoSquareContainer__3Paoc")
// Rule 200: match_Class("_cropped-image-link_image_asin-container__2jyCM")
// Rule 722: match_Class("_fluid-quad-image-label-v2_style_mosaic-card-body__1HmTs")
// Rule 737: active_Class("_fluid-quad-image-label-v2_style_poster-image__1W0yA")
// Rule 805: active_Class("_quad-category-card_mobileStyle_cardBody__3ODbW")
// Rule 847: active_Class("card-flow-row-break")
// Rule 751: active_Class("_fluid-quad-image-label-v2_style_theming-background-override__1HfzJ")
// Rule 52: match_Class("_ameyal-product-shoveler_style_aspect-ratio-5x8__2IaNz")
// Rule 725: active_Class("_fluid-quad-image-label-v2_style_mosaic-card__1C-_R")
// Rule 448: match_Class("_fluid-fat-image-link-v2_style_centerImage__30wh-")
// Rule 54: match_Class("_ameyal-product-shoveler_style_aspect-ratio-dynamic-60vh__3N5g_")
// Rule 51: active_Class("_ameyal-product-shoveler_style_aspect-ratio-4x3__3BewI")
// Rule 372: match_Class("_fluid-fat-image-link-v2_bodyFooterStyle_cardBody__1YuQY")
// Rule 388: match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-container__1Pkva")
// Rule 426: match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-16x9__cBPv8")
// Rule 412: match_Class("_fluid-fat-image-link-v2_style_ad-feedback-text__2HjQ9")
// Rule 343: active_Class("_cropped-image-link_style_stacking-context__3PbQE")
// Rule 33: active_Class("_ameyal-product-shoveler_style_ad-feedback-text-desktop__q3xp_")
// Rule 533: active_Class("_fluid-fat-image-link-v2_style_oneLineTruncation__2WWse")
// Rule 592: match_Class("_fluid-quad-image-label-v2_image_asin-container-white-box__QwmgO")
// Rule 601: active_Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner__1nmZw")
// Rule 128: match_Class("_ameyal-product-shoveler_style_mosaic-card__1C-_R")
// Rule 35: active_Class("_ameyal-product-shoveler_style_ad-feedback-text__2HjQ9")
// Rule 178: match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
// Rule 584: match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
// Rule 610: match_Class("_fluid-quad-image-label-v2_style_ad-feedback-text__2HjQ9")
// Rule 84: match_Class("_ameyal-product-shoveler_style_displayCount__1MVut")
// Rule 615: active_Class("_fluid-quad-image-label-v2_style_apexBadgeMessage__1tHvd")
// Rule 700: match_Class("_fluid-quad-image-label-v2_style_image-container__2OiZA")
// Rule 728: match_Class("_fluid-quad-image-label-v2_style_negativeMarginAdjust__1nqu9")
// Rule 769: active_Class("_fluid-quad-image-label-v2_style_truncation__x9-69")
// Rule 777: active_Class("_fluid-quad-image-label-v2_style_wdHeader__Edrev")
// Rule 855: active_Class("gw-fixed-col")
// Rule 361: active_Class("_cropped-image-link_style_tile-theming__3eeyj")
// Rule 461: active_Class("_fluid-fat-image-link-v2_style_cta-link__2xo74")
// Rule 483: active_Class("_fluid-fat-image-link-v2_style_fluidLandscapeImage__2euAK")
// Rule 308: match_Class("_cropped-image-link_style_logoGap__nKNZ9")
// Rule 800: match_Class("_quad-category-card_image_asin-container__LRY5p")
// Rule 831: active_Class("a-cardui-body")
// Rule 187: active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
// Rule 381: active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
// Rule 654: match_Class("_fluid-quad-image-label-v2_style_close-text__2-gwn")
// Rule 688: match_Class("_fluid-quad-image-label-v2_style_gw-hero-close-button__3svyZ")
// Rule 481: active_Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2vGwp")
// Rule 813: active_Class("_quad-category-card_mobileStyle_heroImage__1SewP")
// Rule 861: active_Class("nav-focus")
// Rule 862: match_Class("nav-spinner")
// Rule 46: match_Class("_ameyal-product-shoveler_style_aspect-ratio-15x3__1h649")
// Rule 498: match_Class("_fluid-fat-image-link-v2_style_header__1vGdj")
// Rule 535: active_Class("_fluid-fat-image-link-v2_style_overlay__3Sx3u")
// Rule 784: match_Class("_quad-category-card_desktopStyle_heroCategory__3KS3k")
// Rule 9: active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
// Rule 102: match_Class("_ameyal-product-shoveler_style_header-icon__2cuVV")
// Rule 254: match_Class("_cropped-image-link_style_carouselContainer__3N7M1")
// Rule 106: match_Class("_ameyal-product-shoveler_style_header__1vGdj")
// Rule 466: match_Class("_fluid-fat-image-link-v2_style_dynamic-portrait-image__1Wrzd")
// Rule 757: active_Class("_fluid-quad-image-label-v2_style_three-pack__5s3hP")
// Rule 794: match_Class("_quad-category-card_fluid_fluidCard__3hmFA")
// Rule 576: match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
// Rule 100: match_Class("_ameyal-product-shoveler_style_haulRibbon__3VZNi")
// Rule 554: match_Class("_fluid-fat-image-link-v2_style_three-pack__5s3hP")
// Rule 870: match_Class("truncate-2line")
// Rule 28: match_Class("_ameyal-product-shoveler_style_ad-feedback-sprite-mobile__2_rj8")
// Rule 244: match_Class("_cropped-image-link_style_aspect-text__S4PU1")
// Rule 465: active_Class("_fluid-fat-image-link-v2_style_displayCount__1MVut")
// Rule 487: active_Class("_fluid-fat-image-link-v2_style_four-pack__1ufgr")
// Rule 223: active_Class("_cropped-image-link_style_apexBadgeLabel__2-Vye")
// Rule 37: active_Class("_ameyal-product-shoveler_style_apexBadgeLabel__2-Vye")
// Rule 593: active_Class("_fluid-quad-image-label-v2_image_asin-container-white-box__QwmgO")
// Rule 882: match_Type("span")
// Rule 226: match_Class("_cropped-image-link_style_aspect-button-group__1LqUG")
// Rule 523: active_Class("_fluid-fat-image-link-v2_style_mobile-close-button__3PB07")
// Rule 114: match_Class("_ameyal-product-shoveler_style_logoRectangle__1VJwu")
// Rule 166: match_Class("_ameyal-product-shoveler_style_tile-theming__3eeyj")
// Rule 415: active_Class("_fluid-fat-image-link-v2_style_apexBadgeLabel__2-Vye")
// Rule 562: match_Class("_fluid-fat-image-link-v2_style_tile-link__38lTa")
// Rule 574: match_Class("_fluid-fat-image-link-v2_style_wdHeader__Edrev")
// Rule 38: match_Class("_ameyal-product-shoveler_style_apexBadgeMessage__1tHvd")
// Rule 80: match_Class("_ameyal-product-shoveler_style_cta-link__2xo74")
// Rule 103: active_Class("_ameyal-product-shoveler_style_header-icon__2cuVV")
// Rule 108: match_Class("_ameyal-product-shoveler_style_image-container__2OiZA")
// Rule 202: match_Class("_cropped-image-link_image_asin-container__LRY5p")
// Rule 697: active_Class("_fluid-quad-image-label-v2_style_header-link__cUhOK")
// Rule 721: active_Class("_fluid-quad-image-label-v2_style_mobile-close-button__3PB07")
// Rule 780: match_Class("_quad-category-card_desktopStyle_categoryImage__35jKN")
// Rule 672: match_Class("_fluid-quad-image-label-v2_style_fluidImageContainer__2SOMr")
// Rule 529: active_Class("_fluid-fat-image-link-v2_style_negative-button__1Dvqz")
// Rule 740: match_Class("_fluid-quad-image-label-v2_style_rightQuadrant__PI01n")
// Rule 457: active_Class("_fluid-fat-image-link-v2_style_close-text__2-gwn")
// Rule 803: active_Class("_quad-category-card_image_round-corners__22iOW")
// Rule 832: match_Class("a-cardui-footer")
// Rule 230: match_Class("_cropped-image-link_style_aspect-ratio-1236x1080__3aEzl")
// Rule 93: active_Class("_ameyal-product-shoveler_style_fluid-landscape-image__TE6PT")
// Rule 77: active_Class("_ameyal-product-shoveler_style_close-text__2-gwn")
// Rule 68: match_Class("_ameyal-product-shoveler_style_carouselContainer__3N7M1")
// Rule 161: active_Class("_ameyal-product-shoveler_style_tile-container__1QgAV")
// Rule 319: active_Class("_cropped-image-link_style_mixed-button__2og-m")
// Rule 382: match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
// Rule 511: active_Class("_fluid-fat-image-link-v2_style_logoSquareContainer__3Paoc")
// Rule 564: match_Class("_fluid-fat-image-link-v2_style_tile-theming__3eeyj")
// Rule 822: match_Class("_quad-category-card_style_fluidPortraitImage__3yQ-X")
// Rule 183: active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
// Rule 553: active_Class("_fluid-fat-image-link-v2_style_themingTextColor__1oQsI")
// Rule 665: active_Class("_fluid-quad-image-label-v2_style_dynamic-portrait-image__1Wrzd")
// Rule 869: active_Class("truncate-1line")
// Rule 767: active_Class("_fluid-quad-image-label-v2_style_tile-theming__3eeyj")
// Rule 874: match_Id("icp-touch-link-cop")
// Rule 530: match_Class("_fluid-fat-image-link-v2_style_negativeMarginAdjust__1nqu9")
// Rule 526: match_Class("_fluid-fat-image-link-v2_style_mosaic-card__1C-_R")
// Rule 503: active_Class("_fluid-fat-image-link-v2_style_imageLabel__3ANSV")
// Rule 135: active_Class("_ameyal-product-shoveler_style_oneLineTruncation__2WWse")
// Rule 134: match_Class("_ameyal-product-shoveler_style_oneLineTruncation__2WWse")
// Rule 327: active_Class("_cropped-image-link_style_negative-button__1Dvqz")
// Rule 273: active_Class("_cropped-image-link_style_desktop-close-button__1iL_P")
// Rule 379: active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
// Rule 392: match_Class("_fluid-fat-image-link-v2_image_asin-container__2jyCM")
// Rule 628: match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-5x8__2IaNz")
// Rule 836: match_Class("a-carousel-container")
// Rule 497: active_Class("_fluid-fat-image-link-v2_style_header-link__cUhOK")
// Rule 851: active_Class("gw-card-layout")
// Rule 799: active_Class("_quad-category-card_image_asin-container-white-box__3Stwp")
// Rule 878: match_Id("icp-touch-link-language")
// Rule 193: active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-container__1Pkva")
// Rule 371: active_Class("_cropped-image-link_style_wdHeader__Edrev")
// Rule 264: match_Class("_cropped-image-link_style_close-text__2-gwn")
// Rule 240: match_Class("_cropped-image-link_style_aspect-ratio-dynamic-60vh__3N5g_")
// Rule 298: match_Class("_cropped-image-link_style_haulRibbon__3VZNi")
// Rule 49: active_Class("_ameyal-product-shoveler_style_aspect-ratio-16x9__cBPv8")
// Rule 389: active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-container__1Pkva")
// Rule 456: match_Class("_fluid-fat-image-link-v2_style_close-text__2-gwn")
// Rule 13: active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-standard__28gp8")
// Rule 5: active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
// Rule 177: active_Class("_ameyal-product-shoveler_style_wdHeader__Edrev")
// Rule 182: match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
// Rule 212: match_Class("_cropped-image-link_style_ad-feedback-primary-link__2bIZi")
// Rule 540: match_Class("_fluid-fat-image-link-v2_style_smartText__ubpEw")
// Rule 558: match_Class("_fluid-fat-image-link-v2_style_tile-container__1QgAV")
// Rule 582: match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
// Rule 679: active_Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabelBody__3tld0")
// Rule 232: match_Class("_cropped-image-link_style_aspect-ratio-15x3__1h649")
// Rule 79: active_Class("_ameyal-product-shoveler_style_cover-portrait-image__2lhzL")
// Rule 234: match_Class("_cropped-image-link_style_aspect-ratio-16x9__cBPv8")
// Rule 208: match_Class("_cropped-image-link_style_ad-feedback-loading-spinnner-rtl__2BoOY")
// Rule 599: active_Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
// Rule 671: active_Class("_fluid-quad-image-label-v2_style_fluid-landscape-image__TE6PT")
// Rule 575: active_Class("_fluid-fat-image-link-v2_style_wdHeader__Edrev")
// Rule 258: match_Class("_cropped-image-link_style_close-black-icon__3hkbe")
// Rule 42: match_Class("_ameyal-product-shoveler_style_aspect-button__7cH_E")
// Rule 675: active_Class("_fluid-quad-image-label-v2_style_fluidLandscapeImage__2euAK")
// Rule 81: active_Class("_ameyal-product-shoveler_style_cta-link__2xo74")
// Rule 521: active_Class("_fluid-fat-image-link-v2_style_mixed-button__2og-m")
// Rule 89: active_Class("_ameyal-product-shoveler_style_empty-footer__2d59h")
// Rule 386: match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
// Rule 86: match_Class("_ameyal-product-shoveler_style_dynamic-portrait-image__1Wrzd")
// Rule 306: match_Class("_cropped-image-link_style_image-container__2OiZA")
// Rule 669: active_Class("_fluid-quad-image-label-v2_style_five-pack__1-Tql")
// Rule 686: match_Class("_fluid-quad-image-label-v2_style_gridRowTwo__15woW")
// Rule 705: active_Class("_fluid-quad-image-label-v2_style_inlineErrorDetails__1NBx-")
// Rule 706: match_Class("_fluid-quad-image-label-v2_style_leftQuadrant__21nVp")
// Rule 711: active_Class("_fluid-quad-image-label-v2_style_logoRectangle__1VJwu")
// Rule 733: active_Class("_fluid-quad-image-label-v2_style_overlay__3Sx3u")
// Rule 791: active_Class("_quad-category-card_desktopStyle_leftMost__1LmQB")
// Rule 825: active_Class("_quad-category-card_style_gwm-link-footer__3EX7d")
// Rule 810: match_Class("_quad-category-card_mobileStyle_category__1amt4")
// Rule 760: match_Class("_fluid-quad-image-label-v2_style_tile-container__1QgAV")
// Rule 552: match_Class("_fluid-fat-image-link-v2_style_themingTextColor__1oQsI")
// Rule 572: match_Class("_fluid-fat-image-link-v2_style_wd-backdrop-data__1znxG")
// Rule 637: active_Class("_fluid-quad-image-label-v2_style_autoplay-span__2CMfc")
// Rule 736: match_Class("_fluid-quad-image-label-v2_style_poster-image__1W0yA")
// Rule 243: active_Class("_cropped-image-link_style_aspect-ratio-fill__2Zjfb")
// Rule 250: match_Class("_cropped-image-link_style_badgeLabel__pJ5rc")
// Rule 96: match_Class("_ameyal-product-shoveler_style_gw-hero-close-button__3svyZ")
// Rule 47: active_Class("_ameyal-product-shoveler_style_aspect-ratio-15x3__1h649")
// Rule 579: active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
// Rule 98: match_Class("_ameyal-product-shoveler_style_gwm-link-footer__3OF47")
// Rule 132: match_Class("_ameyal-product-shoveler_style_negativeMarginAdjust__1nqu9")
// Rule 396: match_Class("_fluid-fat-image-link-v2_singleLinkStyle_bodyFooterLink__9LvH0")
// Rule 242: match_Class("_cropped-image-link_style_aspect-ratio-fill__2Zjfb")
// Rule 421: active_Class("_fluid-fat-image-link-v2_style_aspect-button__7cH_E")
// Rule 534: match_Class("_fluid-fat-image-link-v2_style_overlay__3Sx3u")
// Rule 770: match_Class("_fluid-quad-image-label-v2_style_twoLineTruncation__16TLV")
// Rule 817: active_Class("_quad-category-card_style_dashboard-card-with-border__1e4z_")
// Rule 41: active_Class("_ameyal-product-shoveler_style_aspect-button-group__1LqUG")
// Rule 499: active_Class("_fluid-fat-image-link-v2_style_header__1vGdj")
// Rule 713: active_Class("_fluid-quad-image-label-v2_style_logoSquareContainer__3Paoc")
// Rule 301: active_Class("_cropped-image-link_style_header-icon__2cuVV")
// Rule 818: match_Class("_quad-category-card_style_fluidImageContainer__2jd50")
// Rule 837: active_Class("a-carousel-container")
// Rule 841: active_Class("a-carousel-right")
// Rule 331: active_Class("_cropped-image-link_style_oneLineTruncation__2WWse")
// Rule 845: active_Class("a-link-normal")
// Rule 848: match_Class("gw-auto-height")
// Rule 608: match_Class("_fluid-quad-image-label-v2_style_ad-feedback-text-desktop__q3xp_")
// Rule 332: match_Class("_cropped-image-link_style_overlay__3Sx3u")
// Rule 858: match_Class("gw-row")
// Rule 283: active_Class("_cropped-image-link_style_five-pack__1-Tql")
// Rule 776: match_Class("_fluid-quad-image-label-v2_style_wdHeader__Edrev")
// Rule 846: match_Class("card-flow-row-break")
// Rule 853: active_Class("gw-col")
// Rule 860: match_Class("nav-focus")
// Rule 172: match_Class("_ameyal-product-shoveler_style_video-container__1hKS1")
// Rule 121: active_Class("_ameyal-product-shoveler_style_logo__2ZQ-N")
// Rule 125: active_Class("_ameyal-product-shoveler_style_mobile-close-button__3PB07")
// Rule 399: active_Class("_fluid-fat-image-link-v2_singleLinkStyle_footer__2cH0y")
// Rule 218: match_Class("_cropped-image-link_style_ad-feedback-text-desktop__q3xp_")
// Rule 410: match_Class("_fluid-fat-image-link-v2_style_ad-feedback-text-desktop__q3xp_")
// Rule 115: active_Class("_ameyal-product-shoveler_style_logoRectangle__1VJwu")
// Rule 542: match_Class("_fluid-fat-image-link-v2_style_spCSRFTreatment__-hwVO")
// Rule 544: match_Class("_fluid-fat-image-link-v2_style_spacer__7Pyg3")
// Rule 289: active_Class("_cropped-image-link_style_fluidLandscapeImage__3eTVC")
// Rule 546: match_Class("_fluid-fat-image-link-v2_style_stacking-context__3PbQE")
// Rule 590: match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-container__1Pkva")
// Rule 753: active_Class("_fluid-quad-image-label-v2_style_themingTextColorWhite__1zryO")
// Rule 36: match_Class("_ameyal-product-shoveler_style_apexBadgeLabel__2-Vye")
// Rule 64: match_Class("_ameyal-product-shoveler_style_badgeLabel__pJ5rc")
// Rule 351: active_Class("_cropped-image-link_style_three-pack__5s3hP")
// Rule 236: match_Class("_cropped-image-link_style_aspect-ratio-4x3__3BewI")
// Rule 431: active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-5x8__2IaNz")
// Rule 7: active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
// Rule 427: active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-16x9__cBPv8")
// Rule 525: active_Class("_fluid-fat-image-link-v2_style_mosaic-card-body__1HmTs")
// Rule 611: active_Class("_fluid-quad-image-label-v2_style_ad-feedback-text__2HjQ9")
// Rule 681: active_Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabel__3b-Iv")
// Rule 720: match_Class("_fluid-quad-image-label-v2_style_mobile-close-button__3PB07")
// Rule 127: active_Class("_ameyal-product-shoveler_style_mosaic-card-body__1HmTs")
// Rule 444: match_Class("_fluid-fat-image-link-v2_style_badgeMessage__2Dtw7")
// Rule 739: active_Class("_fluid-quad-image-label-v2_style_quadrantContainer__3TMqG")
// Rule 156: match_Class("_ameyal-product-shoveler_style_three-pack__5s3hP")
// Rule 819: active_Class("_quad-category-card_style_fluidImageContainer__2jd50")
// Rule 241: active_Class("_cropped-image-link_style_aspect-ratio-dynamic-60vh__3N5g_")
// Rule 824: match_Class("_quad-category-card_style_gwm-link-footer__3EX7d")
// Rule 159: active_Class("_ameyal-product-shoveler_style_threeLineTruncation__UkUjj")
// Rule 235: active_Class("_cropped-image-link_style_aspect-ratio-16x9__cBPv8")
// Rule 278: match_Class("_cropped-image-link_style_dynamic-portrait-image__1Wrzd")
// Rule 303: active_Class("_cropped-image-link_style_header-link__cUhOK")
// Rule 82: match_Class("_ameyal-product-shoveler_style_desktop-close-button__1iL_P")
// Rule 617: active_Class("_fluid-quad-image-label-v2_style_aspect-button-group__1LqUG")
// Rule 123: active_Class("_ameyal-product-shoveler_style_mixed-button__2og-m")
// Rule 311: active_Class("_cropped-image-link_style_logoRectangle__1VJwu")
// Rule 827: active_Class("_quad-category-card_style_heading__1mnEu")
// Rule 856: match_Class("gw-media-card")
// Rule 872: match_Class("vjs-fluid")
// Rule 10: match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
// Rule 428: match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-4x3__3BewI")
// Rule 228: match_Class("_cropped-image-link_style_aspect-button__7cH_E")
// Rule 539: active_Class("_fluid-fat-image-link-v2_style_poster-image__1W0yA")
// Rule 480: match_Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2vGwp")
// Rule 90: match_Class("_ameyal-product-shoveler_style_five-pack__1-Tql")
// Rule 120: match_Class("_ameyal-product-shoveler_style_logo__2ZQ-N")
// Rule 346: match_Class("_cropped-image-link_style_themingTextColorWhite__1zryO")
// Rule 614: match_Class("_fluid-quad-image-label-v2_style_apexBadgeMessage__1tHvd")
// Rule 239: active_Class("_cropped-image-link_style_aspect-ratio-5x8__2IaNz")
// Rule 731: active_Class("_fluid-quad-image-label-v2_style_oneLineTruncation__2WWse")
// Rule 744: match_Class("_fluid-quad-image-label-v2_style_spCSRFTreatment__-hwVO")
// Rule 867: active_Class("single-slide-hero")
// Rule 8: match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
// Rule 150: match_Class("_ameyal-product-shoveler_style_theming-background-override__1HfzJ")
// Rule 441: active_Class("_fluid-fat-image-link-v2_style_badge-container__20aJ2")
// Rule 506: match_Class("_fluid-fat-image-link-v2_style_logoGap__nKNZ9")
// Rule 476: match_Class("_fluid-fat-image-link-v2_style_fluidFatImageLink__1nw4J")
// Rule 518: match_Class("_fluid-fat-image-link-v2_style_mergedLinks__10JqZ")
// Rule 330: match_Class("_cropped-image-link_style_oneLineTruncation__2WWse")
// Rule 439: active_Class("_fluid-fat-image-link-v2_style_autoplay-span__2CMfc")
// Rule 139: active_Class("_ameyal-product-shoveler_style_positive-button__3UOC3")
// Rule 422: match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-1236x1080__3aEzl")
// Rule 356: match_Class("_cropped-image-link_style_tile-grid__QMxNY")
// Rule 528: match_Class("_fluid-fat-image-link-v2_style_negative-button__1Dvqz")
// Rule 676: match_Class("_fluid-quad-image-label-v2_style_fluidPortraitImage__2SAYm")
// Rule 205: active_Class("_cropped-image-link_image_round-corners__22iOW")
// Rule 620: match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-1236x1080__3aEzl")
// Rule 551: active_Class("_fluid-fat-image-link-v2_style_themingTextColorWhite__1zryO")
// Rule 475: active_Class("_fluid-fat-image-link-v2_style_fluidFatImageLinkBody__1LsOX")
// Rule 355: active_Class("_cropped-image-link_style_tile-container__1QgAV")
// Rule 684: match_Class("_fluid-quad-image-label-v2_style_gridRowOne__1t0zL")
// Rule 714: match_Class("_fluid-quad-image-label-v2_style_logoSquare__3NZyi")
// Rule 2: match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
// Rule 699: active_Class("_fluid-quad-image-label-v2_style_header__1vGdj")
// Rule 727: active_Class("_fluid-quad-image-label-v2_style_negative-button__1Dvqz")
// Rule 639: active_Class("_fluid-quad-image-label-v2_style_badge-container__20aJ2")
// Rule 734: match_Class("_fluid-quad-image-label-v2_style_positive-button__3UOC3")
// Rule 299: active_Class("_cropped-image-link_style_haulRibbon__3VZNi")
// Rule 266: match_Class("_cropped-image-link_style_cover-portrait-image__2lhzL")
// Rule 583: active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
// Rule 613: active_Class("_fluid-quad-image-label-v2_style_apexBadgeLabel__2-Vye")
// Rule 703: active_Class("_fluid-quad-image-label-v2_style_imageLabel__3ANSV")
// Rule 732: match_Class("_fluid-quad-image-label-v2_style_overlay__3Sx3u")
// Rule 612: match_Class("_fluid-quad-image-label-v2_style_apexBadgeLabel__2-Vye")
// Rule 94: match_Class("_ameyal-product-shoveler_style_four-pack__1ufgr")
// Rule 405: active_Class("_fluid-fat-image-link-v2_style_ad-feedback-primary-link__2bIZi")
// Rule 440: match_Class("_fluid-fat-image-link-v2_style_badge-container__20aJ2")
// Rule 443: active_Class("_fluid-fat-image-link-v2_style_badgeLabel__pJ5rc")
// Rule 548: match_Class("_fluid-fat-image-link-v2_style_theming-background-override__1HfzJ")
// Rule 641: active_Class("_fluid-quad-image-label-v2_style_badgeLabel__pJ5rc")
// Rule 425: active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-15x3__1h649")
// Rule 276: match_Class("_cropped-image-link_style_dt-TextContainer__3nbU9")
// Rule 272: match_Class("_cropped-image-link_style_desktop-close-button__1iL_P")
// Rule 550: match_Class("_fluid-fat-image-link-v2_style_themingTextColorWhite__1zryO")
// Rule 418: match_Class("_fluid-fat-image-link-v2_style_aspect-button-group__1LqUG")
// Rule 646: match_Class("_fluid-quad-image-label-v2_style_centerImage__30wh-")
// Rule 701: active_Class("_fluid-quad-image-label-v2_style_image-container__2OiZA")
// Rule 32: match_Class("_ameyal-product-shoveler_style_ad-feedback-text-desktop__q3xp_")
// Rule 618: match_Class("_fluid-quad-image-label-v2_style_aspect-button__7cH_E")
// Rule 746: match_Class("_fluid-quad-image-label-v2_style_spacer__7Pyg3")
// Rule 723: active_Class("_fluid-quad-image-label-v2_style_mosaic-card-body__1HmTs")
// Rule 747: active_Class("_fluid-quad-image-label-v2_style_spacer__7Pyg3")
// Rule 129: active_Class("_ameyal-product-shoveler_style_mosaic-card__1C-_R")
// Rule 384: match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
// Rule 73: active_Class("_ameyal-product-shoveler_style_close-icon-wrapper__1zvdC")
// Rule 597: active_Class("_fluid-quad-image-label-v2_image_round-corners__2y_fS")
// Rule 616: match_Class("_fluid-quad-image-label-v2_style_aspect-button-group__1LqUG")
// Rule 324: match_Class("_cropped-image-link_style_mosaic-card__1C-_R")
// Rule 492: match_Class("_fluid-fat-image-link-v2_style_haulRibbon__3VZNi")
// Rule 763: active_Class("_fluid-quad-image-label-v2_style_tile-grid__QMxNY")
// Rule 768: match_Class("_fluid-quad-image-label-v2_style_truncation__x9-69")
// Rule 783: active_Class("_quad-category-card_desktopStyle_category__3flCQ")
// Rule 801: active_Class("_quad-category-card_image_asin-container__LRY5p")
// Rule 828: match_Class("_text-link-stripe-v2_style_textlinkstripe__3aQhz")
// Rule 674: match_Class("_fluid-quad-image-label-v2_style_fluidLandscapeImage__2euAK")
// Rule 830: match_Class("a-cardui-body")
// Rule 835: active_Class("a-cardui-header")
// Rule 850: match_Class("gw-card-layout")
// Rule 366: match_Class("_cropped-image-link_style_video-container__1hKS1")
// Rule 387: active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
// Rule 374: match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
// Rule 295: active_Class("_cropped-image-link_style_gw-hero-close-button__3svyZ")
// Rule 472: match_Class("_fluid-fat-image-link-v2_style_fluid-landscape-image__TE6PT")
// Rule 411: active_Class("_fluid-fat-image-link-v2_style_ad-feedback-text-desktop__q3xp_")
// Rule 53: active_Class("_ameyal-product-shoveler_style_aspect-ratio-5x8__2IaNz")
// Rule 220: match_Class("_cropped-image-link_style_ad-feedback-text__2HjQ9")
// Rule 477: active_Class("_fluid-fat-image-link-v2_style_fluidFatImageLink__1nw4J")
// Rule 211: active_Class("_cropped-image-link_style_ad-feedback-loading-spinnner__1nmZw")
// Rule 495: active_Class("_fluid-fat-image-link-v2_style_header-icon__2cuVV")
// Rule 857: active_Class("gw-media-card")
// Rule 30: match_Class("_ameyal-product-shoveler_style_ad-feedback-sprite__28uwB")
// Rule 126: match_Class("_ameyal-product-shoveler_style_mosaic-card-body__1HmTs")
// Rule 19: active_Class("_ameyal-product-shoveler_image_asin-container__2jyCM")
// Rule 452: match_Class("_fluid-fat-image-link-v2_style_close-icon-wrapper__1zvdC")
// Rule 569: active_Class("_fluid-fat-image-link-v2_style_twoLineTruncation__16TLV")
// Rule 716: match_Class("_fluid-quad-image-label-v2_style_logo__2ZQ-N")
// Rule 338: match_Class("_cropped-image-link_style_smartText__ubpEw")
// Rule 280: match_Class("_cropped-image-link_style_empty-footer__2d59h")
// Rule 197: active_Class("_cropped-image-link_image_asin-container-white-box__3Stwp")
// Rule 352: match_Class("_cropped-image-link_style_threeLineTruncation__UkUjj")
// Rule 321: active_Class("_cropped-image-link_style_mobile-close-button__3PB07")
// Rule 357: active_Class("_cropped-image-link_style_tile-grid__QMxNY")
// Rule 434: match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-fill__2Zjfb")
// Rule 424: match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-15x3__1h649")
// Rule 505: active_Class("_fluid-fat-image-link-v2_style_inlineErrorDetails__1NBx-")
// Rule 524: match_Class("_fluid-fat-image-link-v2_style_mosaic-card-body__1HmTs")
// Rule 664: match_Class("_fluid-quad-image-label-v2_style_dynamic-portrait-image__1Wrzd")
// Rule 717: active_Class("_fluid-quad-image-label-v2_style_logo__2ZQ-N")
// Rule 738: match_Class("_fluid-quad-image-label-v2_style_quadrantContainer__3TMqG")
// Rule 335: active_Class("_cropped-image-link_style_positive-button__3UOC3")
// Rule 14: match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-container__1Pkva")
// Rule 40: match_Class("_ameyal-product-shoveler_style_aspect-button-group__1LqUG")
// Rule 97: active_Class("_ameyal-product-shoveler_style_gw-hero-close-button__3svyZ")
// Rule 707: active_Class("_fluid-quad-image-label-v2_style_leftQuadrant__21nVp")
// Rule 27: active_Class("_ameyal-product-shoveler_style_ad-feedback-primary-link__2bIZi")
// Rule 764: match_Class("_fluid-quad-image-label-v2_style_tile-link__38lTa")
// Rule 795: active_Class("_quad-category-card_fluid_fluidCard__3hmFA")
// Rule 648: match_Class("_fluid-quad-image-label-v2_style_close-black-icon__3hkbe")
// Rule 802: match_Class("_quad-category-card_image_round-corners__22iOW")
// Rule 811: active_Class("_quad-category-card_mobileStyle_category__1amt4")
// Rule 842: match_Class("a-carousel-viewport")
// Rule 304: match_Class("_cropped-image-link_style_header__1vGdj")
// Rule 322: match_Class("_cropped-image-link_style_mosaic-card-body__1HmTs")
// Rule 471: active_Class("_fluid-fat-image-link-v2_style_five-pack__1-Tql")
// Rule 522: match_Class("_fluid-fat-image-link-v2_style_mobile-close-button__3PB07")
// Rule 662: match_Class("_fluid-quad-image-label-v2_style_displayCount__1MVut")
// Rule 380: match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
// Rule 467: active_Class("_fluid-fat-image-link-v2_style_dynamic-portrait-image__1Wrzd")
// Rule 416: match_Class("_fluid-fat-image-link-v2_style_apexBadgeMessage__1tHvd")
// Rule 168: match_Class("_ameyal-product-shoveler_style_truncation__x9-69")
// Rule 194: match_Class("_cropped-image-link_image_asin-container-full-height__MOKlF")
// Rule 287: active_Class("_cropped-image-link_style_fluidImageContainer__2jd50")
// Rule 314: match_Class("_cropped-image-link_style_logoSquare__3NZyi")
// Rule 391: active_Class("_fluid-fat-image-link-v2_image_asin-container-white-box__QwmgO")
// Rule 210: match_Class("_cropped-image-link_style_ad-feedback-loading-spinnner__1nmZw")
// Rule 519: active_Class("_fluid-fat-image-link-v2_style_mergedLinks__10JqZ")
// Rule 624: match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-16x9__cBPv8")
// Rule 761: active_Class("_fluid-quad-image-label-v2_style_tile-container__1QgAV")
// Rule 876: match_Id("icp-touch-link-country")
// Rule 463: active_Class("_fluid-fat-image-link-v2_style_desktop-close-button__1iL_P")
// Rule 644: match_Class("_fluid-quad-image-label-v2_style_carouselContainer__3N7M1")
// Rule 685: active_Class("_fluid-quad-image-label-v2_style_gridRowOne__1t0zL")
// Rule 868: match_Class("truncate-1line")
// Rule 821: active_Class("_quad-category-card_style_fluidLandscapeImage__3eTVC")
// Rule 536: match_Class("_fluid-fat-image-link-v2_style_positive-button__3UOC3")
// Rule 607: active_Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite__28uwB")
// Rule 227: active_Class("_cropped-image-link_style_aspect-button-group__1LqUG")
// Rule 190: match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-standard__28gp8")
// Rule 286: match_Class("_cropped-image-link_style_fluidImageContainer__2jd50")
// Rule 493: active_Class("_fluid-fat-image-link-v2_style_haulRibbon__3VZNi")
// Rule 420: match_Class("_fluid-fat-image-link-v2_style_aspect-button__7cH_E")
// Rule 494: match_Class("_fluid-fat-image-link-v2_style_header-icon__2cuVV")
// Rule 798: match_Class("_quad-category-card_image_asin-container-white-box__3Stwp")
// Rule 852: match_Class("gw-col")
// Rule 20: match_Class("_ameyal-product-shoveler_image_round-corners__2y_fS")
// Rule 206: match_Class("_cropped-image-link_image_round-corners__2y_fS")
// Rule 238: match_Class("_cropped-image-link_style_aspect-ratio-5x8__2IaNz")
// Rule 362: match_Class("_cropped-image-link_style_truncation__x9-69")
// Rule 216: match_Class("_cropped-image-link_style_ad-feedback-sprite__28uwB")
// Rule 72: match_Class("_ameyal-product-shoveler_style_close-icon-wrapper__1zvdC")
// Rule 458: match_Class("_fluid-fat-image-link-v2_style_cover-portrait-image__2lhzL")
// Rule 284: match_Class("_cropped-image-link_style_fluid-landscape-image__TE6PT")
// Rule 436: match_Class("_fluid-fat-image-link-v2_style_aspect-text__S4PU1")
// Rule 462: match_Class("_fluid-fat-image-link-v2_style_desktop-close-button__1iL_P")
// Rule 863: active_Class("nav-spinner")
// Rule 782: match_Class("_quad-category-card_desktopStyle_category__3flCQ")
// Rule 571: active_Class("_fluid-fat-image-link-v2_style_video-container__1hKS1")
// Rule 95: active_Class("_ameyal-product-shoveler_style_four-pack__1ufgr")
// Rule 117: active_Class("_ameyal-product-shoveler_style_logoSquareContainer__3Paoc")
// Rule 642: match_Class("_fluid-quad-image-label-v2_style_badgeMessage__2Dtw7")
// Rule 806: match_Class("_quad-category-card_mobileStyle_categoryContainer__2xY0I")
// Rule 758: match_Class("_fluid-quad-image-label-v2_style_threeLineTruncation__UkUjj")
// Rule 541: active_Class("_fluid-fat-image-link-v2_style_smartText__ubpEw")
// Rule 149: active_Class("_ameyal-product-shoveler_style_stacking-context__3PbQE")
// Rule 690: match_Class("_fluid-quad-image-label-v2_style_gwm-link-footer__3OF47")
// Rule 11: active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
// Rule 141: active_Class("_ameyal-product-shoveler_style_poster-image__1W0yA")
// Rule 406: match_Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite-mobile__2_rj8")
// Rule 694: match_Class("_fluid-quad-image-label-v2_style_header-icon__2cuVV")
// Rule 515: active_Class("_fluid-fat-image-link-v2_style_logo__2ZQ-N")
// Rule 71: active_Class("_ameyal-product-shoveler_style_close-black-icon__3hkbe")
// Rule 76: match_Class("_ameyal-product-shoveler_style_close-text__2-gwn")
// Rule 692: match_Class("_fluid-quad-image-label-v2_style_haulRibbon__3VZNi")
// Rule 0: match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
// Rule 368: match_Class("_cropped-image-link_style_wd-backdrop-data__1znxG")
// Rule 224: match_Class("_cropped-image-link_style_apexBadgeMessage__1tHvd")
// Rule 201: active_Class("_cropped-image-link_image_asin-container__2jyCM")
// Rule 545: active_Class("_fluid-fat-image-link-v2_style_spacer__7Pyg3")
// Rule 606: match_Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite__28uwB")
// Rule 752: match_Class("_fluid-quad-image-label-v2_style_themingTextColorWhite__1zryO")
// Rule 844: match_Class("a-link-normal")
// Rule 883: active_Type("span")
// Rule 488: match_Class("_fluid-fat-image-link-v2_style_gw-hero-close-button__3svyZ")
// Rule 222: match_Class("_cropped-image-link_style_apexBadgeLabel__2-Vye")
// Rule 735: active_Class("_fluid-quad-image-label-v2_style_positive-button__3UOC3")
// Rule 840: match_Class("a-carousel-right")
// Rule 491: active_Class("_fluid-fat-image-link-v2_style_gwm-link-footer__3OF47")
// Rule 602: match_Class("_fluid-quad-image-label-v2_style_ad-feedback-primary-link__2bIZi")
// Rule 118: match_Class("_ameyal-product-shoveler_style_logoSquare__3NZyi")
// Rule 604: match_Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite-mobile__2_rj8")
// Rule 184: match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
// Rule 229: active_Class("_cropped-image-link_style_aspect-button__7cH_E")
// Rule 634: match_Class("_fluid-quad-image-label-v2_style_aspect-text__S4PU1")
// Rule 310: match_Class("_cropped-image-link_style_logoRectangle__1VJwu")
// Rule 670: match_Class("_fluid-quad-image-label-v2_style_fluid-landscape-image__TE6PT")
// Rule 500: match_Class("_fluid-fat-image-link-v2_style_image-container__2OiZA")
// Rule 316: match_Class("_cropped-image-link_style_logo__2ZQ-N")
// Rule 91: active_Class("_ameyal-product-shoveler_style_five-pack__1-Tql")
// Rule 786: match_Class("_quad-category-card_desktopStyle_heroImage__2V8-9")
// Rule 496: match_Class("_fluid-fat-image-link-v2_style_header-link__cUhOK")
// Rule 12: match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-standard__28gp8")
// Rule 566: match_Class("_fluid-fat-image-link-v2_style_truncation__x9-69")
// Rule 704: match_Class("_fluid-quad-image-label-v2_style_inlineErrorDetails__1NBx-")
// Rule 257: active_Class("_cropped-image-link_style_centerImage__1rzYI")
// Rule 99: active_Class("_ameyal-product-shoveler_style_gwm-link-footer__3OF47")
// Rule 154: match_Class("_ameyal-product-shoveler_style_themingTextColor__1oQsI")
// Rule 489: active_Class("_fluid-fat-image-link-v2_style_gw-hero-close-button__3svyZ")
// Rule 57: active_Class("_ameyal-product-shoveler_style_aspect-ratio-fill__2Zjfb")
// Rule 485: active_Class("_fluid-fat-image-link-v2_style_fluidPortraitImage__2SAYm")
// Rule 881: active_Type("h3")
// Rule 63: active_Class("_ameyal-product-shoveler_style_badge-container__20aJ2")
// Rule 490: match_Class("_fluid-fat-image-link-v2_style_gwm-link-footer__3OF47")
// Rule 26: match_Class("_ameyal-product-shoveler_style_ad-feedback-primary-link__2bIZi")
// Rule 147: active_Class("_ameyal-product-shoveler_style_spacer__7Pyg3")
// Rule 349: active_Class("_cropped-image-link_style_themingTextColor__1oQsI")
// Rule 403: active_Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner__1nmZw")
// Rule 66: match_Class("_ameyal-product-shoveler_style_badgeMessage__2Dtw7")
// Rule 507: active_Class("_fluid-fat-image-link-v2_style_logoGap__nKNZ9")
// Rule 815: active_Class("_quad-category-card_mobileStyle_leftMost__3WtU6")
// Rule 175: active_Class("_ameyal-product-shoveler_style_wd-backdrop-data__1znxG")
// Rule 83: active_Class("_ameyal-product-shoveler_style_desktop-close-button__1iL_P")
// Rule 373: active_Class("_fluid-fat-image-link-v2_bodyFooterStyle_cardBody__1YuQY")
// Rule 577: active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
// Rule 600: match_Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner__1nmZw")
// Rule 514: match_Class("_fluid-fat-image-link-v2_style_logo__2ZQ-N")
// Rule 231: active_Class("_cropped-image-link_style_aspect-ratio-1236x1080__3aEzl")
// Rule 595: active_Class("_fluid-quad-image-label-v2_image_asin-container__2jyCM")
// Rule 153: active_Class("_ameyal-product-shoveler_style_themingTextColorWhite__1zryO")
// Rule 199: active_Class("_cropped-image-link_image_asin-container-white-box__QwmgO")
// Rule 204: match_Class("_cropped-image-link_image_round-corners__22iOW")
// Rule 124: match_Class("_ameyal-product-shoveler_style_mobile-close-button__3PB07")
// Rule 256: match_Class("_cropped-image-link_style_centerImage__1rzYI")
// Rule 317: active_Class("_cropped-image-link_style_logo__2ZQ-N")
// Rule 385: active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
// Rule 3: active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
// Rule 451: active_Class("_fluid-fat-image-link-v2_style_close-black-icon__3hkbe")
// Rule 512: match_Class("_fluid-fat-image-link-v2_style_logoSquare__3NZyi")
// Rule 660: match_Class("_fluid-quad-image-label-v2_style_desktop-close-button__1iL_P")
// Rule 290: match_Class("_cropped-image-link_style_fluidPortraitImage__3yQ-X")
// Rule 677: active_Class("_fluid-quad-image-label-v2_style_fluidPortraitImage__2SAYm")
// Rule 743: active_Class("_fluid-quad-image-label-v2_style_smartText__ubpEw")
// Rule 797: active_Class("_quad-category-card_image_asin-container-full-height__MOKlF")
// Rule 281: active_Class("_cropped-image-link_style_empty-footer__2d59h")
// Rule 773: active_Class("_fluid-quad-image-label-v2_style_video-container__1hKS1")
// Rule 652: match_Class("_fluid-quad-image-label-v2_style_close-icon__2RJs3")
// Rule 838: match_Class("a-carousel-controls")
// Rule 820: match_Class("_quad-category-card_style_fluidLandscapeImage__3eTVC")
// Rule 864: match_Class("nav-timeline-prime-icon")
// Rule 640: match_Class("_fluid-quad-image-label-v2_style_badgeLabel__pJ5rc")
// Rule 627: active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-4x3__3BewI")
// Rule 666: match_Class("_fluid-quad-image-label-v2_style_empty-footer__2d59h")
// Rule 796: match_Class("_quad-category-card_image_asin-container-full-height__MOKlF")
// Rule 21: active_Class("_ameyal-product-shoveler_image_round-corners__2y_fS")
// Rule 450: match_Class("_fluid-fat-image-link-v2_style_close-black-icon__3hkbe")
// Rule 724: match_Class("_fluid-quad-image-label-v2_style_mosaic-card__1C-_R")
// Rule 70: match_Class("_ameyal-product-shoveler_style_close-black-icon__3hkbe")
// Rule 823: active_Class("_quad-category-card_style_fluidPortraitImage__3yQ-X")
// Rule 454: match_Class("_fluid-fat-image-link-v2_style_close-icon__2RJs3")
// Rule 24: match_Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner__1nmZw")
// Rule 105: active_Class("_ameyal-product-shoveler_style_header-link__cUhOK")
// Rule 807: active_Class("_quad-category-card_mobileStyle_categoryContainer__2xY0I")
// Rule 60: match_Class("_ameyal-product-shoveler_style_autoplay-span__2CMfc")
// Rule 360: match_Class("_cropped-image-link_style_tile-theming__3eeyj")
// Rule 67: active_Class("_ameyal-product-shoveler_style_badgeMessage__2Dtw7")
// Rule 460: match_Class("_fluid-fat-image-link-v2_style_cta-link__2xo74")
// Rule 329: active_Class("_cropped-image-link_style_negativeMarginAdjust__1nqu9")
// Rule 527: active_Class("_fluid-fat-image-link-v2_style_mosaic-card__1C-_R")
// Rule 225: active_Class("_cropped-image-link_style_apexBadgeMessage__1tHvd")
// Rule 353: active_Class("_cropped-image-link_style_threeLineTruncation__UkUjj")
// Rule 288: match_Class("_cropped-image-link_style_fluidLandscapeImage__3eTVC")
// Rule 333: active_Class("_cropped-image-link_style_overlay__3Sx3u")
// Rule 453: active_Class("_fluid-fat-image-link-v2_style_close-icon-wrapper__1zvdC")
// Rule 369: active_Class("_cropped-image-link_style_wd-backdrop-data__1znxG")
// Rule 1: active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
// Rule 170: match_Class("_ameyal-product-shoveler_style_twoLineTruncation__16TLV")
// Rule 408: match_Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite__28uwB")
// Rule 151: active_Class("_ameyal-product-shoveler_style_theming-background-override__1HfzJ")
// Rule 559: active_Class("_fluid-fat-image-link-v2_style_tile-container__1QgAV")
// Rule 591: active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-container__1Pkva")
// Rule 749: active_Class("_fluid-quad-image-label-v2_style_stacking-context__3PbQE")
// Rule 215: active_Class("_cropped-image-link_style_ad-feedback-sprite-mobile__2_rj8")
// Rule 279: active_Class("_cropped-image-link_style_dynamic-portrait-image__1Wrzd")
// Rule 659: active_Class("_fluid-quad-image-label-v2_style_cta-link__2xo74")
// Rule 69: active_Class("_ameyal-product-shoveler_style_carouselContainer__3N7M1")
// Rule 383: active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
// Rule 726: match_Class("_fluid-quad-image-label-v2_style_negative-button__1Dvqz")
// Rule 647: active_Class("_fluid-quad-image-label-v2_style_centerImage__30wh-")
// Rule 774: match_Class("_fluid-quad-image-label-v2_style_wd-backdrop-data__1znxG")
// Rule 181: active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
// Rule 762: match_Class("_fluid-quad-image-label-v2_style_tile-grid__QMxNY")
// Rule 459: active_Class("_fluid-fat-image-link-v2_style_cover-portrait-image__2lhzL")
// Rule 393: active_Class("_fluid-fat-image-link-v2_image_asin-container__2jyCM")
// Rule 587: active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
// Rule 759: active_Class("_fluid-quad-image-label-v2_style_threeLineTruncation__UkUjj")
// Rule 113: active_Class("_ameyal-product-shoveler_style_logoGap__nKNZ9")

pub fn print_node_matches(node: &HtmlNode, matches: &[bool]) {
            println!("Node '{}' matches:", node.tag_name);
            for (i, &matched) in matches.iter().enumerate() {
                if matched {
                    println!("  Rule {}: {}", i, get_rule_name(i));
                }
            }
        }pub fn get_total_rules() -> usize {
    884 // Total number of CSS rules
}
