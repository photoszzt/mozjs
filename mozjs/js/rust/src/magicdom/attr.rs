/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
#[cfg(feature = "native_method")]
use conversions::ToJSValConvertible;
#[cfg(feature = "native_method")]
use glue::CreateCallArgsFromVp;

extern crate libc;

// A simple proto type that's different from what servo has
magic_dom! {
    Attr,
    ATTR_CLASS,
    Attr_constructor,
    magic_dom_spec_Attr,
    struct Attr_spec {
        identifier_local_name: *mut JSString,
        identifier_name: *mut JSString,
        identifier_namespace: *mut JSString,
        identifier_prefix: *mut JSString,
        value: *mut JSString, // TODO should be a enum inside servo fake it with one of possible value
    }
}

// Exposing native rust method to js side
#[cfg(feature = "native_method")]
js_getter!(js_get_identifier_local_name, get_identifier_local_name, Attr);
#[cfg(feature = "native_method")]
js_getter!(js_get_identifier_name, get_identifier_name, Attr);
#[cfg(feature = "native_method")]
js_getter!(js_get_identifier_namespace, get_identifier_namespace, Attr);
#[cfg(feature = "native_method")]
js_getter!(js_get_identifier_prefix, get_identifier_prefix, Attr);
#[cfg(feature = "native_method")]
js_getter!(js_get_value, get_value, Attr);

#[cfg(feature = "native_method")]
lazy_static! {
    pub static ref ATTR_PS_ARR: [JSPropertySpec; 6] = [
        JSPropertySpec::getter(b"local_name\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_identifier_local_name)),
        JSPropertySpec::getter(b"name\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_identifier_name)),
        JSPropertySpec::getter(b"namespace\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_identifier_namespace)),
        JSPropertySpec::getter(b"prefix\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_identifier_prefix)),
        JSPropertySpec::getter(b"value\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_value)),
        JSPropertySpec::end_spec(),
    ];
}

// self hosted getter and setter
#[cfg(not(feature = "native_method"))]
lazy_static! {
    pub static ref ATTR_PS_ARR: [JSPropertySpec; 6] = [
        JSPropertySpec::getter_selfhosted(b"local_name\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "Attr_get_identifier_local_name\0".as_ptr()
                                          as *const libc::c_char,
        ),
        JSPropertySpec::getter_selfhosted(b"name\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "Attr_get_identifier_name\0".as_ptr()
                                          as *const libc::c_char,
        ),
        JSPropertySpec::getter_selfhosted(b"namespace\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "Attr_get_identifier_namespace\0".as_ptr()
                                          as *const libc::c_char,
        ),
        JSPropertySpec::getter_selfhosted(b"prefix\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "Attr_get_identifier_prefix\0".as_ptr()
                                          as *const libc::c_char,
        ),
        JSPropertySpec::getter_selfhosted(b"value\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "Attr_get_value\0".as_ptr()
                                          as *const libc::c_char,
        ),
        JSPropertySpec::end_spec(),
    ];
}
