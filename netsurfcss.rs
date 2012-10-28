use core::libc::{c_void, size_t};
use core::libc::types::common::c99::{uint32_t, int32_t, uint64_t, uint8_t};
use ll::errors::*;
use ll::stylesheet::*;
use ll::types::*;
use ll::select::*;
use ll::hint::*;
use ll::properties::*;
use ll_css_stylesheet_create = ll::stylesheet::css_stylesheet_create;
use ll_css_select_ctx_create = ll::select::css_select_ctx_create;
use ptr::{null, to_unsafe_ptr, to_mut_unsafe_ptr};
use cast::transmute;
use conversions::c_enum_to_rust_enum;
use errors::CssError;

use wapcaplet::ll::lwc_string;
use wapcaplet::{LwcString, from_rust_string};

fn ll_result_to_rust_result<T>(code: css_error, val: T) -> CssResult<T> {
    match code {
        e if e == CSS_OK => Ok(move val),
        _ => Err(c_enum_to_rust_enum(code))
    }
}

type CssResult<T> = Result<T, CssError>;

fn require_ok(code: css_error, what: &str) {
    match code {
        e if e == CSS_OK => (),
        e => fail fmt!("CSS parsing failed while %s. code: %?", what, e)
    }
}

extern fn realloc(ptr: *c_void, len: size_t, _pw: *c_void) -> *c_void {
    libc::realloc(ptr, len)
}

mod types {
    pub enum CssLanguageLevel {
        CssLevel1,
        CssLevel2,
        CssLevel21,
        CssLevel3,
        CssLevelDefault, // NB: This is not the same as the ll value
        // NB: Sentinal variant to prevent the naive transmute conversion from working
        CssLevelNotACLikeEnum(uint)
    }

    pub struct CssColor { a: u8, r: u8, g: u8, b: u8 }

    pub struct CssQName {
        ns: Option<LwcString>,
        name: LwcString
    }

}

mod errors {
    enum CssError {
	CssOk               = 0,
	CssNoMem            = 1,
	CssBadParm          = 2,
	CssInvalid          = 3,
	CssFileNotFound     = 4,
	CssNeedData         = 5,
	CssBadCharset       = 6,
	CssEof              = 7,
	CssImportsPending   = 8,
	CssPropertyNotSet   = 9
    }
}

mod stylesheet {
    use properties::{CssFontStyle, CssFontVariant, CssFontWeight};
    use types::{CssLanguageLevel, CssColor};

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

    pub type CssUrlResolutionFn = ~fn(base: &str, rel: &LwcString) -> CssResult<LwcString>;
    pub type CssImportNotificationFn = ~fn(parent: &CssStylesheet, url: &LwcString) -> CssResult<uint64_t>;
    pub type CssColorResolutionFn = ~fn(name: &LwcString) -> CssResult<CssColor>;
    pub type CssFontResolutionFn = ~fn(name: &LwcString) -> CssResult<CssSystemFont>;

    pub struct CssSystemFont {
        style: CssFontStyle,
        variant: CssFontVariant,
        weight: CssFontWeight,
        size: css_size,
        line_height: css_size,
        family: ~str
    }

    pub struct CssStylesheet {
        priv params: CssStylesheetParams,
        priv sheet: *css_stylesheet,

        drop {
            assert self.sheet.is_not_null();
            let code = css_stylesheet_destroy(self.sheet);
            require_ok(code, "destroying stylesheet");
        }
    }

    fn css_stylesheet_create(params: CssStylesheetParams) -> CssStylesheet {
        let sheet = do params.as_ll |ll_params| {
            let mut sheet: *css_stylesheet = null();
            let code = ll_css_stylesheet_create(
                to_unsafe_ptr(ll_params), realloc, null(), to_mut_unsafe_ptr(&mut sheet));
            require_ok(code, "creating stylesheet");
            assert sheet.is_not_null();
            sheet
        };

        CssStylesheet {
            // Store the params to keep their pointers alive
            params: move params,
            sheet: sheet
        }
    }

