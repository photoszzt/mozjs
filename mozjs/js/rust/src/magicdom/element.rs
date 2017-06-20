/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
use conversions::{ConversionResult, FromJSValConvertible, ToJSValConvertible};
use glue::CreateCallArgsFromVp;

extern crate libc;

magic_dom! {
    Element,
    ELEMENT_CLASS,
    Element_constructor,
    magic_dom_spec_Element,
    struct Element_spec {
        // TODO need to put Node here

        // TODO need to check the local_name, tag_name, namespace and prefix are valid html element
        // They should be Gekco Atom from the servo description
        local_name: String,
        tag_name: String,
        namespace: String,
        prefix: String,
        id: String,
        attrs: Vec<attr::Attr>,
        // TODO some of the fields are pointer to Element, those comes in later
    }
}

pub use self::magic_dom_spec_Element::Element_constructor as Element_constructor;

js_getter!(js_get_local_name, get_local_name, Element);
js_getter!(js_get_tag_name, get_tag_name, Element);
js_getter!(js_get_namespace, get_namespace, Element);
js_getter!(js_get_prefix, get_prefix, Element);
js_getter!(js_get_id, get_id, Element);
js_getter!(js_get_attrs, get_attrs, Element);

js_setter!(js_set_id, set_id, Element, ());

lazy_static! {
    pub static ref ELEMENT_PS_ARR: [JSPropertySpec; 7] = [
        JSPropertySpec::getter(b"local_name\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_local_name)),
        JSPropertySpec::getter(b"tag_name\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_tag_name)),
        JSPropertySpec::getter(b"namespace\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_namespace)),
        JSPropertySpec::getter(b"prefix\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_prefix)),
        JSPropertySpec::getter_setter(b"id\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_id), Some(js_set_id)),
        JSPropertySpec::getter(b"attrs\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_attrs)),
        JSPropertySpec::end_spec(),
    ];
}
