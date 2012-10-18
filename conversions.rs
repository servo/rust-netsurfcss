use ll::*;
use hl::*;

pub trait ToLl<T> {
    fn to_ll(&self) -> T;
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