    impl CssStylesheet {
        fn size() -> uint {
            let mut size = 0;
            let code = css_stylesheet_size(self.sheet, to_mut_unsafe_ptr(&mut size));
            require_ok(code, "getting stylesheet size");
            return size as uint;
        }

        fn append_data(data: &[u8]) {
            // FIXME: For some reason to_const_ptr isn't accessible
            let code = css_stylesheet_append_data(self.sheet, unsafe { transmute(vec::raw::to_ptr(data)) }, data.len() as size_t);
            match code {
                e if e == CSS_NEEDDATA => { /* fine */ },
                _ => require_ok(code, "appending styleshet data")
            }
        }

        fn data_done() {
            let code = css_stylesheet_data_done(self.sheet);
            require_ok(code, "finishing parsing");
        }

        fn ll_sheet() -> *css_stylesheet {
            self.sheet
        }
    }

}

pub mod properties {

    use types::CssColor;

    enum CssProperty {
        CssPropAzimuth			= 0x000,
        CssPropBackgroundAttachment		= 0x001,
        CssPropBackgroundColor		= 0x002,
        CssPropBackgroundImage		= 0x003,
        CssPropBackgroundPosition		= 0x004,
        CssPropBackgroundRepeat		= 0x005,
        CssPropBorderCollapse		= 0x006,
        CssPropBorderSpacing			= 0x007,
        CssPropBorderTopColor		= 0x008,
        CssPropBorderRightColor		= 0x009,
        CssPropBorderBottomColor		= 0x00a,
        CssPropBorderLeftColor		= 0x00b,
        CssPropBorderTopStyle		= 0x00c,
        CssPropBorderRightStyle		= 0x00d,
        CssPropBorderBottomStyle		= 0x00e,
        CssPropBorderLeftStyle		= 0x00f,
        CssPropBorderTopWidth		= 0x010,
        CssPropBorderRightWidth		= 0x011,
        CssPropBorderBottomWidth		= 0x012,
        CssPropBorderLeftWidth		= 0x013,
        CssPropBottom				= 0x014,
        CssPropCaptionSide			= 0x015,
        CssPropClear				= 0x016,
        CssPropClip				= 0x017,
        CssPropColor				= 0x018,
        CssPropContent			= 0x019,
        CssPropCounterIncrement		= 0x01a,
        CssPropCounterReset			= 0x01b,
        CssPropCueAfter			= 0x01c,
        CssPropCueBefore			= 0x01d,
        CssPropCursor				= 0x01e,
        CssPropDirection			= 0x01f,
        CssPropDisplay			= 0x020,
        CssPropElevation			= 0x021,
        CssPropEmptyCells			= 0x022,
        CssPropFloat				= 0x023,
        CssPropFontFamily			= 0x024,
        CssPropFontSize			= 0x025,
        CssPropFontStyle			= 0x026,
        CssPropFontVariant			= 0x027,
        CssPropFontWeight			= 0x028,
        CssPropHeight				= 0x029,
        CssPropLeft				= 0x02a,
        CssPropLetterSpacing			= 0x02b,
        CssPropLineHeight			= 0x02c,
        CssPropListStyleImage		= 0x02d,
        CssPropListStylePosition		= 0x02e,
        CssPropListStyleType		= 0x02f,
        CssPropMarginTop			= 0x030,
        CssPropMarginRight			= 0x031,
        CssPropMarginBottom			= 0x032,
        CssPropMarginLeft			= 0x033,
        CssPropMaxHeight			= 0x034,
        CssPropMaxWidth			= 0x035,
        CssPropMinHeight			= 0x036,
        CssPropMinWidth			= 0x037,
        CssPropOrphans,
        CssPropOutlineColor			= 0x039,
        CssPropOutlineStyle			= 0x03a,
        CssPropOutlineWidth			= 0x03b,
        CssPropOverflow			= 0x03c,
        CssPropPaddingTop			= 0x03d,
        CssPropPaddingRight			= 0x03e,
        CssPropPaddingBottom			= 0x03f,
        CssPropPaddingLeft			= 0x040,
        CssPropPageBreakAfter		= 0x041,
        CssPropPageBreakBefore		= 0x042,
        CssPropPageBreakInside		= 0x043,
        CssPropPauseAfter			= 0x044,
        CssPropPauseBefore			= 0x045,
        CssPropPitchRange			= 0x046,
        CssPropPitch				= 0x047,
        CssPropPlayDuring			= 0x048,
        CssPropPosition			= 0x049,
        CssPropQuotes				= 0x04a,
        CssPropRichness			= 0x04b,
        CssPropRight				= 0x04c,
        CssPropSpeakHeader			= 0x04d,
        CssPropSpeakNumeral			= 0x04e,
        CssPropSpeakPunctuation		= 0x04f,
        CssPropSpeak				= 0x050,
        CssPropSpeechRate			= 0x051,
        CssPropStress				= 0x052,
        CssPropTableLayout			= 0x053,
        CssPropTextAlign			= 0x054,
        CssPropTextDecoration		= 0x055,
        CssPropTextIndent			= 0x056,
        CssPropTextTransform			= 0x057,
        CssPropTop				= 0x058,
        CssPropUnicodeBidi			= 0x059,
        CssPropVerticalAlign			= 0x05a,
        CssPropVisibility			= 0x05b,
        CssPropVoiceFamily			= 0x05c,
        CssPropVolume				= 0x05d,
        CssPropWhiteSpace			= 0x05e,
        CssPropWidows				= 0x05f,
        CssPropWidth				= 0x060,
        CssPropWordSpacing			= 0x061,
        CssPropZIndex			= 0x062,
        CssPropOpacity			= 0x063,
        CssPropBreakAfter			= 0x064,
        CssPropBreakBefore			= 0x065,
        CssPropBreakInside			= 0x066,
        CssPropColumnCount			= 0x067,
        CssPropColumnFill			= 0x068,
        CssPropColumnGap			= 0x069,
        CssPropColumnRuleColor		= 0x06a,
        CssPropColumnRuleStyle		= 0x06b,
        CssPropColumnRuleWidth		= 0x06c,
        CssPropColumnSpan			= 0x06d,
        CssPropClomumnWidth			= 0x06e,
    }

