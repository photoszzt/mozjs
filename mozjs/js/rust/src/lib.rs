/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

#![crate_name = "js"]
#![crate_type = "rlib"]

#![feature(associated_consts)]
#![feature(associated_type_defaults)]
#![feature(link_args)]
#![feature(nonzero)]
#![feature(const_fn)]
#![feature(untagged_unions)]

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, improper_ctypes)]

extern crate core;
#[macro_use]
extern crate heapsize;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate magic_codegen;
extern crate mozjs_sys;
extern crate num_traits;

#[macro_use]
pub mod rust;
#[macro_use]
pub mod magic;

pub mod ac;
pub mod conversions;
pub mod error;
pub mod glue;
pub mod heap;
pub mod jsval;
pub mod panic;
pub mod sc;
pub mod typedarray;

pub mod jsapi;
use self::jsapi::root::*;
pub mod debug;

#[macro_use]
pub mod jsslotconversions;
pub mod magicdom;

#[inline(always)]
pub unsafe fn JS_ARGV(_cx: *mut JSContext, vp: *mut JS::Value) -> *mut JS::Value {
    vp.offset(2)
}

known_heap_size!(0, JS::Value);

impl JS::ObjectOpResult {
    /// Set this ObjectOpResult to true and return true.
    pub fn succeed(&mut self) -> bool {
        self.code_ = JS::ObjectOpResult_SpecialCodes::OkCode as usize;
        true
    }
}
