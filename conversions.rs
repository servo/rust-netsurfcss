use ll::*;
use hl::*;

pub trait ToLl<T> {
    fn to_ll(&self) -> T;
}

pub trait AsLl<T> {
    fn as_ll<U>(&self, f: fn(&T) -> U) -> U;
}

pub trait ToHl<T> {
    fn to_hl(&self) -> T;
}

pub impl CssLanguageLevel: ToLl<css_language_level> {
    pub fn to_ll(&self) -> css_language_level {
        match *self {
            CssLevel1 => CSS_LEVEL_1,
            CssLevel2 => CSS_LEVEL_2,
            CssLevel21 => CSS_LEVEL_21,
            CssLevel3 => CSS_LEVEL_3,
            CssLevelDefault => CSS_LEVEL_DEFAULT
        }
    }
}

pub impl css_language_level: ToHl<CssLanguageLevel> {
    pub fn to_hl(&self) -> CssLanguageLevel {
        if *self == CSS_LEVEL_1 { CssLevel1 }
        else if *self == CSS_LEVEL_2 { CssLevel2 }
        else if *self == CSS_LEVEL_21 { CssLevel21 }
        else if *self == CSS_LEVEL_3 { CssLevel3 }
        else if *self == CSS_LEVEL_DEFAULT { CssLevelDefault }
        else { fail }
    }
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

extern fn resolve(pw: *c_void, base: *c_char, rel: *lwc_string, abs: **lwc_string) -> css_error {
    unsafe {
        let f: &CssUrlResolutionFn = transmute(pw);

        let base = str::raw::from_c_str(base);
        (*f)(base, &*rel, & &**abs)
    }
}
