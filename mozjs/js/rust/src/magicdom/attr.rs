/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
use conversions::ToJSValConvertible;
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

js_getter!(js_get_identifier_local_name, get_identifier_local_name, Attr);
js_getter!(js_get_identifier_name, get_identifier_name, Attr);
js_getter!(js_get_identifier_namespace, get_identifier_namespace, Attr);
js_getter!(js_get_identifier_prefix, get_identifier_prefix, Attr);
js_getter!(js_get_value, get_value, Attr);

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