    fn property_from_uint(property: uint32_t) -> CssProperty {
        unsafe { transmute(property as uint) }
    }

    // Similar to css_color_e
    pub enum CssColorProp {
        CssColorInherit,
        CssColorValue(CssColor)
    }

    pub enum CssFontStyle {
	CssFontStyleInherit			= 0x0,
	CssFontStyleNormal			= 0x1,
	CssFontStyleItalic			= 0x2,
	CssFontStyleOblique			= 0x3
    }

    pub enum CssFontFamily {
	CssFontFamilyInherit			= 0x0,
	/* Named fonts exist if pointer is non-NULL */
	CssFontFamilySerif			= 0x1,
	CssFontFamilySansSerif		= 0x2,
	CssFontFamilyCursive			= 0x3,
	CssFontFamilyFantasy			= 0x4,
	CssFontFamilyMonospace		= 0x5
    }

    pub enum CssFontVariant {
        CssFontVariantInherit = 0,
        CssFontVariantNormal = 1,
        CssFontVariantSmallCaps = 2,
    }

    enum CssFontWeight {
	CssFontWeightInherit			= 0x0,
        CssFontWeightNormal			= 0x1,
        CssFontWeightBold			= 0x2,
        CssFontWeightBolder			= 0x3,
        CssFontWeightLighter			= 0x4,
        CssFontWeight100			= 0x5,
        CssFontWeight200			= 0x6,
        CssFontWeight300			= 0x7,
        CssFontWeight400			= 0x8,
        CssFontWeight500			= 0x9,
        CssFontWeight600			= 0xa,
        CssFontWeight700			= 0xb,
        CssFontWeight800			= 0xc,
        CssFontWeight900			= 0xd
    }

    // NB: This is not identical to css_quotes_e
    pub enum CssQuotes {
	CssQuotesInherit,
        CssQuotesString,
        CssQuotesNone,
        // Sentinal value to give this enum a non-word size, so the
        // naive unsafe conversion to ll fails
        CssQuotesNotACLikeEnum(uint)
    }
}

