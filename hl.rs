use core::libc::c_void;
use ll::*;

pub enum CssError {
    CssOk = 0,
    CssNomem = 1,
    CssBadParm = 2,
    CssInvalid = 3,
    CssFileNotFound = 4,
    CssNeedData = 5,
    CssBadCharset = 6,
    CssEof = 7,
    CssImportsPending = 8,
    CssPropertyNotSet = 9
}

pub enum CssLanguageLevel {
    CssLevel1 = 0,
    CssLevel2 = 1,
    CssLevel21 = 2,
    CssLevel3 = 3,
    CssLevelDefault = 99 // NB: This is not the same as the ll value
}

pub struct CssStylesheetParams<TResolvePw, TImportPw, TColorPw, TFontPw> {
    params_version: CssStylesheetParamsVersion,
    level: CssLanguageLevel,
    charset: ~str,
    url: ~str,
    title: ~str,
    allow_quirks: bool,
    inline_style: bool,
    resolve: Option<CssUrlResolutionFn<TResolvePw>>,
    resolve_pw: Option<TResolvePw>,
    import: Option<CssImportNotificationFn<TImportPw>>,
    import_pw: Option<TImportPw>,
    color: Option<CssColorResolutionFn<TColorPw>>,
    color_pw: Option<TColorPw>,
    font: Option<CssFontResolutionFn<TFontPw>>,
    font_pw: Option<TFontPw>
}

pub enum CssStylesheetParamsVersion {
    CssStylesheetParamsVersion1 = 1
}

// FIXME: Need hl reprs of lwc_string
pub type CssUrlResolutionFn<TResolvePw> = &fn(pw: &TResolvePw, base: &str, rel: &lwc_string, abs: & &lwc_string) -> CssError;
pub type CssImportNotificationFn<TImportPw> = &fn(pw: &TImportPw, parent: &CssStylesheet, url: &lwc_string, media: &uint64_t) -> CssError;
pub type CssColorResolutionFn<TColorPw> = &fn(pw: &TColorPw, name: &lwc_string, color: &CssColor) -> CssError;
pub type CssFontResolutionFn<TFontPw> = &fn(pw: &TFontPw, name: &lwc_string, system_font: &CssSystemFont) -> CssError;

pub struct CssColor { r: u8, g: u8, b: u8, a: u8 }

pub type CssStylesheet = c_void;

pub struct CssSystemFont {
    style: CssFontStyle,
    variant: CssFontVariant,
    weight: CssFontWeight,
    size: CssSize,
    line_height: CssSize,
    family: ~str
}

pub struct CssSize {
    size: CssFixed,
    unit: CssUnit
}

pub enum CssFontStyle {
    CssFontStyleInherit = 0,
    CssFontStyleNormal = 1,
    CssFontStyleItalic = 2,
    CssFontStyleOblique = 3
}

pub enum CssFontVariant {
    CssFontVariantInherit = 0,
    CssFontVariantNormal = 1,
    CssFontVariantSmallCaps = 2
}

pub enum CssFontWeight {
    CssFontWeightInherit = 0x0,
    CssFontWeightNormal = 0x1,
    CssFontWeightBold = 0x2,
    CssFontWeightBolder = 0x3,
    CssFontWeightLighter = 0x4,
    CssFontWeight100 = 0x5,
    CssFontWeight200 = 0x6,
    CssFontWeight300 = 0x7,
    CssFontWeight400 = 0x8,
    CssFontWeight500 = 0x9,
    CssFontWeight600 = 0xa,
    CssFontWeight700 = 0xb,
    CssFontWeight800 = 0xc,
    CssFontWeight900 = 0xd,
}

pub type CssFixed = int32_t;

pub enum CssUnit {
    CssUnitPx = 0x0,
    CssUnitEx = 0x1,
    CssUnitEm = 0x2,
    CssUnitIn = 0x3,
    CssUnitCm = 0x4,
    CssUnitMm = 0x5,
    CssUnitPt = 0x6,
    CssUnitPc = 0x7,
    CssUnitPct = 0x8,
    CssUnitDeg = 0x9,
    CssUnitGrad = 0xa,
    CssUnitRad = 0xb,
    CssUnitMs = 0xc,
    CssUnitS = 0xd,
    CssUnitHz = 0xe,
    CssUnitKhz = 0xf
}