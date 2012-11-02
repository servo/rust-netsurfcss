use ll::stylesheet::css_fixed;
use core::libc::c_void;

pub fn css_fixed_to_float(f: css_fixed) -> float {
    const before: i32 = 10;
    f as float * 1.0f / ((1i32 << before) as float)
}

// FIXME: These methods should be unsafe
pub trait VoidPtrLike {
    static fn from_void_ptr(*c_void) -> self;
    fn to_void_ptr(&self) -> *c_void;
}