/*!

Low-level bindings. These are organized into modules mirroring the libcss
include files

*/

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

mod functypes {
    // (ptr: *c_void, size: size_t, pw: *c_void)
    type css_allocator_fn = *u8;
}

mod types {
    type css_language_level = uint32_t;

    const CSS_LEVEL_1: css_language_level = 0;
    const CSS_LEVEL_2: css_language_level = 1;
    const CSS_LEVEL_21: css_language_level = 2;
    const CSS_LEVEL_3: css_language_level = 3;
    const CSS_LEVEL_DEFAULT: css_language_level = 2;

    type css_color = uint32_t;

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

    enum css_origin {
        CSS_ORIGIN_UA = 0,
        CSS_ORIGIN_USER = 1,
        CSS_ORIGIN_AUTHOR = 2
    }

    const CSS_MEDIA_AURAL: uint64_t = (1 << 0);
    const CSS_MEDIA_BRAILLE: uint64_t = (1 << 1);
    const CSS_MEDIA_EMBOSSED: uint64_t = (1 << 2);
    const CSS_MEDIA_HANDHELD: uint64_t = (1 << 3);
    const CSS_MEDIA_PRINT: uint64_t = (1 << 4);
    const CSS_MEDIA_PROJECTION: uint64_t = (1 << 5);
    const CSS_MEDIA_SCREEN: uint64_t = (1 << 6);
    const CSS_MEDIA_SPEECH: uint64_t = (1 << 7);
    const CSS_MEDIA_TTY: uint64_t = (1 << 8);
    const CSS_MEDIA_TV: uint64_t = (1 << 9);
    const CSS_MEDIA_ALL: uint64_t =
        CSS_MEDIA_AURAL | CSS_MEDIA_BRAILLE |
        CSS_MEDIA_EMBOSSED | CSS_MEDIA_HANDHELD |
        CSS_MEDIA_PRINT | CSS_MEDIA_PROJECTION |
        CSS_MEDIA_SCREEN | CSS_MEDIA_SPEECH |
        CSS_MEDIA_TTY | CSS_MEDIA_TV;

    type css_computed_style = c_void;

    pub struct css_qname {
        mut ns: *lwc_string,
        mut name: *lwc_string
    }
}

mod errors {
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
}

mod hint {
    // FIXME: This is not an opaque type
    type css_hint = c_void;
}

mod properties {

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
}

mod stylesheet {

    use types::{css_language_level, css_unit};
    use properties::{css_font_style_e, css_font_variant_e, css_font_weight_e};
    use functypes::css_allocator_fn;
    use errors::css_error;

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

    type css_url_resolution_fn = *u8; //extern fn(pw: *c_void, base: *c_char, rel: *lwc_string, abs: **lwc_string) -> css_error;
    type css_import_notification_fn = *u8; //extern fn(pw: *c_void, parent: *css_stylesheet, url: *lwc_string, media: *uint64_t) -> css_error;
    type css_color_resolution_fn = *u8; //extern fn(pw: *c_void, name: *lwc_string, color: *css_color) -> css_error;
    type css_font_resolution_fn = *u8; //extern fn(pw: *c_void, name: *lwc_string, system_font: *css_system_font) -> css_error;

    type css_stylesheet = c_void;

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

    type css_fixed = int32_t;

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
}

mod select {
    use functypes::css_allocator_fn;
    use errors::css_error;
    use stylesheet::css_stylesheet;
    use types::{css_origin, css_computed_style};

    type css_select_ctx = c_void;

    pub enum css_pseudo_element {
        CSS_PSEUDO_ELEMENT_NONE = 0,
        CSS_PSEUDO_ELEMENT_FIRST_LINE = 1,
        CSS_PSEUDO_ELEMENT_FIRST_LETTER = 2,
        CSS_PSEUDO_ELEMENT_BEFORE = 3,
        CSS_PESUDO_ELEMENT_AFTER = 4,
        CSS_PSEUDO_ELEMENT_COUNT = 5
    }

    pub struct css_select_results {
        alloc: css_allocator_fn,
        pw: *c_void,
        styles: [*css_computed_style * 5] // 5 == CSS_PSEUDO_ELEMENT_COUNT
    }

    pub type opaque_callback = *u8;

    // See select.h for actual callback signatures
    pub struct css_select_handler {
        handler_version: uint32_t,
        node_name: opaque_callback,
        node_classes: opaque_callback,
        node_id: opaque_callback,
        named_ancestor_node: opaque_callback,
        named_parent_node: opaque_callback,
        named_sibling_node: opaque_callback,
        named_generic_sibling_node: opaque_callback,
        parent_node: opaque_callback,
        sibling_node: opaque_callback,
        node_has_name: opaque_callback,
        node_has_class: opaque_callback,
        node_has_id: opaque_callback,
        node_has_attribute: opaque_callback,
        node_has_attribute_equal: opaque_callback,
        node_has_attribute_dashmatch: opaque_callback,
        node_has_attribute_includes: opaque_callback,
        node_has_attribute_prefix: opaque_callback,
        node_has_attribute_suffix: opaque_callback,
        node_has_attribute_substring: opaque_callback,
        node_is_root: opaque_callback,
        node_count_siblings: opaque_callback,
        node_is_empty: opaque_callback,
        node_is_link: opaque_callback,
        node_is_visited: opaque_callback,
        node_is_hover: opaque_callback,
        node_is_active: opaque_callback,
        node_is_focus: opaque_callback,
        node_is_enabled: opaque_callback,
        node_is_disabled: opaque_callback,
        node_is_checked: opaque_callback,
        node_is_target: opaque_callback,
        node_is_lang: opaque_callback,
        node_presentational_hint: opaque_callback,
        ua_default_for_property: opaque_callback,
        compute_font_size: opaque_callback
    }

    extern {
        fn css_select_ctx_create(alloc: css_allocator_fn, pw: *c_void, result: *mut *css_select_ctx) -> css_error;
        fn css_select_ctx_destroy(ctx: *css_select_ctx) -> css_error;
        fn css_select_ctx_append_sheet(ctx: *css_select_ctx, sheet: *css_stylesheet, origin: css_origin, media: uint64_t) -> css_error;
        fn css_select_ctx_count_sheets(ctx: *css_select_ctx, count: *mut uint32_t) -> css_error;
        fn css_select_style(ctx: *css_select_ctx, node: *c_void, media: uint64_t, inline_style: *css_stylesheet, handler: *css_select_handler, pw: *c_void, result: *mut *css_select_results) -> css_error;
        fn css_select_results_destroy(results: *css_select_results) -> css_error;
    }
}