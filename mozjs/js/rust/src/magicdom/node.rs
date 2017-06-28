/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
use conversions::{ConversionResult, FromJSValConvertible, ToJSValConvertible};
use glue::CreateCallArgsFromVp;

extern crate libc;

magic_dom! {
    Node,
    NODE_CLASS,
    Node_constructor,
    magic_dom_spec_Node,
    struct Node_struct {
        node_type: u16,
        node_name: String, // DOMString
        base_uri: String, // USVString
        is_connected: bool,
        node_value: String, // DOMString
        text_content: String,  // DOMString
    }
}

js_getter!(js_get_node_type, get_node_type, Node);
js_getter!(js_get_node_name, get_node_name, Node);
js_getter!(js_get_base_uri, get_base_uri, Node);
js_getter!(js_get_is_connected, get_is_connected, Node);
js_getter!(js_get_node_value, get_node_value, Node);
js_getter!(js_get_text_content, get_text_content, Node);

js_setter!(js_set_node_value, set_node_value, Node, ());
js_setter!(js_set_text_content, set_text_content, Node, ());

lazy_static! {
    pub static ref NODE_PS_ARR: [JSPropertySpec; 7] = [
        JSPropertySpec::getter(b"node_type\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_node_type)),
        JSPropertySpec::getter(b"node_name\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_node_name)),
        JSPropertySpec::getter(b"base_uri\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_base_uri)),
        JSPropertySpec::getter(b"is_connected\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_is_connected)),
        JSPropertySpec::getter_setter(b"node_value\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_node_value), Some(js_set_node_value)),
        JSPropertySpec::getter_setter(b"text_content\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_text_content), Some(js_set_text_content)),
        JSPropertySpec::end_spec(),
    ];
}