pub mod hint {

    use properties::*;

    // An interpretation of the delightful css_hint union
    pub enum CssHint {
        CssHintFontFamily(~[LwcString], CssFontFamily),
        CssHintDefault,
        CssHintUnknown
    }

    impl CssHint {
        fn write_to_ll(&self, property: CssProperty, llhint: &mut css_hint) -> css_error {
            match (property, self) {
                (CssPropFontFamily, &CssHintDefault) => {
                    let strings: &mut **lwc_string = hint_data_field(llhint);
                    *strings = null();
                    set_css_hint_status(llhint, CSS_FONT_FAMILY_SANS_SERIF as uint8_t);
                }
                (CssPropQuotes, &CssHintDefault) => {
                    let strings: &mut **lwc_string = hint_data_field(llhint);
                    *strings = null();
                    set_css_hint_status(llhint, CSS_QUOTES_NONE as uint8_t);
                }
                (CssPropColor, &CssHintDefault) => {
                    let color: &mut css_color = hint_data_field(llhint);
                    *color = CssColor { a: 255, r: 0, g: 0, b: 0 }.to_ll();
                    set_css_hint_status(llhint, CSS_COLOR_COLOR as uint8_t);
                }
                (_, &CssHintUnknown) => {
                    fail fmt!("unknown css hint %?", property);
                }
                (_, _) => {
                    fail fmt!("incorrectly handled property hint: %?, %?", property, self);
                }
            }

            return CSS_OK;
        }
    }

    fn set_css_hint_status(llhint: &mut css_hint, status: uint8_t) unsafe {
        // So gnarly. The status field is a uint8_t that comes after a union type.
        // We're just going to calculate it's address and write it
        let llhint_bytes: *mut uint8_t = to_mut_unsafe_ptr(transmute(llhint));
        let status_field: *mut uint8_t = ptr::mut_offset(llhint_bytes, status_field_offset());

        *status_field = status;

        #[cfg(target_arch = "x86_64")]
        fn status_field_offset() -> uint { 16 }

        #[cfg(target_arch = "x86")]
        fn status_field_offset() -> uint { 16 }
    }

    priv fn hint_data_field<T>(llhint: &mut css_hint) -> &mut T {
        unsafe { transmute(llhint) }
    }
}

mod select {

    use types::CssQName;
    use stylesheet::CssStylesheet;
    use properties::{CssProperty, CssColorProp};
    use computed::CssComputedStyle;

    enum CssPseudoElement {
	CssPseudoElementNone         = 0,
	CssPseudoElementFirstLine   = 1,
	CssPseudoElementFirstLetter = 2,
	CssPseudoElementBefore       = 3,
	CssPseudoElementAfter        = 4,
	CssPseudoElementCount	= 5
    }

    struct CssSelectCtx {
        priv select_ctx: *css_select_ctx,
        // Whenever a sheet is added to the select ctx we will take ownership of it
        // to ensure that it stays alive
        priv mut sheets: ~[CssStylesheet],

        drop {
            assert self.select_ctx.is_not_null();
            let code = css_select_ctx_destroy(self.select_ctx);
            require_ok(code, "destroying select ctx");
        }
    }

    fn css_select_ctx_create() -> CssSelectCtx {
        let mut select_ctx: *css_select_ctx = null();
        let code = ll_css_select_ctx_create(realloc, null(), to_mut_unsafe_ptr(&mut select_ctx));
        require_ok(code, "creating select context");
        assert select_ctx.is_not_null();

        CssSelectCtx {
            select_ctx: select_ctx,
            mut sheets: ~[]
        }
    }

    impl CssSelectCtx {
        fn append_sheet(sheet: CssStylesheet, origin: css_origin, media: uint64_t) {
            let code = css_select_ctx_append_sheet(self.select_ctx, sheet.ll_sheet(), origin, media);
            require_ok(code, "adding sheet to select ctx");

            self.sheets.push(move sheet);
        }

