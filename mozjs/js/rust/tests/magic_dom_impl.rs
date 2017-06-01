/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate magic_codegen;
#[macro_use]
extern crate js;
extern crate libc;

use js::rust::{Runtime, SIMPLE_GLOBAL_CLASS, ToNumber};
use js::rust;
use js::glue::CreateCallArgsFromVp;
use js::jsapi;
use js::jsapi::JSPropertySpec;
use js::jsapi::root::JS;
use js::jsapi::root::{JS_ReportErrorASCII, JS_NewGlobalObject,
                      JS_InitClass, JSContext};
use js::jsapi::root::JS::CompartmentOptions;
use js::jsapi::root::JS::OnNewGlobalHookOption;
use js::jsval::{DoubleValue, ObjectValue, UndefinedValue};
use std::ffi::CString;
use std::ptr;

magic_dom! {
    DOMPoint,
    struct DOMPoint_spec {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }
}

use magic_dom_spec::set_x as set_x;
use magic_dom_spec::set_y as set_y;
use magic_dom_spec::set_z as set_z;
use magic_dom_spec::set_w as set_w;
use magic_dom_spec::get_x as get_x;
use magic_dom_spec::get_y as get_y;
use magic_dom_spec::get_z as get_z;
use magic_dom_spec::get_w as get_w;
use magic_dom_spec::check_this as check_this;

extern "C" fn DOMPoint_constructor(cx: *mut JSContext, argc: u32, vp: *const JS::Value) -> bool {
    let call_args = CreateCallArgsFromVp(argc, vp);
    if call_args._base.argc_ != 4 {
        JS_ReportErrorASCII(cx, b"constructor requires exactly 4 arguments\0".as_ptr() as *const libc::c_char);
        return false;
    }
    get_js_val_number!(x, cx, call_args, 0);
    get_js_val_number!(y, cx, call_args, 1);
    get_js_val_number!(z, cx, call_args, 2);
    get_js_val_number!(w, cx, call_args, 3);
    rooted!(in(cx) let jsobj = jsapi::JS_NewObject(cx, magic_dom_spec::DOMPoint_class));
    if jsobj.is_null() {
        JS_ReportErrorASCII(cx, b"Fail to construct JS object\0".as_ptr() as *const libc::c_char);
        return false;
    }
    let d = match DOMPoint::from_object(jsobj) {
        Some(o) => o,
        None => {
            JS_ReportErrorASCII(cx, b"Fail to construct DOMPoint from JS object\0".as_ptr()
                                as *const libc::c_char);
            return false;
        }
    };
    set_x(&d, cx, x);
    set_y(&d, cx, y);
    set_z(&d, cx, z);
    set_w(&d, cx, w);
    call_args.rval.set(ObjectValue(jsobj));
    true
}

js_getter!(DOMPoint, js_get_x, get_x, DoubleValue);
js_getter!(DOMPoint, js_get_y, get_y, DoubleValue);
js_getter!(DOMPoint, js_get_z, get_z, DoubleValue);
js_getter!(DOMPoint, js_get_w, get_w, DoubleValue);
js_getter!(DOMPoint, js_set_x, set_x, DoubleValue);
js_getter!(DOMPoint, js_set_y, set_y, DoubleValue);
js_getter!(DOMPoint, js_set_z, set_z, DoubleValue);
js_getter!(DOMPoint, js_set_w, set_w, DoubleValue);

static ps_arr: [JSPropertySpec; 5] = [
    JSPropertySpec::getter_and_setter("x", 0, js_set_x, js_get_x),
    JSPropertySpec::getter_and_setter("y", 0, js_set_y, js_get_y),
    JSPropertySpec::getter_and_setter("z", 0, js_set_z, js_get_z),
    JSPropertySpec::getter_and_setter("w", 0, js_set_w, js_get_w),
    JSPropertySpec::end()
];

#[test]
fn get_and_set() {
    let rt = Runtime::new().unwrap();
    let cx = rt.cx();


    unsafe {
        rooted!(in(cx) let global =
                JS_NewGlobalObject(cx, &SIMPLE_GLOBAL_CLASS, std::ptr::null_mut(),
                                   OnNewGlobalHookOption::FireOnNewGlobalHook,
                                   &CompartmentOptions::default())
        );

        rooted!(in(cx) let dom_point =
                JS_InitClass(cx, global, std::ptr::null_mut(),
                             &magic_dom_spec::DOMPoint_class, DOMPoint_constructor,
                             4, &ps_arr as *const _, std::ptr::null(), std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), "1 + 1",
                                   "test", 1, rval.handle_mut()).is_ok());
    }
}
