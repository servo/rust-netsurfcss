use core::libc::c_void;
use ll::*;
use ptr::{null, to_unsafe_ptr, to_mut_unsafe_ptr};
use cast::transmute;

pub type CssStylesheet = c_void;

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

pub struct CssStylesheetParams {
    params_version: CssStylesheetParamsVersion,
    level: CssLanguageLevel,
    charset: ~str,
    url: ~str,
    title: ~str,
    allow_quirks: bool,
    inline_style: bool,
    resolve: Option<CssUrlResolutionFn>,
    import: Option<CssImportNotificationFn>,
    color: Option<CssColorResolutionFn>,
    font: Option<CssFontResolutionFn>,
}

pub enum CssStylesheetParamsVersion {
    CssStylesheetParamsVersion1 = 1
}

// FIXME: Need hl reprs of lwc_string
pub type CssUrlResolutionFn = @fn(base: &str, rel: &lwc_string, abs: & &lwc_string) -> CssError;
pub type CssImportNotificationFn = @fn(parent: &CssStylesheet, url: &lwc_string, media: &uint64_t) -> CssError;
pub type CssColorResolutionFn = @fn(name: &lwc_string, color: &CssColor) -> CssError;
pub type CssFontResolutionFn = @fn(name: &lwc_string, system_font: &CssSystemFont) -> CssError;

pub struct CssColor { r: u8, g: u8, b: u8, a: u8 }

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





type CssResult<T> = Result<T, CssError>;

pub struct CssStylesheetRef {
    priv sheet: *css_stylesheet,

    drop {
        css_stylesheet_destroy(self.sheet);
    }
}

fn CssStylesheetCreate(params: &CssStylesheetParams) -> CssResult<CssStylesheetRef> {
    do params.as_ll |ll_params| {
        let mut sheet: *css_stylesheet = null();
        let code = css_stylesheet_create(
            to_unsafe_ptr(ll_params), null(), null(), to_mut_unsafe_ptr(&mut sheet));

        match code {
            CSS_OK => {
                Ok(CssStylesheetRef {
                    sheet: move sheet
                })
            }
            _ => {
                Err(unsafe { transmute(code) })
            }
        }
    }
}