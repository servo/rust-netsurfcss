use wapcaplet::LwcString;
use wapcaplet::ll::lwc_string;
use core::libc::{c_void, c_char};
use core::libc::types::common::c99::{uint32_t};
use cast::transmute;
use core::ptr::null;
use ll::{c_enum, rust_enum};
use ll_lwcstr_to_hl_lwcstr = wapcaplet::from_lwc_string;
use ll::types::{css_language_level, CSS_LEVEL_1, CSS_LEVEL_2, CSS_LEVEL_21, CSS_LEVEL_3, CSS_LEVEL_DEFAULT};
use types::{CssLanguageLevel, CssLevel1, CssLevel2, CssLevel21, CssLevel3, CssLevelDefault, CssLevelNotACLikeEnum};
use ll::errors::{css_error, CSS_OK};
use errors::CssError;
use ll::properties::css_font_family_e;
use properties::CssFontFamily;
use ll::types::{css_color, css_unit, css_qname};
use types::{CssColor, CssUnit, CssQName};
use ll::stylesheet::{css_fixed, css_stylesheet_params};
use stylesheet::{CssStylesheetParams};
use ll::select::{css_pseudo_element};
use select::{CssPseudoElement};

pub trait ToLl<T> {
    fn to_ll(&self) -> T;
}

pub trait AsLl<T> {
    fn as_ll<U>(&self, f: fn(&T) -> U) -> U;
}

pub impl CssLanguageLevel: ToLl<css_language_level> {
    pub fn to_ll(&self) -> css_language_level {
        match *self {
            CssLevel1 => CSS_LEVEL_1,
            CssLevel2 => CSS_LEVEL_2,
            CssLevel21 => CSS_LEVEL_21,
            CssLevel3 => CSS_LEVEL_3,
            CssLevelDefault => CSS_LEVEL_DEFAULT,
            CssLevelNotACLikeEnum(*) => fail!()
        }
    }
}

pub impl CssError: ToLl<css_error> {
    pub fn to_ll(&self) -> css_error {
        *self as css_error
    }
}

pub impl CssFontFamily: ToLl<css_font_family_e> {
    pub fn to_ll(&self) -> css_font_family_e {
        *self as css_font_family_e
    }
}

pub impl CssColor: ToLl<css_color> {
    pub fn to_ll(&self) -> css_color {
        assert sys::size_of::<CssColor>() == sys::size_of::<css_color>();
        unsafe { transmute(*self) }
    }
}

pub fn ll_color_to_hl_color(color: css_color) -> CssColor {
    assert sys::size_of::<CssColor>() == sys::size_of::<css_color>();
    unsafe { transmute(color) }
}

pub impl CssUnit: ToLl<(css_unit, css_fixed)> {
    pub fn to_ll(&self) -> (css_unit, css_fixed) {
        use ll::types::*;
        use types::*;
        match *self {
            CssUnitPx(value) => (CSS_UNIT_PX, value),
            CssUnitEx(value) => (CSS_UNIT_EX, value),
            CssUnitEm(value) => (CSS_UNIT_EM, value),
            CssUnitIn(value) => (CSS_UNIT_IN, value),
            CssUnitCm(value) => (CSS_UNIT_CM, value),
            CssUnitMm(value) => (CSS_UNIT_MM, value),
            CssUnitPt(value) => (CSS_UNIT_PT, value),
            CssUnitPc(value) => (CSS_UNIT_PC, value),
            CssUnitPct(value) => (CSS_UNIT_PCT, value),
            CssUnitDeg(value) => (CSS_UNIT_DEG, value),
            CssUnitGrad(value) => (CSS_UNIT_GRAD, value),
            CssUnitRad(value) => (CSS_UNIT_RAD, value),
            CssUnitMs(value) => (CSS_UNIT_MS, value),
            CssUnitS(value) => (CSS_UNIT_S, value),
            CssUnitHz(value) => (CSS_UNIT_HZ, value),
            CssUnitKHz(value) => (CSS_UNIT_KHZ, value)
        }
    }
}

