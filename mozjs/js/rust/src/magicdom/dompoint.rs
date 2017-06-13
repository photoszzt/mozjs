/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
use conversions::{ConversionResult, FromJSValConvertible, ToJSValConvertible};
use glue::CreateCallArgsFromVp;

extern crate libc;

magic_dom! {
    DOMPoint,
    DOMPOINT_CLASS,
    DOMPoint_constructor,
    magic_dom_spec_DOMPoint,
    struct DOMPoint_spec {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }
}

js_getter!(js_get_x, get_x, DOMPoint);
js_getter!(js_get_y, get_y, DOMPoint);
js_getter!(js_get_z, get_z, DOMPoint);
js_getter!(js_get_w, get_w, DOMPoint);

js_setter!(js_set_x, set_x, DOMPoint);
js_setter!(js_set_y, set_y, DOMPoint);
js_setter!(js_set_z, set_z, DOMPoint);
js_setter!(js_set_w, set_w, DOMPoint);

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

