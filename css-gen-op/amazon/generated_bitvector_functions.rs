use crate::{BitVector, HtmlNode, SimpleSelector};
use std::collections::HashMap;
use std::sync::OnceLock;

const BITVECTOR_CAPACITY: usize = 884;

// String interning for optimized selector matching
static STRING_TO_ID: OnceLock<HashMap<&'static str, u32>> = OnceLock::new();

fn get_string_to_id_map() -> &'static HashMap<&'static str, u32> {
    STRING_TO_ID.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("_fluid-fat-image-link-v2_style_fluid-landscape-image__TE6PT", 236);
        map.insert("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P", 290);
        map.insert("_quad-category-card_desktopStyle_heroLink__1EhW2", 394);
        map.insert("_cropped-image-link_style_themingTextColor__1oQsI", 174);
        map.insert("_fluid-fat-image-link-v2_style_positive-button__3UOC3", 268);
        map.insert("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating__3_0eN", 4);
        map.insert("_fluid-quad-image-label-v2_style_gw-hero-close-button__3svyZ", 344);
        map.insert("_cropped-image-link_style_fluidImageContainer__2jd50", 143);
        map.insert("_ameyal-product-shoveler_style_aspect-ratio-15x3__1h649", 23);
        map.insert("_cropped-image-link_style_aspect-button__7cH_E", 114);
        map.insert("_cropped-image-link_style_badgeMessage__2Dtw7", 126);
        map.insert("_fluid-fat-image-link-v2_style_badge-container__20aJ2", 220);
        map.insert("_fluid-fat-image-link-v2_style_smartText__ubpEw", 270);
        map.insert("_fluid-quad-image-label-v2_style_themingTextColor__1oQsI", 377);
        map.insert("_quad-category-card_image_asin-container-white-box__3Stwp", 399);
        map.insert("_fluid-quad-image-label-v2_style_twoLineTruncation__16TLV", 385);
        map.insert("_fluid-fat-image-link-v2_style_dynamic-portrait-image__1Wrzd", 233);
        map.insert("_fluid-fat-image-link-v2_style_twoLineTruncation__16TLV", 284);
        map.insert("_cropped-image-link_energy-efficiency_energy-efficiency-badge-standard__28gp8", 95);
        map.insert("_cropped-image-link_image_asin-container-full-height__MOKlF", 97);
        map.insert("_fluid-fat-image-link-v2_style_truncation__x9-69", 283);
        map.insert("_fluid-quad-image-label-v2_style_empty-footer__2d59h", 333);
        map.insert("_cropped-image-link_style_aspect-ratio-5x8__2IaNz", 119);
        map.insert("_quad-category-card_style_fluidImageContainer__2jd50", 409);
        map.insert("a-link-normal", 422);
        map.insert("_cropped-image-link_style_dt-TextContainer__3nbU9", 138);
        map.insert("_cropped-image-link_style_smartText__ubpEw", 169);
        map.insert("_fluid-quad-image-label-v2_style_badge-container__20aJ2", 319);
        map.insert("_ameyal-product-shoveler_style_ad-feedback-text__2HjQ9", 17);
        map.insert("_ameyal-product-shoveler_style_header-icon__2cuVV", 51);
        map.insert("_ameyal-product-shoveler_style_fluid-landscape-image__TE6PT", 46);
        map.insert("_cropped-image-link_style_close-black-icon__3hkbe", 129);
        map.insert("_fluid-fat-image-link-v2_style_image-container__2OiZA", 250);
        map.insert("_fluid-quad-image-label-v2_style_desktop-close-button__1iL_P", 330);
        map.insert("_ameyal-product-shoveler_style_three-pack__5s3hP", 78);
        map.insert("_fluid-fat-image-link-v2_style_gwm-link-footer__3OF47", 245);
        map.insert("_fluid-quad-image-label-v2_style_centerImage__30wh-", 323);
        map.insert("_fluid-quad-image-label-v2_style_tile-container__1QgAV", 380);
        map.insert("_quad-category-card_image_round-corners__22iOW", 401);
        map.insert("_ameyal-product-shoveler_style_ad-feedback-primary-link__2bIZi", 13);
        map.insert("_ameyal-product-shoveler_style_close-text__2-gwn", 38);
        map.insert("_ameyal-product-shoveler_style_five-pack__1-Tql", 45);
        map.insert("_cropped-image-link_style_cover-portrait-image__2lhzL", 133);
        map.insert("_fluid-fat-image-link-v2_style_four-pack__1ufgr", 243);
        map.insert("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating__3_0eN", 93);
        map.insert("a-carousel-container", 418);
        map.insert("gw-media-card", 428);
        map.insert("span", 441);
        map.insert("_fluid-quad-image-label-v2_style_themingTextColorWhite__1zryO", 376);
        map.insert("_cropped-image-link_style_twoLineTruncation__16TLV", 182);
        map.insert("_cropped-image-link_style_haulRibbon__3VZNi", 149);
        map.insert("_fluid-quad-image-label-v2_style_autoplay-span__2CMfc", 318);
        map.insert("_fluid-fat-image-link-v2_style_aspect-ratio-dynamic-60vh__3N5g_", 216);
        map.insert("_cropped-image-link_style_ad-feedback-text-desktop__q3xp_", 109);
        map.insert("_fluid-quad-image-label-v2_style_aspect-button__7cH_E", 309);
        map.insert("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2", 0);
        map.insert("_quad-category-card_mobileStyle_heroImage__1SewP", 406);
        map.insert("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2", 89);
        map.insert("_cropped-image-link_style_header-link__cUhOK", 151);
        map.insert("_ameyal-product-shoveler_style_threeLineTruncation__UkUjj", 79);
        map.insert("_fluid-quad-image-label-v2_style_theming-background-override__1HfzJ", 375);
        map.insert("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3", 1);
        map.insert("_fluid-quad-image-label-v2_style_badgeLabel__pJ5rc", 320);
        map.insert("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2", 288);
        map.insert("_cropped-image-link_style_aspect-text__S4PU1", 122);
        map.insert("_fluid-quad-image-label-v2_style_five-pack__1-Tql", 334);
        map.insert("_ameyal-product-shoveler_style_themingTextColor__1oQsI", 77);
        map.insert("_fluid-quad-image-label-v2_style_cta-link__2xo74", 329);
        map.insert("a-carousel-viewport", 421);
        map.insert("_fluid-fat-image-link-v2_style_fluidPortraitImage__2SAYm", 242);
        map.insert("_fluid-quad-image-label-v2_style_mobile-close-button__3PB07", 360);
        map.insert("icp-touch-link-country", 438);
        map.insert("_quad-category-card_fluid_fluidCard__3hmFA", 397);
        map.insert("a-cardui-header", 417);
        map.insert("truncate-1line", 434);
        map.insert("_ameyal-product-shoveler_style_gw-hero-close-button__3svyZ", 48);
        map.insert("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-container__1Pkva", 194);
        map.insert("_fluid-fat-image-link-v2_style_fluidFatImageLink__1nw4J", 238);
        map.insert("_fluid-fat-image-link-v2_style_oneLineTruncation__2WWse", 266);
        map.insert("_fluid-fat-image-link-v2_style_logoRectangle__1VJwu", 254);
        map.insert("_fluid-quad-image-label-v2_style_wdHeader__Edrev", 388);
        map.insert("_quad-category-card_fluid_fluidCardBody__3TzJ4", 396);
        map.insert("_cropped-image-link_style_video-container__1hKS1", 183);
        map.insert("_ameyal-product-shoveler_style_video-container__1hKS1", 86);
        map.insert("_fluid-fat-image-link-v2_style_negative-button__1Dvqz", 264);
        map.insert("_fluid-quad-image-label-v2_style_displayCount__1MVut", 331);
        map.insert("_fluid-fat-image-link-v2_style_ad-feedback-text-desktop__q3xp_", 205);
        map.insert("_fluid-fat-image-link-v2_style_haulRibbon__3VZNi", 246);
        map.insert("_fluid-fat-image-link-v2_style_threeLineTruncation__UkUjj", 278);
        map.insert("_ameyal-product-shoveler_style_twoLineTruncation__16TLV", 85);
        map.insert("_fluid-quad-image-label-v2_style_spCSRFTreatment__-hwVO", 372);
        map.insert("_ameyal-product-shoveler_style_header-link__cUhOK", 52);
        map.insert("_cropped-image-link_style_aspect-ratio-15x3__1h649", 116);
        map.insert("_ameyal-product-shoveler_style_spCSRFTreatment__-hwVO", 72);
        map.insert("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner__1nmZw", 300);
        map.insert("_cropped-image-link_style_aspect-ratio-fill__2Zjfb", 121);
        map.insert("_fluid-quad-image-label-v2_style_negativeMarginAdjust__1nqu9", 364);
        map.insert("_fluid-fat-image-link-v2_image_asin-container__2jyCM", 196);
        map.insert("_fluid-quad-image-label-v2_style_aspect-ratio-1236x1080__3aEzl", 310);
        map.insert("a-cardui-body", 415);
        map.insert("_fluid-quad-image-label-v2_style_aspect-ratio-16x9__cBPv8", 312);
        map.insert("_cropped-image-link_image_round-corners__2y_fS", 103);
        map.insert("_ameyal-product-shoveler_style_themingTextColorWhite__1zryO", 76);
        map.insert("_ameyal-product-shoveler_style_logoRectangle__1VJwu", 57);
        map.insert("_cropped-image-link_style_cropped-image-link__3winf", 134);
        map.insert("_cropped-image-link_style_apexBadgeMessage__1tHvd", 112);
        map.insert("_fluid-quad-image-label-v2_style_logo__2ZQ-N", 358);
        map.insert("nav-timeline-prime-icon", 432);
        map.insert("_cropped-image-link_style_header-icon__2cuVV", 150);
        map.insert("_ameyal-product-shoveler_style_theming-background-override__1HfzJ", 75);
        map.insert("_ameyal-product-shoveler_style_tile-theming__3eeyj", 83);
        map.insert("_cropped-image-link_style_four-pack__1ufgr", 146);
        map.insert("_fluid-fat-image-link-v2_style_negativeMarginAdjust__1nqu9", 265);
        map.insert("_fluid-fat-image-link-v2_style_aspect-button__7cH_E", 210);
        map.insert("_fluid-fat-image-link-v2_style_header__1vGdj", 249);
        map.insert("_fluid-fat-image-link-v2_style_inlineErrorDetails__1NBx-", 252);
        map.insert("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner__1nmZw", 12);
        map.insert("_cropped-image-link_style_ad-feedback-loading-spinnner-rtl__2BoOY", 104);
        map.insert("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-container__1Pkva", 295);
        map.insert("_fluid-quad-image-label-v2_style_close-icon-wrapper__1zvdC", 325);
        map.insert("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY", 200);
        map.insert("_fluid-fat-image-link-v2_style_logoGap__nKNZ9", 253);
        map.insert("_fluid-quad-image-label-v2_style_three-pack__5s3hP", 378);
        map.insert("_fluid-quad-image-label-v2_style_imageLabel__3ANSV", 351);
        map.insert("icp-touch-link-cop", 437);
        map.insert("_fluid-quad-image-label-v2_style_close-black-icon__3hkbe", 324);
        map.insert("_fluid-fat-image-link-v2_style_tile-container__1QgAV", 279);
        map.insert("_ameyal-product-shoveler_style_mosaic-card-body__1HmTs", 63);
        map.insert("_cropped-image-link_style_close-text__2-gwn", 132);
        map.insert("_fluid-fat-image-link-v2_style_gw-hero-close-button__3svyZ", 244);
        map.insert("_fluid-fat-image-link-v2_style_themingTextColorWhite__1zryO", 275);
        map.insert("_fluid-quad-image-label-v2_style_ad-feedback-sprite__28uwB", 303);
        map.insert("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN", 292);
        map.insert("_fluid-quad-image-label-v2_image_asin-container-white-box__QwmgO", 296);
        map.insert("_fluid-quad-image-label-v2_style_aspect-button-group__1LqUG", 308);
        map.insert("_fluid-quad-image-label-v2_style_quadrantContainer__3TMqG", 369);
        map.insert("_ameyal-product-shoveler_style_inlineErrorDetails__1NBx-", 55);
        map.insert("_cropped-image-link_style_oneLineTruncation__2WWse", 165);
        map.insert("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P", 189);
        map.insert("_fluid-fat-image-link-v2_style_spCSRFTreatment__-hwVO", 271);
        map.insert("_fluid-fat-image-link-v2_image_round-corners__2y_fS", 197);
        map.insert("_fluid-fat-image-link-v2_style_mixed-button__2og-m", 260);
        map.insert("_cropped-image-link_style_tile-container__1QgAV", 177);
        map.insert("_cropped-image-link_style_empty-footer__2d59h", 140);
        map.insert("_fluid-fat-image-link-v2_style_tile-grid__QMxNY", 280);
        map.insert("_fluid-fat-image-link-v2_style_tile-link__38lTa", 281);
        map.insert("_fluid-quad-image-label-v2_style_aspect-ratio-4x3__3BewI", 313);
        map.insert("_fluid-quad-image-label-v2_style_dynamic-portrait-image__1Wrzd", 332);
        map.insert("_fluid-quad-image-label-v2_style_gridRowTwo__15woW", 343);
        map.insert("_quad-category-card_desktopStyle_category__3flCQ", 391);
        map.insert("_quad-category-card_style_fluidPortraitImage__3yQ-X", 411);
        map.insert("_ameyal-product-shoveler_style_logoGap__nKNZ9", 56);
        map.insert("_cropped-image-link_style_logoGap__nKNZ9", 154);
        map.insert("_fluid-quad-image-label-v2_style_mixed-button__2og-m", 359);
        map.insert("_fluid-fat-image-link-v2_style_header-icon__2cuVV", 247);
        map.insert("_ameyal-product-shoveler_style_aspect-ratio-4x3__3BewI", 25);
        map.insert("card-flow-row-break", 423);
        map.insert("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8", 193);
        map.insert("_fluid-quad-image-label-v2_style_fluidPortraitImage__2SAYm", 338);
        map.insert("_fluid-quad-image-label-v2_style_fluidQuadImageLabel__3b-Iv", 340);
        map.insert("_fluid-quad-image-label-v2_style_rightQuadrant__PI01n", 370);
        map.insert("_fluid-fat-image-link-v2_style_aspect-ratio-5x8__2IaNz", 215);
        map.insert("_fluid-fat-image-link-v2_image_asin-container-white-box__QwmgO", 195);
        map.insert("_fluid-fat-image-link-v2_style_mergedLinks__10JqZ", 259);
        map.insert("_fluid-quad-image-label-v2_style_spacer__7Pyg3", 373);
        map.insert("_cropped-image-link_style_ad-feedback-loading-spinnner__1nmZw", 105);
        map.insert("_ameyal-product-shoveler_style_mosaic-card__1C-_R", 64);
        map.insert("_cropped-image-link_style_aspect-ratio-1236x1080__3aEzl", 115);
        map.insert("_ameyal-product-shoveler_style_apexBadgeLabel__2-Vye", 18);
        map.insert("_fluid-quad-image-label-v2_style_logoSquareContainer__3Paoc", 356);
        map.insert("_quad-category-card_mobileStyle_categoryImage__3hSFw", 404);
        map.insert("_cropped-image-link_style_header__1vGdj", 152);
        map.insert("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3", 289);
        map.insert("_ameyal-product-shoveler_style_ad-feedback-sprite__28uwB", 15);
        map.insert("nav-focus", 430);
        map.insert("_fluid-fat-image-link-v2_style_displayCount__1MVut", 232);
        map.insert("_fluid-quad-image-label-v2_style_close-text__2-gwn", 327);
        map.insert("_ameyal-product-shoveler_style_badgeMessage__2Dtw7", 33);
        map.insert("_ameyal-product-shoveler_style_desktop-close-button__1iL_P", 41);
        map.insert("_cropped-image-link_style_negativeMarginAdjust__1nqu9", 164);
        map.insert("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK", 190);
        map.insert("_cropped-image-link_style_logo__2ZQ-N", 158);
        map.insert("_ameyal-product-shoveler_style_aspect-ratio-fill__2Zjfb", 28);
        map.insert("_cropped-image-link_image_round-corners__22iOW", 102);
        map.insert("_cropped-image-link_style_displayCount__1MVut", 137);
        map.insert("_ameyal-product-shoveler_image_round-corners__2y_fS", 10);
        map.insert("_ameyal-product-shoveler_style_aspect-ratio-5x8__2IaNz", 26);
        map.insert("_ameyal-product-shoveler_style_aspect-button__7cH_E", 21);
        map.insert("_ameyal-product-shoveler_style_aspect-text__S4PU1", 29);
        map.insert("_cropped-image-link_style_tile-grid__QMxNY", 178);
        map.insert("_cropped-image-link_style_truncation__x9-69", 181);
        map.insert("_fluid-fat-image-link-v2_style_fluidFatImageLinkBody__1LsOX", 237);
        map.insert("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK", 291);
        map.insert("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8", 294);
        map.insert("_fluid-fat-image-link-v2_style_themingTextColor__1oQsI", 276);
        map.insert("_fluid-quad-image-label-v2_style_aspect-text__S4PU1", 317);
        map.insert("_fluid-quad-image-label-v2_style_header-icon__2cuVV", 347);
        map.insert("_cropped-image-link_style_close-icon__2RJs3", 131);
        map.insert("_fluid-fat-image-link-v2_style_ad-feedback-primary-link__2bIZi", 202);
        map.insert("_ameyal-product-shoveler_style_tile-container__1QgAV", 80);
        map.insert("_ameyal-product-shoveler_style_spacer__7Pyg3", 73);
        map.insert("_cropped-image-link_style_five-pack__1-Tql", 141);
        map.insert("_fluid-fat-image-link-v2_style_aspect-ratio-16x9__cBPv8", 213);
        map.insert("_fluid-quad-image-label-v2_image_round-corners__2y_fS", 298);
        map.insert("_fluid-quad-image-label-v2_style_header__1vGdj", 349);
        map.insert("_fluid-quad-image-label-v2_style_negative-button__1Dvqz", 363);
        map.insert("_fluid-quad-image-label-v2_style_overlay__3Sx3u", 366);
        map.insert("_cropped-image-link_style_themingTextColorWhite__1zryO", 173);
        map.insert("_fluid-fat-image-link-v2_style_autoplay-span__2CMfc", 219);
        map.insert("_fluid-fat-image-link-v2_singleLinkStyle_footer__2cH0y", 199);
        map.insert("_fluid-quad-image-label-v2_style_logoSquare__3NZyi", 357);
        map.insert("_fluid-quad-image-label-v2_style_truncation__x9-69", 384);
        map.insert("_fluid-quad-image-label-v2_style_wd-backdrop-data__1znxG", 387);
        map.insert("_cropped-image-link_style_fluidLandscapeImage__3eTVC", 144);
        map.insert("_cropped-image-link_style_desktop-close-button__1iL_P", 136);
        map.insert("_quad-category-card_mobileStyle_category__1amt4", 405);
        map.insert("_quad-category-card_mobileStyle_leftMost__3WtU6", 407);
        map.insert("_ameyal-product-shoveler_style_positive-button__3UOC3", 69);
        map.insert("_quad-category-card_style_fluidLandscapeImage__3eTVC", 410);
        map.insert("_quad-category-card_desktopStyle_categoryImage__35jKN", 390);
        map.insert("h3", 440);
        map.insert("_cropped-image-link_style_theming-background-override__1HfzJ", 172);
        map.insert("_ameyal-product-shoveler_style_carouselContainer__3N7M1", 34);
        map.insert("_cropped-image-link_style_spacer__7Pyg3", 170);
        map.insert("_ameyal-product-shoveler_style_four-pack__1ufgr", 47);
        map.insert("_cropped-image-link_style_badge-container__20aJ2", 124);
        map.insert("_ameyal-product-shoveler_style_logo__2ZQ-N", 60);
        map.insert("_cropped-image-link_style_fluidPortraitImage__3yQ-X", 145);
        map.insert("_fluid-fat-image-link-v2_style_apexBadgeLabel__2-Vye", 207);
        map.insert("_fluid-fat-image-link-v2_style_stacking-context__3PbQE", 273);
        map.insert("_fluid-quad-image-label-v2_style_fluidImageContainer__2SOMr", 336);
        map.insert("_ameyal-product-shoveler_style_close-black-icon__3hkbe", 35);
        map.insert("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN", 191);
        map.insert("_fluid-quad-image-label-v2_style_smartText__ubpEw", 371);
        map.insert("_cropped-image-link_style_aspect-ratio-16x9__cBPv8", 117);
        map.insert("_cropped-image-link_style_fluid-landscape-image__TE6PT", 142);
        map.insert("_ameyal-product-shoveler_style_aspect-ratio-16x9__cBPv8", 24);
        map.insert("_cropped-image-link_style_gwm-link-footer__3OF47", 148);
        map.insert("_fluid-quad-image-label-v2_style_aspect-ratio-5x8__2IaNz", 314);
        map.insert("_fluid-quad-image-label-v2_style_oneLineTruncation__2WWse", 365);
        map.insert("_cropped-image-link_style_aspect-ratio-4x3__3BewI", 118);
        map.insert("_fluid-quad-image-label-v2_style_poster-image__1W0yA", 368);
        map.insert("_fluid-quad-image-label-v2_style_threeLineTruncation__UkUjj", 379);
        map.insert("_ameyal-product-shoveler_style_smartText__ubpEw", 71);
        map.insert("_ameyal-product-shoveler_style_badgeLabel__pJ5rc", 32);
        map.insert("_quad-category-card_desktopStyle_leftMost__1LmQB", 395);
        map.insert("gw-auto-height", 424);
        map.insert("_cropped-image-link_style_mosaic-card-body__1HmTs", 161);
        map.insert("truncate-2line", 435);
        map.insert("_cropped-image-link_energy-efficiency_energy-efficiency-container__1Pkva", 96);
        map.insert("_fluid-quad-image-label-v2_style_fluidQuadImageLabelBody__3tld0", 339);
        map.insert("_cropped-image-link_style_logoSquare__3NZyi", 157);
        map.insert("_cropped-image-link_image_asin-container-white-box__QwmgO", 99);
        map.insert("_ameyal-product-shoveler_style_aspect-ratio-dynamic-60vh__3N5g_", 27);
        map.insert("_ameyal-product-shoveler_style_tile-link__38lTa", 82);
        map.insert("_cropped-image-link_style_aspect-ratio-dynamic-60vh__3N5g_", 120);
        map.insert("_cropped-image-link_style_logoRectangle__1VJwu", 155);
        map.insert("_cropped-image-link_style_positive-button__3UOC3", 167);
        map.insert("_fluid-fat-image-link-v2_style_ad-feedback-sprite__28uwB", 204);
        map.insert("_fluid-fat-image-link-v2_style_aspect-button-group__1LqUG", 209);
        map.insert("_fluid-fat-image-link-v2_style_fluidImageContainer__2vGwp", 240);
        map.insert("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3", 90);
        map.insert("_fluid-fat-image-link-v2_style_video-container__1hKS1", 285);
        map.insert("gw-row", 429);
        map.insert("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3", 188);
        map.insert("_fluid-quad-image-label-v2_style_video-container__1hKS1", 386);
        map.insert("_ameyal-product-shoveler_style_ad-feedback-sprite-mobile__2_rj8", 14);
        map.insert("_fluid-fat-image-link-v2_style_wdHeader__Edrev", 287);
        map.insert("_fluid-quad-image-label-v2_style_haulRibbon__3VZNi", 346);
        map.insert("_fluid-fat-image-link-v2_style_empty-footer__2d59h", 234);
        map.insert("_cropped-image-link_style_threeLineTruncation__UkUjj", 176);
        map.insert("_fluid-fat-image-link-v2_style_header-link__cUhOK", 248);
        map.insert("_fluid-fat-image-link-v2_style_mobile-close-button__3PB07", 261);
        map.insert("_fluid-quad-image-label-v2_style_logoRectangle__1VJwu", 355);
        map.insert("_fluid-quad-image-label-v2_style_mosaic-card__1C-_R", 362);
        map.insert("_quad-category-card_image_asin-container-full-height__MOKlF", 398);
        map.insert("_fluid-fat-image-link-v2_style_close-black-icon__3hkbe", 225);
        map.insert("_fluid-fat-image-link-v2_style_mosaic-card-body__1HmTs", 262);
        map.insert("_cropped-image-link_style_tile-theming__3eeyj", 180);
        map.insert("_cropped-image-link_style_mosaic-card__1C-_R", 162);
        map.insert("_fluid-quad-image-label-v2_style_close-icon__2RJs3", 326);
        map.insert("_ameyal-product-shoveler_style_aspect-button-group__1LqUG", 20);
        map.insert("_fluid-fat-image-link-v2_singleLinkStyle_bodyFooterLink__9LvH0", 198);
        map.insert("_fluid-fat-image-link-v2_style_logo__2ZQ-N", 257);
        map.insert("_fluid-quad-image-label-v2_style_mosaic-card-body__1HmTs", 361);
        map.insert("_ameyal-product-shoveler_style_cover-portrait-image__2lhzL", 39);
        map.insert("_cropped-image-link_style_badgeLabel__pJ5rc", 125);
        map.insert("_ameyal-product-shoveler_style_gwm-link-footer__3OF47", 49);
        map.insert("_ameyal-product-shoveler_style_autoplay-span__2CMfc", 30);
        map.insert("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK", 92);
        map.insert("icp-touch-link-language", 439);
        map.insert("_ameyal-product-shoveler_style_wd-backdrop-data__1znxG", 87);
        map.insert("_ameyal-product-shoveler_style_header__1vGdj", 53);
        map.insert("_fluid-quad-image-label-v2_style_carouselContainer__3N7M1", 322);
        map.insert("_fluid-quad-image-label-v2_style_positive-button__3UOC3", 367);
        map.insert("_cropped-image-link_style_carouselContainer__3N7M1", 127);
        map.insert("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner-rtl__2BoOY", 11);
        map.insert("_fluid-fat-image-link-v2_style_close-icon-wrapper__1zvdC", 226);
        map.insert("_ameyal-product-shoveler_image_asin-container__2jyCM", 9);
        map.insert("_cropped-image-link_style_aspect-button-group__1LqUG", 113);
        map.insert("_cropped-image-link_style_mobile-close-button__3PB07", 160);
        map.insert("_ameyal-product-shoveler_style_logoSquareContainer__3Paoc", 58);
        map.insert("_cropped-image-link_style_logoSquareContainer__3Paoc", 156);
        map.insert("_ameyal-product-shoveler_style_cta-link__2xo74", 40);
        map.insert("_fluid-quad-image-label-v2_style_fluid-landscape-image__TE6PT", 335);
        map.insert("_fluid-quad-image-label-v2_style_image-container__2OiZA", 350);
        map.insert("_ameyal-product-shoveler_style_close-icon__2RJs3", 37);
        map.insert("_ameyal-product-shoveler_style_aspect-ratio-1236x1080__3aEzl", 22);
        map.insert("_ameyal-product-shoveler_style_haulRibbon__3VZNi", 50);
        map.insert("_cropped-image-link_style_ad-feedback-text__2HjQ9", 110);
        map.insert("_ameyal-product-shoveler_style_poster-image__1W0yA", 70);
        map.insert("_fluid-quad-image-label-v2_style_leftQuadrant__21nVp", 353);
        map.insert("_fluid-fat-image-link-v2_style_desktop-close-button__1iL_P", 231);
        map.insert("_cropped-image-link_style_three-pack__5s3hP", 175);
        map.insert("_ameyal-product-shoveler_style_close-icon-wrapper__1zvdC", 36);
        map.insert("_fluid-fat-image-link-v2_style_badgeMessage__2Dtw7", 222);
        map.insert("_fluid-quad-image-label-v2_image_asin-container__2jyCM", 297);
        map.insert("_fluid-quad-image-label-v2_style_ad-feedback-text-desktop__q3xp_", 304);
        map.insert("_fluid-quad-image-label-v2_style_apexBadgeMessage__1tHvd", 307);
        map.insert("_fluid-quad-image-label-v2_style_aspect-ratio-dynamic-60vh__3N5g_", 315);
        map.insert("_fluid-fat-image-link-v2_style_close-text__2-gwn", 228);
        map.insert("_cropped-image-link_style_gw-hero-close-button__3svyZ", 147);
        map.insert("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY", 293);
        map.insert("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY", 192);
        map.insert("nav-spinner", 431);
        map.insert("_fluid-fat-image-link-v2_style_fluidLandscapeImage__2euAK", 241);
        map.insert("_fluid-quad-image-label-v2_style_ad-feedback-text__2HjQ9", 305);
        map.insert("_ameyal-product-shoveler_style_displayCount__1MVut", 42);
        map.insert("_ameyal-product-shoveler_style_logoSquare__3NZyi", 59);
        map.insert("_cropped-image-link_image_asin-container__LRY5p", 101);
        map.insert("_cropped-image-link_style_autoplay-span__2CMfc", 123);
        map.insert("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P", 2);
        map.insert("_ameyal-product-shoveler_style_image-container__2OiZA", 54);
        map.insert("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P", 91);
        map.insert("_cropped-image-link_style_tile-link__38lTa", 179);
        map.insert("_cropped-image-link_style_negative-button__1Dvqz", 163);
        map.insert("_ameyal-product-shoveler_style_negativeMarginAdjust__1nqu9", 66);
        map.insert("_fluid-fat-image-link-v2_style_apexBadgeMessage__1tHvd", 208);
        map.insert("_fluid-fat-image-link-v2_style_carouselContainer__3N7M1", 223);
        map.insert("_fluid-fat-image-link-v2_style_cta-link__2xo74", 230);
        map.insert("_fluid-fat-image-link-v2_style_logoSquareContainer__3Paoc", 255);
        map.insert("_fluid-quad-image-label-v2_style_aspect-ratio-fill__2Zjfb", 316);
        map.insert("_fluid-quad-image-label-v2_style_badgeMessage__2Dtw7", 321);
        map.insert("_cropped-image-link_style_overlay__3Sx3u", 166);
        map.insert("_fluid-fat-image-link-v2_style_ad-feedback-text__2HjQ9", 206);
        map.insert("_cropped-image-link_style_stacking-context__3PbQE", 171);
        map.insert("_fluid-fat-image-link-v2_style_aspect-ratio-4x3__3BewI", 214);
        map.insert("_fluid-quad-image-label-v2_style_fluidLandscapeImage__2euAK", 337);
        map.insert("_fluid-quad-image-label-v2_style_inlineErrorDetails__1NBx-", 352);
        map.insert("_fluid-quad-image-label-v2_style_tile-theming__3eeyj", 383);
        map.insert("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2", 187);
        map.insert("_quad-category-card_desktopStyle_heroCategory__3KS3k", 392);
        map.insert("_quad-category-card_mobileStyle_cardBody__3ODbW", 402);
        map.insert("_quad-category-card_mobileStyle_categoryContainer__2xY0I", 403);
        map.insert("_ameyal-product-shoveler_style_ad-feedback-text-desktop__q3xp_", 16);
        map.insert("_ameyal-product-shoveler_style_truncation__x9-69", 84);
        map.insert("_cropped-image-link_image_asin-container-white-box__3Stwp", 98);
        map.insert("a-cardui-footer", 416);
        map.insert("_ameyal-product-shoveler_style_empty-footer__2d59h", 44);
        map.insert("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-container__1Pkva", 7);
        map.insert("_ameyal-product-shoveler_style_stacking-context__3PbQE", 74);
        map.insert("_quad-category-card_image_asin-container__LRY5p", 400);
        map.insert("gw-col", 426);
        map.insert("single-slide-hero", 433);
        map.insert("_text-link-stripe-v2_style_textlinkstripe__3aQhz", 414);
        map.insert("_ameyal-product-shoveler_style_badge-container__20aJ2", 31);
        map.insert("_cropped-image-link_style_apexBadgeLabel__2-Vye", 111);
        map.insert("_fluid-fat-image-link-v2_style_aspect-ratio-fill__2Zjfb", 217);
        map.insert("_fluid-fat-image-link-v2_bodyFooterStyle_cardBody__1YuQY", 186);
        map.insert("_cropped-image-link_style_ad-feedback-sprite__28uwB", 108);
        map.insert("_fluid-quad-image-label-v2_style_logoGap__nKNZ9", 354);
        map.insert("_fluid-fat-image-link-v2_style_aspect-text__S4PU1", 218);
        map.insert("_fluid-fat-image-link-v2_style_wd-backdrop-data__1znxG", 286);
        map.insert("_fluid-fat-image-link-v2_style_mergedLinksCta__3Npog", 258);
        map.insert("gw-card-layout", 425);
        map.insert("_fluid-fat-image-link-v2_style_tile-theming__3eeyj", 282);
        map.insert("_cropped-image-link_style_ad-feedback-sprite-mobile__2_rj8", 107);
        map.insert("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-shape__1IcJY", 5);
        map.insert("_fluid-quad-image-label-v2_style_gwm-link-footer__3OF47", 345);
        map.insert("_quad-category-card_style_gwm-link-footer__3EX7d", 412);
        map.insert("_fluid-fat-image-link-v2_style_centerImage__30wh-", 224);
        map.insert("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-standard__28gp8", 6);
        map.insert("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK", 3);
        map.insert("_ameyal-product-shoveler_style_negative-button__1Dvqz", 65);
        map.insert("_cropped-image-link_style_ad-feedback-primary-link__2bIZi", 106);
        map.insert("_fluid-fat-image-link-v2_style_mosaic-card__1C-_R", 263);
        map.insert("_fluid-quad-image-label-v2_style_stacking-context__3PbQE", 374);
        map.insert("_fluid-fat-image-link-v2_style_overlay__3Sx3u", 267);
        map.insert("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY", 299);
        map.insert("_fluid-fat-image-link-v2_style_five-pack__1-Tql", 235);
        map.insert("_cropped-image-link_style_wdHeader__Edrev", 185);
        map.insert("_quad-category-card_style_heading__1mnEu", 413);
        map.insert("gw-fixed-col", 427);
        map.insert("_fluid-quad-image-label-v2_style_aspect-ratio-15x3__1h649", 311);
        map.insert("_fluid-quad-image-label-v2_style_tile-link__38lTa", 382);
        map.insert("_quad-category-card_style_dashboard-card-with-border__1e4z_", 408);
        map.insert("vjs-fluid", 436);
        map.insert("_fluid-quad-image-label-v2_style_apexBadgeLabel__2-Vye", 306);
        map.insert("_cropped-image-link_energy-efficiency_energy-efficiency-badge-shape__1IcJY", 94);
        map.insert("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner__1nmZw", 201);
        map.insert("_ameyal-product-shoveler_image_asin-container-white-box__QwmgO", 8);
        map.insert("_ameyal-product-shoveler_style_mixed-button__2og-m", 61);
        map.insert("_fluid-fat-image-link-v2_style_badgeLabel__pJ5rc", 221);
        map.insert("_ameyal-product-shoveler_style_oneLineTruncation__2WWse", 67);
        map.insert("_cropped-image-link_style_mixed-button__2og-m", 159);
        map.insert("_cropped-image-link_style_poster-image__1W0yA", 168);
        map.insert("_fluid-quad-image-label-v2_style_cover-portrait-image__2lhzL", 328);
        map.insert("_cropped-image-link_style_image-container__2OiZA", 153);
        map.insert("_quad-category-card_desktopStyle_cardBody__3Rdh1", 389);
        map.insert("_ameyal-product-shoveler_style_apexBadgeMessage__1tHvd", 19);
        map.insert("_cropped-image-link_style_centerImage__1rzYI", 128);
        map.insert("_cropped-image-link_style_cta-link__2xo74", 135);
        map.insert("_ameyal-product-shoveler_style_dynamic-portrait-image__1Wrzd", 43);
        map.insert("_fluid-fat-image-link-v2_style_aspect-ratio-1236x1080__3aEzl", 211);
        map.insert("_fluid-quad-image-label-v2_style_four-pack__1ufgr", 341);
        map.insert("_fluid-fat-image-link-v2_style_imageLabel__3ANSV", 251);
        map.insert("_fluid-quad-image-label-v2_style_header-link__cUhOK", 348);
        map.insert("_quad-category-card_desktopStyle_heroImage__2V8-9", 393);
        map.insert("_ameyal-product-shoveler_style_overlay__3Sx3u", 68);
        map.insert("_fluid-fat-image-link-v2_style_cover-portrait-image__2lhzL", 229);
        map.insert("_cropped-image-link_style_close-icon-wrapper__1zvdC", 130);
        map.insert("_ameyal-product-shoveler_style_wdHeader__Edrev", 88);
        map.insert("_cropped-image-link_image_asin-container__2jyCM", 100);
        map.insert("_fluid-fat-image-link-v2_style_logoSquare__3NZyi", 256);
        map.insert("_fluid-fat-image-link-v2_style_theming-background-override__1HfzJ", 274);
        map.insert("_cropped-image-link_style_dynamic-portrait-image__1Wrzd", 139);
        map.insert("_fluid-fat-image-link-v2_style_ad-feedback-sprite-mobile__2_rj8", 203);
        map.insert("_fluid-fat-image-link-v2_style_three-pack__5s3hP", 277);
        map.insert("_fluid-quad-image-label-v2_style_ad-feedback-primary-link__2bIZi", 301);
        map.insert("_ameyal-product-shoveler_style_tile-grid__QMxNY", 81);
        map.insert("_fluid-fat-image-link-v2_style_aspect-ratio-15x3__1h649", 212);
        map.insert("_cropped-image-link_style_wd-backdrop-data__1znxG", 184);
        map.insert("_fluid-fat-image-link-v2_style_close-icon__2RJs3", 227);
        map.insert("_fluid-fat-image-link-v2_style_poster-image__1W0yA", 269);
        map.insert("_fluid-quad-image-label-v2_style_ad-feedback-sprite-mobile__2_rj8", 302);
        map.insert("_fluid-fat-image-link-v2_style_fluidImageContainer__2SOMr", 239);
        map.insert("_fluid-quad-image-label-v2_style_gridRowOne__1t0zL", 342);
        map.insert("_fluid-quad-image-label-v2_style_tile-grid__QMxNY", 381);
        map.insert("a-carousel-controls", 419);
        map.insert("_ameyal-product-shoveler_style_mobile-close-button__3PB07", 62);
        map.insert("a-carousel-right", 420);
        map.insert("_fluid-fat-image-link-v2_style_spacer__7Pyg3", 272);
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
// --- BitVector-only Incremental Processing Functions ---
        pub fn process_node_generated_bitvector_incremental(
            node: &mut HtmlNode,
            parent_state: &BitVector,
        ) -> BitVector { // returns child_states
            // Check if we need to recompute using BitVector-only tracking
            if !node.needs_any_recomputation_bitvector(parent_state) {
                // Return cached result - entire subtree can be skipped
                return node.cached_child_states.clone().unwrap();
            }
            // Recompute node intrinsic matches if needed
            if node.cached_node_intrinsic.is_none() || node.is_self_dirty {
        let mut intrinsic_matches = BitVector::with_capacity(BITVECTOR_CAPACITY);
match get_node_tag_id(node) {
// Instruction 880: CheckAndSetBit { selector: Type("h3"), bit_pos: 880 }
 Some(440)  => {
            intrinsic_matches.set_bit(880); // match_Type("h3")
        }

// Instruction 882: CheckAndSetBit { selector: Type("span"), bit_pos: 882 }
 Some(441)  => {
            intrinsic_matches.set_bit(882); // match_Type("span")
        }

_ => {}}
        // Instruction 0: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2"), bit_pos: 0 }
        if node_has_class_id(node, 0) {
            intrinsic_matches.set_bit(0); // match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
        }

        // Instruction 2: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3"), bit_pos: 2 }
        if node_has_class_id(node, 1) {
            intrinsic_matches.set_bit(2); // match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
        }

        // Instruction 4: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P"), bit_pos: 4 }
        if node_has_class_id(node, 2) {
            intrinsic_matches.set_bit(4); // match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
        }

        // Instruction 6: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK"), bit_pos: 6 }
        if node_has_class_id(node, 3) {
            intrinsic_matches.set_bit(6); // match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
        }

        // Instruction 8: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating__3_0eN"), bit_pos: 8 }
        if node_has_class_id(node, 4) {
            intrinsic_matches.set_bit(8); // match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
        }

        // Instruction 10: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-shape__1IcJY"), bit_pos: 10 }
        if node_has_class_id(node, 5) {
            intrinsic_matches.set_bit(10); // match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
        }

        // Instruction 12: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-standard__28gp8"), bit_pos: 12 }
        if node_has_class_id(node, 6) {
            intrinsic_matches.set_bit(12); // match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-standard__28gp8")
        }

        // Instruction 14: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-container__1Pkva"), bit_pos: 14 }
        if node_has_class_id(node, 7) {
            intrinsic_matches.set_bit(14); // match_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-container__1Pkva")
        }

        // Instruction 16: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_image_asin-container-white-box__QwmgO"), bit_pos: 16 }
        if node_has_class_id(node, 8) {
            intrinsic_matches.set_bit(16); // match_Class("_ameyal-product-shoveler_image_asin-container-white-box__QwmgO")
        }

        // Instruction 18: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_image_asin-container__2jyCM"), bit_pos: 18 }
        if node_has_class_id(node, 9) {
            intrinsic_matches.set_bit(18); // match_Class("_ameyal-product-shoveler_image_asin-container__2jyCM")
        }

        // Instruction 20: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_image_round-corners__2y_fS"), bit_pos: 20 }
        if node_has_class_id(node, 10) {
            intrinsic_matches.set_bit(20); // match_Class("_ameyal-product-shoveler_image_round-corners__2y_fS")
        }

        // Instruction 22: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner-rtl__2BoOY"), bit_pos: 22 }
        if node_has_class_id(node, 11) {
            intrinsic_matches.set_bit(22); // match_Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner-rtl__2BoOY")
        }

        // Instruction 24: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner__1nmZw"), bit_pos: 24 }
        if node_has_class_id(node, 12) {
            intrinsic_matches.set_bit(24); // match_Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner__1nmZw")
        }

        // Instruction 26: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-primary-link__2bIZi"), bit_pos: 26 }
        if node_has_class_id(node, 13) {
            intrinsic_matches.set_bit(26); // match_Class("_ameyal-product-shoveler_style_ad-feedback-primary-link__2bIZi")
        }

        // Instruction 28: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-sprite-mobile__2_rj8"), bit_pos: 28 }
        if node_has_class_id(node, 14) {
            intrinsic_matches.set_bit(28); // match_Class("_ameyal-product-shoveler_style_ad-feedback-sprite-mobile__2_rj8")
        }

        // Instruction 30: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-sprite__28uwB"), bit_pos: 30 }
        if node_has_class_id(node, 15) {
            intrinsic_matches.set_bit(30); // match_Class("_ameyal-product-shoveler_style_ad-feedback-sprite__28uwB")
        }

        // Instruction 32: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-text-desktop__q3xp_"), bit_pos: 32 }
        if node_has_class_id(node, 16) {
            intrinsic_matches.set_bit(32); // match_Class("_ameyal-product-shoveler_style_ad-feedback-text-desktop__q3xp_")
        }

        // Instruction 34: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_ad-feedback-text__2HjQ9"), bit_pos: 34 }
        if node_has_class_id(node, 17) {
            intrinsic_matches.set_bit(34); // match_Class("_ameyal-product-shoveler_style_ad-feedback-text__2HjQ9")
        }

        // Instruction 36: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_apexBadgeLabel__2-Vye"), bit_pos: 36 }
        if node_has_class_id(node, 18) {
            intrinsic_matches.set_bit(36); // match_Class("_ameyal-product-shoveler_style_apexBadgeLabel__2-Vye")
        }

        // Instruction 38: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_apexBadgeMessage__1tHvd"), bit_pos: 38 }
        if node_has_class_id(node, 19) {
            intrinsic_matches.set_bit(38); // match_Class("_ameyal-product-shoveler_style_apexBadgeMessage__1tHvd")
        }

        // Instruction 40: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-button-group__1LqUG"), bit_pos: 40 }
        if node_has_class_id(node, 20) {
            intrinsic_matches.set_bit(40); // match_Class("_ameyal-product-shoveler_style_aspect-button-group__1LqUG")
        }

        // Instruction 42: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-button__7cH_E"), bit_pos: 42 }
        if node_has_class_id(node, 21) {
            intrinsic_matches.set_bit(42); // match_Class("_ameyal-product-shoveler_style_aspect-button__7cH_E")
        }

        // Instruction 44: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-1236x1080__3aEzl"), bit_pos: 44 }
        if node_has_class_id(node, 22) {
            intrinsic_matches.set_bit(44); // match_Class("_ameyal-product-shoveler_style_aspect-ratio-1236x1080__3aEzl")
        }

        // Instruction 46: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-15x3__1h649"), bit_pos: 46 }
        if node_has_class_id(node, 23) {
            intrinsic_matches.set_bit(46); // match_Class("_ameyal-product-shoveler_style_aspect-ratio-15x3__1h649")
        }

        // Instruction 48: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-16x9__cBPv8"), bit_pos: 48 }
        if node_has_class_id(node, 24) {
            intrinsic_matches.set_bit(48); // match_Class("_ameyal-product-shoveler_style_aspect-ratio-16x9__cBPv8")
        }

        // Instruction 50: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-4x3__3BewI"), bit_pos: 50 }
        if node_has_class_id(node, 25) {
            intrinsic_matches.set_bit(50); // match_Class("_ameyal-product-shoveler_style_aspect-ratio-4x3__3BewI")
        }

        // Instruction 52: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-5x8__2IaNz"), bit_pos: 52 }
        if node_has_class_id(node, 26) {
            intrinsic_matches.set_bit(52); // match_Class("_ameyal-product-shoveler_style_aspect-ratio-5x8__2IaNz")
        }

        // Instruction 54: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-dynamic-60vh__3N5g_"), bit_pos: 54 }
        if node_has_class_id(node, 27) {
            intrinsic_matches.set_bit(54); // match_Class("_ameyal-product-shoveler_style_aspect-ratio-dynamic-60vh__3N5g_")
        }

        // Instruction 56: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-ratio-fill__2Zjfb"), bit_pos: 56 }
        if node_has_class_id(node, 28) {
            intrinsic_matches.set_bit(56); // match_Class("_ameyal-product-shoveler_style_aspect-ratio-fill__2Zjfb")
        }

        // Instruction 58: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_aspect-text__S4PU1"), bit_pos: 58 }
        if node_has_class_id(node, 29) {
            intrinsic_matches.set_bit(58); // match_Class("_ameyal-product-shoveler_style_aspect-text__S4PU1")
        }

        // Instruction 60: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_autoplay-span__2CMfc"), bit_pos: 60 }
        if node_has_class_id(node, 30) {
            intrinsic_matches.set_bit(60); // match_Class("_ameyal-product-shoveler_style_autoplay-span__2CMfc")
        }

        // Instruction 62: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_badge-container__20aJ2"), bit_pos: 62 }
        if node_has_class_id(node, 31) {
            intrinsic_matches.set_bit(62); // match_Class("_ameyal-product-shoveler_style_badge-container__20aJ2")
        }

        // Instruction 64: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_badgeLabel__pJ5rc"), bit_pos: 64 }
        if node_has_class_id(node, 32) {
            intrinsic_matches.set_bit(64); // match_Class("_ameyal-product-shoveler_style_badgeLabel__pJ5rc")
        }

        // Instruction 66: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_badgeMessage__2Dtw7"), bit_pos: 66 }
        if node_has_class_id(node, 33) {
            intrinsic_matches.set_bit(66); // match_Class("_ameyal-product-shoveler_style_badgeMessage__2Dtw7")
        }

        // Instruction 68: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_carouselContainer__3N7M1"), bit_pos: 68 }
        if node_has_class_id(node, 34) {
            intrinsic_matches.set_bit(68); // match_Class("_ameyal-product-shoveler_style_carouselContainer__3N7M1")
        }

        // Instruction 70: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_close-black-icon__3hkbe"), bit_pos: 70 }
        if node_has_class_id(node, 35) {
            intrinsic_matches.set_bit(70); // match_Class("_ameyal-product-shoveler_style_close-black-icon__3hkbe")
        }

        // Instruction 72: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_close-icon-wrapper__1zvdC"), bit_pos: 72 }
        if node_has_class_id(node, 36) {
            intrinsic_matches.set_bit(72); // match_Class("_ameyal-product-shoveler_style_close-icon-wrapper__1zvdC")
        }

        // Instruction 74: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_close-icon__2RJs3"), bit_pos: 74 }
        if node_has_class_id(node, 37) {
            intrinsic_matches.set_bit(74); // match_Class("_ameyal-product-shoveler_style_close-icon__2RJs3")
        }

        // Instruction 76: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_close-text__2-gwn"), bit_pos: 76 }
        if node_has_class_id(node, 38) {
            intrinsic_matches.set_bit(76); // match_Class("_ameyal-product-shoveler_style_close-text__2-gwn")
        }

        // Instruction 78: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_cover-portrait-image__2lhzL"), bit_pos: 78 }
        if node_has_class_id(node, 39) {
            intrinsic_matches.set_bit(78); // match_Class("_ameyal-product-shoveler_style_cover-portrait-image__2lhzL")
        }

        // Instruction 80: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_cta-link__2xo74"), bit_pos: 80 }
        if node_has_class_id(node, 40) {
            intrinsic_matches.set_bit(80); // match_Class("_ameyal-product-shoveler_style_cta-link__2xo74")
        }

        // Instruction 82: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_desktop-close-button__1iL_P"), bit_pos: 82 }
        if node_has_class_id(node, 41) {
            intrinsic_matches.set_bit(82); // match_Class("_ameyal-product-shoveler_style_desktop-close-button__1iL_P")
        }

        // Instruction 84: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_displayCount__1MVut"), bit_pos: 84 }
        if node_has_class_id(node, 42) {
            intrinsic_matches.set_bit(84); // match_Class("_ameyal-product-shoveler_style_displayCount__1MVut")
        }

        // Instruction 86: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_dynamic-portrait-image__1Wrzd"), bit_pos: 86 }
        if node_has_class_id(node, 43) {
            intrinsic_matches.set_bit(86); // match_Class("_ameyal-product-shoveler_style_dynamic-portrait-image__1Wrzd")
        }

        // Instruction 88: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_empty-footer__2d59h"), bit_pos: 88 }
        if node_has_class_id(node, 44) {
            intrinsic_matches.set_bit(88); // match_Class("_ameyal-product-shoveler_style_empty-footer__2d59h")
        }

        // Instruction 90: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_five-pack__1-Tql"), bit_pos: 90 }
        if node_has_class_id(node, 45) {
            intrinsic_matches.set_bit(90); // match_Class("_ameyal-product-shoveler_style_five-pack__1-Tql")
        }

        // Instruction 92: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_fluid-landscape-image__TE6PT"), bit_pos: 92 }
        if node_has_class_id(node, 46) {
            intrinsic_matches.set_bit(92); // match_Class("_ameyal-product-shoveler_style_fluid-landscape-image__TE6PT")
        }

        // Instruction 94: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_four-pack__1ufgr"), bit_pos: 94 }
        if node_has_class_id(node, 47) {
            intrinsic_matches.set_bit(94); // match_Class("_ameyal-product-shoveler_style_four-pack__1ufgr")
        }

        // Instruction 96: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_gw-hero-close-button__3svyZ"), bit_pos: 96 }
        if node_has_class_id(node, 48) {
            intrinsic_matches.set_bit(96); // match_Class("_ameyal-product-shoveler_style_gw-hero-close-button__3svyZ")
        }

        // Instruction 98: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_gwm-link-footer__3OF47"), bit_pos: 98 }
        if node_has_class_id(node, 49) {
            intrinsic_matches.set_bit(98); // match_Class("_ameyal-product-shoveler_style_gwm-link-footer__3OF47")
        }

        // Instruction 100: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_haulRibbon__3VZNi"), bit_pos: 100 }
        if node_has_class_id(node, 50) {
            intrinsic_matches.set_bit(100); // match_Class("_ameyal-product-shoveler_style_haulRibbon__3VZNi")
        }

        // Instruction 102: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_header-icon__2cuVV"), bit_pos: 102 }
        if node_has_class_id(node, 51) {
            intrinsic_matches.set_bit(102); // match_Class("_ameyal-product-shoveler_style_header-icon__2cuVV")
        }

        // Instruction 104: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_header-link__cUhOK"), bit_pos: 104 }
        if node_has_class_id(node, 52) {
            intrinsic_matches.set_bit(104); // match_Class("_ameyal-product-shoveler_style_header-link__cUhOK")
        }

        // Instruction 106: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_header__1vGdj"), bit_pos: 106 }
        if node_has_class_id(node, 53) {
            intrinsic_matches.set_bit(106); // match_Class("_ameyal-product-shoveler_style_header__1vGdj")
        }

        // Instruction 108: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_image-container__2OiZA"), bit_pos: 108 }
        if node_has_class_id(node, 54) {
            intrinsic_matches.set_bit(108); // match_Class("_ameyal-product-shoveler_style_image-container__2OiZA")
        }

        // Instruction 110: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_inlineErrorDetails__1NBx-"), bit_pos: 110 }
        if node_has_class_id(node, 55) {
            intrinsic_matches.set_bit(110); // match_Class("_ameyal-product-shoveler_style_inlineErrorDetails__1NBx-")
        }

        // Instruction 112: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logoGap__nKNZ9"), bit_pos: 112 }
        if node_has_class_id(node, 56) {
            intrinsic_matches.set_bit(112); // match_Class("_ameyal-product-shoveler_style_logoGap__nKNZ9")
        }

        // Instruction 114: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logoRectangle__1VJwu"), bit_pos: 114 }
        if node_has_class_id(node, 57) {
            intrinsic_matches.set_bit(114); // match_Class("_ameyal-product-shoveler_style_logoRectangle__1VJwu")
        }

        // Instruction 116: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logoSquareContainer__3Paoc"), bit_pos: 116 }
        if node_has_class_id(node, 58) {
            intrinsic_matches.set_bit(116); // match_Class("_ameyal-product-shoveler_style_logoSquareContainer__3Paoc")
        }

        // Instruction 118: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logoSquare__3NZyi"), bit_pos: 118 }
        if node_has_class_id(node, 59) {
            intrinsic_matches.set_bit(118); // match_Class("_ameyal-product-shoveler_style_logoSquare__3NZyi")
        }

        // Instruction 120: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_logo__2ZQ-N"), bit_pos: 120 }
        if node_has_class_id(node, 60) {
            intrinsic_matches.set_bit(120); // match_Class("_ameyal-product-shoveler_style_logo__2ZQ-N")
        }

        // Instruction 122: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_mixed-button__2og-m"), bit_pos: 122 }
        if node_has_class_id(node, 61) {
            intrinsic_matches.set_bit(122); // match_Class("_ameyal-product-shoveler_style_mixed-button__2og-m")
        }

        // Instruction 124: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_mobile-close-button__3PB07"), bit_pos: 124 }
        if node_has_class_id(node, 62) {
            intrinsic_matches.set_bit(124); // match_Class("_ameyal-product-shoveler_style_mobile-close-button__3PB07")
        }

        // Instruction 126: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_mosaic-card-body__1HmTs"), bit_pos: 126 }
        if node_has_class_id(node, 63) {
            intrinsic_matches.set_bit(126); // match_Class("_ameyal-product-shoveler_style_mosaic-card-body__1HmTs")
        }

        // Instruction 128: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_mosaic-card__1C-_R"), bit_pos: 128 }
        if node_has_class_id(node, 64) {
            intrinsic_matches.set_bit(128); // match_Class("_ameyal-product-shoveler_style_mosaic-card__1C-_R")
        }

        // Instruction 130: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_negative-button__1Dvqz"), bit_pos: 130 }
        if node_has_class_id(node, 65) {
            intrinsic_matches.set_bit(130); // match_Class("_ameyal-product-shoveler_style_negative-button__1Dvqz")
        }

        // Instruction 132: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_negativeMarginAdjust__1nqu9"), bit_pos: 132 }
        if node_has_class_id(node, 66) {
            intrinsic_matches.set_bit(132); // match_Class("_ameyal-product-shoveler_style_negativeMarginAdjust__1nqu9")
        }

        // Instruction 134: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_oneLineTruncation__2WWse"), bit_pos: 134 }
        if node_has_class_id(node, 67) {
            intrinsic_matches.set_bit(134); // match_Class("_ameyal-product-shoveler_style_oneLineTruncation__2WWse")
        }

        // Instruction 136: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_overlay__3Sx3u"), bit_pos: 136 }
        if node_has_class_id(node, 68) {
            intrinsic_matches.set_bit(136); // match_Class("_ameyal-product-shoveler_style_overlay__3Sx3u")
        }

        // Instruction 138: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_positive-button__3UOC3"), bit_pos: 138 }
        if node_has_class_id(node, 69) {
            intrinsic_matches.set_bit(138); // match_Class("_ameyal-product-shoveler_style_positive-button__3UOC3")
        }

        // Instruction 140: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_poster-image__1W0yA"), bit_pos: 140 }
        if node_has_class_id(node, 70) {
            intrinsic_matches.set_bit(140); // match_Class("_ameyal-product-shoveler_style_poster-image__1W0yA")
        }

        // Instruction 142: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_smartText__ubpEw"), bit_pos: 142 }
        if node_has_class_id(node, 71) {
            intrinsic_matches.set_bit(142); // match_Class("_ameyal-product-shoveler_style_smartText__ubpEw")
        }

        // Instruction 144: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_spCSRFTreatment__-hwVO"), bit_pos: 144 }
        if node_has_class_id(node, 72) {
            intrinsic_matches.set_bit(144); // match_Class("_ameyal-product-shoveler_style_spCSRFTreatment__-hwVO")
        }

        // Instruction 146: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_spacer__7Pyg3"), bit_pos: 146 }
        if node_has_class_id(node, 73) {
            intrinsic_matches.set_bit(146); // match_Class("_ameyal-product-shoveler_style_spacer__7Pyg3")
        }

        // Instruction 148: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_stacking-context__3PbQE"), bit_pos: 148 }
        if node_has_class_id(node, 74) {
            intrinsic_matches.set_bit(148); // match_Class("_ameyal-product-shoveler_style_stacking-context__3PbQE")
        }

        // Instruction 150: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_theming-background-override__1HfzJ"), bit_pos: 150 }
        if node_has_class_id(node, 75) {
            intrinsic_matches.set_bit(150); // match_Class("_ameyal-product-shoveler_style_theming-background-override__1HfzJ")
        }

        // Instruction 152: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_themingTextColorWhite__1zryO"), bit_pos: 152 }
        if node_has_class_id(node, 76) {
            intrinsic_matches.set_bit(152); // match_Class("_ameyal-product-shoveler_style_themingTextColorWhite__1zryO")
        }

        // Instruction 154: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_themingTextColor__1oQsI"), bit_pos: 154 }
        if node_has_class_id(node, 77) {
            intrinsic_matches.set_bit(154); // match_Class("_ameyal-product-shoveler_style_themingTextColor__1oQsI")
        }

        // Instruction 156: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_three-pack__5s3hP"), bit_pos: 156 }
        if node_has_class_id(node, 78) {
            intrinsic_matches.set_bit(156); // match_Class("_ameyal-product-shoveler_style_three-pack__5s3hP")
        }

        // Instruction 158: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_threeLineTruncation__UkUjj"), bit_pos: 158 }
        if node_has_class_id(node, 79) {
            intrinsic_matches.set_bit(158); // match_Class("_ameyal-product-shoveler_style_threeLineTruncation__UkUjj")
        }

        // Instruction 160: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_tile-container__1QgAV"), bit_pos: 160 }
        if node_has_class_id(node, 80) {
            intrinsic_matches.set_bit(160); // match_Class("_ameyal-product-shoveler_style_tile-container__1QgAV")
        }

        // Instruction 162: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_tile-grid__QMxNY"), bit_pos: 162 }
        if node_has_class_id(node, 81) {
            intrinsic_matches.set_bit(162); // match_Class("_ameyal-product-shoveler_style_tile-grid__QMxNY")
        }

        // Instruction 164: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_tile-link__38lTa"), bit_pos: 164 }
        if node_has_class_id(node, 82) {
            intrinsic_matches.set_bit(164); // match_Class("_ameyal-product-shoveler_style_tile-link__38lTa")
        }

        // Instruction 166: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_tile-theming__3eeyj"), bit_pos: 166 }
        if node_has_class_id(node, 83) {
            intrinsic_matches.set_bit(166); // match_Class("_ameyal-product-shoveler_style_tile-theming__3eeyj")
        }

        // Instruction 168: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_truncation__x9-69"), bit_pos: 168 }
        if node_has_class_id(node, 84) {
            intrinsic_matches.set_bit(168); // match_Class("_ameyal-product-shoveler_style_truncation__x9-69")
        }

        // Instruction 170: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_twoLineTruncation__16TLV"), bit_pos: 170 }
        if node_has_class_id(node, 85) {
            intrinsic_matches.set_bit(170); // match_Class("_ameyal-product-shoveler_style_twoLineTruncation__16TLV")
        }

        // Instruction 172: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_video-container__1hKS1"), bit_pos: 172 }
        if node_has_class_id(node, 86) {
            intrinsic_matches.set_bit(172); // match_Class("_ameyal-product-shoveler_style_video-container__1hKS1")
        }

        // Instruction 174: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_wd-backdrop-data__1znxG"), bit_pos: 174 }
        if node_has_class_id(node, 87) {
            intrinsic_matches.set_bit(174); // match_Class("_ameyal-product-shoveler_style_wd-backdrop-data__1znxG")
        }

        // Instruction 176: CheckAndSetBit { selector: Class("_ameyal-product-shoveler_style_wdHeader__Edrev"), bit_pos: 176 }
        if node_has_class_id(node, 88) {
            intrinsic_matches.set_bit(176); // match_Class("_ameyal-product-shoveler_style_wdHeader__Edrev")
        }

        // Instruction 178: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2"), bit_pos: 178 }
        if node_has_class_id(node, 89) {
            intrinsic_matches.set_bit(178); // match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
        }

        // Instruction 180: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3"), bit_pos: 180 }
        if node_has_class_id(node, 90) {
            intrinsic_matches.set_bit(180); // match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
        }

        // Instruction 182: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P"), bit_pos: 182 }
        if node_has_class_id(node, 91) {
            intrinsic_matches.set_bit(182); // match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
        }

        // Instruction 184: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK"), bit_pos: 184 }
        if node_has_class_id(node, 92) {
            intrinsic_matches.set_bit(184); // match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
        }

        // Instruction 186: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating__3_0eN"), bit_pos: 186 }
        if node_has_class_id(node, 93) {
            intrinsic_matches.set_bit(186); // match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
        }

        // Instruction 188: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-shape__1IcJY"), bit_pos: 188 }
        if node_has_class_id(node, 94) {
            intrinsic_matches.set_bit(188); // match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
        }

        // Instruction 190: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-standard__28gp8"), bit_pos: 190 }
        if node_has_class_id(node, 95) {
            intrinsic_matches.set_bit(190); // match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-standard__28gp8")
        }

        // Instruction 192: CheckAndSetBit { selector: Class("_cropped-image-link_energy-efficiency_energy-efficiency-container__1Pkva"), bit_pos: 192 }
        if node_has_class_id(node, 96) {
            intrinsic_matches.set_bit(192); // match_Class("_cropped-image-link_energy-efficiency_energy-efficiency-container__1Pkva")
        }

        // Instruction 194: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container-full-height__MOKlF"), bit_pos: 194 }
        if node_has_class_id(node, 97) {
            intrinsic_matches.set_bit(194); // match_Class("_cropped-image-link_image_asin-container-full-height__MOKlF")
        }

        // Instruction 196: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container-white-box__3Stwp"), bit_pos: 196 }
        if node_has_class_id(node, 98) {
            intrinsic_matches.set_bit(196); // match_Class("_cropped-image-link_image_asin-container-white-box__3Stwp")
        }

        // Instruction 198: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container-white-box__QwmgO"), bit_pos: 198 }
        if node_has_class_id(node, 99) {
            intrinsic_matches.set_bit(198); // match_Class("_cropped-image-link_image_asin-container-white-box__QwmgO")
        }

        // Instruction 200: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container__2jyCM"), bit_pos: 200 }
        if node_has_class_id(node, 100) {
            intrinsic_matches.set_bit(200); // match_Class("_cropped-image-link_image_asin-container__2jyCM")
        }

        // Instruction 202: CheckAndSetBit { selector: Class("_cropped-image-link_image_asin-container__LRY5p"), bit_pos: 202 }
        if node_has_class_id(node, 101) {
            intrinsic_matches.set_bit(202); // match_Class("_cropped-image-link_image_asin-container__LRY5p")
        }

        // Instruction 204: CheckAndSetBit { selector: Class("_cropped-image-link_image_round-corners__22iOW"), bit_pos: 204 }
        if node_has_class_id(node, 102) {
            intrinsic_matches.set_bit(204); // match_Class("_cropped-image-link_image_round-corners__22iOW")
        }

        // Instruction 206: CheckAndSetBit { selector: Class("_cropped-image-link_image_round-corners__2y_fS"), bit_pos: 206 }
        if node_has_class_id(node, 103) {
            intrinsic_matches.set_bit(206); // match_Class("_cropped-image-link_image_round-corners__2y_fS")
        }

        // Instruction 208: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-loading-spinnner-rtl__2BoOY"), bit_pos: 208 }
        if node_has_class_id(node, 104) {
            intrinsic_matches.set_bit(208); // match_Class("_cropped-image-link_style_ad-feedback-loading-spinnner-rtl__2BoOY")
        }

        // Instruction 210: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-loading-spinnner__1nmZw"), bit_pos: 210 }
        if node_has_class_id(node, 105) {
            intrinsic_matches.set_bit(210); // match_Class("_cropped-image-link_style_ad-feedback-loading-spinnner__1nmZw")
        }

        // Instruction 212: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-primary-link__2bIZi"), bit_pos: 212 }
        if node_has_class_id(node, 106) {
            intrinsic_matches.set_bit(212); // match_Class("_cropped-image-link_style_ad-feedback-primary-link__2bIZi")
        }

        // Instruction 214: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-sprite-mobile__2_rj8"), bit_pos: 214 }
        if node_has_class_id(node, 107) {
            intrinsic_matches.set_bit(214); // match_Class("_cropped-image-link_style_ad-feedback-sprite-mobile__2_rj8")
        }

        // Instruction 216: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-sprite__28uwB"), bit_pos: 216 }
        if node_has_class_id(node, 108) {
            intrinsic_matches.set_bit(216); // match_Class("_cropped-image-link_style_ad-feedback-sprite__28uwB")
        }

        // Instruction 218: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-text-desktop__q3xp_"), bit_pos: 218 }
        if node_has_class_id(node, 109) {
            intrinsic_matches.set_bit(218); // match_Class("_cropped-image-link_style_ad-feedback-text-desktop__q3xp_")
        }

        // Instruction 220: CheckAndSetBit { selector: Class("_cropped-image-link_style_ad-feedback-text__2HjQ9"), bit_pos: 220 }
        if node_has_class_id(node, 110) {
            intrinsic_matches.set_bit(220); // match_Class("_cropped-image-link_style_ad-feedback-text__2HjQ9")
        }

        // Instruction 222: CheckAndSetBit { selector: Class("_cropped-image-link_style_apexBadgeLabel__2-Vye"), bit_pos: 222 }
        if node_has_class_id(node, 111) {
            intrinsic_matches.set_bit(222); // match_Class("_cropped-image-link_style_apexBadgeLabel__2-Vye")
        }

        // Instruction 224: CheckAndSetBit { selector: Class("_cropped-image-link_style_apexBadgeMessage__1tHvd"), bit_pos: 224 }
        if node_has_class_id(node, 112) {
            intrinsic_matches.set_bit(224); // match_Class("_cropped-image-link_style_apexBadgeMessage__1tHvd")
        }

        // Instruction 226: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-button-group__1LqUG"), bit_pos: 226 }
        if node_has_class_id(node, 113) {
            intrinsic_matches.set_bit(226); // match_Class("_cropped-image-link_style_aspect-button-group__1LqUG")
        }

        // Instruction 228: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-button__7cH_E"), bit_pos: 228 }
        if node_has_class_id(node, 114) {
            intrinsic_matches.set_bit(228); // match_Class("_cropped-image-link_style_aspect-button__7cH_E")
        }

        // Instruction 230: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-1236x1080__3aEzl"), bit_pos: 230 }
        if node_has_class_id(node, 115) {
            intrinsic_matches.set_bit(230); // match_Class("_cropped-image-link_style_aspect-ratio-1236x1080__3aEzl")
        }

        // Instruction 232: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-15x3__1h649"), bit_pos: 232 }
        if node_has_class_id(node, 116) {
            intrinsic_matches.set_bit(232); // match_Class("_cropped-image-link_style_aspect-ratio-15x3__1h649")
        }

        // Instruction 234: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-16x9__cBPv8"), bit_pos: 234 }
        if node_has_class_id(node, 117) {
            intrinsic_matches.set_bit(234); // match_Class("_cropped-image-link_style_aspect-ratio-16x9__cBPv8")
        }

        // Instruction 236: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-4x3__3BewI"), bit_pos: 236 }
        if node_has_class_id(node, 118) {
            intrinsic_matches.set_bit(236); // match_Class("_cropped-image-link_style_aspect-ratio-4x3__3BewI")
        }

        // Instruction 238: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-5x8__2IaNz"), bit_pos: 238 }
        if node_has_class_id(node, 119) {
            intrinsic_matches.set_bit(238); // match_Class("_cropped-image-link_style_aspect-ratio-5x8__2IaNz")
        }

        // Instruction 240: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-dynamic-60vh__3N5g_"), bit_pos: 240 }
        if node_has_class_id(node, 120) {
            intrinsic_matches.set_bit(240); // match_Class("_cropped-image-link_style_aspect-ratio-dynamic-60vh__3N5g_")
        }

        // Instruction 242: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-ratio-fill__2Zjfb"), bit_pos: 242 }
        if node_has_class_id(node, 121) {
            intrinsic_matches.set_bit(242); // match_Class("_cropped-image-link_style_aspect-ratio-fill__2Zjfb")
        }

        // Instruction 244: CheckAndSetBit { selector: Class("_cropped-image-link_style_aspect-text__S4PU1"), bit_pos: 244 }
        if node_has_class_id(node, 122) {
            intrinsic_matches.set_bit(244); // match_Class("_cropped-image-link_style_aspect-text__S4PU1")
        }

        // Instruction 246: CheckAndSetBit { selector: Class("_cropped-image-link_style_autoplay-span__2CMfc"), bit_pos: 246 }
        if node_has_class_id(node, 123) {
            intrinsic_matches.set_bit(246); // match_Class("_cropped-image-link_style_autoplay-span__2CMfc")
        }

        // Instruction 248: CheckAndSetBit { selector: Class("_cropped-image-link_style_badge-container__20aJ2"), bit_pos: 248 }
        if node_has_class_id(node, 124) {
            intrinsic_matches.set_bit(248); // match_Class("_cropped-image-link_style_badge-container__20aJ2")
        }

        // Instruction 250: CheckAndSetBit { selector: Class("_cropped-image-link_style_badgeLabel__pJ5rc"), bit_pos: 250 }
        if node_has_class_id(node, 125) {
            intrinsic_matches.set_bit(250); // match_Class("_cropped-image-link_style_badgeLabel__pJ5rc")
        }

        // Instruction 252: CheckAndSetBit { selector: Class("_cropped-image-link_style_badgeMessage__2Dtw7"), bit_pos: 252 }
        if node_has_class_id(node, 126) {
            intrinsic_matches.set_bit(252); // match_Class("_cropped-image-link_style_badgeMessage__2Dtw7")
        }

        // Instruction 254: CheckAndSetBit { selector: Class("_cropped-image-link_style_carouselContainer__3N7M1"), bit_pos: 254 }
        if node_has_class_id(node, 127) {
            intrinsic_matches.set_bit(254); // match_Class("_cropped-image-link_style_carouselContainer__3N7M1")
        }

        // Instruction 256: CheckAndSetBit { selector: Class("_cropped-image-link_style_centerImage__1rzYI"), bit_pos: 256 }
        if node_has_class_id(node, 128) {
            intrinsic_matches.set_bit(256); // match_Class("_cropped-image-link_style_centerImage__1rzYI")
        }

        // Instruction 258: CheckAndSetBit { selector: Class("_cropped-image-link_style_close-black-icon__3hkbe"), bit_pos: 258 }
        if node_has_class_id(node, 129) {
            intrinsic_matches.set_bit(258); // match_Class("_cropped-image-link_style_close-black-icon__3hkbe")
        }

        // Instruction 260: CheckAndSetBit { selector: Class("_cropped-image-link_style_close-icon-wrapper__1zvdC"), bit_pos: 260 }
        if node_has_class_id(node, 130) {
            intrinsic_matches.set_bit(260); // match_Class("_cropped-image-link_style_close-icon-wrapper__1zvdC")
        }

        // Instruction 262: CheckAndSetBit { selector: Class("_cropped-image-link_style_close-icon__2RJs3"), bit_pos: 262 }
        if node_has_class_id(node, 131) {
            intrinsic_matches.set_bit(262); // match_Class("_cropped-image-link_style_close-icon__2RJs3")
        }

        // Instruction 264: CheckAndSetBit { selector: Class("_cropped-image-link_style_close-text__2-gwn"), bit_pos: 264 }
        if node_has_class_id(node, 132) {
            intrinsic_matches.set_bit(264); // match_Class("_cropped-image-link_style_close-text__2-gwn")
        }

        // Instruction 266: CheckAndSetBit { selector: Class("_cropped-image-link_style_cover-portrait-image__2lhzL"), bit_pos: 266 }
        if node_has_class_id(node, 133) {
            intrinsic_matches.set_bit(266); // match_Class("_cropped-image-link_style_cover-portrait-image__2lhzL")
        }

        // Instruction 268: CheckAndSetBit { selector: Class("_cropped-image-link_style_cropped-image-link__3winf"), bit_pos: 268 }
        if node_has_class_id(node, 134) {
            intrinsic_matches.set_bit(268); // match_Class("_cropped-image-link_style_cropped-image-link__3winf")
        }

        // Instruction 270: CheckAndSetBit { selector: Class("_cropped-image-link_style_cta-link__2xo74"), bit_pos: 270 }
        if node_has_class_id(node, 135) {
            intrinsic_matches.set_bit(270); // match_Class("_cropped-image-link_style_cta-link__2xo74")
        }

        // Instruction 272: CheckAndSetBit { selector: Class("_cropped-image-link_style_desktop-close-button__1iL_P"), bit_pos: 272 }
        if node_has_class_id(node, 136) {
            intrinsic_matches.set_bit(272); // match_Class("_cropped-image-link_style_desktop-close-button__1iL_P")
        }

        // Instruction 274: CheckAndSetBit { selector: Class("_cropped-image-link_style_displayCount__1MVut"), bit_pos: 274 }
        if node_has_class_id(node, 137) {
            intrinsic_matches.set_bit(274); // match_Class("_cropped-image-link_style_displayCount__1MVut")
        }

        // Instruction 276: CheckAndSetBit { selector: Class("_cropped-image-link_style_dt-TextContainer__3nbU9"), bit_pos: 276 }
        if node_has_class_id(node, 138) {
            intrinsic_matches.set_bit(276); // match_Class("_cropped-image-link_style_dt-TextContainer__3nbU9")
        }

        // Instruction 278: CheckAndSetBit { selector: Class("_cropped-image-link_style_dynamic-portrait-image__1Wrzd"), bit_pos: 278 }
        if node_has_class_id(node, 139) {
            intrinsic_matches.set_bit(278); // match_Class("_cropped-image-link_style_dynamic-portrait-image__1Wrzd")
        }

        // Instruction 280: CheckAndSetBit { selector: Class("_cropped-image-link_style_empty-footer__2d59h"), bit_pos: 280 }
        if node_has_class_id(node, 140) {
            intrinsic_matches.set_bit(280); // match_Class("_cropped-image-link_style_empty-footer__2d59h")
        }

        // Instruction 282: CheckAndSetBit { selector: Class("_cropped-image-link_style_five-pack__1-Tql"), bit_pos: 282 }
        if node_has_class_id(node, 141) {
            intrinsic_matches.set_bit(282); // match_Class("_cropped-image-link_style_five-pack__1-Tql")
        }

        // Instruction 284: CheckAndSetBit { selector: Class("_cropped-image-link_style_fluid-landscape-image__TE6PT"), bit_pos: 284 }
        if node_has_class_id(node, 142) {
            intrinsic_matches.set_bit(284); // match_Class("_cropped-image-link_style_fluid-landscape-image__TE6PT")
        }

        // Instruction 286: CheckAndSetBit { selector: Class("_cropped-image-link_style_fluidImageContainer__2jd50"), bit_pos: 286 }
        if node_has_class_id(node, 143) {
            intrinsic_matches.set_bit(286); // match_Class("_cropped-image-link_style_fluidImageContainer__2jd50")
        }

        // Instruction 288: CheckAndSetBit { selector: Class("_cropped-image-link_style_fluidLandscapeImage__3eTVC"), bit_pos: 288 }
        if node_has_class_id(node, 144) {
            intrinsic_matches.set_bit(288); // match_Class("_cropped-image-link_style_fluidLandscapeImage__3eTVC")
        }

        // Instruction 290: CheckAndSetBit { selector: Class("_cropped-image-link_style_fluidPortraitImage__3yQ-X"), bit_pos: 290 }
        if node_has_class_id(node, 145) {
            intrinsic_matches.set_bit(290); // match_Class("_cropped-image-link_style_fluidPortraitImage__3yQ-X")
        }

        // Instruction 292: CheckAndSetBit { selector: Class("_cropped-image-link_style_four-pack__1ufgr"), bit_pos: 292 }
        if node_has_class_id(node, 146) {
            intrinsic_matches.set_bit(292); // match_Class("_cropped-image-link_style_four-pack__1ufgr")
        }

        // Instruction 294: CheckAndSetBit { selector: Class("_cropped-image-link_style_gw-hero-close-button__3svyZ"), bit_pos: 294 }
        if node_has_class_id(node, 147) {
            intrinsic_matches.set_bit(294); // match_Class("_cropped-image-link_style_gw-hero-close-button__3svyZ")
        }

        // Instruction 296: CheckAndSetBit { selector: Class("_cropped-image-link_style_gwm-link-footer__3OF47"), bit_pos: 296 }
        if node_has_class_id(node, 148) {
            intrinsic_matches.set_bit(296); // match_Class("_cropped-image-link_style_gwm-link-footer__3OF47")
        }

        // Instruction 298: CheckAndSetBit { selector: Class("_cropped-image-link_style_haulRibbon__3VZNi"), bit_pos: 298 }
        if node_has_class_id(node, 149) {
            intrinsic_matches.set_bit(298); // match_Class("_cropped-image-link_style_haulRibbon__3VZNi")
        }

        // Instruction 300: CheckAndSetBit { selector: Class("_cropped-image-link_style_header-icon__2cuVV"), bit_pos: 300 }
        if node_has_class_id(node, 150) {
            intrinsic_matches.set_bit(300); // match_Class("_cropped-image-link_style_header-icon__2cuVV")
        }

        // Instruction 302: CheckAndSetBit { selector: Class("_cropped-image-link_style_header-link__cUhOK"), bit_pos: 302 }
        if node_has_class_id(node, 151) {
            intrinsic_matches.set_bit(302); // match_Class("_cropped-image-link_style_header-link__cUhOK")
        }

        // Instruction 304: CheckAndSetBit { selector: Class("_cropped-image-link_style_header__1vGdj"), bit_pos: 304 }
        if node_has_class_id(node, 152) {
            intrinsic_matches.set_bit(304); // match_Class("_cropped-image-link_style_header__1vGdj")
        }

        // Instruction 306: CheckAndSetBit { selector: Class("_cropped-image-link_style_image-container__2OiZA"), bit_pos: 306 }
        if node_has_class_id(node, 153) {
            intrinsic_matches.set_bit(306); // match_Class("_cropped-image-link_style_image-container__2OiZA")
        }

        // Instruction 308: CheckAndSetBit { selector: Class("_cropped-image-link_style_logoGap__nKNZ9"), bit_pos: 308 }
        if node_has_class_id(node, 154) {
            intrinsic_matches.set_bit(308); // match_Class("_cropped-image-link_style_logoGap__nKNZ9")
        }

        // Instruction 310: CheckAndSetBit { selector: Class("_cropped-image-link_style_logoRectangle__1VJwu"), bit_pos: 310 }
        if node_has_class_id(node, 155) {
            intrinsic_matches.set_bit(310); // match_Class("_cropped-image-link_style_logoRectangle__1VJwu")
        }

        // Instruction 312: CheckAndSetBit { selector: Class("_cropped-image-link_style_logoSquareContainer__3Paoc"), bit_pos: 312 }
        if node_has_class_id(node, 156) {
            intrinsic_matches.set_bit(312); // match_Class("_cropped-image-link_style_logoSquareContainer__3Paoc")
        }

        // Instruction 314: CheckAndSetBit { selector: Class("_cropped-image-link_style_logoSquare__3NZyi"), bit_pos: 314 }
        if node_has_class_id(node, 157) {
            intrinsic_matches.set_bit(314); // match_Class("_cropped-image-link_style_logoSquare__3NZyi")
        }

        // Instruction 316: CheckAndSetBit { selector: Class("_cropped-image-link_style_logo__2ZQ-N"), bit_pos: 316 }
        if node_has_class_id(node, 158) {
            intrinsic_matches.set_bit(316); // match_Class("_cropped-image-link_style_logo__2ZQ-N")
        }

        // Instruction 318: CheckAndSetBit { selector: Class("_cropped-image-link_style_mixed-button__2og-m"), bit_pos: 318 }
        if node_has_class_id(node, 159) {
            intrinsic_matches.set_bit(318); // match_Class("_cropped-image-link_style_mixed-button__2og-m")
        }

        // Instruction 320: CheckAndSetBit { selector: Class("_cropped-image-link_style_mobile-close-button__3PB07"), bit_pos: 320 }
        if node_has_class_id(node, 160) {
            intrinsic_matches.set_bit(320); // match_Class("_cropped-image-link_style_mobile-close-button__3PB07")
        }

        // Instruction 322: CheckAndSetBit { selector: Class("_cropped-image-link_style_mosaic-card-body__1HmTs"), bit_pos: 322 }
        if node_has_class_id(node, 161) {
            intrinsic_matches.set_bit(322); // match_Class("_cropped-image-link_style_mosaic-card-body__1HmTs")
        }

        // Instruction 324: CheckAndSetBit { selector: Class("_cropped-image-link_style_mosaic-card__1C-_R"), bit_pos: 324 }
        if node_has_class_id(node, 162) {
            intrinsic_matches.set_bit(324); // match_Class("_cropped-image-link_style_mosaic-card__1C-_R")
        }

        // Instruction 326: CheckAndSetBit { selector: Class("_cropped-image-link_style_negative-button__1Dvqz"), bit_pos: 326 }
        if node_has_class_id(node, 163) {
            intrinsic_matches.set_bit(326); // match_Class("_cropped-image-link_style_negative-button__1Dvqz")
        }

        // Instruction 328: CheckAndSetBit { selector: Class("_cropped-image-link_style_negativeMarginAdjust__1nqu9"), bit_pos: 328 }
        if node_has_class_id(node, 164) {
            intrinsic_matches.set_bit(328); // match_Class("_cropped-image-link_style_negativeMarginAdjust__1nqu9")
        }

        // Instruction 330: CheckAndSetBit { selector: Class("_cropped-image-link_style_oneLineTruncation__2WWse"), bit_pos: 330 }
        if node_has_class_id(node, 165) {
            intrinsic_matches.set_bit(330); // match_Class("_cropped-image-link_style_oneLineTruncation__2WWse")
        }

        // Instruction 332: CheckAndSetBit { selector: Class("_cropped-image-link_style_overlay__3Sx3u"), bit_pos: 332 }
        if node_has_class_id(node, 166) {
            intrinsic_matches.set_bit(332); // match_Class("_cropped-image-link_style_overlay__3Sx3u")
        }

        // Instruction 334: CheckAndSetBit { selector: Class("_cropped-image-link_style_positive-button__3UOC3"), bit_pos: 334 }
        if node_has_class_id(node, 167) {
            intrinsic_matches.set_bit(334); // match_Class("_cropped-image-link_style_positive-button__3UOC3")
        }

        // Instruction 336: CheckAndSetBit { selector: Class("_cropped-image-link_style_poster-image__1W0yA"), bit_pos: 336 }
        if node_has_class_id(node, 168) {
            intrinsic_matches.set_bit(336); // match_Class("_cropped-image-link_style_poster-image__1W0yA")
        }

        // Instruction 338: CheckAndSetBit { selector: Class("_cropped-image-link_style_smartText__ubpEw"), bit_pos: 338 }
        if node_has_class_id(node, 169) {
            intrinsic_matches.set_bit(338); // match_Class("_cropped-image-link_style_smartText__ubpEw")
        }

        // Instruction 340: CheckAndSetBit { selector: Class("_cropped-image-link_style_spacer__7Pyg3"), bit_pos: 340 }
        if node_has_class_id(node, 170) {
            intrinsic_matches.set_bit(340); // match_Class("_cropped-image-link_style_spacer__7Pyg3")
        }

        // Instruction 342: CheckAndSetBit { selector: Class("_cropped-image-link_style_stacking-context__3PbQE"), bit_pos: 342 }
        if node_has_class_id(node, 171) {
            intrinsic_matches.set_bit(342); // match_Class("_cropped-image-link_style_stacking-context__3PbQE")
        }

        // Instruction 344: CheckAndSetBit { selector: Class("_cropped-image-link_style_theming-background-override__1HfzJ"), bit_pos: 344 }
        if node_has_class_id(node, 172) {
            intrinsic_matches.set_bit(344); // match_Class("_cropped-image-link_style_theming-background-override__1HfzJ")
        }

        // Instruction 346: CheckAndSetBit { selector: Class("_cropped-image-link_style_themingTextColorWhite__1zryO"), bit_pos: 346 }
        if node_has_class_id(node, 173) {
            intrinsic_matches.set_bit(346); // match_Class("_cropped-image-link_style_themingTextColorWhite__1zryO")
        }

        // Instruction 348: CheckAndSetBit { selector: Class("_cropped-image-link_style_themingTextColor__1oQsI"), bit_pos: 348 }
        if node_has_class_id(node, 174) {
            intrinsic_matches.set_bit(348); // match_Class("_cropped-image-link_style_themingTextColor__1oQsI")
        }

        // Instruction 350: CheckAndSetBit { selector: Class("_cropped-image-link_style_three-pack__5s3hP"), bit_pos: 350 }
        if node_has_class_id(node, 175) {
            intrinsic_matches.set_bit(350); // match_Class("_cropped-image-link_style_three-pack__5s3hP")
        }

        // Instruction 352: CheckAndSetBit { selector: Class("_cropped-image-link_style_threeLineTruncation__UkUjj"), bit_pos: 352 }
        if node_has_class_id(node, 176) {
            intrinsic_matches.set_bit(352); // match_Class("_cropped-image-link_style_threeLineTruncation__UkUjj")
        }

        // Instruction 354: CheckAndSetBit { selector: Class("_cropped-image-link_style_tile-container__1QgAV"), bit_pos: 354 }
        if node_has_class_id(node, 177) {
            intrinsic_matches.set_bit(354); // match_Class("_cropped-image-link_style_tile-container__1QgAV")
        }

        // Instruction 356: CheckAndSetBit { selector: Class("_cropped-image-link_style_tile-grid__QMxNY"), bit_pos: 356 }
        if node_has_class_id(node, 178) {
            intrinsic_matches.set_bit(356); // match_Class("_cropped-image-link_style_tile-grid__QMxNY")
        }

        // Instruction 358: CheckAndSetBit { selector: Class("_cropped-image-link_style_tile-link__38lTa"), bit_pos: 358 }
        if node_has_class_id(node, 179) {
            intrinsic_matches.set_bit(358); // match_Class("_cropped-image-link_style_tile-link__38lTa")
        }

        // Instruction 360: CheckAndSetBit { selector: Class("_cropped-image-link_style_tile-theming__3eeyj"), bit_pos: 360 }
        if node_has_class_id(node, 180) {
            intrinsic_matches.set_bit(360); // match_Class("_cropped-image-link_style_tile-theming__3eeyj")
        }

        // Instruction 362: CheckAndSetBit { selector: Class("_cropped-image-link_style_truncation__x9-69"), bit_pos: 362 }
        if node_has_class_id(node, 181) {
            intrinsic_matches.set_bit(362); // match_Class("_cropped-image-link_style_truncation__x9-69")
        }

        // Instruction 364: CheckAndSetBit { selector: Class("_cropped-image-link_style_twoLineTruncation__16TLV"), bit_pos: 364 }
        if node_has_class_id(node, 182) {
            intrinsic_matches.set_bit(364); // match_Class("_cropped-image-link_style_twoLineTruncation__16TLV")
        }

        // Instruction 366: CheckAndSetBit { selector: Class("_cropped-image-link_style_video-container__1hKS1"), bit_pos: 366 }
        if node_has_class_id(node, 183) {
            intrinsic_matches.set_bit(366); // match_Class("_cropped-image-link_style_video-container__1hKS1")
        }

        // Instruction 368: CheckAndSetBit { selector: Class("_cropped-image-link_style_wd-backdrop-data__1znxG"), bit_pos: 368 }
        if node_has_class_id(node, 184) {
            intrinsic_matches.set_bit(368); // match_Class("_cropped-image-link_style_wd-backdrop-data__1znxG")
        }

        // Instruction 370: CheckAndSetBit { selector: Class("_cropped-image-link_style_wdHeader__Edrev"), bit_pos: 370 }
        if node_has_class_id(node, 185) {
            intrinsic_matches.set_bit(370); // match_Class("_cropped-image-link_style_wdHeader__Edrev")
        }

        // Instruction 372: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_bodyFooterStyle_cardBody__1YuQY"), bit_pos: 372 }
        if node_has_class_id(node, 186) {
            intrinsic_matches.set_bit(372); // match_Class("_fluid-fat-image-link-v2_bodyFooterStyle_cardBody__1YuQY")
        }

        // Instruction 374: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2"), bit_pos: 374 }
        if node_has_class_id(node, 187) {
            intrinsic_matches.set_bit(374); // match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
        }

        // Instruction 376: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3"), bit_pos: 376 }
        if node_has_class_id(node, 188) {
            intrinsic_matches.set_bit(376); // match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
        }

        // Instruction 378: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P"), bit_pos: 378 }
        if node_has_class_id(node, 189) {
            intrinsic_matches.set_bit(378); // match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
        }

        // Instruction 380: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK"), bit_pos: 380 }
        if node_has_class_id(node, 190) {
            intrinsic_matches.set_bit(380); // match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
        }

        // Instruction 382: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN"), bit_pos: 382 }
        if node_has_class_id(node, 191) {
            intrinsic_matches.set_bit(382); // match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
        }

        // Instruction 384: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY"), bit_pos: 384 }
        if node_has_class_id(node, 192) {
            intrinsic_matches.set_bit(384); // match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
        }

        // Instruction 386: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8"), bit_pos: 386 }
        if node_has_class_id(node, 193) {
            intrinsic_matches.set_bit(386); // match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
        }

        // Instruction 388: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-container__1Pkva"), bit_pos: 388 }
        if node_has_class_id(node, 194) {
            intrinsic_matches.set_bit(388); // match_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-container__1Pkva")
        }

        // Instruction 390: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_image_asin-container-white-box__QwmgO"), bit_pos: 390 }
        if node_has_class_id(node, 195) {
            intrinsic_matches.set_bit(390); // match_Class("_fluid-fat-image-link-v2_image_asin-container-white-box__QwmgO")
        }

        // Instruction 392: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_image_asin-container__2jyCM"), bit_pos: 392 }
        if node_has_class_id(node, 196) {
            intrinsic_matches.set_bit(392); // match_Class("_fluid-fat-image-link-v2_image_asin-container__2jyCM")
        }

        // Instruction 394: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_image_round-corners__2y_fS"), bit_pos: 394 }
        if node_has_class_id(node, 197) {
            intrinsic_matches.set_bit(394); // match_Class("_fluid-fat-image-link-v2_image_round-corners__2y_fS")
        }

        // Instruction 396: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_singleLinkStyle_bodyFooterLink__9LvH0"), bit_pos: 396 }
        if node_has_class_id(node, 198) {
            intrinsic_matches.set_bit(396); // match_Class("_fluid-fat-image-link-v2_singleLinkStyle_bodyFooterLink__9LvH0")
        }

        // Instruction 398: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_singleLinkStyle_footer__2cH0y"), bit_pos: 398 }
        if node_has_class_id(node, 199) {
            intrinsic_matches.set_bit(398); // match_Class("_fluid-fat-image-link-v2_singleLinkStyle_footer__2cH0y")
        }

        // Instruction 400: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY"), bit_pos: 400 }
        if node_has_class_id(node, 200) {
            intrinsic_matches.set_bit(400); // match_Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
        }

        // Instruction 402: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner__1nmZw"), bit_pos: 402 }
        if node_has_class_id(node, 201) {
            intrinsic_matches.set_bit(402); // match_Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner__1nmZw")
        }

        // Instruction 404: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-primary-link__2bIZi"), bit_pos: 404 }
        if node_has_class_id(node, 202) {
            intrinsic_matches.set_bit(404); // match_Class("_fluid-fat-image-link-v2_style_ad-feedback-primary-link__2bIZi")
        }

        // Instruction 406: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite-mobile__2_rj8"), bit_pos: 406 }
        if node_has_class_id(node, 203) {
            intrinsic_matches.set_bit(406); // match_Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite-mobile__2_rj8")
        }

        // Instruction 408: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite__28uwB"), bit_pos: 408 }
        if node_has_class_id(node, 204) {
            intrinsic_matches.set_bit(408); // match_Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite__28uwB")
        }

        // Instruction 410: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-text-desktop__q3xp_"), bit_pos: 410 }
        if node_has_class_id(node, 205) {
            intrinsic_matches.set_bit(410); // match_Class("_fluid-fat-image-link-v2_style_ad-feedback-text-desktop__q3xp_")
        }

        // Instruction 412: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_ad-feedback-text__2HjQ9"), bit_pos: 412 }
        if node_has_class_id(node, 206) {
            intrinsic_matches.set_bit(412); // match_Class("_fluid-fat-image-link-v2_style_ad-feedback-text__2HjQ9")
        }

        // Instruction 414: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_apexBadgeLabel__2-Vye"), bit_pos: 414 }
        if node_has_class_id(node, 207) {
            intrinsic_matches.set_bit(414); // match_Class("_fluid-fat-image-link-v2_style_apexBadgeLabel__2-Vye")
        }

        // Instruction 416: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_apexBadgeMessage__1tHvd"), bit_pos: 416 }
        if node_has_class_id(node, 208) {
            intrinsic_matches.set_bit(416); // match_Class("_fluid-fat-image-link-v2_style_apexBadgeMessage__1tHvd")
        }

        // Instruction 418: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-button-group__1LqUG"), bit_pos: 418 }
        if node_has_class_id(node, 209) {
            intrinsic_matches.set_bit(418); // match_Class("_fluid-fat-image-link-v2_style_aspect-button-group__1LqUG")
        }

        // Instruction 420: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-button__7cH_E"), bit_pos: 420 }
        if node_has_class_id(node, 210) {
            intrinsic_matches.set_bit(420); // match_Class("_fluid-fat-image-link-v2_style_aspect-button__7cH_E")
        }

        // Instruction 422: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-1236x1080__3aEzl"), bit_pos: 422 }
        if node_has_class_id(node, 211) {
            intrinsic_matches.set_bit(422); // match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-1236x1080__3aEzl")
        }

        // Instruction 424: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-15x3__1h649"), bit_pos: 424 }
        if node_has_class_id(node, 212) {
            intrinsic_matches.set_bit(424); // match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-15x3__1h649")
        }

        // Instruction 426: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-16x9__cBPv8"), bit_pos: 426 }
        if node_has_class_id(node, 213) {
            intrinsic_matches.set_bit(426); // match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-16x9__cBPv8")
        }

        // Instruction 428: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-4x3__3BewI"), bit_pos: 428 }
        if node_has_class_id(node, 214) {
            intrinsic_matches.set_bit(428); // match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-4x3__3BewI")
        }

        // Instruction 430: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-5x8__2IaNz"), bit_pos: 430 }
        if node_has_class_id(node, 215) {
            intrinsic_matches.set_bit(430); // match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-5x8__2IaNz")
        }

        // Instruction 432: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-dynamic-60vh__3N5g_"), bit_pos: 432 }
        if node_has_class_id(node, 216) {
            intrinsic_matches.set_bit(432); // match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
        }

        // Instruction 434: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-ratio-fill__2Zjfb"), bit_pos: 434 }
        if node_has_class_id(node, 217) {
            intrinsic_matches.set_bit(434); // match_Class("_fluid-fat-image-link-v2_style_aspect-ratio-fill__2Zjfb")
        }

        // Instruction 436: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_aspect-text__S4PU1"), bit_pos: 436 }
        if node_has_class_id(node, 218) {
            intrinsic_matches.set_bit(436); // match_Class("_fluid-fat-image-link-v2_style_aspect-text__S4PU1")
        }

        // Instruction 438: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_autoplay-span__2CMfc"), bit_pos: 438 }
        if node_has_class_id(node, 219) {
            intrinsic_matches.set_bit(438); // match_Class("_fluid-fat-image-link-v2_style_autoplay-span__2CMfc")
        }

        // Instruction 440: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_badge-container__20aJ2"), bit_pos: 440 }
        if node_has_class_id(node, 220) {
            intrinsic_matches.set_bit(440); // match_Class("_fluid-fat-image-link-v2_style_badge-container__20aJ2")
        }

        // Instruction 442: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_badgeLabel__pJ5rc"), bit_pos: 442 }
        if node_has_class_id(node, 221) {
            intrinsic_matches.set_bit(442); // match_Class("_fluid-fat-image-link-v2_style_badgeLabel__pJ5rc")
        }

        // Instruction 444: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_badgeMessage__2Dtw7"), bit_pos: 444 }
        if node_has_class_id(node, 222) {
            intrinsic_matches.set_bit(444); // match_Class("_fluid-fat-image-link-v2_style_badgeMessage__2Dtw7")
        }

        // Instruction 446: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_carouselContainer__3N7M1"), bit_pos: 446 }
        if node_has_class_id(node, 223) {
            intrinsic_matches.set_bit(446); // match_Class("_fluid-fat-image-link-v2_style_carouselContainer__3N7M1")
        }

        // Instruction 448: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_centerImage__30wh-"), bit_pos: 448 }
        if node_has_class_id(node, 224) {
            intrinsic_matches.set_bit(448); // match_Class("_fluid-fat-image-link-v2_style_centerImage__30wh-")
        }

        // Instruction 450: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_close-black-icon__3hkbe"), bit_pos: 450 }
        if node_has_class_id(node, 225) {
            intrinsic_matches.set_bit(450); // match_Class("_fluid-fat-image-link-v2_style_close-black-icon__3hkbe")
        }

        // Instruction 452: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_close-icon-wrapper__1zvdC"), bit_pos: 452 }
        if node_has_class_id(node, 226) {
            intrinsic_matches.set_bit(452); // match_Class("_fluid-fat-image-link-v2_style_close-icon-wrapper__1zvdC")
        }

        // Instruction 454: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_close-icon__2RJs3"), bit_pos: 454 }
        if node_has_class_id(node, 227) {
            intrinsic_matches.set_bit(454); // match_Class("_fluid-fat-image-link-v2_style_close-icon__2RJs3")
        }

        // Instruction 456: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_close-text__2-gwn"), bit_pos: 456 }
        if node_has_class_id(node, 228) {
            intrinsic_matches.set_bit(456); // match_Class("_fluid-fat-image-link-v2_style_close-text__2-gwn")
        }

        // Instruction 458: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_cover-portrait-image__2lhzL"), bit_pos: 458 }
        if node_has_class_id(node, 229) {
            intrinsic_matches.set_bit(458); // match_Class("_fluid-fat-image-link-v2_style_cover-portrait-image__2lhzL")
        }

        // Instruction 460: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_cta-link__2xo74"), bit_pos: 460 }
        if node_has_class_id(node, 230) {
            intrinsic_matches.set_bit(460); // match_Class("_fluid-fat-image-link-v2_style_cta-link__2xo74")
        }

        // Instruction 462: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_desktop-close-button__1iL_P"), bit_pos: 462 }
        if node_has_class_id(node, 231) {
            intrinsic_matches.set_bit(462); // match_Class("_fluid-fat-image-link-v2_style_desktop-close-button__1iL_P")
        }

        // Instruction 464: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_displayCount__1MVut"), bit_pos: 464 }
        if node_has_class_id(node, 232) {
            intrinsic_matches.set_bit(464); // match_Class("_fluid-fat-image-link-v2_style_displayCount__1MVut")
        }

        // Instruction 466: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_dynamic-portrait-image__1Wrzd"), bit_pos: 466 }
        if node_has_class_id(node, 233) {
            intrinsic_matches.set_bit(466); // match_Class("_fluid-fat-image-link-v2_style_dynamic-portrait-image__1Wrzd")
        }

        // Instruction 468: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_empty-footer__2d59h"), bit_pos: 468 }
        if node_has_class_id(node, 234) {
            intrinsic_matches.set_bit(468); // match_Class("_fluid-fat-image-link-v2_style_empty-footer__2d59h")
        }

        // Instruction 470: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_five-pack__1-Tql"), bit_pos: 470 }
        if node_has_class_id(node, 235) {
            intrinsic_matches.set_bit(470); // match_Class("_fluid-fat-image-link-v2_style_five-pack__1-Tql")
        }

        // Instruction 472: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluid-landscape-image__TE6PT"), bit_pos: 472 }
        if node_has_class_id(node, 236) {
            intrinsic_matches.set_bit(472); // match_Class("_fluid-fat-image-link-v2_style_fluid-landscape-image__TE6PT")
        }

        // Instruction 474: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidFatImageLinkBody__1LsOX"), bit_pos: 474 }
        if node_has_class_id(node, 237) {
            intrinsic_matches.set_bit(474); // match_Class("_fluid-fat-image-link-v2_style_fluidFatImageLinkBody__1LsOX")
        }

        // Instruction 476: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidFatImageLink__1nw4J"), bit_pos: 476 }
        if node_has_class_id(node, 238) {
            intrinsic_matches.set_bit(476); // match_Class("_fluid-fat-image-link-v2_style_fluidFatImageLink__1nw4J")
        }

        // Instruction 478: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2SOMr"), bit_pos: 478 }
        if node_has_class_id(node, 239) {
            intrinsic_matches.set_bit(478); // match_Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2SOMr")
        }

        // Instruction 480: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2vGwp"), bit_pos: 480 }
        if node_has_class_id(node, 240) {
            intrinsic_matches.set_bit(480); // match_Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2vGwp")
        }

        // Instruction 482: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidLandscapeImage__2euAK"), bit_pos: 482 }
        if node_has_class_id(node, 241) {
            intrinsic_matches.set_bit(482); // match_Class("_fluid-fat-image-link-v2_style_fluidLandscapeImage__2euAK")
        }

        // Instruction 484: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_fluidPortraitImage__2SAYm"), bit_pos: 484 }
        if node_has_class_id(node, 242) {
            intrinsic_matches.set_bit(484); // match_Class("_fluid-fat-image-link-v2_style_fluidPortraitImage__2SAYm")
        }

        // Instruction 486: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_four-pack__1ufgr"), bit_pos: 486 }
        if node_has_class_id(node, 243) {
            intrinsic_matches.set_bit(486); // match_Class("_fluid-fat-image-link-v2_style_four-pack__1ufgr")
        }

        // Instruction 488: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_gw-hero-close-button__3svyZ"), bit_pos: 488 }
        if node_has_class_id(node, 244) {
            intrinsic_matches.set_bit(488); // match_Class("_fluid-fat-image-link-v2_style_gw-hero-close-button__3svyZ")
        }

        // Instruction 490: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_gwm-link-footer__3OF47"), bit_pos: 490 }
        if node_has_class_id(node, 245) {
            intrinsic_matches.set_bit(490); // match_Class("_fluid-fat-image-link-v2_style_gwm-link-footer__3OF47")
        }

        // Instruction 492: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_haulRibbon__3VZNi"), bit_pos: 492 }
        if node_has_class_id(node, 246) {
            intrinsic_matches.set_bit(492); // match_Class("_fluid-fat-image-link-v2_style_haulRibbon__3VZNi")
        }

        // Instruction 494: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_header-icon__2cuVV"), bit_pos: 494 }
        if node_has_class_id(node, 247) {
            intrinsic_matches.set_bit(494); // match_Class("_fluid-fat-image-link-v2_style_header-icon__2cuVV")
        }

        // Instruction 496: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_header-link__cUhOK"), bit_pos: 496 }
        if node_has_class_id(node, 248) {
            intrinsic_matches.set_bit(496); // match_Class("_fluid-fat-image-link-v2_style_header-link__cUhOK")
        }

        // Instruction 498: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_header__1vGdj"), bit_pos: 498 }
        if node_has_class_id(node, 249) {
            intrinsic_matches.set_bit(498); // match_Class("_fluid-fat-image-link-v2_style_header__1vGdj")
        }

        // Instruction 500: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_image-container__2OiZA"), bit_pos: 500 }
        if node_has_class_id(node, 250) {
            intrinsic_matches.set_bit(500); // match_Class("_fluid-fat-image-link-v2_style_image-container__2OiZA")
        }

        // Instruction 502: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_imageLabel__3ANSV"), bit_pos: 502 }
        if node_has_class_id(node, 251) {
            intrinsic_matches.set_bit(502); // match_Class("_fluid-fat-image-link-v2_style_imageLabel__3ANSV")
        }

        // Instruction 504: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_inlineErrorDetails__1NBx-"), bit_pos: 504 }
        if node_has_class_id(node, 252) {
            intrinsic_matches.set_bit(504); // match_Class("_fluid-fat-image-link-v2_style_inlineErrorDetails__1NBx-")
        }

        // Instruction 506: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logoGap__nKNZ9"), bit_pos: 506 }
        if node_has_class_id(node, 253) {
            intrinsic_matches.set_bit(506); // match_Class("_fluid-fat-image-link-v2_style_logoGap__nKNZ9")
        }

        // Instruction 508: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logoRectangle__1VJwu"), bit_pos: 508 }
        if node_has_class_id(node, 254) {
            intrinsic_matches.set_bit(508); // match_Class("_fluid-fat-image-link-v2_style_logoRectangle__1VJwu")
        }

        // Instruction 510: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logoSquareContainer__3Paoc"), bit_pos: 510 }
        if node_has_class_id(node, 255) {
            intrinsic_matches.set_bit(510); // match_Class("_fluid-fat-image-link-v2_style_logoSquareContainer__3Paoc")
        }

        // Instruction 512: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logoSquare__3NZyi"), bit_pos: 512 }
        if node_has_class_id(node, 256) {
            intrinsic_matches.set_bit(512); // match_Class("_fluid-fat-image-link-v2_style_logoSquare__3NZyi")
        }

        // Instruction 514: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_logo__2ZQ-N"), bit_pos: 514 }
        if node_has_class_id(node, 257) {
            intrinsic_matches.set_bit(514); // match_Class("_fluid-fat-image-link-v2_style_logo__2ZQ-N")
        }

        // Instruction 516: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mergedLinksCta__3Npog"), bit_pos: 516 }
        if node_has_class_id(node, 258) {
            intrinsic_matches.set_bit(516); // match_Class("_fluid-fat-image-link-v2_style_mergedLinksCta__3Npog")
        }

        // Instruction 518: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mergedLinks__10JqZ"), bit_pos: 518 }
        if node_has_class_id(node, 259) {
            intrinsic_matches.set_bit(518); // match_Class("_fluid-fat-image-link-v2_style_mergedLinks__10JqZ")
        }

        // Instruction 520: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mixed-button__2og-m"), bit_pos: 520 }
        if node_has_class_id(node, 260) {
            intrinsic_matches.set_bit(520); // match_Class("_fluid-fat-image-link-v2_style_mixed-button__2og-m")
        }

        // Instruction 522: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mobile-close-button__3PB07"), bit_pos: 522 }
        if node_has_class_id(node, 261) {
            intrinsic_matches.set_bit(522); // match_Class("_fluid-fat-image-link-v2_style_mobile-close-button__3PB07")
        }

        // Instruction 524: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mosaic-card-body__1HmTs"), bit_pos: 524 }
        if node_has_class_id(node, 262) {
            intrinsic_matches.set_bit(524); // match_Class("_fluid-fat-image-link-v2_style_mosaic-card-body__1HmTs")
        }

        // Instruction 526: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_mosaic-card__1C-_R"), bit_pos: 526 }
        if node_has_class_id(node, 263) {
            intrinsic_matches.set_bit(526); // match_Class("_fluid-fat-image-link-v2_style_mosaic-card__1C-_R")
        }

        // Instruction 528: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_negative-button__1Dvqz"), bit_pos: 528 }
        if node_has_class_id(node, 264) {
            intrinsic_matches.set_bit(528); // match_Class("_fluid-fat-image-link-v2_style_negative-button__1Dvqz")
        }

        // Instruction 530: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_negativeMarginAdjust__1nqu9"), bit_pos: 530 }
        if node_has_class_id(node, 265) {
            intrinsic_matches.set_bit(530); // match_Class("_fluid-fat-image-link-v2_style_negativeMarginAdjust__1nqu9")
        }

        // Instruction 532: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_oneLineTruncation__2WWse"), bit_pos: 532 }
        if node_has_class_id(node, 266) {
            intrinsic_matches.set_bit(532); // match_Class("_fluid-fat-image-link-v2_style_oneLineTruncation__2WWse")
        }

        // Instruction 534: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_overlay__3Sx3u"), bit_pos: 534 }
        if node_has_class_id(node, 267) {
            intrinsic_matches.set_bit(534); // match_Class("_fluid-fat-image-link-v2_style_overlay__3Sx3u")
        }

        // Instruction 536: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_positive-button__3UOC3"), bit_pos: 536 }
        if node_has_class_id(node, 268) {
            intrinsic_matches.set_bit(536); // match_Class("_fluid-fat-image-link-v2_style_positive-button__3UOC3")
        }

        // Instruction 538: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_poster-image__1W0yA"), bit_pos: 538 }
        if node_has_class_id(node, 269) {
            intrinsic_matches.set_bit(538); // match_Class("_fluid-fat-image-link-v2_style_poster-image__1W0yA")
        }

        // Instruction 540: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_smartText__ubpEw"), bit_pos: 540 }
        if node_has_class_id(node, 270) {
            intrinsic_matches.set_bit(540); // match_Class("_fluid-fat-image-link-v2_style_smartText__ubpEw")
        }

        // Instruction 542: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_spCSRFTreatment__-hwVO"), bit_pos: 542 }
        if node_has_class_id(node, 271) {
            intrinsic_matches.set_bit(542); // match_Class("_fluid-fat-image-link-v2_style_spCSRFTreatment__-hwVO")
        }

        // Instruction 544: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_spacer__7Pyg3"), bit_pos: 544 }
        if node_has_class_id(node, 272) {
            intrinsic_matches.set_bit(544); // match_Class("_fluid-fat-image-link-v2_style_spacer__7Pyg3")
        }

        // Instruction 546: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_stacking-context__3PbQE"), bit_pos: 546 }
        if node_has_class_id(node, 273) {
            intrinsic_matches.set_bit(546); // match_Class("_fluid-fat-image-link-v2_style_stacking-context__3PbQE")
        }

        // Instruction 548: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_theming-background-override__1HfzJ"), bit_pos: 548 }
        if node_has_class_id(node, 274) {
            intrinsic_matches.set_bit(548); // match_Class("_fluid-fat-image-link-v2_style_theming-background-override__1HfzJ")
        }

        // Instruction 550: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_themingTextColorWhite__1zryO"), bit_pos: 550 }
        if node_has_class_id(node, 275) {
            intrinsic_matches.set_bit(550); // match_Class("_fluid-fat-image-link-v2_style_themingTextColorWhite__1zryO")
        }

        // Instruction 552: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_themingTextColor__1oQsI"), bit_pos: 552 }
        if node_has_class_id(node, 276) {
            intrinsic_matches.set_bit(552); // match_Class("_fluid-fat-image-link-v2_style_themingTextColor__1oQsI")
        }

        // Instruction 554: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_three-pack__5s3hP"), bit_pos: 554 }
        if node_has_class_id(node, 277) {
            intrinsic_matches.set_bit(554); // match_Class("_fluid-fat-image-link-v2_style_three-pack__5s3hP")
        }

        // Instruction 556: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_threeLineTruncation__UkUjj"), bit_pos: 556 }
        if node_has_class_id(node, 278) {
            intrinsic_matches.set_bit(556); // match_Class("_fluid-fat-image-link-v2_style_threeLineTruncation__UkUjj")
        }

        // Instruction 558: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_tile-container__1QgAV"), bit_pos: 558 }
        if node_has_class_id(node, 279) {
            intrinsic_matches.set_bit(558); // match_Class("_fluid-fat-image-link-v2_style_tile-container__1QgAV")
        }

        // Instruction 560: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_tile-grid__QMxNY"), bit_pos: 560 }
        if node_has_class_id(node, 280) {
            intrinsic_matches.set_bit(560); // match_Class("_fluid-fat-image-link-v2_style_tile-grid__QMxNY")
        }

        // Instruction 562: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_tile-link__38lTa"), bit_pos: 562 }
        if node_has_class_id(node, 281) {
            intrinsic_matches.set_bit(562); // match_Class("_fluid-fat-image-link-v2_style_tile-link__38lTa")
        }

        // Instruction 564: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_tile-theming__3eeyj"), bit_pos: 564 }
        if node_has_class_id(node, 282) {
            intrinsic_matches.set_bit(564); // match_Class("_fluid-fat-image-link-v2_style_tile-theming__3eeyj")
        }

        // Instruction 566: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_truncation__x9-69"), bit_pos: 566 }
        if node_has_class_id(node, 283) {
            intrinsic_matches.set_bit(566); // match_Class("_fluid-fat-image-link-v2_style_truncation__x9-69")
        }

        // Instruction 568: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_twoLineTruncation__16TLV"), bit_pos: 568 }
        if node_has_class_id(node, 284) {
            intrinsic_matches.set_bit(568); // match_Class("_fluid-fat-image-link-v2_style_twoLineTruncation__16TLV")
        }

        // Instruction 570: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_video-container__1hKS1"), bit_pos: 570 }
        if node_has_class_id(node, 285) {
            intrinsic_matches.set_bit(570); // match_Class("_fluid-fat-image-link-v2_style_video-container__1hKS1")
        }

        // Instruction 572: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_wd-backdrop-data__1znxG"), bit_pos: 572 }
        if node_has_class_id(node, 286) {
            intrinsic_matches.set_bit(572); // match_Class("_fluid-fat-image-link-v2_style_wd-backdrop-data__1znxG")
        }

        // Instruction 574: CheckAndSetBit { selector: Class("_fluid-fat-image-link-v2_style_wdHeader__Edrev"), bit_pos: 574 }
        if node_has_class_id(node, 287) {
            intrinsic_matches.set_bit(574); // match_Class("_fluid-fat-image-link-v2_style_wdHeader__Edrev")
        }

        // Instruction 576: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2"), bit_pos: 576 }
        if node_has_class_id(node, 288) {
            intrinsic_matches.set_bit(576); // match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
        }

        // Instruction 578: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3"), bit_pos: 578 }
        if node_has_class_id(node, 289) {
            intrinsic_matches.set_bit(578); // match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
        }

        // Instruction 580: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P"), bit_pos: 580 }
        if node_has_class_id(node, 290) {
            intrinsic_matches.set_bit(580); // match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
        }

        // Instruction 582: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK"), bit_pos: 582 }
        if node_has_class_id(node, 291) {
            intrinsic_matches.set_bit(582); // match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
        }

        // Instruction 584: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN"), bit_pos: 584 }
        if node_has_class_id(node, 292) {
            intrinsic_matches.set_bit(584); // match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
        }

        // Instruction 586: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY"), bit_pos: 586 }
        if node_has_class_id(node, 293) {
            intrinsic_matches.set_bit(586); // match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
        }

        // Instruction 588: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8"), bit_pos: 588 }
        if node_has_class_id(node, 294) {
            intrinsic_matches.set_bit(588); // match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
        }

        // Instruction 590: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-container__1Pkva"), bit_pos: 590 }
        if node_has_class_id(node, 295) {
            intrinsic_matches.set_bit(590); // match_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-container__1Pkva")
        }

        // Instruction 592: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_image_asin-container-white-box__QwmgO"), bit_pos: 592 }
        if node_has_class_id(node, 296) {
            intrinsic_matches.set_bit(592); // match_Class("_fluid-quad-image-label-v2_image_asin-container-white-box__QwmgO")
        }

        // Instruction 594: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_image_asin-container__2jyCM"), bit_pos: 594 }
        if node_has_class_id(node, 297) {
            intrinsic_matches.set_bit(594); // match_Class("_fluid-quad-image-label-v2_image_asin-container__2jyCM")
        }

        // Instruction 596: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_image_round-corners__2y_fS"), bit_pos: 596 }
        if node_has_class_id(node, 298) {
            intrinsic_matches.set_bit(596); // match_Class("_fluid-quad-image-label-v2_image_round-corners__2y_fS")
        }

        // Instruction 598: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY"), bit_pos: 598 }
        if node_has_class_id(node, 299) {
            intrinsic_matches.set_bit(598); // match_Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
        }

        // Instruction 600: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner__1nmZw"), bit_pos: 600 }
        if node_has_class_id(node, 300) {
            intrinsic_matches.set_bit(600); // match_Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner__1nmZw")
        }

        // Instruction 602: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-primary-link__2bIZi"), bit_pos: 602 }
        if node_has_class_id(node, 301) {
            intrinsic_matches.set_bit(602); // match_Class("_fluid-quad-image-label-v2_style_ad-feedback-primary-link__2bIZi")
        }

        // Instruction 604: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite-mobile__2_rj8"), bit_pos: 604 }
        if node_has_class_id(node, 302) {
            intrinsic_matches.set_bit(604); // match_Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite-mobile__2_rj8")
        }

        // Instruction 606: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite__28uwB"), bit_pos: 606 }
        if node_has_class_id(node, 303) {
            intrinsic_matches.set_bit(606); // match_Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite__28uwB")
        }

        // Instruction 608: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-text-desktop__q3xp_"), bit_pos: 608 }
        if node_has_class_id(node, 304) {
            intrinsic_matches.set_bit(608); // match_Class("_fluid-quad-image-label-v2_style_ad-feedback-text-desktop__q3xp_")
        }

        // Instruction 610: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_ad-feedback-text__2HjQ9"), bit_pos: 610 }
        if node_has_class_id(node, 305) {
            intrinsic_matches.set_bit(610); // match_Class("_fluid-quad-image-label-v2_style_ad-feedback-text__2HjQ9")
        }

        // Instruction 612: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_apexBadgeLabel__2-Vye"), bit_pos: 612 }
        if node_has_class_id(node, 306) {
            intrinsic_matches.set_bit(612); // match_Class("_fluid-quad-image-label-v2_style_apexBadgeLabel__2-Vye")
        }

        // Instruction 614: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_apexBadgeMessage__1tHvd"), bit_pos: 614 }
        if node_has_class_id(node, 307) {
            intrinsic_matches.set_bit(614); // match_Class("_fluid-quad-image-label-v2_style_apexBadgeMessage__1tHvd")
        }

        // Instruction 616: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-button-group__1LqUG"), bit_pos: 616 }
        if node_has_class_id(node, 308) {
            intrinsic_matches.set_bit(616); // match_Class("_fluid-quad-image-label-v2_style_aspect-button-group__1LqUG")
        }

        // Instruction 618: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-button__7cH_E"), bit_pos: 618 }
        if node_has_class_id(node, 309) {
            intrinsic_matches.set_bit(618); // match_Class("_fluid-quad-image-label-v2_style_aspect-button__7cH_E")
        }

        // Instruction 620: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-1236x1080__3aEzl"), bit_pos: 620 }
        if node_has_class_id(node, 310) {
            intrinsic_matches.set_bit(620); // match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-1236x1080__3aEzl")
        }

        // Instruction 622: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-15x3__1h649"), bit_pos: 622 }
        if node_has_class_id(node, 311) {
            intrinsic_matches.set_bit(622); // match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-15x3__1h649")
        }

        // Instruction 624: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-16x9__cBPv8"), bit_pos: 624 }
        if node_has_class_id(node, 312) {
            intrinsic_matches.set_bit(624); // match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-16x9__cBPv8")
        }

        // Instruction 626: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-4x3__3BewI"), bit_pos: 626 }
        if node_has_class_id(node, 313) {
            intrinsic_matches.set_bit(626); // match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-4x3__3BewI")
        }

        // Instruction 628: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-5x8__2IaNz"), bit_pos: 628 }
        if node_has_class_id(node, 314) {
            intrinsic_matches.set_bit(628); // match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-5x8__2IaNz")
        }

        // Instruction 630: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-dynamic-60vh__3N5g_"), bit_pos: 630 }
        if node_has_class_id(node, 315) {
            intrinsic_matches.set_bit(630); // match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
        }

        // Instruction 632: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-ratio-fill__2Zjfb"), bit_pos: 632 }
        if node_has_class_id(node, 316) {
            intrinsic_matches.set_bit(632); // match_Class("_fluid-quad-image-label-v2_style_aspect-ratio-fill__2Zjfb")
        }

        // Instruction 634: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_aspect-text__S4PU1"), bit_pos: 634 }
        if node_has_class_id(node, 317) {
            intrinsic_matches.set_bit(634); // match_Class("_fluid-quad-image-label-v2_style_aspect-text__S4PU1")
        }

        // Instruction 636: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_autoplay-span__2CMfc"), bit_pos: 636 }
        if node_has_class_id(node, 318) {
            intrinsic_matches.set_bit(636); // match_Class("_fluid-quad-image-label-v2_style_autoplay-span__2CMfc")
        }

        // Instruction 638: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_badge-container__20aJ2"), bit_pos: 638 }
        if node_has_class_id(node, 319) {
            intrinsic_matches.set_bit(638); // match_Class("_fluid-quad-image-label-v2_style_badge-container__20aJ2")
        }

        // Instruction 640: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_badgeLabel__pJ5rc"), bit_pos: 640 }
        if node_has_class_id(node, 320) {
            intrinsic_matches.set_bit(640); // match_Class("_fluid-quad-image-label-v2_style_badgeLabel__pJ5rc")
        }

        // Instruction 642: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_badgeMessage__2Dtw7"), bit_pos: 642 }
        if node_has_class_id(node, 321) {
            intrinsic_matches.set_bit(642); // match_Class("_fluid-quad-image-label-v2_style_badgeMessage__2Dtw7")
        }

        // Instruction 644: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_carouselContainer__3N7M1"), bit_pos: 644 }
        if node_has_class_id(node, 322) {
            intrinsic_matches.set_bit(644); // match_Class("_fluid-quad-image-label-v2_style_carouselContainer__3N7M1")
        }

        // Instruction 646: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_centerImage__30wh-"), bit_pos: 646 }
        if node_has_class_id(node, 323) {
            intrinsic_matches.set_bit(646); // match_Class("_fluid-quad-image-label-v2_style_centerImage__30wh-")
        }

        // Instruction 648: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_close-black-icon__3hkbe"), bit_pos: 648 }
        if node_has_class_id(node, 324) {
            intrinsic_matches.set_bit(648); // match_Class("_fluid-quad-image-label-v2_style_close-black-icon__3hkbe")
        }

        // Instruction 650: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_close-icon-wrapper__1zvdC"), bit_pos: 650 }
        if node_has_class_id(node, 325) {
            intrinsic_matches.set_bit(650); // match_Class("_fluid-quad-image-label-v2_style_close-icon-wrapper__1zvdC")
        }

        // Instruction 652: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_close-icon__2RJs3"), bit_pos: 652 }
        if node_has_class_id(node, 326) {
            intrinsic_matches.set_bit(652); // match_Class("_fluid-quad-image-label-v2_style_close-icon__2RJs3")
        }

        // Instruction 654: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_close-text__2-gwn"), bit_pos: 654 }
        if node_has_class_id(node, 327) {
            intrinsic_matches.set_bit(654); // match_Class("_fluid-quad-image-label-v2_style_close-text__2-gwn")
        }

        // Instruction 656: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_cover-portrait-image__2lhzL"), bit_pos: 656 }
        if node_has_class_id(node, 328) {
            intrinsic_matches.set_bit(656); // match_Class("_fluid-quad-image-label-v2_style_cover-portrait-image__2lhzL")
        }

        // Instruction 658: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_cta-link__2xo74"), bit_pos: 658 }
        if node_has_class_id(node, 329) {
            intrinsic_matches.set_bit(658); // match_Class("_fluid-quad-image-label-v2_style_cta-link__2xo74")
        }

        // Instruction 660: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_desktop-close-button__1iL_P"), bit_pos: 660 }
        if node_has_class_id(node, 330) {
            intrinsic_matches.set_bit(660); // match_Class("_fluid-quad-image-label-v2_style_desktop-close-button__1iL_P")
        }

        // Instruction 662: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_displayCount__1MVut"), bit_pos: 662 }
        if node_has_class_id(node, 331) {
            intrinsic_matches.set_bit(662); // match_Class("_fluid-quad-image-label-v2_style_displayCount__1MVut")
        }

        // Instruction 664: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_dynamic-portrait-image__1Wrzd"), bit_pos: 664 }
        if node_has_class_id(node, 332) {
            intrinsic_matches.set_bit(664); // match_Class("_fluid-quad-image-label-v2_style_dynamic-portrait-image__1Wrzd")
        }

        // Instruction 666: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_empty-footer__2d59h"), bit_pos: 666 }
        if node_has_class_id(node, 333) {
            intrinsic_matches.set_bit(666); // match_Class("_fluid-quad-image-label-v2_style_empty-footer__2d59h")
        }

        // Instruction 668: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_five-pack__1-Tql"), bit_pos: 668 }
        if node_has_class_id(node, 334) {
            intrinsic_matches.set_bit(668); // match_Class("_fluid-quad-image-label-v2_style_five-pack__1-Tql")
        }

        // Instruction 670: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluid-landscape-image__TE6PT"), bit_pos: 670 }
        if node_has_class_id(node, 335) {
            intrinsic_matches.set_bit(670); // match_Class("_fluid-quad-image-label-v2_style_fluid-landscape-image__TE6PT")
        }

        // Instruction 672: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidImageContainer__2SOMr"), bit_pos: 672 }
        if node_has_class_id(node, 336) {
            intrinsic_matches.set_bit(672); // match_Class("_fluid-quad-image-label-v2_style_fluidImageContainer__2SOMr")
        }

        // Instruction 674: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidLandscapeImage__2euAK"), bit_pos: 674 }
        if node_has_class_id(node, 337) {
            intrinsic_matches.set_bit(674); // match_Class("_fluid-quad-image-label-v2_style_fluidLandscapeImage__2euAK")
        }

        // Instruction 676: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidPortraitImage__2SAYm"), bit_pos: 676 }
        if node_has_class_id(node, 338) {
            intrinsic_matches.set_bit(676); // match_Class("_fluid-quad-image-label-v2_style_fluidPortraitImage__2SAYm")
        }

        // Instruction 678: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabelBody__3tld0"), bit_pos: 678 }
        if node_has_class_id(node, 339) {
            intrinsic_matches.set_bit(678); // match_Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabelBody__3tld0")
        }

        // Instruction 680: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabel__3b-Iv"), bit_pos: 680 }
        if node_has_class_id(node, 340) {
            intrinsic_matches.set_bit(680); // match_Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabel__3b-Iv")
        }

        // Instruction 682: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_four-pack__1ufgr"), bit_pos: 682 }
        if node_has_class_id(node, 341) {
            intrinsic_matches.set_bit(682); // match_Class("_fluid-quad-image-label-v2_style_four-pack__1ufgr")
        }

        // Instruction 684: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_gridRowOne__1t0zL"), bit_pos: 684 }
        if node_has_class_id(node, 342) {
            intrinsic_matches.set_bit(684); // match_Class("_fluid-quad-image-label-v2_style_gridRowOne__1t0zL")
        }

        // Instruction 686: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_gridRowTwo__15woW"), bit_pos: 686 }
        if node_has_class_id(node, 343) {
            intrinsic_matches.set_bit(686); // match_Class("_fluid-quad-image-label-v2_style_gridRowTwo__15woW")
        }

        // Instruction 688: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_gw-hero-close-button__3svyZ"), bit_pos: 688 }
        if node_has_class_id(node, 344) {
            intrinsic_matches.set_bit(688); // match_Class("_fluid-quad-image-label-v2_style_gw-hero-close-button__3svyZ")
        }

        // Instruction 690: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_gwm-link-footer__3OF47"), bit_pos: 690 }
        if node_has_class_id(node, 345) {
            intrinsic_matches.set_bit(690); // match_Class("_fluid-quad-image-label-v2_style_gwm-link-footer__3OF47")
        }

        // Instruction 692: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_haulRibbon__3VZNi"), bit_pos: 692 }
        if node_has_class_id(node, 346) {
            intrinsic_matches.set_bit(692); // match_Class("_fluid-quad-image-label-v2_style_haulRibbon__3VZNi")
        }

        // Instruction 694: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_header-icon__2cuVV"), bit_pos: 694 }
        if node_has_class_id(node, 347) {
            intrinsic_matches.set_bit(694); // match_Class("_fluid-quad-image-label-v2_style_header-icon__2cuVV")
        }

        // Instruction 696: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_header-link__cUhOK"), bit_pos: 696 }
        if node_has_class_id(node, 348) {
            intrinsic_matches.set_bit(696); // match_Class("_fluid-quad-image-label-v2_style_header-link__cUhOK")
        }

        // Instruction 698: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_header__1vGdj"), bit_pos: 698 }
        if node_has_class_id(node, 349) {
            intrinsic_matches.set_bit(698); // match_Class("_fluid-quad-image-label-v2_style_header__1vGdj")
        }

        // Instruction 700: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_image-container__2OiZA"), bit_pos: 700 }
        if node_has_class_id(node, 350) {
            intrinsic_matches.set_bit(700); // match_Class("_fluid-quad-image-label-v2_style_image-container__2OiZA")
        }

        // Instruction 702: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_imageLabel__3ANSV"), bit_pos: 702 }
        if node_has_class_id(node, 351) {
            intrinsic_matches.set_bit(702); // match_Class("_fluid-quad-image-label-v2_style_imageLabel__3ANSV")
        }

        // Instruction 704: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_inlineErrorDetails__1NBx-"), bit_pos: 704 }
        if node_has_class_id(node, 352) {
            intrinsic_matches.set_bit(704); // match_Class("_fluid-quad-image-label-v2_style_inlineErrorDetails__1NBx-")
        }

        // Instruction 706: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_leftQuadrant__21nVp"), bit_pos: 706 }
        if node_has_class_id(node, 353) {
            intrinsic_matches.set_bit(706); // match_Class("_fluid-quad-image-label-v2_style_leftQuadrant__21nVp")
        }

        // Instruction 708: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logoGap__nKNZ9"), bit_pos: 708 }
        if node_has_class_id(node, 354) {
            intrinsic_matches.set_bit(708); // match_Class("_fluid-quad-image-label-v2_style_logoGap__nKNZ9")
        }

        // Instruction 710: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logoRectangle__1VJwu"), bit_pos: 710 }
        if node_has_class_id(node, 355) {
            intrinsic_matches.set_bit(710); // match_Class("_fluid-quad-image-label-v2_style_logoRectangle__1VJwu")
        }

        // Instruction 712: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logoSquareContainer__3Paoc"), bit_pos: 712 }
        if node_has_class_id(node, 356) {
            intrinsic_matches.set_bit(712); // match_Class("_fluid-quad-image-label-v2_style_logoSquareContainer__3Paoc")
        }

        // Instruction 714: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logoSquare__3NZyi"), bit_pos: 714 }
        if node_has_class_id(node, 357) {
            intrinsic_matches.set_bit(714); // match_Class("_fluid-quad-image-label-v2_style_logoSquare__3NZyi")
        }

        // Instruction 716: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_logo__2ZQ-N"), bit_pos: 716 }
        if node_has_class_id(node, 358) {
            intrinsic_matches.set_bit(716); // match_Class("_fluid-quad-image-label-v2_style_logo__2ZQ-N")
        }

        // Instruction 718: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_mixed-button__2og-m"), bit_pos: 718 }
        if node_has_class_id(node, 359) {
            intrinsic_matches.set_bit(718); // match_Class("_fluid-quad-image-label-v2_style_mixed-button__2og-m")
        }

        // Instruction 720: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_mobile-close-button__3PB07"), bit_pos: 720 }
        if node_has_class_id(node, 360) {
            intrinsic_matches.set_bit(720); // match_Class("_fluid-quad-image-label-v2_style_mobile-close-button__3PB07")
        }

        // Instruction 722: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_mosaic-card-body__1HmTs"), bit_pos: 722 }
        if node_has_class_id(node, 361) {
            intrinsic_matches.set_bit(722); // match_Class("_fluid-quad-image-label-v2_style_mosaic-card-body__1HmTs")
        }

        // Instruction 724: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_mosaic-card__1C-_R"), bit_pos: 724 }
        if node_has_class_id(node, 362) {
            intrinsic_matches.set_bit(724); // match_Class("_fluid-quad-image-label-v2_style_mosaic-card__1C-_R")
        }

        // Instruction 726: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_negative-button__1Dvqz"), bit_pos: 726 }
        if node_has_class_id(node, 363) {
            intrinsic_matches.set_bit(726); // match_Class("_fluid-quad-image-label-v2_style_negative-button__1Dvqz")
        }

        // Instruction 728: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_negativeMarginAdjust__1nqu9"), bit_pos: 728 }
        if node_has_class_id(node, 364) {
            intrinsic_matches.set_bit(728); // match_Class("_fluid-quad-image-label-v2_style_negativeMarginAdjust__1nqu9")
        }

        // Instruction 730: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_oneLineTruncation__2WWse"), bit_pos: 730 }
        if node_has_class_id(node, 365) {
            intrinsic_matches.set_bit(730); // match_Class("_fluid-quad-image-label-v2_style_oneLineTruncation__2WWse")
        }

        // Instruction 732: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_overlay__3Sx3u"), bit_pos: 732 }
        if node_has_class_id(node, 366) {
            intrinsic_matches.set_bit(732); // match_Class("_fluid-quad-image-label-v2_style_overlay__3Sx3u")
        }

        // Instruction 734: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_positive-button__3UOC3"), bit_pos: 734 }
        if node_has_class_id(node, 367) {
            intrinsic_matches.set_bit(734); // match_Class("_fluid-quad-image-label-v2_style_positive-button__3UOC3")
        }

        // Instruction 736: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_poster-image__1W0yA"), bit_pos: 736 }
        if node_has_class_id(node, 368) {
            intrinsic_matches.set_bit(736); // match_Class("_fluid-quad-image-label-v2_style_poster-image__1W0yA")
        }

        // Instruction 738: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_quadrantContainer__3TMqG"), bit_pos: 738 }
        if node_has_class_id(node, 369) {
            intrinsic_matches.set_bit(738); // match_Class("_fluid-quad-image-label-v2_style_quadrantContainer__3TMqG")
        }

        // Instruction 740: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_rightQuadrant__PI01n"), bit_pos: 740 }
        if node_has_class_id(node, 370) {
            intrinsic_matches.set_bit(740); // match_Class("_fluid-quad-image-label-v2_style_rightQuadrant__PI01n")
        }

        // Instruction 742: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_smartText__ubpEw"), bit_pos: 742 }
        if node_has_class_id(node, 371) {
            intrinsic_matches.set_bit(742); // match_Class("_fluid-quad-image-label-v2_style_smartText__ubpEw")
        }

        // Instruction 744: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_spCSRFTreatment__-hwVO"), bit_pos: 744 }
        if node_has_class_id(node, 372) {
            intrinsic_matches.set_bit(744); // match_Class("_fluid-quad-image-label-v2_style_spCSRFTreatment__-hwVO")
        }

        // Instruction 746: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_spacer__7Pyg3"), bit_pos: 746 }
        if node_has_class_id(node, 373) {
            intrinsic_matches.set_bit(746); // match_Class("_fluid-quad-image-label-v2_style_spacer__7Pyg3")
        }

        // Instruction 748: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_stacking-context__3PbQE"), bit_pos: 748 }
        if node_has_class_id(node, 374) {
            intrinsic_matches.set_bit(748); // match_Class("_fluid-quad-image-label-v2_style_stacking-context__3PbQE")
        }

        // Instruction 750: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_theming-background-override__1HfzJ"), bit_pos: 750 }
        if node_has_class_id(node, 375) {
            intrinsic_matches.set_bit(750); // match_Class("_fluid-quad-image-label-v2_style_theming-background-override__1HfzJ")
        }

        // Instruction 752: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_themingTextColorWhite__1zryO"), bit_pos: 752 }
        if node_has_class_id(node, 376) {
            intrinsic_matches.set_bit(752); // match_Class("_fluid-quad-image-label-v2_style_themingTextColorWhite__1zryO")
        }

        // Instruction 754: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_themingTextColor__1oQsI"), bit_pos: 754 }
        if node_has_class_id(node, 377) {
            intrinsic_matches.set_bit(754); // match_Class("_fluid-quad-image-label-v2_style_themingTextColor__1oQsI")
        }

        // Instruction 756: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_three-pack__5s3hP"), bit_pos: 756 }
        if node_has_class_id(node, 378) {
            intrinsic_matches.set_bit(756); // match_Class("_fluid-quad-image-label-v2_style_three-pack__5s3hP")
        }

        // Instruction 758: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_threeLineTruncation__UkUjj"), bit_pos: 758 }
        if node_has_class_id(node, 379) {
            intrinsic_matches.set_bit(758); // match_Class("_fluid-quad-image-label-v2_style_threeLineTruncation__UkUjj")
        }

        // Instruction 760: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_tile-container__1QgAV"), bit_pos: 760 }
        if node_has_class_id(node, 380) {
            intrinsic_matches.set_bit(760); // match_Class("_fluid-quad-image-label-v2_style_tile-container__1QgAV")
        }

        // Instruction 762: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_tile-grid__QMxNY"), bit_pos: 762 }
        if node_has_class_id(node, 381) {
            intrinsic_matches.set_bit(762); // match_Class("_fluid-quad-image-label-v2_style_tile-grid__QMxNY")
        }

        // Instruction 764: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_tile-link__38lTa"), bit_pos: 764 }
        if node_has_class_id(node, 382) {
            intrinsic_matches.set_bit(764); // match_Class("_fluid-quad-image-label-v2_style_tile-link__38lTa")
        }

        // Instruction 766: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_tile-theming__3eeyj"), bit_pos: 766 }
        if node_has_class_id(node, 383) {
            intrinsic_matches.set_bit(766); // match_Class("_fluid-quad-image-label-v2_style_tile-theming__3eeyj")
        }

        // Instruction 768: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_truncation__x9-69"), bit_pos: 768 }
        if node_has_class_id(node, 384) {
            intrinsic_matches.set_bit(768); // match_Class("_fluid-quad-image-label-v2_style_truncation__x9-69")
        }

        // Instruction 770: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_twoLineTruncation__16TLV"), bit_pos: 770 }
        if node_has_class_id(node, 385) {
            intrinsic_matches.set_bit(770); // match_Class("_fluid-quad-image-label-v2_style_twoLineTruncation__16TLV")
        }

        // Instruction 772: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_video-container__1hKS1"), bit_pos: 772 }
        if node_has_class_id(node, 386) {
            intrinsic_matches.set_bit(772); // match_Class("_fluid-quad-image-label-v2_style_video-container__1hKS1")
        }

        // Instruction 774: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_wd-backdrop-data__1znxG"), bit_pos: 774 }
        if node_has_class_id(node, 387) {
            intrinsic_matches.set_bit(774); // match_Class("_fluid-quad-image-label-v2_style_wd-backdrop-data__1znxG")
        }

        // Instruction 776: CheckAndSetBit { selector: Class("_fluid-quad-image-label-v2_style_wdHeader__Edrev"), bit_pos: 776 }
        if node_has_class_id(node, 388) {
            intrinsic_matches.set_bit(776); // match_Class("_fluid-quad-image-label-v2_style_wdHeader__Edrev")
        }

        // Instruction 778: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_cardBody__3Rdh1"), bit_pos: 778 }
        if node_has_class_id(node, 389) {
            intrinsic_matches.set_bit(778); // match_Class("_quad-category-card_desktopStyle_cardBody__3Rdh1")
        }

        // Instruction 780: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_categoryImage__35jKN"), bit_pos: 780 }
        if node_has_class_id(node, 390) {
            intrinsic_matches.set_bit(780); // match_Class("_quad-category-card_desktopStyle_categoryImage__35jKN")
        }

        // Instruction 782: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_category__3flCQ"), bit_pos: 782 }
        if node_has_class_id(node, 391) {
            intrinsic_matches.set_bit(782); // match_Class("_quad-category-card_desktopStyle_category__3flCQ")
        }

        // Instruction 784: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_heroCategory__3KS3k"), bit_pos: 784 }
        if node_has_class_id(node, 392) {
            intrinsic_matches.set_bit(784); // match_Class("_quad-category-card_desktopStyle_heroCategory__3KS3k")
        }

        // Instruction 786: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_heroImage__2V8-9"), bit_pos: 786 }
        if node_has_class_id(node, 393) {
            intrinsic_matches.set_bit(786); // match_Class("_quad-category-card_desktopStyle_heroImage__2V8-9")
        }

        // Instruction 788: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_heroLink__1EhW2"), bit_pos: 788 }
        if node_has_class_id(node, 394) {
            intrinsic_matches.set_bit(788); // match_Class("_quad-category-card_desktopStyle_heroLink__1EhW2")
        }

        // Instruction 790: CheckAndSetBit { selector: Class("_quad-category-card_desktopStyle_leftMost__1LmQB"), bit_pos: 790 }
        if node_has_class_id(node, 395) {
            intrinsic_matches.set_bit(790); // match_Class("_quad-category-card_desktopStyle_leftMost__1LmQB")
        }

        // Instruction 792: CheckAndSetBit { selector: Class("_quad-category-card_fluid_fluidCardBody__3TzJ4"), bit_pos: 792 }
        if node_has_class_id(node, 396) {
            intrinsic_matches.set_bit(792); // match_Class("_quad-category-card_fluid_fluidCardBody__3TzJ4")
        }

        // Instruction 794: CheckAndSetBit { selector: Class("_quad-category-card_fluid_fluidCard__3hmFA"), bit_pos: 794 }
        if node_has_class_id(node, 397) {
            intrinsic_matches.set_bit(794); // match_Class("_quad-category-card_fluid_fluidCard__3hmFA")
        }

        // Instruction 796: CheckAndSetBit { selector: Class("_quad-category-card_image_asin-container-full-height__MOKlF"), bit_pos: 796 }
        if node_has_class_id(node, 398) {
            intrinsic_matches.set_bit(796); // match_Class("_quad-category-card_image_asin-container-full-height__MOKlF")
        }

        // Instruction 798: CheckAndSetBit { selector: Class("_quad-category-card_image_asin-container-white-box__3Stwp"), bit_pos: 798 }
        if node_has_class_id(node, 399) {
            intrinsic_matches.set_bit(798); // match_Class("_quad-category-card_image_asin-container-white-box__3Stwp")
        }

        // Instruction 800: CheckAndSetBit { selector: Class("_quad-category-card_image_asin-container__LRY5p"), bit_pos: 800 }
        if node_has_class_id(node, 400) {
            intrinsic_matches.set_bit(800); // match_Class("_quad-category-card_image_asin-container__LRY5p")
        }

        // Instruction 802: CheckAndSetBit { selector: Class("_quad-category-card_image_round-corners__22iOW"), bit_pos: 802 }
        if node_has_class_id(node, 401) {
            intrinsic_matches.set_bit(802); // match_Class("_quad-category-card_image_round-corners__22iOW")
        }

        // Instruction 804: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_cardBody__3ODbW"), bit_pos: 804 }
        if node_has_class_id(node, 402) {
            intrinsic_matches.set_bit(804); // match_Class("_quad-category-card_mobileStyle_cardBody__3ODbW")
        }

        // Instruction 806: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_categoryContainer__2xY0I"), bit_pos: 806 }
        if node_has_class_id(node, 403) {
            intrinsic_matches.set_bit(806); // match_Class("_quad-category-card_mobileStyle_categoryContainer__2xY0I")
        }

        // Instruction 808: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_categoryImage__3hSFw"), bit_pos: 808 }
        if node_has_class_id(node, 404) {
            intrinsic_matches.set_bit(808); // match_Class("_quad-category-card_mobileStyle_categoryImage__3hSFw")
        }

        // Instruction 810: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_category__1amt4"), bit_pos: 810 }
        if node_has_class_id(node, 405) {
            intrinsic_matches.set_bit(810); // match_Class("_quad-category-card_mobileStyle_category__1amt4")
        }

        // Instruction 812: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_heroImage__1SewP"), bit_pos: 812 }
        if node_has_class_id(node, 406) {
            intrinsic_matches.set_bit(812); // match_Class("_quad-category-card_mobileStyle_heroImage__1SewP")
        }

        // Instruction 814: CheckAndSetBit { selector: Class("_quad-category-card_mobileStyle_leftMost__3WtU6"), bit_pos: 814 }
        if node_has_class_id(node, 407) {
            intrinsic_matches.set_bit(814); // match_Class("_quad-category-card_mobileStyle_leftMost__3WtU6")
        }

        // Instruction 816: CheckAndSetBit { selector: Class("_quad-category-card_style_dashboard-card-with-border__1e4z_"), bit_pos: 816 }
        if node_has_class_id(node, 408) {
            intrinsic_matches.set_bit(816); // match_Class("_quad-category-card_style_dashboard-card-with-border__1e4z_")
        }

        // Instruction 818: CheckAndSetBit { selector: Class("_quad-category-card_style_fluidImageContainer__2jd50"), bit_pos: 818 }
        if node_has_class_id(node, 409) {
            intrinsic_matches.set_bit(818); // match_Class("_quad-category-card_style_fluidImageContainer__2jd50")
        }

        // Instruction 820: CheckAndSetBit { selector: Class("_quad-category-card_style_fluidLandscapeImage__3eTVC"), bit_pos: 820 }
        if node_has_class_id(node, 410) {
            intrinsic_matches.set_bit(820); // match_Class("_quad-category-card_style_fluidLandscapeImage__3eTVC")
        }

        // Instruction 822: CheckAndSetBit { selector: Class("_quad-category-card_style_fluidPortraitImage__3yQ-X"), bit_pos: 822 }
        if node_has_class_id(node, 411) {
            intrinsic_matches.set_bit(822); // match_Class("_quad-category-card_style_fluidPortraitImage__3yQ-X")
        }

        // Instruction 824: CheckAndSetBit { selector: Class("_quad-category-card_style_gwm-link-footer__3EX7d"), bit_pos: 824 }
        if node_has_class_id(node, 412) {
            intrinsic_matches.set_bit(824); // match_Class("_quad-category-card_style_gwm-link-footer__3EX7d")
        }

        // Instruction 826: CheckAndSetBit { selector: Class("_quad-category-card_style_heading__1mnEu"), bit_pos: 826 }
        if node_has_class_id(node, 413) {
            intrinsic_matches.set_bit(826); // match_Class("_quad-category-card_style_heading__1mnEu")
        }

        // Instruction 828: CheckAndSetBit { selector: Class("_text-link-stripe-v2_style_textlinkstripe__3aQhz"), bit_pos: 828 }
        if node_has_class_id(node, 414) {
            intrinsic_matches.set_bit(828); // match_Class("_text-link-stripe-v2_style_textlinkstripe__3aQhz")
        }

        // Instruction 830: CheckAndSetBit { selector: Class("a-cardui-body"), bit_pos: 830 }
        if node_has_class_id(node, 415) {
            intrinsic_matches.set_bit(830); // match_Class("a-cardui-body")
        }

        // Instruction 832: CheckAndSetBit { selector: Class("a-cardui-footer"), bit_pos: 832 }
        if node_has_class_id(node, 416) {
            intrinsic_matches.set_bit(832); // match_Class("a-cardui-footer")
        }

        // Instruction 834: CheckAndSetBit { selector: Class("a-cardui-header"), bit_pos: 834 }
        if node_has_class_id(node, 417) {
            intrinsic_matches.set_bit(834); // match_Class("a-cardui-header")
        }

        // Instruction 836: CheckAndSetBit { selector: Class("a-carousel-container"), bit_pos: 836 }
        if node_has_class_id(node, 418) {
            intrinsic_matches.set_bit(836); // match_Class("a-carousel-container")
        }

        // Instruction 838: CheckAndSetBit { selector: Class("a-carousel-controls"), bit_pos: 838 }
        if node_has_class_id(node, 419) {
            intrinsic_matches.set_bit(838); // match_Class("a-carousel-controls")
        }

        // Instruction 840: CheckAndSetBit { selector: Class("a-carousel-right"), bit_pos: 840 }
        if node_has_class_id(node, 420) {
            intrinsic_matches.set_bit(840); // match_Class("a-carousel-right")
        }

        // Instruction 842: CheckAndSetBit { selector: Class("a-carousel-viewport"), bit_pos: 842 }
        if node_has_class_id(node, 421) {
            intrinsic_matches.set_bit(842); // match_Class("a-carousel-viewport")
        }

        // Instruction 844: CheckAndSetBit { selector: Class("a-link-normal"), bit_pos: 844 }
        if node_has_class_id(node, 422) {
            intrinsic_matches.set_bit(844); // match_Class("a-link-normal")
        }

        // Instruction 846: CheckAndSetBit { selector: Class("card-flow-row-break"), bit_pos: 846 }
        if node_has_class_id(node, 423) {
            intrinsic_matches.set_bit(846); // match_Class("card-flow-row-break")
        }

        // Instruction 848: CheckAndSetBit { selector: Class("gw-auto-height"), bit_pos: 848 }
        if node_has_class_id(node, 424) {
            intrinsic_matches.set_bit(848); // match_Class("gw-auto-height")
        }

        // Instruction 850: CheckAndSetBit { selector: Class("gw-card-layout"), bit_pos: 850 }
        if node_has_class_id(node, 425) {
            intrinsic_matches.set_bit(850); // match_Class("gw-card-layout")
        }

        // Instruction 852: CheckAndSetBit { selector: Class("gw-col"), bit_pos: 852 }
        if node_has_class_id(node, 426) {
            intrinsic_matches.set_bit(852); // match_Class("gw-col")
        }

        // Instruction 854: CheckAndSetBit { selector: Class("gw-fixed-col"), bit_pos: 854 }
        if node_has_class_id(node, 427) {
            intrinsic_matches.set_bit(854); // match_Class("gw-fixed-col")
        }

        // Instruction 856: CheckAndSetBit { selector: Class("gw-media-card"), bit_pos: 856 }
        if node_has_class_id(node, 428) {
            intrinsic_matches.set_bit(856); // match_Class("gw-media-card")
        }

        // Instruction 858: CheckAndSetBit { selector: Class("gw-row"), bit_pos: 858 }
        if node_has_class_id(node, 429) {
            intrinsic_matches.set_bit(858); // match_Class("gw-row")
        }

        // Instruction 860: CheckAndSetBit { selector: Class("nav-focus"), bit_pos: 860 }
        if node_has_class_id(node, 430) {
            intrinsic_matches.set_bit(860); // match_Class("nav-focus")
        }

        // Instruction 862: CheckAndSetBit { selector: Class("nav-spinner"), bit_pos: 862 }
        if node_has_class_id(node, 431) {
            intrinsic_matches.set_bit(862); // match_Class("nav-spinner")
        }

        // Instruction 864: CheckAndSetBit { selector: Class("nav-timeline-prime-icon"), bit_pos: 864 }
        if node_has_class_id(node, 432) {
            intrinsic_matches.set_bit(864); // match_Class("nav-timeline-prime-icon")
        }

        // Instruction 866: CheckAndSetBit { selector: Class("single-slide-hero"), bit_pos: 866 }
        if node_has_class_id(node, 433) {
            intrinsic_matches.set_bit(866); // match_Class("single-slide-hero")
        }

        // Instruction 868: CheckAndSetBit { selector: Class("truncate-1line"), bit_pos: 868 }
        if node_has_class_id(node, 434) {
            intrinsic_matches.set_bit(868); // match_Class("truncate-1line")
        }

        // Instruction 870: CheckAndSetBit { selector: Class("truncate-2line"), bit_pos: 870 }
        if node_has_class_id(node, 435) {
            intrinsic_matches.set_bit(870); // match_Class("truncate-2line")
        }

        // Instruction 872: CheckAndSetBit { selector: Class("vjs-fluid"), bit_pos: 872 }
        if node_has_class_id(node, 436) {
            intrinsic_matches.set_bit(872); // match_Class("vjs-fluid")
        }

match get_node_id_id(node) {
        // Instruction 874: CheckAndSetBit { selector: Id("icp-touch-link-cop"), bit_pos: 874 }
        Some(437) => {
            intrinsic_matches.set_bit(874); // match_Id("icp-touch-link-cop")
        }
        // Instruction 876: CheckAndSetBit { selector: Id("icp-touch-link-country"), bit_pos: 876 }
        Some(438) => {
            intrinsic_matches.set_bit(876); // match_Id("icp-touch-link-country")
        }
        // Instruction 878: CheckAndSetBit { selector: Id("icp-touch-link-language"), bit_pos: 878 }
        Some(439) => {
            intrinsic_matches.set_bit(878); // match_Id("icp-touch-link-language")
        }
_ => {}}
        node.cached_node_intrinsic = Some(intrinsic_matches);
    }

    let mut current_matches = node.cached_node_intrinsic.clone().unwrap();
    
    // BitVector-only parent state tracking
    let mut parent_bits_read = BitVector::with_capacity(parent_state.capacity);
    let mut parent_values_read = BitVector::with_capacity(parent_state.capacity);
    let mut child_states = BitVector::with_capacity(BITVECTOR_CAPACITY);
    if current_matches.is_bit_set(0) {
        child_states.set_bit(1); // active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
    }
    if current_matches.is_bit_set(2) {
        child_states.set_bit(3); // active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
    }
    if current_matches.is_bit_set(4) {
        child_states.set_bit(5); // active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
    }
    if current_matches.is_bit_set(6) {
        child_states.set_bit(7); // active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
    }
    if current_matches.is_bit_set(8) {
        child_states.set_bit(9); // active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
    }
    if current_matches.is_bit_set(10) {
        child_states.set_bit(11); // active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
    }
    if current_matches.is_bit_set(12) {
        child_states.set_bit(13); // active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-badge-standard__28gp8")
    }
    if current_matches.is_bit_set(14) {
        child_states.set_bit(15); // active_Class("_ameyal-product-shoveler_energy-efficiency_energy-efficiency-container__1Pkva")
    }
    if current_matches.is_bit_set(16) {
        child_states.set_bit(17); // active_Class("_ameyal-product-shoveler_image_asin-container-white-box__QwmgO")
    }
    if current_matches.is_bit_set(18) {
        child_states.set_bit(19); // active_Class("_ameyal-product-shoveler_image_asin-container__2jyCM")
    }
    if current_matches.is_bit_set(20) {
        child_states.set_bit(21); // active_Class("_ameyal-product-shoveler_image_round-corners__2y_fS")
    }
    if current_matches.is_bit_set(22) {
        child_states.set_bit(23); // active_Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner-rtl__2BoOY")
    }
    if current_matches.is_bit_set(24) {
        child_states.set_bit(25); // active_Class("_ameyal-product-shoveler_style_ad-feedback-loading-spinnner__1nmZw")
    }
    if current_matches.is_bit_set(26) {
        child_states.set_bit(27); // active_Class("_ameyal-product-shoveler_style_ad-feedback-primary-link__2bIZi")
    }
    if current_matches.is_bit_set(28) {
        child_states.set_bit(29); // active_Class("_ameyal-product-shoveler_style_ad-feedback-sprite-mobile__2_rj8")
    }
    if current_matches.is_bit_set(30) {
        child_states.set_bit(31); // active_Class("_ameyal-product-shoveler_style_ad-feedback-sprite__28uwB")
    }
    if current_matches.is_bit_set(32) {
        child_states.set_bit(33); // active_Class("_ameyal-product-shoveler_style_ad-feedback-text-desktop__q3xp_")
    }
    if current_matches.is_bit_set(34) {
        child_states.set_bit(35); // active_Class("_ameyal-product-shoveler_style_ad-feedback-text__2HjQ9")
    }
    if current_matches.is_bit_set(36) {
        child_states.set_bit(37); // active_Class("_ameyal-product-shoveler_style_apexBadgeLabel__2-Vye")
    }
    if current_matches.is_bit_set(38) {
        child_states.set_bit(39); // active_Class("_ameyal-product-shoveler_style_apexBadgeMessage__1tHvd")
    }
    if current_matches.is_bit_set(40) {
        child_states.set_bit(41); // active_Class("_ameyal-product-shoveler_style_aspect-button-group__1LqUG")
    }
    if current_matches.is_bit_set(42) {
        child_states.set_bit(43); // active_Class("_ameyal-product-shoveler_style_aspect-button__7cH_E")
    }
    if current_matches.is_bit_set(44) {
        child_states.set_bit(45); // active_Class("_ameyal-product-shoveler_style_aspect-ratio-1236x1080__3aEzl")
    }
    if current_matches.is_bit_set(46) {
        child_states.set_bit(47); // active_Class("_ameyal-product-shoveler_style_aspect-ratio-15x3__1h649")
    }
    if current_matches.is_bit_set(48) {
        child_states.set_bit(49); // active_Class("_ameyal-product-shoveler_style_aspect-ratio-16x9__cBPv8")
    }
    if current_matches.is_bit_set(50) {
        child_states.set_bit(51); // active_Class("_ameyal-product-shoveler_style_aspect-ratio-4x3__3BewI")
    }
    if current_matches.is_bit_set(52) {
        child_states.set_bit(53); // active_Class("_ameyal-product-shoveler_style_aspect-ratio-5x8__2IaNz")
    }
    if current_matches.is_bit_set(54) {
        child_states.set_bit(55); // active_Class("_ameyal-product-shoveler_style_aspect-ratio-dynamic-60vh__3N5g_")
    }
    if current_matches.is_bit_set(56) {
        child_states.set_bit(57); // active_Class("_ameyal-product-shoveler_style_aspect-ratio-fill__2Zjfb")
    }
    if current_matches.is_bit_set(58) {
        child_states.set_bit(59); // active_Class("_ameyal-product-shoveler_style_aspect-text__S4PU1")
    }
    if current_matches.is_bit_set(60) {
        child_states.set_bit(61); // active_Class("_ameyal-product-shoveler_style_autoplay-span__2CMfc")
    }
    if current_matches.is_bit_set(62) {
        child_states.set_bit(63); // active_Class("_ameyal-product-shoveler_style_badge-container__20aJ2")
    }
    if current_matches.is_bit_set(64) {
        child_states.set_bit(65); // active_Class("_ameyal-product-shoveler_style_badgeLabel__pJ5rc")
    }
    if current_matches.is_bit_set(66) {
        child_states.set_bit(67); // active_Class("_ameyal-product-shoveler_style_badgeMessage__2Dtw7")
    }
    if current_matches.is_bit_set(68) {
        child_states.set_bit(69); // active_Class("_ameyal-product-shoveler_style_carouselContainer__3N7M1")
    }
    if current_matches.is_bit_set(70) {
        child_states.set_bit(71); // active_Class("_ameyal-product-shoveler_style_close-black-icon__3hkbe")
    }
    if current_matches.is_bit_set(72) {
        child_states.set_bit(73); // active_Class("_ameyal-product-shoveler_style_close-icon-wrapper__1zvdC")
    }
    if current_matches.is_bit_set(74) {
        child_states.set_bit(75); // active_Class("_ameyal-product-shoveler_style_close-icon__2RJs3")
    }
    if current_matches.is_bit_set(76) {
        child_states.set_bit(77); // active_Class("_ameyal-product-shoveler_style_close-text__2-gwn")
    }
    if current_matches.is_bit_set(78) {
        child_states.set_bit(79); // active_Class("_ameyal-product-shoveler_style_cover-portrait-image__2lhzL")
    }
    if current_matches.is_bit_set(80) {
        child_states.set_bit(81); // active_Class("_ameyal-product-shoveler_style_cta-link__2xo74")
    }
    if current_matches.is_bit_set(82) {
        child_states.set_bit(83); // active_Class("_ameyal-product-shoveler_style_desktop-close-button__1iL_P")
    }
    if current_matches.is_bit_set(84) {
        child_states.set_bit(85); // active_Class("_ameyal-product-shoveler_style_displayCount__1MVut")
    }
    if current_matches.is_bit_set(86) {
        child_states.set_bit(87); // active_Class("_ameyal-product-shoveler_style_dynamic-portrait-image__1Wrzd")
    }
    if current_matches.is_bit_set(88) {
        child_states.set_bit(89); // active_Class("_ameyal-product-shoveler_style_empty-footer__2d59h")
    }
    if current_matches.is_bit_set(90) {
        child_states.set_bit(91); // active_Class("_ameyal-product-shoveler_style_five-pack__1-Tql")
    }
    if current_matches.is_bit_set(92) {
        child_states.set_bit(93); // active_Class("_ameyal-product-shoveler_style_fluid-landscape-image__TE6PT")
    }
    if current_matches.is_bit_set(94) {
        child_states.set_bit(95); // active_Class("_ameyal-product-shoveler_style_four-pack__1ufgr")
    }
    if current_matches.is_bit_set(96) {
        child_states.set_bit(97); // active_Class("_ameyal-product-shoveler_style_gw-hero-close-button__3svyZ")
    }
    if current_matches.is_bit_set(98) {
        child_states.set_bit(99); // active_Class("_ameyal-product-shoveler_style_gwm-link-footer__3OF47")
    }
    if current_matches.is_bit_set(100) {
        child_states.set_bit(101); // active_Class("_ameyal-product-shoveler_style_haulRibbon__3VZNi")
    }
    if current_matches.is_bit_set(102) {
        child_states.set_bit(103); // active_Class("_ameyal-product-shoveler_style_header-icon__2cuVV")
    }
    if current_matches.is_bit_set(104) {
        child_states.set_bit(105); // active_Class("_ameyal-product-shoveler_style_header-link__cUhOK")
    }
    if current_matches.is_bit_set(106) {
        child_states.set_bit(107); // active_Class("_ameyal-product-shoveler_style_header__1vGdj")
    }
    if current_matches.is_bit_set(108) {
        child_states.set_bit(109); // active_Class("_ameyal-product-shoveler_style_image-container__2OiZA")
    }
    if current_matches.is_bit_set(110) {
        child_states.set_bit(111); // active_Class("_ameyal-product-shoveler_style_inlineErrorDetails__1NBx-")
    }
    if current_matches.is_bit_set(112) {
        child_states.set_bit(113); // active_Class("_ameyal-product-shoveler_style_logoGap__nKNZ9")
    }
    if current_matches.is_bit_set(114) {
        child_states.set_bit(115); // active_Class("_ameyal-product-shoveler_style_logoRectangle__1VJwu")
    }
    if current_matches.is_bit_set(116) {
        child_states.set_bit(117); // active_Class("_ameyal-product-shoveler_style_logoSquareContainer__3Paoc")
    }
    if current_matches.is_bit_set(118) {
        child_states.set_bit(119); // active_Class("_ameyal-product-shoveler_style_logoSquare__3NZyi")
    }
    if current_matches.is_bit_set(120) {
        child_states.set_bit(121); // active_Class("_ameyal-product-shoveler_style_logo__2ZQ-N")
    }
    if current_matches.is_bit_set(122) {
        child_states.set_bit(123); // active_Class("_ameyal-product-shoveler_style_mixed-button__2og-m")
    }
    if current_matches.is_bit_set(124) {
        child_states.set_bit(125); // active_Class("_ameyal-product-shoveler_style_mobile-close-button__3PB07")
    }
    if current_matches.is_bit_set(126) {
        child_states.set_bit(127); // active_Class("_ameyal-product-shoveler_style_mosaic-card-body__1HmTs")
    }
    if current_matches.is_bit_set(128) {
        child_states.set_bit(129); // active_Class("_ameyal-product-shoveler_style_mosaic-card__1C-_R")
    }
    if current_matches.is_bit_set(130) {
        child_states.set_bit(131); // active_Class("_ameyal-product-shoveler_style_negative-button__1Dvqz")
    }
    if current_matches.is_bit_set(132) {
        child_states.set_bit(133); // active_Class("_ameyal-product-shoveler_style_negativeMarginAdjust__1nqu9")
    }
    if current_matches.is_bit_set(134) {
        child_states.set_bit(135); // active_Class("_ameyal-product-shoveler_style_oneLineTruncation__2WWse")
    }
    if current_matches.is_bit_set(136) {
        child_states.set_bit(137); // active_Class("_ameyal-product-shoveler_style_overlay__3Sx3u")
    }
    if current_matches.is_bit_set(138) {
        child_states.set_bit(139); // active_Class("_ameyal-product-shoveler_style_positive-button__3UOC3")
    }
    if current_matches.is_bit_set(140) {
        child_states.set_bit(141); // active_Class("_ameyal-product-shoveler_style_poster-image__1W0yA")
    }
    if current_matches.is_bit_set(142) {
        child_states.set_bit(143); // active_Class("_ameyal-product-shoveler_style_smartText__ubpEw")
    }
    if current_matches.is_bit_set(144) {
        child_states.set_bit(145); // active_Class("_ameyal-product-shoveler_style_spCSRFTreatment__-hwVO")
    }
    if current_matches.is_bit_set(146) {
        child_states.set_bit(147); // active_Class("_ameyal-product-shoveler_style_spacer__7Pyg3")
    }
    if current_matches.is_bit_set(148) {
        child_states.set_bit(149); // active_Class("_ameyal-product-shoveler_style_stacking-context__3PbQE")
    }
    if current_matches.is_bit_set(150) {
        child_states.set_bit(151); // active_Class("_ameyal-product-shoveler_style_theming-background-override__1HfzJ")
    }
    if current_matches.is_bit_set(152) {
        child_states.set_bit(153); // active_Class("_ameyal-product-shoveler_style_themingTextColorWhite__1zryO")
    }
    if current_matches.is_bit_set(154) {
        child_states.set_bit(155); // active_Class("_ameyal-product-shoveler_style_themingTextColor__1oQsI")
    }
    if current_matches.is_bit_set(156) {
        child_states.set_bit(157); // active_Class("_ameyal-product-shoveler_style_three-pack__5s3hP")
    }
    if current_matches.is_bit_set(158) {
        child_states.set_bit(159); // active_Class("_ameyal-product-shoveler_style_threeLineTruncation__UkUjj")
    }
    if current_matches.is_bit_set(160) {
        child_states.set_bit(161); // active_Class("_ameyal-product-shoveler_style_tile-container__1QgAV")
    }
    if current_matches.is_bit_set(162) {
        child_states.set_bit(163); // active_Class("_ameyal-product-shoveler_style_tile-grid__QMxNY")
    }
    if current_matches.is_bit_set(164) {
        child_states.set_bit(165); // active_Class("_ameyal-product-shoveler_style_tile-link__38lTa")
    }
    if current_matches.is_bit_set(166) {
        child_states.set_bit(167); // active_Class("_ameyal-product-shoveler_style_tile-theming__3eeyj")
    }
    if current_matches.is_bit_set(168) {
        child_states.set_bit(169); // active_Class("_ameyal-product-shoveler_style_truncation__x9-69")
    }
    if current_matches.is_bit_set(170) {
        child_states.set_bit(171); // active_Class("_ameyal-product-shoveler_style_twoLineTruncation__16TLV")
    }
    if current_matches.is_bit_set(172) {
        child_states.set_bit(173); // active_Class("_ameyal-product-shoveler_style_video-container__1hKS1")
    }
    if current_matches.is_bit_set(174) {
        child_states.set_bit(175); // active_Class("_ameyal-product-shoveler_style_wd-backdrop-data__1znxG")
    }
    if current_matches.is_bit_set(176) {
        child_states.set_bit(177); // active_Class("_ameyal-product-shoveler_style_wdHeader__Edrev")
    }
    if current_matches.is_bit_set(178) {
        child_states.set_bit(179); // active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
    }
    if current_matches.is_bit_set(180) {
        child_states.set_bit(181); // active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
    }
    if current_matches.is_bit_set(182) {
        child_states.set_bit(183); // active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
    }
    if current_matches.is_bit_set(184) {
        child_states.set_bit(185); // active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
    }
    if current_matches.is_bit_set(186) {
        child_states.set_bit(187); // active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
    }
    if current_matches.is_bit_set(188) {
        child_states.set_bit(189); // active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
    }
    if current_matches.is_bit_set(190) {
        child_states.set_bit(191); // active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-badge-standard__28gp8")
    }
    if current_matches.is_bit_set(192) {
        child_states.set_bit(193); // active_Class("_cropped-image-link_energy-efficiency_energy-efficiency-container__1Pkva")
    }
    if current_matches.is_bit_set(194) {
        child_states.set_bit(195); // active_Class("_cropped-image-link_image_asin-container-full-height__MOKlF")
    }
    if current_matches.is_bit_set(196) {
        child_states.set_bit(197); // active_Class("_cropped-image-link_image_asin-container-white-box__3Stwp")
    }
    if current_matches.is_bit_set(198) {
        child_states.set_bit(199); // active_Class("_cropped-image-link_image_asin-container-white-box__QwmgO")
    }
    if current_matches.is_bit_set(200) {
        child_states.set_bit(201); // active_Class("_cropped-image-link_image_asin-container__2jyCM")
    }
    if current_matches.is_bit_set(202) {
        child_states.set_bit(203); // active_Class("_cropped-image-link_image_asin-container__LRY5p")
    }
    if current_matches.is_bit_set(204) {
        child_states.set_bit(205); // active_Class("_cropped-image-link_image_round-corners__22iOW")
    }
    if current_matches.is_bit_set(206) {
        child_states.set_bit(207); // active_Class("_cropped-image-link_image_round-corners__2y_fS")
    }
    if current_matches.is_bit_set(208) {
        child_states.set_bit(209); // active_Class("_cropped-image-link_style_ad-feedback-loading-spinnner-rtl__2BoOY")
    }
    if current_matches.is_bit_set(210) {
        child_states.set_bit(211); // active_Class("_cropped-image-link_style_ad-feedback-loading-spinnner__1nmZw")
    }
    if current_matches.is_bit_set(212) {
        child_states.set_bit(213); // active_Class("_cropped-image-link_style_ad-feedback-primary-link__2bIZi")
    }
    if current_matches.is_bit_set(214) {
        child_states.set_bit(215); // active_Class("_cropped-image-link_style_ad-feedback-sprite-mobile__2_rj8")
    }
    if current_matches.is_bit_set(216) {
        child_states.set_bit(217); // active_Class("_cropped-image-link_style_ad-feedback-sprite__28uwB")
    }
    if current_matches.is_bit_set(218) {
        child_states.set_bit(219); // active_Class("_cropped-image-link_style_ad-feedback-text-desktop__q3xp_")
    }
    if current_matches.is_bit_set(220) {
        child_states.set_bit(221); // active_Class("_cropped-image-link_style_ad-feedback-text__2HjQ9")
    }
    if current_matches.is_bit_set(222) {
        child_states.set_bit(223); // active_Class("_cropped-image-link_style_apexBadgeLabel__2-Vye")
    }
    if current_matches.is_bit_set(224) {
        child_states.set_bit(225); // active_Class("_cropped-image-link_style_apexBadgeMessage__1tHvd")
    }
    if current_matches.is_bit_set(226) {
        child_states.set_bit(227); // active_Class("_cropped-image-link_style_aspect-button-group__1LqUG")
    }
    if current_matches.is_bit_set(228) {
        child_states.set_bit(229); // active_Class("_cropped-image-link_style_aspect-button__7cH_E")
    }
    if current_matches.is_bit_set(230) {
        child_states.set_bit(231); // active_Class("_cropped-image-link_style_aspect-ratio-1236x1080__3aEzl")
    }
    if current_matches.is_bit_set(232) {
        child_states.set_bit(233); // active_Class("_cropped-image-link_style_aspect-ratio-15x3__1h649")
    }
    if current_matches.is_bit_set(234) {
        child_states.set_bit(235); // active_Class("_cropped-image-link_style_aspect-ratio-16x9__cBPv8")
    }
    if current_matches.is_bit_set(236) {
        child_states.set_bit(237); // active_Class("_cropped-image-link_style_aspect-ratio-4x3__3BewI")
    }
    if current_matches.is_bit_set(238) {
        child_states.set_bit(239); // active_Class("_cropped-image-link_style_aspect-ratio-5x8__2IaNz")
    }
    if current_matches.is_bit_set(240) {
        child_states.set_bit(241); // active_Class("_cropped-image-link_style_aspect-ratio-dynamic-60vh__3N5g_")
    }
    if current_matches.is_bit_set(242) {
        child_states.set_bit(243); // active_Class("_cropped-image-link_style_aspect-ratio-fill__2Zjfb")
    }
    if current_matches.is_bit_set(244) {
        child_states.set_bit(245); // active_Class("_cropped-image-link_style_aspect-text__S4PU1")
    }
    if current_matches.is_bit_set(246) {
        child_states.set_bit(247); // active_Class("_cropped-image-link_style_autoplay-span__2CMfc")
    }
    if current_matches.is_bit_set(248) {
        child_states.set_bit(249); // active_Class("_cropped-image-link_style_badge-container__20aJ2")
    }
    if current_matches.is_bit_set(250) {
        child_states.set_bit(251); // active_Class("_cropped-image-link_style_badgeLabel__pJ5rc")
    }
    if current_matches.is_bit_set(252) {
        child_states.set_bit(253); // active_Class("_cropped-image-link_style_badgeMessage__2Dtw7")
    }
    if current_matches.is_bit_set(254) {
        child_states.set_bit(255); // active_Class("_cropped-image-link_style_carouselContainer__3N7M1")
    }
    if current_matches.is_bit_set(256) {
        child_states.set_bit(257); // active_Class("_cropped-image-link_style_centerImage__1rzYI")
    }
    if current_matches.is_bit_set(258) {
        child_states.set_bit(259); // active_Class("_cropped-image-link_style_close-black-icon__3hkbe")
    }
    if current_matches.is_bit_set(260) {
        child_states.set_bit(261); // active_Class("_cropped-image-link_style_close-icon-wrapper__1zvdC")
    }
    if current_matches.is_bit_set(262) {
        child_states.set_bit(263); // active_Class("_cropped-image-link_style_close-icon__2RJs3")
    }
    if current_matches.is_bit_set(264) {
        child_states.set_bit(265); // active_Class("_cropped-image-link_style_close-text__2-gwn")
    }
    if current_matches.is_bit_set(266) {
        child_states.set_bit(267); // active_Class("_cropped-image-link_style_cover-portrait-image__2lhzL")
    }
    if current_matches.is_bit_set(268) {
        child_states.set_bit(269); // active_Class("_cropped-image-link_style_cropped-image-link__3winf")
    }
    if current_matches.is_bit_set(270) {
        child_states.set_bit(271); // active_Class("_cropped-image-link_style_cta-link__2xo74")
    }
    if current_matches.is_bit_set(272) {
        child_states.set_bit(273); // active_Class("_cropped-image-link_style_desktop-close-button__1iL_P")
    }
    if current_matches.is_bit_set(274) {
        child_states.set_bit(275); // active_Class("_cropped-image-link_style_displayCount__1MVut")
    }
    if current_matches.is_bit_set(276) {
        child_states.set_bit(277); // active_Class("_cropped-image-link_style_dt-TextContainer__3nbU9")
    }
    if current_matches.is_bit_set(278) {
        child_states.set_bit(279); // active_Class("_cropped-image-link_style_dynamic-portrait-image__1Wrzd")
    }
    if current_matches.is_bit_set(280) {
        child_states.set_bit(281); // active_Class("_cropped-image-link_style_empty-footer__2d59h")
    }
    if current_matches.is_bit_set(282) {
        child_states.set_bit(283); // active_Class("_cropped-image-link_style_five-pack__1-Tql")
    }
    if current_matches.is_bit_set(284) {
        child_states.set_bit(285); // active_Class("_cropped-image-link_style_fluid-landscape-image__TE6PT")
    }
    if current_matches.is_bit_set(286) {
        child_states.set_bit(287); // active_Class("_cropped-image-link_style_fluidImageContainer__2jd50")
    }
    if current_matches.is_bit_set(288) {
        child_states.set_bit(289); // active_Class("_cropped-image-link_style_fluidLandscapeImage__3eTVC")
    }
    if current_matches.is_bit_set(290) {
        child_states.set_bit(291); // active_Class("_cropped-image-link_style_fluidPortraitImage__3yQ-X")
    }
    if current_matches.is_bit_set(292) {
        child_states.set_bit(293); // active_Class("_cropped-image-link_style_four-pack__1ufgr")
    }
    if current_matches.is_bit_set(294) {
        child_states.set_bit(295); // active_Class("_cropped-image-link_style_gw-hero-close-button__3svyZ")
    }
    if current_matches.is_bit_set(296) {
        child_states.set_bit(297); // active_Class("_cropped-image-link_style_gwm-link-footer__3OF47")
    }
    if current_matches.is_bit_set(298) {
        child_states.set_bit(299); // active_Class("_cropped-image-link_style_haulRibbon__3VZNi")
    }
    if current_matches.is_bit_set(300) {
        child_states.set_bit(301); // active_Class("_cropped-image-link_style_header-icon__2cuVV")
    }
    if current_matches.is_bit_set(302) {
        child_states.set_bit(303); // active_Class("_cropped-image-link_style_header-link__cUhOK")
    }
    if current_matches.is_bit_set(304) {
        child_states.set_bit(305); // active_Class("_cropped-image-link_style_header__1vGdj")
    }
    if current_matches.is_bit_set(306) {
        child_states.set_bit(307); // active_Class("_cropped-image-link_style_image-container__2OiZA")
    }
    if current_matches.is_bit_set(308) {
        child_states.set_bit(309); // active_Class("_cropped-image-link_style_logoGap__nKNZ9")
    }
    if current_matches.is_bit_set(310) {
        child_states.set_bit(311); // active_Class("_cropped-image-link_style_logoRectangle__1VJwu")
    }
    if current_matches.is_bit_set(312) {
        child_states.set_bit(313); // active_Class("_cropped-image-link_style_logoSquareContainer__3Paoc")
    }
    if current_matches.is_bit_set(314) {
        child_states.set_bit(315); // active_Class("_cropped-image-link_style_logoSquare__3NZyi")
    }
    if current_matches.is_bit_set(316) {
        child_states.set_bit(317); // active_Class("_cropped-image-link_style_logo__2ZQ-N")
    }
    if current_matches.is_bit_set(318) {
        child_states.set_bit(319); // active_Class("_cropped-image-link_style_mixed-button__2og-m")
    }
    if current_matches.is_bit_set(320) {
        child_states.set_bit(321); // active_Class("_cropped-image-link_style_mobile-close-button__3PB07")
    }
    if current_matches.is_bit_set(322) {
        child_states.set_bit(323); // active_Class("_cropped-image-link_style_mosaic-card-body__1HmTs")
    }
    if current_matches.is_bit_set(324) {
        child_states.set_bit(325); // active_Class("_cropped-image-link_style_mosaic-card__1C-_R")
    }
    if current_matches.is_bit_set(326) {
        child_states.set_bit(327); // active_Class("_cropped-image-link_style_negative-button__1Dvqz")
    }
    if current_matches.is_bit_set(328) {
        child_states.set_bit(329); // active_Class("_cropped-image-link_style_negativeMarginAdjust__1nqu9")
    }
    if current_matches.is_bit_set(330) {
        child_states.set_bit(331); // active_Class("_cropped-image-link_style_oneLineTruncation__2WWse")
    }
    if current_matches.is_bit_set(332) {
        child_states.set_bit(333); // active_Class("_cropped-image-link_style_overlay__3Sx3u")
    }
    if current_matches.is_bit_set(334) {
        child_states.set_bit(335); // active_Class("_cropped-image-link_style_positive-button__3UOC3")
    }
    if current_matches.is_bit_set(336) {
        child_states.set_bit(337); // active_Class("_cropped-image-link_style_poster-image__1W0yA")
    }
    if current_matches.is_bit_set(338) {
        child_states.set_bit(339); // active_Class("_cropped-image-link_style_smartText__ubpEw")
    }
    if current_matches.is_bit_set(340) {
        child_states.set_bit(341); // active_Class("_cropped-image-link_style_spacer__7Pyg3")
    }
    if current_matches.is_bit_set(342) {
        child_states.set_bit(343); // active_Class("_cropped-image-link_style_stacking-context__3PbQE")
    }
    if current_matches.is_bit_set(344) {
        child_states.set_bit(345); // active_Class("_cropped-image-link_style_theming-background-override__1HfzJ")
    }
    if current_matches.is_bit_set(346) {
        child_states.set_bit(347); // active_Class("_cropped-image-link_style_themingTextColorWhite__1zryO")
    }
    if current_matches.is_bit_set(348) {
        child_states.set_bit(349); // active_Class("_cropped-image-link_style_themingTextColor__1oQsI")
    }
    if current_matches.is_bit_set(350) {
        child_states.set_bit(351); // active_Class("_cropped-image-link_style_three-pack__5s3hP")
    }
    if current_matches.is_bit_set(352) {
        child_states.set_bit(353); // active_Class("_cropped-image-link_style_threeLineTruncation__UkUjj")
    }
    if current_matches.is_bit_set(354) {
        child_states.set_bit(355); // active_Class("_cropped-image-link_style_tile-container__1QgAV")
    }
    if current_matches.is_bit_set(356) {
        child_states.set_bit(357); // active_Class("_cropped-image-link_style_tile-grid__QMxNY")
    }
    if current_matches.is_bit_set(358) {
        child_states.set_bit(359); // active_Class("_cropped-image-link_style_tile-link__38lTa")
    }
    if current_matches.is_bit_set(360) {
        child_states.set_bit(361); // active_Class("_cropped-image-link_style_tile-theming__3eeyj")
    }
    if current_matches.is_bit_set(362) {
        child_states.set_bit(363); // active_Class("_cropped-image-link_style_truncation__x9-69")
    }
    if current_matches.is_bit_set(364) {
        child_states.set_bit(365); // active_Class("_cropped-image-link_style_twoLineTruncation__16TLV")
    }
    if current_matches.is_bit_set(366) {
        child_states.set_bit(367); // active_Class("_cropped-image-link_style_video-container__1hKS1")
    }
    if current_matches.is_bit_set(368) {
        child_states.set_bit(369); // active_Class("_cropped-image-link_style_wd-backdrop-data__1znxG")
    }
    if current_matches.is_bit_set(370) {
        child_states.set_bit(371); // active_Class("_cropped-image-link_style_wdHeader__Edrev")
    }
    if current_matches.is_bit_set(372) {
        child_states.set_bit(373); // active_Class("_fluid-fat-image-link-v2_bodyFooterStyle_cardBody__1YuQY")
    }
    if current_matches.is_bit_set(374) {
        child_states.set_bit(375); // active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
    }
    if current_matches.is_bit_set(376) {
        child_states.set_bit(377); // active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
    }
    if current_matches.is_bit_set(378) {
        child_states.set_bit(379); // active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
    }
    if current_matches.is_bit_set(380) {
        child_states.set_bit(381); // active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
    }
    if current_matches.is_bit_set(382) {
        child_states.set_bit(383); // active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
    }
    if current_matches.is_bit_set(384) {
        child_states.set_bit(385); // active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
    }
    if current_matches.is_bit_set(386) {
        child_states.set_bit(387); // active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
    }
    if current_matches.is_bit_set(388) {
        child_states.set_bit(389); // active_Class("_fluid-fat-image-link-v2_energy-efficiency_energy-efficiency-container__1Pkva")
    }
    if current_matches.is_bit_set(390) {
        child_states.set_bit(391); // active_Class("_fluid-fat-image-link-v2_image_asin-container-white-box__QwmgO")
    }
    if current_matches.is_bit_set(392) {
        child_states.set_bit(393); // active_Class("_fluid-fat-image-link-v2_image_asin-container__2jyCM")
    }
    if current_matches.is_bit_set(394) {
        child_states.set_bit(395); // active_Class("_fluid-fat-image-link-v2_image_round-corners__2y_fS")
    }
    if current_matches.is_bit_set(396) {
        child_states.set_bit(397); // active_Class("_fluid-fat-image-link-v2_singleLinkStyle_bodyFooterLink__9LvH0")
    }
    if current_matches.is_bit_set(398) {
        child_states.set_bit(399); // active_Class("_fluid-fat-image-link-v2_singleLinkStyle_footer__2cH0y")
    }
    if current_matches.is_bit_set(400) {
        child_states.set_bit(401); // active_Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
    }
    if current_matches.is_bit_set(402) {
        child_states.set_bit(403); // active_Class("_fluid-fat-image-link-v2_style_ad-feedback-loading-spinnner__1nmZw")
    }
    if current_matches.is_bit_set(404) {
        child_states.set_bit(405); // active_Class("_fluid-fat-image-link-v2_style_ad-feedback-primary-link__2bIZi")
    }
    if current_matches.is_bit_set(406) {
        child_states.set_bit(407); // active_Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite-mobile__2_rj8")
    }
    if current_matches.is_bit_set(408) {
        child_states.set_bit(409); // active_Class("_fluid-fat-image-link-v2_style_ad-feedback-sprite__28uwB")
    }
    if current_matches.is_bit_set(410) {
        child_states.set_bit(411); // active_Class("_fluid-fat-image-link-v2_style_ad-feedback-text-desktop__q3xp_")
    }
    if current_matches.is_bit_set(412) {
        child_states.set_bit(413); // active_Class("_fluid-fat-image-link-v2_style_ad-feedback-text__2HjQ9")
    }
    if current_matches.is_bit_set(414) {
        child_states.set_bit(415); // active_Class("_fluid-fat-image-link-v2_style_apexBadgeLabel__2-Vye")
    }
    if current_matches.is_bit_set(416) {
        child_states.set_bit(417); // active_Class("_fluid-fat-image-link-v2_style_apexBadgeMessage__1tHvd")
    }
    if current_matches.is_bit_set(418) {
        child_states.set_bit(419); // active_Class("_fluid-fat-image-link-v2_style_aspect-button-group__1LqUG")
    }
    if current_matches.is_bit_set(420) {
        child_states.set_bit(421); // active_Class("_fluid-fat-image-link-v2_style_aspect-button__7cH_E")
    }
    if current_matches.is_bit_set(422) {
        child_states.set_bit(423); // active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-1236x1080__3aEzl")
    }
    if current_matches.is_bit_set(424) {
        child_states.set_bit(425); // active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-15x3__1h649")
    }
    if current_matches.is_bit_set(426) {
        child_states.set_bit(427); // active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-16x9__cBPv8")
    }
    if current_matches.is_bit_set(428) {
        child_states.set_bit(429); // active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-4x3__3BewI")
    }
    if current_matches.is_bit_set(430) {
        child_states.set_bit(431); // active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-5x8__2IaNz")
    }
    if current_matches.is_bit_set(432) {
        child_states.set_bit(433); // active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
    }
    if current_matches.is_bit_set(434) {
        child_states.set_bit(435); // active_Class("_fluid-fat-image-link-v2_style_aspect-ratio-fill__2Zjfb")
    }
    if current_matches.is_bit_set(436) {
        child_states.set_bit(437); // active_Class("_fluid-fat-image-link-v2_style_aspect-text__S4PU1")
    }
    if current_matches.is_bit_set(438) {
        child_states.set_bit(439); // active_Class("_fluid-fat-image-link-v2_style_autoplay-span__2CMfc")
    }
    if current_matches.is_bit_set(440) {
        child_states.set_bit(441); // active_Class("_fluid-fat-image-link-v2_style_badge-container__20aJ2")
    }
    if current_matches.is_bit_set(442) {
        child_states.set_bit(443); // active_Class("_fluid-fat-image-link-v2_style_badgeLabel__pJ5rc")
    }
    if current_matches.is_bit_set(444) {
        child_states.set_bit(445); // active_Class("_fluid-fat-image-link-v2_style_badgeMessage__2Dtw7")
    }
    if current_matches.is_bit_set(446) {
        child_states.set_bit(447); // active_Class("_fluid-fat-image-link-v2_style_carouselContainer__3N7M1")
    }
    if current_matches.is_bit_set(448) {
        child_states.set_bit(449); // active_Class("_fluid-fat-image-link-v2_style_centerImage__30wh-")
    }
    if current_matches.is_bit_set(450) {
        child_states.set_bit(451); // active_Class("_fluid-fat-image-link-v2_style_close-black-icon__3hkbe")
    }
    if current_matches.is_bit_set(452) {
        child_states.set_bit(453); // active_Class("_fluid-fat-image-link-v2_style_close-icon-wrapper__1zvdC")
    }
    if current_matches.is_bit_set(454) {
        child_states.set_bit(455); // active_Class("_fluid-fat-image-link-v2_style_close-icon__2RJs3")
    }
    if current_matches.is_bit_set(456) {
        child_states.set_bit(457); // active_Class("_fluid-fat-image-link-v2_style_close-text__2-gwn")
    }
    if current_matches.is_bit_set(458) {
        child_states.set_bit(459); // active_Class("_fluid-fat-image-link-v2_style_cover-portrait-image__2lhzL")
    }
    if current_matches.is_bit_set(460) {
        child_states.set_bit(461); // active_Class("_fluid-fat-image-link-v2_style_cta-link__2xo74")
    }
    if current_matches.is_bit_set(462) {
        child_states.set_bit(463); // active_Class("_fluid-fat-image-link-v2_style_desktop-close-button__1iL_P")
    }
    if current_matches.is_bit_set(464) {
        child_states.set_bit(465); // active_Class("_fluid-fat-image-link-v2_style_displayCount__1MVut")
    }
    if current_matches.is_bit_set(466) {
        child_states.set_bit(467); // active_Class("_fluid-fat-image-link-v2_style_dynamic-portrait-image__1Wrzd")
    }
    if current_matches.is_bit_set(468) {
        child_states.set_bit(469); // active_Class("_fluid-fat-image-link-v2_style_empty-footer__2d59h")
    }
    if current_matches.is_bit_set(470) {
        child_states.set_bit(471); // active_Class("_fluid-fat-image-link-v2_style_five-pack__1-Tql")
    }
    if current_matches.is_bit_set(472) {
        child_states.set_bit(473); // active_Class("_fluid-fat-image-link-v2_style_fluid-landscape-image__TE6PT")
    }
    if current_matches.is_bit_set(474) {
        child_states.set_bit(475); // active_Class("_fluid-fat-image-link-v2_style_fluidFatImageLinkBody__1LsOX")
    }
    if current_matches.is_bit_set(476) {
        child_states.set_bit(477); // active_Class("_fluid-fat-image-link-v2_style_fluidFatImageLink__1nw4J")
    }
    if current_matches.is_bit_set(478) {
        child_states.set_bit(479); // active_Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2SOMr")
    }
    if current_matches.is_bit_set(480) {
        child_states.set_bit(481); // active_Class("_fluid-fat-image-link-v2_style_fluidImageContainer__2vGwp")
    }
    if current_matches.is_bit_set(482) {
        child_states.set_bit(483); // active_Class("_fluid-fat-image-link-v2_style_fluidLandscapeImage__2euAK")
    }
    if current_matches.is_bit_set(484) {
        child_states.set_bit(485); // active_Class("_fluid-fat-image-link-v2_style_fluidPortraitImage__2SAYm")
    }
    if current_matches.is_bit_set(486) {
        child_states.set_bit(487); // active_Class("_fluid-fat-image-link-v2_style_four-pack__1ufgr")
    }
    if current_matches.is_bit_set(488) {
        child_states.set_bit(489); // active_Class("_fluid-fat-image-link-v2_style_gw-hero-close-button__3svyZ")
    }
    if current_matches.is_bit_set(490) {
        child_states.set_bit(491); // active_Class("_fluid-fat-image-link-v2_style_gwm-link-footer__3OF47")
    }
    if current_matches.is_bit_set(492) {
        child_states.set_bit(493); // active_Class("_fluid-fat-image-link-v2_style_haulRibbon__3VZNi")
    }
    if current_matches.is_bit_set(494) {
        child_states.set_bit(495); // active_Class("_fluid-fat-image-link-v2_style_header-icon__2cuVV")
    }
    if current_matches.is_bit_set(496) {
        child_states.set_bit(497); // active_Class("_fluid-fat-image-link-v2_style_header-link__cUhOK")
    }
    if current_matches.is_bit_set(498) {
        child_states.set_bit(499); // active_Class("_fluid-fat-image-link-v2_style_header__1vGdj")
    }
    if current_matches.is_bit_set(500) {
        child_states.set_bit(501); // active_Class("_fluid-fat-image-link-v2_style_image-container__2OiZA")
    }
    if current_matches.is_bit_set(502) {
        child_states.set_bit(503); // active_Class("_fluid-fat-image-link-v2_style_imageLabel__3ANSV")
    }
    if current_matches.is_bit_set(504) {
        child_states.set_bit(505); // active_Class("_fluid-fat-image-link-v2_style_inlineErrorDetails__1NBx-")
    }
    if current_matches.is_bit_set(506) {
        child_states.set_bit(507); // active_Class("_fluid-fat-image-link-v2_style_logoGap__nKNZ9")
    }
    if current_matches.is_bit_set(508) {
        child_states.set_bit(509); // active_Class("_fluid-fat-image-link-v2_style_logoRectangle__1VJwu")
    }
    if current_matches.is_bit_set(510) {
        child_states.set_bit(511); // active_Class("_fluid-fat-image-link-v2_style_logoSquareContainer__3Paoc")
    }
    if current_matches.is_bit_set(512) {
        child_states.set_bit(513); // active_Class("_fluid-fat-image-link-v2_style_logoSquare__3NZyi")
    }
    if current_matches.is_bit_set(514) {
        child_states.set_bit(515); // active_Class("_fluid-fat-image-link-v2_style_logo__2ZQ-N")
    }
    if current_matches.is_bit_set(516) {
        child_states.set_bit(517); // active_Class("_fluid-fat-image-link-v2_style_mergedLinksCta__3Npog")
    }
    if current_matches.is_bit_set(518) {
        child_states.set_bit(519); // active_Class("_fluid-fat-image-link-v2_style_mergedLinks__10JqZ")
    }
    if current_matches.is_bit_set(520) {
        child_states.set_bit(521); // active_Class("_fluid-fat-image-link-v2_style_mixed-button__2og-m")
    }
    if current_matches.is_bit_set(522) {
        child_states.set_bit(523); // active_Class("_fluid-fat-image-link-v2_style_mobile-close-button__3PB07")
    }
    if current_matches.is_bit_set(524) {
        child_states.set_bit(525); // active_Class("_fluid-fat-image-link-v2_style_mosaic-card-body__1HmTs")
    }
    if current_matches.is_bit_set(526) {
        child_states.set_bit(527); // active_Class("_fluid-fat-image-link-v2_style_mosaic-card__1C-_R")
    }
    if current_matches.is_bit_set(528) {
        child_states.set_bit(529); // active_Class("_fluid-fat-image-link-v2_style_negative-button__1Dvqz")
    }
    if current_matches.is_bit_set(530) {
        child_states.set_bit(531); // active_Class("_fluid-fat-image-link-v2_style_negativeMarginAdjust__1nqu9")
    }
    if current_matches.is_bit_set(532) {
        child_states.set_bit(533); // active_Class("_fluid-fat-image-link-v2_style_oneLineTruncation__2WWse")
    }
    if current_matches.is_bit_set(534) {
        child_states.set_bit(535); // active_Class("_fluid-fat-image-link-v2_style_overlay__3Sx3u")
    }
    if current_matches.is_bit_set(536) {
        child_states.set_bit(537); // active_Class("_fluid-fat-image-link-v2_style_positive-button__3UOC3")
    }
    if current_matches.is_bit_set(538) {
        child_states.set_bit(539); // active_Class("_fluid-fat-image-link-v2_style_poster-image__1W0yA")
    }
    if current_matches.is_bit_set(540) {
        child_states.set_bit(541); // active_Class("_fluid-fat-image-link-v2_style_smartText__ubpEw")
    }
    if current_matches.is_bit_set(542) {
        child_states.set_bit(543); // active_Class("_fluid-fat-image-link-v2_style_spCSRFTreatment__-hwVO")
    }
    if current_matches.is_bit_set(544) {
        child_states.set_bit(545); // active_Class("_fluid-fat-image-link-v2_style_spacer__7Pyg3")
    }
    if current_matches.is_bit_set(546) {
        child_states.set_bit(547); // active_Class("_fluid-fat-image-link-v2_style_stacking-context__3PbQE")
    }
    if current_matches.is_bit_set(548) {
        child_states.set_bit(549); // active_Class("_fluid-fat-image-link-v2_style_theming-background-override__1HfzJ")
    }
    if current_matches.is_bit_set(550) {
        child_states.set_bit(551); // active_Class("_fluid-fat-image-link-v2_style_themingTextColorWhite__1zryO")
    }
    if current_matches.is_bit_set(552) {
        child_states.set_bit(553); // active_Class("_fluid-fat-image-link-v2_style_themingTextColor__1oQsI")
    }
    if current_matches.is_bit_set(554) {
        child_states.set_bit(555); // active_Class("_fluid-fat-image-link-v2_style_three-pack__5s3hP")
    }
    if current_matches.is_bit_set(556) {
        child_states.set_bit(557); // active_Class("_fluid-fat-image-link-v2_style_threeLineTruncation__UkUjj")
    }
    if current_matches.is_bit_set(558) {
        child_states.set_bit(559); // active_Class("_fluid-fat-image-link-v2_style_tile-container__1QgAV")
    }
    if current_matches.is_bit_set(560) {
        child_states.set_bit(561); // active_Class("_fluid-fat-image-link-v2_style_tile-grid__QMxNY")
    }
    if current_matches.is_bit_set(562) {
        child_states.set_bit(563); // active_Class("_fluid-fat-image-link-v2_style_tile-link__38lTa")
    }
    if current_matches.is_bit_set(564) {
        child_states.set_bit(565); // active_Class("_fluid-fat-image-link-v2_style_tile-theming__3eeyj")
    }
    if current_matches.is_bit_set(566) {
        child_states.set_bit(567); // active_Class("_fluid-fat-image-link-v2_style_truncation__x9-69")
    }
    if current_matches.is_bit_set(568) {
        child_states.set_bit(569); // active_Class("_fluid-fat-image-link-v2_style_twoLineTruncation__16TLV")
    }
    if current_matches.is_bit_set(570) {
        child_states.set_bit(571); // active_Class("_fluid-fat-image-link-v2_style_video-container__1hKS1")
    }
    if current_matches.is_bit_set(572) {
        child_states.set_bit(573); // active_Class("_fluid-fat-image-link-v2_style_wd-backdrop-data__1znxG")
    }
    if current_matches.is_bit_set(574) {
        child_states.set_bit(575); // active_Class("_fluid-fat-image-link-v2_style_wdHeader__Edrev")
    }
    if current_matches.is_bit_set(576) {
        child_states.set_bit(577); // active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2")
    }
    if current_matches.is_bit_set(578) {
        child_states.set_bit(579); // active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3")
    }
    if current_matches.is_bit_set(580) {
        child_states.set_bit(581); // active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P")
    }
    if current_matches.is_bit_set(582) {
        child_states.set_bit(583); // active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK")
    }
    if current_matches.is_bit_set(584) {
        child_states.set_bit(585); // active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-rating__3_0eN")
    }
    if current_matches.is_bit_set(586) {
        child_states.set_bit(587); // active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-shape__1IcJY")
    }
    if current_matches.is_bit_set(588) {
        child_states.set_bit(589); // active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-badge-standard__28gp8")
    }
    if current_matches.is_bit_set(590) {
        child_states.set_bit(591); // active_Class("_fluid-quad-image-label-v2_energy-efficiency_energy-efficiency-container__1Pkva")
    }
    if current_matches.is_bit_set(592) {
        child_states.set_bit(593); // active_Class("_fluid-quad-image-label-v2_image_asin-container-white-box__QwmgO")
    }
    if current_matches.is_bit_set(594) {
        child_states.set_bit(595); // active_Class("_fluid-quad-image-label-v2_image_asin-container__2jyCM")
    }
    if current_matches.is_bit_set(596) {
        child_states.set_bit(597); // active_Class("_fluid-quad-image-label-v2_image_round-corners__2y_fS")
    }
    if current_matches.is_bit_set(598) {
        child_states.set_bit(599); // active_Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner-rtl__2BoOY")
    }
    if current_matches.is_bit_set(600) {
        child_states.set_bit(601); // active_Class("_fluid-quad-image-label-v2_style_ad-feedback-loading-spinnner__1nmZw")
    }
    if current_matches.is_bit_set(602) {
        child_states.set_bit(603); // active_Class("_fluid-quad-image-label-v2_style_ad-feedback-primary-link__2bIZi")
    }
    if current_matches.is_bit_set(604) {
        child_states.set_bit(605); // active_Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite-mobile__2_rj8")
    }
    if current_matches.is_bit_set(606) {
        child_states.set_bit(607); // active_Class("_fluid-quad-image-label-v2_style_ad-feedback-sprite__28uwB")
    }
    if current_matches.is_bit_set(608) {
        child_states.set_bit(609); // active_Class("_fluid-quad-image-label-v2_style_ad-feedback-text-desktop__q3xp_")
    }
    if current_matches.is_bit_set(610) {
        child_states.set_bit(611); // active_Class("_fluid-quad-image-label-v2_style_ad-feedback-text__2HjQ9")
    }
    if current_matches.is_bit_set(612) {
        child_states.set_bit(613); // active_Class("_fluid-quad-image-label-v2_style_apexBadgeLabel__2-Vye")
    }
    if current_matches.is_bit_set(614) {
        child_states.set_bit(615); // active_Class("_fluid-quad-image-label-v2_style_apexBadgeMessage__1tHvd")
    }
    if current_matches.is_bit_set(616) {
        child_states.set_bit(617); // active_Class("_fluid-quad-image-label-v2_style_aspect-button-group__1LqUG")
    }
    if current_matches.is_bit_set(618) {
        child_states.set_bit(619); // active_Class("_fluid-quad-image-label-v2_style_aspect-button__7cH_E")
    }
    if current_matches.is_bit_set(620) {
        child_states.set_bit(621); // active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-1236x1080__3aEzl")
    }
    if current_matches.is_bit_set(622) {
        child_states.set_bit(623); // active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-15x3__1h649")
    }
    if current_matches.is_bit_set(624) {
        child_states.set_bit(625); // active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-16x9__cBPv8")
    }
    if current_matches.is_bit_set(626) {
        child_states.set_bit(627); // active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-4x3__3BewI")
    }
    if current_matches.is_bit_set(628) {
        child_states.set_bit(629); // active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-5x8__2IaNz")
    }
    if current_matches.is_bit_set(630) {
        child_states.set_bit(631); // active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-dynamic-60vh__3N5g_")
    }
    if current_matches.is_bit_set(632) {
        child_states.set_bit(633); // active_Class("_fluid-quad-image-label-v2_style_aspect-ratio-fill__2Zjfb")
    }
    if current_matches.is_bit_set(634) {
        child_states.set_bit(635); // active_Class("_fluid-quad-image-label-v2_style_aspect-text__S4PU1")
    }
    if current_matches.is_bit_set(636) {
        child_states.set_bit(637); // active_Class("_fluid-quad-image-label-v2_style_autoplay-span__2CMfc")
    }
    if current_matches.is_bit_set(638) {
        child_states.set_bit(639); // active_Class("_fluid-quad-image-label-v2_style_badge-container__20aJ2")
    }
    if current_matches.is_bit_set(640) {
        child_states.set_bit(641); // active_Class("_fluid-quad-image-label-v2_style_badgeLabel__pJ5rc")
    }
    if current_matches.is_bit_set(642) {
        child_states.set_bit(643); // active_Class("_fluid-quad-image-label-v2_style_badgeMessage__2Dtw7")
    }
    if current_matches.is_bit_set(644) {
        child_states.set_bit(645); // active_Class("_fluid-quad-image-label-v2_style_carouselContainer__3N7M1")
    }
    if current_matches.is_bit_set(646) {
        child_states.set_bit(647); // active_Class("_fluid-quad-image-label-v2_style_centerImage__30wh-")
    }
    if current_matches.is_bit_set(648) {
        child_states.set_bit(649); // active_Class("_fluid-quad-image-label-v2_style_close-black-icon__3hkbe")
    }
    if current_matches.is_bit_set(650) {
        child_states.set_bit(651); // active_Class("_fluid-quad-image-label-v2_style_close-icon-wrapper__1zvdC")
    }
    if current_matches.is_bit_set(652) {
        child_states.set_bit(653); // active_Class("_fluid-quad-image-label-v2_style_close-icon__2RJs3")
    }
    if current_matches.is_bit_set(654) {
        child_states.set_bit(655); // active_Class("_fluid-quad-image-label-v2_style_close-text__2-gwn")
    }
    if current_matches.is_bit_set(656) {
        child_states.set_bit(657); // active_Class("_fluid-quad-image-label-v2_style_cover-portrait-image__2lhzL")
    }
    if current_matches.is_bit_set(658) {
        child_states.set_bit(659); // active_Class("_fluid-quad-image-label-v2_style_cta-link__2xo74")
    }
    if current_matches.is_bit_set(660) {
        child_states.set_bit(661); // active_Class("_fluid-quad-image-label-v2_style_desktop-close-button__1iL_P")
    }
    if current_matches.is_bit_set(662) {
        child_states.set_bit(663); // active_Class("_fluid-quad-image-label-v2_style_displayCount__1MVut")
    }
    if current_matches.is_bit_set(664) {
        child_states.set_bit(665); // active_Class("_fluid-quad-image-label-v2_style_dynamic-portrait-image__1Wrzd")
    }
    if current_matches.is_bit_set(666) {
        child_states.set_bit(667); // active_Class("_fluid-quad-image-label-v2_style_empty-footer__2d59h")
    }
    if current_matches.is_bit_set(668) {
        child_states.set_bit(669); // active_Class("_fluid-quad-image-label-v2_style_five-pack__1-Tql")
    }
    if current_matches.is_bit_set(670) {
        child_states.set_bit(671); // active_Class("_fluid-quad-image-label-v2_style_fluid-landscape-image__TE6PT")
    }
    if current_matches.is_bit_set(672) {
        child_states.set_bit(673); // active_Class("_fluid-quad-image-label-v2_style_fluidImageContainer__2SOMr")
    }
    if current_matches.is_bit_set(674) {
        child_states.set_bit(675); // active_Class("_fluid-quad-image-label-v2_style_fluidLandscapeImage__2euAK")
    }
    if current_matches.is_bit_set(676) {
        child_states.set_bit(677); // active_Class("_fluid-quad-image-label-v2_style_fluidPortraitImage__2SAYm")
    }
    if current_matches.is_bit_set(678) {
        child_states.set_bit(679); // active_Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabelBody__3tld0")
    }
    if current_matches.is_bit_set(680) {
        child_states.set_bit(681); // active_Class("_fluid-quad-image-label-v2_style_fluidQuadImageLabel__3b-Iv")
    }
    if current_matches.is_bit_set(682) {
        child_states.set_bit(683); // active_Class("_fluid-quad-image-label-v2_style_four-pack__1ufgr")
    }
    if current_matches.is_bit_set(684) {
        child_states.set_bit(685); // active_Class("_fluid-quad-image-label-v2_style_gridRowOne__1t0zL")
    }
    if current_matches.is_bit_set(686) {
        child_states.set_bit(687); // active_Class("_fluid-quad-image-label-v2_style_gridRowTwo__15woW")
    }
    if current_matches.is_bit_set(688) {
        child_states.set_bit(689); // active_Class("_fluid-quad-image-label-v2_style_gw-hero-close-button__3svyZ")
    }
    if current_matches.is_bit_set(690) {
        child_states.set_bit(691); // active_Class("_fluid-quad-image-label-v2_style_gwm-link-footer__3OF47")
    }
    if current_matches.is_bit_set(692) {
        child_states.set_bit(693); // active_Class("_fluid-quad-image-label-v2_style_haulRibbon__3VZNi")
    }
    if current_matches.is_bit_set(694) {
        child_states.set_bit(695); // active_Class("_fluid-quad-image-label-v2_style_header-icon__2cuVV")
    }
    if current_matches.is_bit_set(696) {
        child_states.set_bit(697); // active_Class("_fluid-quad-image-label-v2_style_header-link__cUhOK")
    }
    if current_matches.is_bit_set(698) {
        child_states.set_bit(699); // active_Class("_fluid-quad-image-label-v2_style_header__1vGdj")
    }
    if current_matches.is_bit_set(700) {
        child_states.set_bit(701); // active_Class("_fluid-quad-image-label-v2_style_image-container__2OiZA")
    }
    if current_matches.is_bit_set(702) {
        child_states.set_bit(703); // active_Class("_fluid-quad-image-label-v2_style_imageLabel__3ANSV")
    }
    if current_matches.is_bit_set(704) {
        child_states.set_bit(705); // active_Class("_fluid-quad-image-label-v2_style_inlineErrorDetails__1NBx-")
    }
    if current_matches.is_bit_set(706) {
        child_states.set_bit(707); // active_Class("_fluid-quad-image-label-v2_style_leftQuadrant__21nVp")
    }
    if current_matches.is_bit_set(708) {
        child_states.set_bit(709); // active_Class("_fluid-quad-image-label-v2_style_logoGap__nKNZ9")
    }
    if current_matches.is_bit_set(710) {
        child_states.set_bit(711); // active_Class("_fluid-quad-image-label-v2_style_logoRectangle__1VJwu")
    }
    if current_matches.is_bit_set(712) {
        child_states.set_bit(713); // active_Class("_fluid-quad-image-label-v2_style_logoSquareContainer__3Paoc")
    }
    if current_matches.is_bit_set(714) {
        child_states.set_bit(715); // active_Class("_fluid-quad-image-label-v2_style_logoSquare__3NZyi")
    }
    if current_matches.is_bit_set(716) {
        child_states.set_bit(717); // active_Class("_fluid-quad-image-label-v2_style_logo__2ZQ-N")
    }
    if current_matches.is_bit_set(718) {
        child_states.set_bit(719); // active_Class("_fluid-quad-image-label-v2_style_mixed-button__2og-m")
    }
    if current_matches.is_bit_set(720) {
        child_states.set_bit(721); // active_Class("_fluid-quad-image-label-v2_style_mobile-close-button__3PB07")
    }
    if current_matches.is_bit_set(722) {
        child_states.set_bit(723); // active_Class("_fluid-quad-image-label-v2_style_mosaic-card-body__1HmTs")
    }
    if current_matches.is_bit_set(724) {
        child_states.set_bit(725); // active_Class("_fluid-quad-image-label-v2_style_mosaic-card__1C-_R")
    }
    if current_matches.is_bit_set(726) {
        child_states.set_bit(727); // active_Class("_fluid-quad-image-label-v2_style_negative-button__1Dvqz")
    }
    if current_matches.is_bit_set(728) {
        child_states.set_bit(729); // active_Class("_fluid-quad-image-label-v2_style_negativeMarginAdjust__1nqu9")
    }
    if current_matches.is_bit_set(730) {
        child_states.set_bit(731); // active_Class("_fluid-quad-image-label-v2_style_oneLineTruncation__2WWse")
    }
    if current_matches.is_bit_set(732) {
        child_states.set_bit(733); // active_Class("_fluid-quad-image-label-v2_style_overlay__3Sx3u")
    }
    if current_matches.is_bit_set(734) {
        child_states.set_bit(735); // active_Class("_fluid-quad-image-label-v2_style_positive-button__3UOC3")
    }
    if current_matches.is_bit_set(736) {
        child_states.set_bit(737); // active_Class("_fluid-quad-image-label-v2_style_poster-image__1W0yA")
    }
    if current_matches.is_bit_set(738) {
        child_states.set_bit(739); // active_Class("_fluid-quad-image-label-v2_style_quadrantContainer__3TMqG")
    }
    if current_matches.is_bit_set(740) {
        child_states.set_bit(741); // active_Class("_fluid-quad-image-label-v2_style_rightQuadrant__PI01n")
    }
    if current_matches.is_bit_set(742) {
        child_states.set_bit(743); // active_Class("_fluid-quad-image-label-v2_style_smartText__ubpEw")
    }
    if current_matches.is_bit_set(744) {
        child_states.set_bit(745); // active_Class("_fluid-quad-image-label-v2_style_spCSRFTreatment__-hwVO")
    }
    if current_matches.is_bit_set(746) {
        child_states.set_bit(747); // active_Class("_fluid-quad-image-label-v2_style_spacer__7Pyg3")
    }
    if current_matches.is_bit_set(748) {
        child_states.set_bit(749); // active_Class("_fluid-quad-image-label-v2_style_stacking-context__3PbQE")
    }
    if current_matches.is_bit_set(750) {
        child_states.set_bit(751); // active_Class("_fluid-quad-image-label-v2_style_theming-background-override__1HfzJ")
    }
    if current_matches.is_bit_set(752) {
        child_states.set_bit(753); // active_Class("_fluid-quad-image-label-v2_style_themingTextColorWhite__1zryO")
    }
    if current_matches.is_bit_set(754) {
        child_states.set_bit(755); // active_Class("_fluid-quad-image-label-v2_style_themingTextColor__1oQsI")
    }
    if current_matches.is_bit_set(756) {
        child_states.set_bit(757); // active_Class("_fluid-quad-image-label-v2_style_three-pack__5s3hP")
    }
    if current_matches.is_bit_set(758) {
        child_states.set_bit(759); // active_Class("_fluid-quad-image-label-v2_style_threeLineTruncation__UkUjj")
    }
    if current_matches.is_bit_set(760) {
        child_states.set_bit(761); // active_Class("_fluid-quad-image-label-v2_style_tile-container__1QgAV")
    }
    if current_matches.is_bit_set(762) {
        child_states.set_bit(763); // active_Class("_fluid-quad-image-label-v2_style_tile-grid__QMxNY")
    }
    if current_matches.is_bit_set(764) {
        child_states.set_bit(765); // active_Class("_fluid-quad-image-label-v2_style_tile-link__38lTa")
    }
    if current_matches.is_bit_set(766) {
        child_states.set_bit(767); // active_Class("_fluid-quad-image-label-v2_style_tile-theming__3eeyj")
    }
    if current_matches.is_bit_set(768) {
        child_states.set_bit(769); // active_Class("_fluid-quad-image-label-v2_style_truncation__x9-69")
    }
    if current_matches.is_bit_set(770) {
        child_states.set_bit(771); // active_Class("_fluid-quad-image-label-v2_style_twoLineTruncation__16TLV")
    }
    if current_matches.is_bit_set(772) {
        child_states.set_bit(773); // active_Class("_fluid-quad-image-label-v2_style_video-container__1hKS1")
    }
    if current_matches.is_bit_set(774) {
        child_states.set_bit(775); // active_Class("_fluid-quad-image-label-v2_style_wd-backdrop-data__1znxG")
    }
    if current_matches.is_bit_set(776) {
        child_states.set_bit(777); // active_Class("_fluid-quad-image-label-v2_style_wdHeader__Edrev")
    }
    if current_matches.is_bit_set(778) {
        child_states.set_bit(779); // active_Class("_quad-category-card_desktopStyle_cardBody__3Rdh1")
    }
    if current_matches.is_bit_set(780) {
        child_states.set_bit(781); // active_Class("_quad-category-card_desktopStyle_categoryImage__35jKN")
    }
    if current_matches.is_bit_set(782) {
        child_states.set_bit(783); // active_Class("_quad-category-card_desktopStyle_category__3flCQ")
    }
    if current_matches.is_bit_set(784) {
        child_states.set_bit(785); // active_Class("_quad-category-card_desktopStyle_heroCategory__3KS3k")
    }
    if current_matches.is_bit_set(786) {
        child_states.set_bit(787); // active_Class("_quad-category-card_desktopStyle_heroImage__2V8-9")
    }
    if current_matches.is_bit_set(788) {
        child_states.set_bit(789); // active_Class("_quad-category-card_desktopStyle_heroLink__1EhW2")
    }
    if current_matches.is_bit_set(790) {
        child_states.set_bit(791); // active_Class("_quad-category-card_desktopStyle_leftMost__1LmQB")
    }
    if current_matches.is_bit_set(792) {
        child_states.set_bit(793); // active_Class("_quad-category-card_fluid_fluidCardBody__3TzJ4")
    }
    if current_matches.is_bit_set(794) {
        child_states.set_bit(795); // active_Class("_quad-category-card_fluid_fluidCard__3hmFA")
    }
    if current_matches.is_bit_set(796) {
        child_states.set_bit(797); // active_Class("_quad-category-card_image_asin-container-full-height__MOKlF")
    }
    if current_matches.is_bit_set(798) {
        child_states.set_bit(799); // active_Class("_quad-category-card_image_asin-container-white-box__3Stwp")
    }
    if current_matches.is_bit_set(800) {
        child_states.set_bit(801); // active_Class("_quad-category-card_image_asin-container__LRY5p")
    }
    if current_matches.is_bit_set(802) {
        child_states.set_bit(803); // active_Class("_quad-category-card_image_round-corners__22iOW")
    }
    if current_matches.is_bit_set(804) {
        child_states.set_bit(805); // active_Class("_quad-category-card_mobileStyle_cardBody__3ODbW")
    }
    if current_matches.is_bit_set(806) {
        child_states.set_bit(807); // active_Class("_quad-category-card_mobileStyle_categoryContainer__2xY0I")
    }
    if current_matches.is_bit_set(808) {
        child_states.set_bit(809); // active_Class("_quad-category-card_mobileStyle_categoryImage__3hSFw")
    }
    if current_matches.is_bit_set(810) {
        child_states.set_bit(811); // active_Class("_quad-category-card_mobileStyle_category__1amt4")
    }
    if current_matches.is_bit_set(812) {
        child_states.set_bit(813); // active_Class("_quad-category-card_mobileStyle_heroImage__1SewP")
    }
    if current_matches.is_bit_set(814) {
        child_states.set_bit(815); // active_Class("_quad-category-card_mobileStyle_leftMost__3WtU6")
    }
    if current_matches.is_bit_set(816) {
        child_states.set_bit(817); // active_Class("_quad-category-card_style_dashboard-card-with-border__1e4z_")
    }
    if current_matches.is_bit_set(818) {
        child_states.set_bit(819); // active_Class("_quad-category-card_style_fluidImageContainer__2jd50")
    }
    if current_matches.is_bit_set(820) {
        child_states.set_bit(821); // active_Class("_quad-category-card_style_fluidLandscapeImage__3eTVC")
    }
    if current_matches.is_bit_set(822) {
        child_states.set_bit(823); // active_Class("_quad-category-card_style_fluidPortraitImage__3yQ-X")
    }
    if current_matches.is_bit_set(824) {
        child_states.set_bit(825); // active_Class("_quad-category-card_style_gwm-link-footer__3EX7d")
    }
    if current_matches.is_bit_set(826) {
        child_states.set_bit(827); // active_Class("_quad-category-card_style_heading__1mnEu")
    }
    if current_matches.is_bit_set(828) {
        child_states.set_bit(829); // active_Class("_text-link-stripe-v2_style_textlinkstripe__3aQhz")
    }
    if current_matches.is_bit_set(830) {
        child_states.set_bit(831); // active_Class("a-cardui-body")
    }
    if current_matches.is_bit_set(832) {
        child_states.set_bit(833); // active_Class("a-cardui-footer")
    }
    if current_matches.is_bit_set(834) {
        child_states.set_bit(835); // active_Class("a-cardui-header")
    }
    if current_matches.is_bit_set(836) {
        child_states.set_bit(837); // active_Class("a-carousel-container")
    }
    if current_matches.is_bit_set(838) {
        child_states.set_bit(839); // active_Class("a-carousel-controls")
    }
    if current_matches.is_bit_set(840) {
        child_states.set_bit(841); // active_Class("a-carousel-right")
    }
    if current_matches.is_bit_set(842) {
        child_states.set_bit(843); // active_Class("a-carousel-viewport")
    }
    if current_matches.is_bit_set(844) {
        child_states.set_bit(845); // active_Class("a-link-normal")
    }
    if current_matches.is_bit_set(846) {
        child_states.set_bit(847); // active_Class("card-flow-row-break")
    }
    if current_matches.is_bit_set(848) {
        child_states.set_bit(849); // active_Class("gw-auto-height")
    }
    if current_matches.is_bit_set(850) {
        child_states.set_bit(851); // active_Class("gw-card-layout")
    }
    if current_matches.is_bit_set(852) {
        child_states.set_bit(853); // active_Class("gw-col")
    }
    if current_matches.is_bit_set(854) {
        child_states.set_bit(855); // active_Class("gw-fixed-col")
    }
    if current_matches.is_bit_set(856) {
        child_states.set_bit(857); // active_Class("gw-media-card")
    }
    if current_matches.is_bit_set(858) {
        child_states.set_bit(859); // active_Class("gw-row")
    }
    if current_matches.is_bit_set(860) {
        child_states.set_bit(861); // active_Class("nav-focus")
    }
    if current_matches.is_bit_set(862) {
        child_states.set_bit(863); // active_Class("nav-spinner")
    }
    if current_matches.is_bit_set(864) {
        child_states.set_bit(865); // active_Class("nav-timeline-prime-icon")
    }
    if current_matches.is_bit_set(866) {
        child_states.set_bit(867); // active_Class("single-slide-hero")
    }
    if current_matches.is_bit_set(868) {
        child_states.set_bit(869); // active_Class("truncate-1line")
    }
    if current_matches.is_bit_set(870) {
        child_states.set_bit(871); // active_Class("truncate-2line")
    }
    if current_matches.is_bit_set(872) {
        child_states.set_bit(873); // active_Class("vjs-fluid")
    }
    if current_matches.is_bit_set(874) {
        child_states.set_bit(875); // active_Id("icp-touch-link-cop")
    }
    if current_matches.is_bit_set(876) {
        child_states.set_bit(877); // active_Id("icp-touch-link-country")
    }
    if current_matches.is_bit_set(878) {
        child_states.set_bit(879); // active_Id("icp-touch-link-language")
    }
    if current_matches.is_bit_set(880) {
        child_states.set_bit(881); // active_Type("h3")
    }
    if current_matches.is_bit_set(882) {
        child_states.set_bit(883); // active_Type("span")
    }
    node.css_match_bitvector = current_matches;
    node.set_parent_state_cache_bitvector(parent_bits_read, parent_values_read);
    node.cached_child_states = Some(child_states.clone());
    node.mark_clean();

    child_states
}


/// BitVector-only incremental processing driver with statistics tracking
pub fn process_tree_bitvector(root: &mut HtmlNode) -> (usize, usize, usize) {
    let mut total_nodes = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let initial_state = BitVector::with_capacity(BITVECTOR_CAPACITY);
    process_tree_recursive_bitvector_incremental(root, &initial_state, &mut total_nodes, &mut cache_hits, &mut cache_misses);
    (total_nodes, cache_hits, cache_misses)
}

fn process_tree_recursive_bitvector_incremental(node: &mut HtmlNode, parent_state: &BitVector,
                                               total: &mut usize, hits: &mut usize, misses: &mut usize) {
    *total += 1;
    
    // Logic 1: Check if node itself needs recomputation using BitVector-only tracking
    let child_states = if node.needs_self_recomputation_bitvector(parent_state) {
        *misses += 1;
        // Recompute node and get fresh child_states
        process_node_generated_bitvector_incremental(node, parent_state)
    } else {
        *hits += 1;
        // Use cached child_states - major optimization for internal nodes!
        node.cached_child_states.clone().unwrap_or_else(|| BitVector::with_capacity(BITVECTOR_CAPACITY))
    };
    
    // Logic 2: Check if we need to recurse (only if there are dirty descendants)
    if node.has_dirty_descendant {
        // Recurse into children only if there are dirty descendants
        for child in node.children.iter_mut() {
            process_tree_recursive_bitvector_incremental(child, &child_states, total, hits, misses);
        }
    }
    // If no dirty descendants, skip entire subtree recursion - major optimization!
}