pub fn ll_unit_to_hl_unit(unit: css_unit, value: css_fixed) -> CssUnit {
    use ll::types::*;
    use types::*;
    if unit == CSS_UNIT_PX {
        CssUnitPx(value)
    } else if unit == CSS_UNIT_EX {
        CssUnitEx(value)
    } else if unit == CSS_UNIT_EM {
        CssUnitEm(value)
    } else if unit == CSS_UNIT_IN {
        CssUnitIn(value)
    } else if unit == CSS_UNIT_CM {
        CssUnitCm(value)
    } else if unit == CSS_UNIT_MM {
        CssUnitMm(value)
    } else if unit == CSS_UNIT_PT {
        CssUnitPt(value)
    } else if unit == CSS_UNIT_PC {
        CssUnitPc(value)
    } else if unit == CSS_UNIT_PCT {
        CssUnitPct(value)
    } else if unit == CSS_UNIT_DEG {
        CssUnitDeg(value)
    } else if unit == CSS_UNIT_GRAD {
        CssUnitGrad(value)
    } else if unit == CSS_UNIT_RAD {
        CssUnitRad(value)
    } else if unit == CSS_UNIT_MS {
        CssUnitMs(value)
    } else if unit == CSS_UNIT_S {
        CssUnitS(value)
    } else if unit == CSS_UNIT_HZ {
        CssUnitHz(value)
    } else if unit == CSS_UNIT_KHZ {
        CssUnitKHz(value)
    } else {
        fail!()
    }
}

pub fn ll_qname_to_hl_qname(qname: *css_qname) -> CssQName {
    unsafe {
        CssQName {
            ns: if (*qname).ns.is_not_null() {
                Some(ll_lwcstr_to_hl_lwcstr((*qname).ns))
            } else {
                None
            },
            name: {
                assert (*qname).name.is_not_null();
                ll_lwcstr_to_hl_lwcstr((*qname).name)
            }
        }
    }
}

pub impl CssPseudoElement: ToLl<css_pseudo_element> {
    pub fn to_ll(&self) -> css_pseudo_element {
        *self as css_pseudo_element
    }
}

pub fn c_enum_to_rust_enum<T>(val: c_enum) -> T {
    // Sanity check that this is actually a 'c-like' (har) enum
    assert sys::size_of::<T>() == sys::size_of::<rust_enum>();
    unsafe { transmute(val as rust_enum) }
}

pub impl CssStylesheetParams: AsLl<css_stylesheet_params> {
    pub fn as_ll<U>(&self, f: fn(&css_stylesheet_params) -> U) -> U {
        do str::as_c_str(self.charset) |charset| {
            do str::as_c_str(self.url) |url| {
                do str::as_c_str(self.title) |title| {
                    let params = css_stylesheet_params {
                        params_version: self.params_version as uint32_t,
                        level: self.level.to_ll(),
                        charset: charset,
                        url: url,
                        title: title,
                        allow_quirks: self.allow_quirks,
                        inline_style: self.inline_style,
                        resolve: resolve,
                        resolve_pw: unsafe { transmute(&self.resolve) },
                        import: null(),
                        import_pw: null(),
                        color: null(),
                        color_pw: null(),
                        font: null(),
                        font_pw: null()
                    };
                    f(&params)
                }
            }
        }
    }
}

extern fn resolve(_pw: *c_void, _base: *c_char, rel: *lwc_string, abs: *mut *lwc_string) -> css_error {
    unsafe {
        // TODO
        *abs = rel;
    }
    CSS_OK
}

pub fn write_ll_qname(hlqname: &CssQName, llqname: *css_qname) {
    unsafe {
        match &hlqname.ns {
            &Some(ref ns) => {
                (*llqname).ns = ns.raw_reffed();
            }
            _ => ()
        }
        (*llqname).name = hlqname.name.raw_reffed();
    }
}

pub fn lwc_string_buf_to_hl_vec(names: **lwc_string) -> ~[LwcString] {
    unsafe {
        let mut result = ~[];
        let mut names = names;
        while (*names).is_not_null() {
            result.push(ll_lwcstr_to_hl_lwcstr(*names));
            names = names.offset(1);
        }
        return move result;
    }
}
