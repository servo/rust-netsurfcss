use core::libc::{c_char, c_void, size_t};
use core::libc::types::common::c99::{int32_t, uint32_t, uint64_t, uint8_t};

use wapcaplet::ll::lwc_string;

#[cfg(target_os = "macos")]
#[nolink]
#[link_args="-L../libcss -lcss -L../libparserutils -lparserutils -L../libwapcaplet -lwapcaplet -liconv"]
pub extern mod linking { }

#[cfg(target_os = "linux")]
#[nolink]
#[link_args="-L../libcss -lcss -L../libparserutils -lparserutils -L../libwapcaplet -lwapcaplet"]
pub extern mod linking { }

pub struct css_stylesheet_params {
    params_version: uint32_t,
    level: css_language_level,
    charset: *c_char,
    url: *c_char,
    title: *c_char,
    allow_quirks: bool,
    inline_style: bool,
    resolve: css_url_resolution_fn,
    resolve_pw: *c_void,
    import: css_import_notification_fn,
    import_pw: *c_void,
    color: css_color_resolution_fn,
    color_pw: *c_void,
    font: css_font_resolution_fn,
    font_pw: *c_void
}

const CSS_STYLESHEET_PARAMS_VERSION_1: uint32_t = 1;

type css_language_level = uint32_t;

const CSS_LEVEL_1: css_language_level = 0;
const CSS_LEVEL_2: css_language_level = 1;
const CSS_LEVEL_21: css_language_level = 2;
const CSS_LEVEL_3: css_language_level = 3;
const CSS_LEVEL_DEFAULT: css_language_level = 2;

type css_url_resolution_fn = *u8; //extern fn(pw: *c_void, base: *c_char, rel: *lwc_string, abs: **lwc_string) -> css_error;
type css_import_notification_fn = *u8; //extern fn(pw: *c_void, parent: *css_stylesheet, url: *lwc_string, media: *uint64_t) -> css_error;
type css_color_resolution_fn = *u8; //extern fn(pw: *c_void, name: *lwc_string, color: *css_color) -> css_error;
type css_font_resolution_fn = *u8; //extern fn(pw: *c_void, name: *lwc_string, system_font: *css_system_font) -> css_error;

enum css_error {
    CSS_OK = 0,
    CSS_NOMEM = 1,
    CSS_BADPARM = 2,
    CSS_INVALID = 3,
    CSS_FILENOTFOUND = 4,
    CSS_NEEDDATA = 5,
    CSS_BADCHARSET = 6,
    CSS_EOF = 7,
    CSS_IMPORTS_PENDING = 8,
    CSS_PROPERTY_NOT_SET = 9
}

type css_stylesheet = c_void;

type css_color = uint32_t;

struct css_system_font {
    style: css_font_style_e,
    variant: css_font_variant_e,
    weight: css_font_weight_e,
    size: css_size,
    line_height: css_size,
    family: *lwc_string
}

// This isn't in the libcss source. In C it is defined inline with css_system_font
struct css_size {
    size: css_fixed,
    unit: css_unit
}

enum css_font_style_e {
    CSS_FONT_STYLE_INHERIT = 0,
    CSS_FONT_STYLE_NORMAL = 1,
    CSS_FONT_STYLE_ITALIC = 2,
    CSS_FONT_STYLE_OBLIQUE = 3
}

enum css_font_variant_e {
    CSS_FONT_VARIANT_INHERIT = 0,
    CSS_FONT_VARIANT_NORMAL = 1,
    CSS_FONT_VARIANT_SMALL_CAPS = 2
}

enum css_font_weight_e {
    CSS_FONT_WEIGHT_INHERIT = 0x0,
    CSS_FONT_WEIGHT_NORMAL = 0x1,
    CSS_FONT_WEIGHT_BOLD = 0x2,
    CSS_FONT_WEIGHT_BOLDER = 0x3,
    CSS_FONT_WEIGHT_LIGHTER = 0x4,
    CSS_FONT_WEIGHT_100 = 0x5,
    CSS_FONT_WEIGHT_200 = 0x6,
    CSS_FONT_WEIGHT_300 = 0x7,
    CSS_FONT_WEIGHT_400 = 0x8,
    CSS_FONT_WEIGHT_500 = 0x9,
    CSS_FONT_WEIGHT_600 = 0xa,
    CSS_FONT_WEIGHT_700 = 0xb,
    CSS_FONT_WEIGHT_800 = 0xc,
    CSS_FONT_WEIGHT_900 = 0xd,
}

type css_fixed = int32_t;

enum css_unit {
    CSS_UNIT_PX = 0x0,
    CSS_UNIT_EX = 0x1,
    CSS_UNIT_EM = 0x2,
    CSS_UNIT_IN = 0x3,
    CSS_UNIT_CM = 0x4,
    CSS_UNIT_MM = 0x5,
    CSS_UNIT_PT = 0x6,
    CSS_UNIT_PC = 0x7,
    CSS_UNIT_PCT = 0x8,
    CSS_UNIT_DEG= 0x9,
    CSS_UNIT_GRAD = 0xa,
    CSS_UNIT_RAD = 0xb,
    CSS_UNIT_MS = 0xc,
    CSS_UNIT_S = 0xd,
    CSS_UNIT_HZ = 0xe,
    CSS_UNIT_KHZ = 0xf
}

// (ptr: *c_void, size: size_t, pw: *c_void)
type css_allocator_fn = *u8;

extern {
    fn css_stylesheet_create(params: *css_stylesheet_params,
                             alloc: css_allocator_fn,
                             alloc_pw: *c_void,
                             stylesheet: *mut *css_stylesheet) -> css_error;
    fn css_stylesheet_destroy(sheet: *css_stylesheet) -> css_error;
    fn css_stylesheet_size(sheet: *css_stylesheet, size: *mut size_t) -> css_error;
    fn css_stylesheet_append_data(sheet: *css_stylesheet, data: *const uint8_t, len: size_t) -> css_error;
    fn css_stylesheet_data_done(sheet: *css_stylesheet) -> css_error;
}