        fn count_sheets() -> uint {
            let mut count = 0;
            let code = css_select_ctx_count_sheets(self.select_ctx, to_mut_unsafe_ptr(&mut count));
            require_ok(code, "counting sheets");
            return count as uint;
        }

        fn select_style<N, H: CssSelectHandler<N>>(node: &N, media: uint64_t,
                                                   _inline_style: Option<&CssStylesheet>,
                                                   handler: &H) -> CssSelectResults {
            do with_untyped_handler(handler) |untyped_handler| {
                let raw_handler = build_raw_handler();
                let mut results: *css_select_results = null();
                let code = css_select_style(self.select_ctx,
                                            unsafe { transmute(to_unsafe_ptr(node)) },
                                            media,
                                            null(), // FIXME,
                                            to_unsafe_ptr(&raw_handler),
                                            unsafe { transmute(to_unsafe_ptr(untyped_handler)) },
                                            to_mut_unsafe_ptr(&mut results));
                require_ok(code, "selecting style");

                CssSelectResults {
                    results: results
                }
            }
        }
    }

    priv fn build_raw_handler() -> css_select_handler {
        css_select_handler {
            handler_version: CSS_SELECT_HANDLER_VERSION_1,
            node_name: raw_handler::node_name,
            node_classes: raw_handler::node_classes,
            node_id: raw_handler::node_id,
            named_ancestor_node: raw_handler::named_ancestor_node,
            named_parent_node: raw_handler::named_parent_node,
            named_sibling_node: raw_handler::named_sibling_node,
            named_generic_sibling_node: raw_handler::named_generic_sibling_node,
            parent_node: raw_handler::parent_node,
            sibling_node: raw_handler::sibling_node,
            node_has_name: raw_handler::node_has_name,
            node_has_class: raw_handler::node_has_class,
            node_has_id: raw_handler::node_has_id,
            node_has_attribute: raw_handler::node_has_attribute,
            node_has_attribute_equal: raw_handler::node_has_attribute_equal,
            node_has_attribute_dashmatch: raw_handler::node_has_attribute_dashmatch,
            node_has_attribute_includes: raw_handler::node_has_attribute_includes,
            node_has_attribute_prefix: raw_handler::node_has_attribute_prefix,
            node_has_attribute_suffix: raw_handler::node_has_attribute_suffix,
            node_has_attribute_substring: raw_handler::node_has_attribute_substring,
            node_is_root: raw_handler::node_is_root,
            node_count_siblings: raw_handler::node_count_siblings,
            node_is_empty: raw_handler::node_is_empty,
            node_is_link: raw_handler::node_is_link,
            node_is_visited: raw_handler::node_is_visited,
            node_is_hover: raw_handler::node_is_hover,
            node_is_active: raw_handler::node_is_active,
            node_is_focus: raw_handler::node_is_focus,
            node_is_enabled: raw_handler::node_is_enabled,
            node_is_disabled: raw_handler::node_is_disabled,
            node_is_checked: raw_handler::node_is_checked,
            node_is_target: raw_handler::node_is_target,
            node_is_lang: raw_handler::node_is_lang,
            node_presentational_hint: raw_handler::node_presentational_hint,
            ua_default_for_property: raw_handler::ua_default_for_property,
            compute_font_size: raw_handler::compute_font_size
        }
    }

