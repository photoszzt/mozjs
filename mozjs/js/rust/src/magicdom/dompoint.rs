/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
use jsapi;
use jsval::ObjectValue;
use conversions::{ConversionResult, FromJSValConvertible, ToJSValConvertible};
use glue::CreateCallArgsFromVp;
use jsslotconversions::ToFromJsSlots;

extern crate libc;

magic_dom! {
    DOMPoint,
    DOMPOINT_CLASS,
    DOMPoint_constructor,
    magic_dom_spec_DOMPoint,
    struct DOMPoint_spec {
        _inherit: dompointreadonly::DOMPointReadOnly,
    }
}

impl DOMPoint {
    gen_getter_inherit!(get_x, f64, as_DOMPointReadOnly);
    gen_getter_inherit!(get_y, f64, as_DOMPointReadOnly);
    gen_getter_inherit!(get_z, f64, as_DOMPointReadOnly);
    gen_getter_inherit!(get_w, f64, as_DOMPointReadOnly);

    gen_setter_inherit!(set_x, f64, as_DOMPointReadOnly);
    gen_setter_inherit!(set_y, f64, as_DOMPointReadOnly);
    gen_setter_inherit!(set_z, f64, as_DOMPointReadOnly);
    gen_setter_inherit!(set_w, f64, as_DOMPointReadOnly);
}

js_getter!(js_get_x, get_x, DOMPoint);
js_getter!(js_get_y, get_y, DOMPoint);
js_getter!(js_get_z, get_z, DOMPoint);
js_getter!(js_get_w, get_w, DOMPoint);

js_setter!(js_set_x, set_x, DOMPoint, ());
js_setter!(js_set_y, set_y, DOMPoint, ());
js_setter!(js_set_z, set_z, DOMPoint, ());
js_setter!(js_set_w, set_w, DOMPoint, ());

lazy_static! {
    pub static ref DOMPOINT_PS_ARR: [JSPropertySpec; 5] = [
        JSPropertySpec::getter_setter("x\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_x), Some(js_set_x)),
        JSPropertySpec::getter_setter("y\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_y), Some(js_set_y)),
        JSPropertySpec::getter_setter("z\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_z), Some(js_set_z)),
        JSPropertySpec::getter_setter("w\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_w), Some(js_set_w)),
        JSPropertySpec::end_spec(),
    ];
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn DOMPoint_constructor(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
    let call_args = CreateCallArgsFromVp(argc, vp);
    if call_args._base.argc_ != 4 {
        JS_ReportErrorASCII(cx, b"constructor requires exactly 4 \
                                  arguments\0".as_ptr() as *const
                            libc::c_char);
        return false;
    }

    rooted!(in(cx) let jsobj = jsapi::JS_NewObjectForConstructor(cx,
                                                                 &DOMPOINT_CLASS as *const _,
                                                                 &call_args as *const _));
    if jsobj.is_null() {
        JS_ReportErrorASCII(cx, b"Fail to construct JS object\0".as_ptr() as *const libc::c_char);
        return false;
    }
    let obj = match DOMPoint::from_object(jsobj.get()) {
        Some(o) => o,
        None => {
            JS_ReportErrorASCII(cx, b"Fail to construct DOMPoint from JS \
                                object\0" as *const u8 as *const
                                libc::c_char);
            return false;
        }
    };

    get_js_arg!(x, cx, call_args, 0, ());
    get_js_arg!(y, cx, call_args, 1, ());
    get_js_arg!(z, cx, call_args, 2, ());
    get_js_arg!(w, cx, call_args, 3, ());

    obj.set_x(cx, x);
    obj.set_y(cx, y);
    obj.set_z(cx, z);
    obj.set_w(cx, w);

    call_args.rval().set(ObjectValue(jsobj.get()));
    true
}
