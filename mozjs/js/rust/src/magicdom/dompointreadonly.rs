/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;

extern crate libc;

magic_dom! {
    DOMPointReadOnly,
    DOMPOINTREADONLY_CLASS,
    DOMPointReadOnly_constructor,
    magic_dom_spec_DOMPointReadOnly,
    struct DOMPointReadOnly_spec {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }
}

// Exposing native rust method to js side
#[cfg(feature = "native_method")]
mod native {
    use jsapi::root::*;
    use conversions::ToJSValConvertible;
    use glue::CreateCallArgsFromVp;
    use super::*;

    js_getter!(js_get_x, get_x, DOMPointReadOnly);
    js_getter!(js_get_y, get_y, DOMPointReadOnly);
    js_getter!(js_get_z, get_z, DOMPointReadOnly);
    js_getter!(js_get_w, get_w, DOMPointReadOnly);

    lazy_static! {
        pub static ref DOMPOINTREADONLY_PS_ARR: [JSPropertySpec; 5] = [
            JSPropertySpec::getter("x\0".as_ptr() as *const libc::c_char,
                                   JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                   Some(js_get_x)),
            JSPropertySpec::getter("y\0".as_ptr() as *const libc::c_char,
                                   JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                   Some(js_get_y)),
            JSPropertySpec::getter("z\0".as_ptr() as *const libc::c_char,
                                   JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                   Some(js_get_z)),
            JSPropertySpec::getter("w\0".as_ptr() as *const libc::c_char,
                                   JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                   Some(js_get_w)),
            JSPropertySpec::end_spec(),
        ];
    }
}
#[cfg(feature = "native_method")]
pub use self::native::*;

#[cfg(not(feature = "native_method"))]
lazy_static! {
    pub static ref DOMPOINTREADONLY_PS_ARR: [JSPropertySpec; 5] = [
        JSPropertySpec::getter_selfhosted("x\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "DOMPointReadOnly_get_x\0".as_ptr() as *const libc::c_char),
        JSPropertySpec::getter_selfhosted("y\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "DOMPointReadOnly_get_y\0".as_ptr() as *const libc::c_char),
        JSPropertySpec::getter_selfhosted("z\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "DOMPointReadOnly_get_z\0".as_ptr() as *const libc::c_char),
        JSPropertySpec::getter_selfhosted("w\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "DOMPointReadOnly_get_w\0".as_ptr() as *const libc::c_char),
        JSPropertySpec::end_spec(),
    ];
}
