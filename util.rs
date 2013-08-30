// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ll::stylesheet::css_fixed;
use types::CssQName;
use std::libc::c_void;
use lwcstr_from_rust_str = wapcaplet::from_rust_string;

pub fn css_fixed_to_float(f: css_fixed) -> float {
    static BEFORE: i32 = 10;
    f as float * 1.0f / ((1i32 << BEFORE) as float)
}

pub fn float_to_css_fixed(f: float) -> css_fixed {
    static BEFORE: i32 = 10;
    (f * ((1 << BEFORE) as float)) as css_fixed
}

pub fn rust_str_to_net_qname(s: &str) -> CssQName {
    CssQName {
        ns: None,
        name: lwcstr_from_rust_str(s)
    }
}

pub fn net_qname_to_rust_str<'a>(qname: &'a CssQName) -> &'a str {
    qname.name.to_str_slice()
}


// FIXME: These methods should be unsafe
pub trait VoidPtrLike {
    fn from_void_ptr(ptr: *c_void) -> Self;
    fn to_void_ptr(&self) -> *c_void;
}
