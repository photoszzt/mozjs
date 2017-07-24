/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;

extern crate libc;

#[cfg(feature = "native_array")]
magic_dom! {
    Node,
    NODE_CLASS,
    Node_constructor,
    magic_dom_spec_Node,
    struct Node_struct {
        node_type: u16,
        node_name: *mut JSString, // DOMString
        base_uri: *mut JSString, // USVString
        is_connected: bool,
        node_value: *mut JSString, // DOMString
        text_content: *mut JSString,  // DOMString
        child_nodes: Vec<Node>, // array of nodes
    }
}

#[cfg(not(feature = "native_array"))]
magic_dom! {
    Node,
    NODE_CLASS,
    Node_constructor,
    magic_dom_spec_Node,
    struct Node_struct {
        node_type: u16,
        node_name: *mut JSString, // DOMString
        base_uri: *mut JSString, // USVString
        is_connected: bool,
        node_value: *mut JSString, // DOMString
        text_content: *mut JSString,  // DOMString
        child_nodes: *mut JSObject, // array of nodes
    }
}

#[cfg(any(feature = "native_method",feature = "native_array"))]
mod native_method {
    use conversions::{ConversionResult, FromJSValConvertible, ToJSValConvertible};
    use glue::CreateCallArgsFromVp;
    use super::*;

    js_getter!(js_get_node_type, get_node_type, Node);
    js_getter!(js_get_node_name, get_node_name, Node);
    js_getter!(js_get_base_uri, get_base_uri, Node);
    js_getter!(js_get_is_connected, get_is_connected, Node);
    js_getter!(js_get_node_value, get_node_value, Node);
    js_getter!(js_get_text_content, get_text_content, Node);
    js_getter!(js_get_child_nodes, get_child_nodes, Node);

    js_setter!(js_set_node_value, set_node_value, Node, ());
    js_setter!(js_set_text_content, set_text_content, Node, ());

    #[cfg(feature = "native_array")]
    mod native_array {
        use std::ptr;
        use super::*;

        pub extern "C" fn js_appendChild(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
            let res = unsafe {
                let call_args = CreateCallArgsFromVp(argc, vp);
                if call_args._base.argc_ != 1 {
                    JS_ReportErrorASCII(cx,
                                        b"appendChild requires 1 argument\0".as_ptr() as
                                        *const libc::c_char);
                    return false;
                }
                let obj = match Node::check_this(cx, &call_args) {
                    Some(obj_) => obj_,
                    None => {
                        JS_ReportErrorASCII(cx,
                                            b"Can't convert JSObject\0".as_ptr() as *const libc::c_char);
                        return false;
                    }
                };
                get_js_arg_inheritance!(arg1, cx, call_args, 0, &NODE_CLASS, Node);

                let mut nodes = match obj.get_child_nodes(cx) {
                    Some(v) => v,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't get child nodes back\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    }
                };
                nodes.push(arg1);
                obj.set_child_nodes(cx, nodes);
                true
            };
            res
        }
    }
    #[cfg(feature = "native_array")]
    pub use self::native_array::*;

    #[cfg(not(feature = "native_array"))]
    mod js_array {
        use super::*;

        pub extern "C" fn js_appendChild(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
            let res = unsafe {
                let call_args = CreateCallArgsFromVp(argc, vp);
                if call_args._base.argc_ != 1 {
                    JS_ReportErrorASCII(cx,
                                        b"appendChild requires 1 argument\0".as_ptr() as
                                        *const libc::c_char);
                    return false;
                }
                let obj = match Node::check_this(cx, &call_args) {
                    Some(obj_) => obj_,
                    None => {
                        JS_ReportErrorASCII(cx,
                                            b"Can't convert JSObject\0".as_ptr() as *const libc::c_char);
                        return false;
                    }
                };
                let arg1 = call_args.index(0);

                rooted!(in(cx) let nodes = obj.get_child_nodes(cx));
                let mut length: u32 = 0;
                JS_GetArrayLength(cx, nodes.handle(), &mut length);
                JS_SetArrayLength(cx, nodes.handle(), length + 1);
                JS_SetElement(cx, nodes.handle(), length, arg1);
                true
            };
            res
        }
    }
    #[cfg(not(feature = "native_array"))]
    pub use self::js_array::*;

    lazy_static! {
        pub static ref NODE_PS_ARR: [JSPropertySpec; 8] = [
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
            JSPropertySpec::getter(b"child_nodes\0".as_ptr() as *const libc::c_char,
                                   JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                   Some(js_get_child_nodes)),
            JSPropertySpec::end_spec(),
        ];
    }

    lazy_static! {
        pub static ref NODE_FN_ARR: [JSFunctionSpec; 2] = [
            JSFunctionSpec::js_fs(b"appendChild\0".as_ptr() as *const libc::c_char,
                                  Some(js_appendChild),
                                  1,
                                  0
            ),
            JSFunctionSpec::end_spec(),
        ];
    }
}
#[cfg(any(feature = "native_method",feature = "native_array"))]
pub use self::native_method::*;

// self hosted getter and setter
#[cfg(not(any(feature = "native_method",feature = "native_array")))]
mod selfhosted {
    use super::*;

    lazy_static! {
        pub static ref NODE_PS_ARR: [JSPropertySpec; 8] = [
            JSPropertySpec::getter_selfhosted(b"node_type\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              b"Node_get_node_type\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_selfhosted(b"node_name\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              b"Node_get_node_name\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_selfhosted(b"base_uri\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              b"Node_get_base_uri\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_selfhosted(b"is_connected\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              b"Node_get_is_connected\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_setter_selfhosted(b"node_value\0".as_ptr() as *const libc::c_char,
                                                     JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                     b"Node_get_node_value\0".as_ptr()
                                                     as *const libc::c_char,
                                                     b"Node_set_node_value\0".as_ptr()
                                                     as *const libc::c_char,
            ),
            JSPropertySpec::getter_setter_selfhosted(b"text_content\0".as_ptr() as *const libc::c_char,
                                                     JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                     b"Node_get_text_content\0".as_ptr()
                                                     as *const libc::c_char,
                                                     b"Node_set_text_content\0".as_ptr()
                                                     as *const libc::c_char,
            ),
            JSPropertySpec::getter_selfhosted(b"child_nodes\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              b"Node_get_child_nodes\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::end_spec(),
        ];
    }

    lazy_static! {
        pub static ref NODE_FN_ARR: [JSFunctionSpec; 2] = [
            JSFunctionSpec::js_selfhosted_fn(b"appendChild\0".as_ptr() as *const libc::c_char,
                                             b"Node_appendChild\0".as_ptr() as *const libc::c_char,
                                             1,
                                             0
            ),
            JSFunctionSpec::end_spec(),
        ];
    }
}
#[cfg(not(any(feature = "native_method",feature = "native_array")))]
pub use self::selfhosted::*;