    priv mod raw_handler {
        priv fn unimpl(n: &str) -> ! {
            fail fmt!("unimplemented css callback handler: %s", n)
        }
        priv fn enter(n: &str) {
            debug!("entering raw handler: %s", n);
        }
        priv fn ph(pw: *c_void) -> &UntypedHandler unsafe {
            transmute(pw)
        }
        pub extern fn node_name(pw: *c_void, node: *c_void, qname: *css_qname) -> css_error {
            enter("node_name");
            ph(pw).node_name(node, qname)
        }
        pub extern fn node_classes(pw: *c_void, node: *c_void, classes: *mut **lwc_string, n_classes: *mut uint32_t) -> css_error {
            enter("node_classes");
            ph(pw).node_classes(node, classes, n_classes)
        }
        pub extern fn node_id(pw: *c_void, node: *c_void, id: *mut *lwc_string) -> css_error {
            enter("node_id");
            ph(pw).node_id(node, id)
        }
        pub extern fn named_ancestor_node(_pw: *c_void, _node: *c_void, _qname: *css_qname, _parent: **c_void) -> css_error {
            unimpl("named_ancestor_node")
        }
        pub extern fn named_parent_node(_pw: *c_void, _node: *c_void, _qname: *css_qname, _parent: **c_void) -> css_error {
            unimpl("named_parent_node")
        }
        pub extern fn named_sibling_node(_pw: *c_void, _node: *c_void, _qname: *css_qname, _sibling: **c_void) -> css_error {
            unimpl("named_sibling_node")
        }
        pub extern fn named_generic_sibling_node(_pw: *c_void, _node: *c_void, _qname: *css_qname, _sibling: **c_void) -> css_error {
            unimpl("named_generic_sibling_node")
        }
        pub extern fn parent_node(pw: *c_void, node: *c_void, parent: *mut *c_void) -> css_error {
            enter("parent_node");
            ph(pw).parent_node(node, parent)
        }
        pub extern fn sibling_node(_pw: *c_void, _node: *c_void, _sibling: **c_void) -> css_error {
            unimpl("sibling_node")
        }
        pub extern fn node_has_name(_pw: *c_void, _node: *c_void, _qname: *css_qname, _match_: *bool) -> css_error {
            unimpl("node_has_name")
        }
        pub extern fn node_has_class(_pw: *c_void, _node: *c_void, _name: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_has_class")
        }
        pub extern fn node_has_id(_pw: *c_void, _node: *c_void, _name: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_has_id")
        }
        pub extern fn node_has_attribute(_pw: *c_void, _node: *c_void, _qname: *css_qname, _match_: *bool) -> css_error {
            unimpl("node_has_attribute")
        }
        pub extern fn node_has_attribute_equal(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_has_attribute_equal")
        }
        pub extern fn node_has_attribute_dashmatch(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_has_attribute_dashmatch")
        }
        pub extern fn node_has_attribute_includes(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_has_attribute_includes")
        }
        pub extern fn node_has_attribute_prefix(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_has_attribute_prefix")
        }
        pub extern fn node_has_attribute_suffix(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_has_attribute_suffix")
        }
        pub extern fn node_has_attribute_substring(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_has_attribute_substring")
        }
        pub extern fn node_is_root(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_root")
        }
        pub extern fn node_count_siblings(_pw: *c_void, _node: *c_void, _same_name: bool, _after: bool, _count: *int32_t) -> css_error {
            unimpl("node_count_siblings")
        }
        pub extern fn node_is_empty(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_empty")
        }
        pub extern fn node_is_link(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_link")
        }
        pub extern fn node_is_visited(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_visited")
        }
        pub extern fn node_is_hover(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_hover")
        }
        pub extern fn node_is_active(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_active")
        }
        pub extern fn node_is_focus(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_focus")
        }
        pub extern fn node_is_enabled(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_enabled")
        }
        pub extern fn node_is_disabled(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_disabled")
        }
        pub extern fn node_is_checked(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_checked")
        }
        pub extern fn node_is_target(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
            unimpl("node_is_target")
        }
        pub extern fn node_is_lang(_pw: *c_void, _node: *c_void, _lang: *lwc_string, _match_: *bool) -> css_error {
            unimpl("node_is_lang")
        }
        pub extern fn node_presentational_hint(_pw: *c_void, _node: *c_void, _property: uint32_t, _hint: *css_hint) -> css_error {
            enter("node_presentational_hint");
            CSS_PROPERTY_NOT_SET
        }
        pub extern fn ua_default_for_property(pw: *c_void, property: uint32_t, hint: *mut css_hint) -> css_error {
            enter("ua_default_for_property");
            ph(pw).ua_default_for_property(property, hint)
        }
        pub extern fn compute_font_size(_pw: *c_void, _parent: *css_hint, _size: *css_hint) -> css_error {
            enter("compute_font_size");
            // FIXME
            CSS_OK
        }
    }

