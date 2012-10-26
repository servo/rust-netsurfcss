use wapcaplet::ll::lwc_string;
use core::libc::{c_void, c_char};
use core::libc::types::common::c99::{uint32_t};
use cast::transmute;
use core::ptr::null;
use ll::{c_enum, rust_enum};
use ll::types::*;
use hl::types::*;
use ll::stylesheet::*;
use hl::stylesheet::*;
use ll::errors::*;
use hl::errors::*;
use ll::properties::*;
use hl::properties::*;

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
            CssLevelNotACLikeEnum(*) => fail
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

extern fn resolve(_pw: *c_void, _base: *c_char, _rel: *lwc_string, _abs: **lwc_string) -> css_error {
    fail ~"css resolve function called";
}
