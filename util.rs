use ll::stylesheet::css_fixed;
use types::CssQName;
use core::libc::c_void;
use lwcstr_from_rust_str = wapcaplet::from_rust_string;

pub fn css_fixed_to_float(f: css_fixed) -> float {
    const before: i32 = 10;
    f as float * 1.0f / ((1i32 << before) as float)
}

pub fn rust_str_to_net_qname(s: &str) -> CssQName {
    CssQName {
        ns: None,
        name: lwcstr_from_rust_str(s)
    }
}

pub fn net_qname_to_rust_str(qname: &CssQName) -> ~str {
    qname.name.to_str()
}


// FIXME: These methods should be unsafe
pub trait VoidPtrLike {
    static fn from_void_ptr(*c_void) -> self;
    fn to_void_ptr(&self) -> *c_void;
}