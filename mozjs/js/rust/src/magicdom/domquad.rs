/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
#[cfg(feature = "native_method")]
use conversions::ToJSValConvertible;
#[cfg(feature = "native_method")]
use glue::CreateCallArgsFromVp;

extern crate libc;

magic_dom! {
    DOMQuad,
    DOMQUAD_CLASS,
    DOMQuad_constructor,
    magic_dom_spec_DOMQuad,
    struct DOMQuad_spec {
        p1: dompoint::DOMPoint,
        p2: dompoint::DOMPoint,
        p3: dompoint::DOMPoint,
        p4: dompoint::DOMPoint,
    }
}

// Exposing native rust method to js side
#[cfg(feature = "native_method")]
js_getter!(js_get_p1, get_p1, DOMQuad);
#[cfg(feature = "native_method")]
js_getter!(js_get_p2, get_p2, DOMQuad);
#[cfg(feature = "native_method")]
js_getter!(js_get_p3, get_p3, DOMQuad);
#[cfg(feature = "native_method")]
js_getter!(js_get_p4, get_p4, DOMQuad);

#[cfg(feature = "native_method")]
lazy_static! {
    pub static ref DOMQUAD_PS_ARR: [JSPropertySpec; 5] = [
        JSPropertySpec::getter(b"p1\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_p1)),
        JSPropertySpec::getter(b"p2\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_p2)),
        JSPropertySpec::getter(b"p3\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_p3)),
        JSPropertySpec::getter(b"p4\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_p4)),
        JSPropertySpec::end_spec(),
    ];
}

// self hosted getter and setter
#[cfg(not(feature = "native_method"))]
lazy_static! {
    pub static ref DOMQUAD_PS_ARR: [JSPropertySpec; 5] = [
        JSPropertySpec::getter_selfhosted("p1\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "DOMQuad_get_p1\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_selfhosted("p2\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "DOMQuad_get_p2\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_selfhosted("p3\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "DOMQuad_get_p3\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_selfhosted("p4\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "DOMQuad_get_p4\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::end_spec(),
    ];
}