    priv struct UntypedHandler {
        node_name: &fn(node: *c_void, qname: *css_qname) -> css_error,
        node_classes: &fn(node: *c_void, classes: *mut **lwc_string, n_classes: *mut uint32_t) -> css_error,
        node_id: &fn(node: *c_void, id: *mut *lwc_string) -> css_error,
        parent_node: &fn(node: *c_void, parent: *mut *c_void) -> css_error,
        ua_default_for_property: &fn(property: uint32_t, hint: *mut css_hint) -> css_error,
    }

    priv fn with_untyped_handler<N, H: CssSelectHandler<N>, R>(handler: &H, f: fn(&UntypedHandler) -> R) -> R {
        unsafe {
            let untyped_handler = UntypedHandler {
                node_name: |node: *c_void, qname: *css_qname| -> css_error {
                    let hlnode: &N = transmute(node);
                    let hlqname = handler.node_name(hlnode);
                    match hlqname.ns {
                        Some(ns) => {
                            (*qname).ns = ns.raw_reffed();
                        }
                        _ => ()
                    }
                    (*qname).name = hlqname.name.raw_reffed();
                    CSS_OK
                },
                node_classes: |_node: *c_void, classes: *mut **lwc_string, n_classes: *mut uint32_t| -> css_error {
                    // FIXME
                    *classes = null();
                    *n_classes = 0;
                    CSS_OK
                },
                node_id: |_node: *c_void, id: *mut *lwc_string| -> css_error {
                    // FIXME
                    *id = null();
                    CSS_OK
                },
                parent_node: |_node: *c_void, parent: *mut *c_void| -> css_error {
                    /*let hlnode: &N = transmute(node);
                    let hlparent: Option<&N> = handler.parent_node(hlnode);
                    match hlparent {
                        Some(p) => *parent = transmute(p),
                        None => *parent = null()
                    }*/
                    // FIXME
                    *parent = null();
                    CSS_OK
                },
                ua_default_for_property: |property: uint32_t, hint: *mut css_hint| -> css_error {
                    use properties::property_from_uint;
                    let hlproperty = property_from_uint(property);
                    let hlhint = handler.ua_default_for_property(hlproperty);
                    hlhint.write_to_ll(hlproperty, &mut *hint)
                },
            };

            f(&untyped_handler)
        }
    }

    pub trait CssSelectHandler<N> {
        fn node_name(node: &N) -> CssQName;
        //fn parent_node(node: &a/N) -> Option<&a/N>;
        fn ua_default_for_property(property: CssProperty) -> hint::CssHint;
    }

    pub struct CssSelectResults {
        priv results: *css_select_results,

        drop {
            assert self.results.is_not_null();
            let code = css_select_results_destroy(self.results);
            require_ok(code, "destroying select results");
        }
    }

    impl CssSelectResults {
        fn computed_style(&self, element: CssPseudoElement) -> CssComputedStyle/&self {
            let element = element.to_ll();
            let llstyle = unsafe { *self.results }.styles[element];
            assert llstyle.is_not_null();

            CssComputedStyle {
                result_backref: self,
                computed_style: llstyle
            }
        }
    }

}

mod computed {
    use select::CssSelectResults;
    use properties::*;
    use ll::properties::*;
    use ll::computed::css_computed_color;
    use conversions::ll_color_to_hl_color;

    pub struct CssComputedStyle {
        // A borrowed back reference to ensure this outlives the results
        result_backref: &CssSelectResults,
        computed_style: *css_computed_style,
    }

    impl CssComputedStyle {
        fn color() -> CssColorProp {
            let mut llcolor = 0;
            let type_ = css_computed_color(self.computed_style, to_mut_unsafe_ptr(&mut llcolor));

            if type_ == CSS_COLOR_INHERIT as uint8_t {
                CssColorInherit
            } else {
                CssColorValue(ll_color_to_hl_color(llcolor))
            }
        }
    }
}
