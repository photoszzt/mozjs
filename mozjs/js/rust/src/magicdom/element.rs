/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
use jsslotconversions::ToFromJsSlots;
use magicdom::node::Node;

extern crate libc;

#[cfg(feature = "native_array")]
magic_dom! {
    Element,
    ELEMENT_CLASS,
    Element_constructor,
    magic_dom_spec_Element,
    struct Element_spec {
        // TODO need to check the local_name, tag_name, namespace and prefix are valid html element
        // They should be Gekco Atom from the servo description
        _inherit: node::Node,
        local_name: *mut JSString,
        tag_name: *mut JSString,
        namespace: *mut JSString,
        prefix: *mut JSString,
        id: *mut JSString,
        attrs: Vec<attr::Attr>,
        // TODO some of the fields are pointer to Element, those comes in later
    }
}

#[cfg(not(feature = "native_array"))]
magic_dom! {
    Element,
    ELEMENT_CLASS,
    Element_constructor,
    magic_dom_spec_Element,
    struct Element_spec {
        // TODO need to check the local_name, tag_name, namespace and prefix are valid html element
        // They should be Gekco Atom from the servo description
        _inherit: node::Node,
        local_name: *mut JSString,
        tag_name: *mut JSString,
        namespace: *mut JSString,
        prefix: *mut JSString,
        id: *mut JSString,
        attrs: *mut JSObject,
        // TODO some of the fields are pointer to Element, those comes in later
    }
}

impl Element {
    gen_getter_inherit!(get_node_type, u16, as_Node);
    gen_getter_inherit!(get_node_name, *mut JSString, as_Node);
    gen_getter_inherit!(get_base_uri, *mut JSString, as_Node);
    gen_getter_inherit!(get_is_connected, bool, as_Node);
    gen_getter_inherit!(get_node_value, *mut JSString, as_Node);
    gen_getter_inherit!(get_text_content, *mut JSString, as_Node);
    #[cfg(feature = "native_array")]
    gen_getter_inherit!(get_child_nodes, Vec<Node>, as_Node);
    #[cfg(not(feature = "native_array"))]
    gen_getter_inherit!(get_child_nodes, *mut JSObject, as_Node);
}

#[cfg(any(feature = "native_method",feature = "native_array"))]
mod native {
    use jsapi::root::*;
    use conversions::{ConversionResult, FromJSValConvertible};
    use jsapi::{JS_CompareStrings, JS_AtomizeAndPinString};
    use magicdom::attr::{Attr, ATTR_CLASS};
    use conversions::ToJSValConvertible;
    use glue::CreateCallArgsFromVp;
    use super::*;
    use std::ptr;

    extern crate libc;

    // Exposing native rust method to js side
    js_getter!(js_get_local_name, get_local_name, Element);
    js_getter!(js_get_tag_name, get_tag_name, Element);
    js_getter!(js_get_namespace, get_namespace, Element);
    js_getter!(js_get_prefix, get_prefix, Element);
    js_getter!(js_get_id, get_id, Element);
    js_getter!(js_get_attrs, get_attrs, Element);

    js_setter!(js_set_id, set_id, Element, ());

    #[cfg(feature = "native_array")]
    mod native_array_getsetattr {
        use super::*;

        macro_rules! get_qualified_name {
            ($cx:ident, $attr:ident) => {
                {
                    rooted!(in($cx) let prefix = $attr.get_identifier_prefix($cx));
                    rooted!(in($cx) let local_name = $attr.get_identifier_local_name($cx));
                    if prefix.is_null() {
                        local_name.get()
                    } else {
                        rooted!(in($cx) let column =
                                JS_AtomizeAndPinString($cx, b":\0".as_ptr() as *const libc::c_char));
                        rooted!(in($cx) let str1 = JS_ConcatStrings($cx, prefix.handle(), column.handle()));
                        JS_ConcatStrings($cx, str1.handle(), local_name.handle())
                    }
                }
            }
        }

        pub extern "C" fn js_getAttributes(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
            let res = unsafe {
                let call_args = CreateCallArgsFromVp(argc, vp);
                if call_args._base.argc_ != 1 {
                    JS_ReportErrorASCII(cx, b"getAttributes requires 1 argument\0".as_ptr()
                                        as *const libc::c_char);
                    return false;
                }
                let obj = match Element::check_this(cx, &call_args) {
                    Some(obj_) => obj_,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't convert JSObject\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    },
                };
                get_js_arg!(arg1, cx, call_args, 0, ());

                let attrs = match obj.get_attrs(cx) {
                    Some(v) => v,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't get attribute back\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    }
                };
                let mut ret: *mut JSString = ptr::null_mut();
                let mut mat_res : i32 = 0;
                for attr in attrs.iter() {
                    let qname = get_qualified_name!(cx, attr);
                    JS_CompareStrings(cx, qname, arg1, &mut mat_res);
                    if mat_res == 0 {
                        ret = attr.get_value(cx);
                        break;
                    }
                }
                ret.to_jsval(cx, call_args.rval());
                // Need to set it back, otherwise calling this function the second time will break.
                obj.set_attrs(cx, attrs);
                true
            };
            res
        }

        pub extern "C" fn js_setAttributes(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
            println!("set attribute");
            let res = unsafe {
                let call_args = CreateCallArgsFromVp(argc, vp);
                if call_args._base.argc_ != 2 {
                    JS_ReportErrorASCII(cx, b"getAttributes requires 2 argument\0".as_ptr()
                                        as *const libc::c_char);
                    return false;
                }
                let obj = match Element::check_this(cx, &call_args) {
                    Some(obj_) => obj_,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't convert JSObject\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    },
                };
                get_js_arg!(arg1, cx, call_args, 0, ());
                get_js_arg!(arg2, cx, call_args, 1, ());
                let mut attrs = match obj.get_attrs(cx) {
                    Some(v) => v,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't get attribute back\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    }
                };

                let mut found = false;
                let mut mat_res : i32 = 0;
                for attr in attrs.iter() {
                    let qname = get_qualified_name!(cx, attr);
                    JS_CompareStrings(cx, qname, arg1, &mut mat_res);
                    if mat_res == 0 {
                        found = true;
                        attr.set_value(cx, arg2);
                        break;
                    }
                }
                if !found {
                    rooted!(in(cx) let jsobj = JS_NewObjectForConstructor(cx, &ATTR_CLASS as *const _,
                                                                          &call_args as *const _));
                    if jsobj.is_null() {
                        JS_ReportErrorASCII(cx, b"Fail to construct JS object\0".as_ptr() as *const libc::c_char);
                        return false;
                    }
                    let obj = match Attr::from_object(jsobj.get()) {
                        Some(o) => o,
                        None => {
                            JS_ReportErrorASCII(cx, b"Fail to construct Attr from JS \
                                                      object\0" as *const u8 as *const
                                                libc::c_char);
                            return false;
                        }
                    };
                    obj.set_identifier_local_name(cx, arg1);
                    obj.set_value(cx, arg2);
                    obj.set_identifier_name(cx, ptr::null_mut());
                    obj.set_identifier_prefix(cx, ptr::null_mut());
                    obj.set_identifier_namespace(cx, ptr::null_mut());
                    attrs.push(obj);
                }
                // Need to set it back, otherwise calling this function the second time will break.
                obj.set_attrs(cx, attrs);
                true
            };
            res
        }
    }
    #[cfg(feature = "native_array")]
    pub use self::native_array_getsetattr::*;

    #[cfg(not(feature = "native_array"))]
    mod js_array_getsetattr {
        use super::*;
        use conversions::ForOfIteratorGuard;
        use jsval::{ObjectValue, UndefinedValue};

        macro_rules! get_attr_and_qualified_name {
            ($cx:ident, $val:ident) => {
                {
                    let arr_val = match <Attr as FromJSValConvertible>::from_jsval($cx, $val.handle(), ()) {
                        Ok(val) => {
                            match val {
                                ConversionResult::Success(v) => v,
                                ConversionResult::Failure(e) => {
                                    JS_ReportErrorASCII($cx, b"Should never put anything into a slot that we \
                                                               can't convert from JS Value\0".as_ptr()
                                                        as *const libc::c_char);
                                    debug!("{}", e);
                                    return false;
                                },
                            }
                        },
                        Err(_) => {
                            JS_ReportErrorASCII($cx, b"Can't recognize val\0".as_ptr() as *const libc::c_char);
                            return false;
                        },
                    };
                    rooted!(in($cx) let prefix = arr_val.get_identifier_prefix($cx));
                    rooted!(in($cx) let local_name = arr_val.get_identifier_local_name($cx));
                    if prefix.is_null() {
                        (arr_val, local_name.get())
                    } else {
                        rooted!(in($cx) let column =
                                JS_AtomizeAndPinString($cx, b":\0".as_ptr() as *const libc::c_char));
                        rooted!(in($cx) let str1 = JS_ConcatStrings($cx, prefix.handle(), column.handle()));
                        (arr_val, JS_ConcatStrings($cx, str1.handle(), local_name.handle()))
                    }
                }
            }
        }

        pub extern "C" fn js_getAttributes(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
            let res = unsafe {
                let call_args = CreateCallArgsFromVp(argc, vp);
                if call_args._base.argc_ != 1 {
                    JS_ReportErrorASCII(cx, b"getAttributes requires 1 argument\0".as_ptr()
                                        as *const libc::c_char);
                    return false;
                }
                let obj = match Element::check_this(cx, &call_args) {
                    Some(obj_) => obj_,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't convert JSObject\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    },
                };
                get_js_arg!(arg1, cx, call_args, 0, ());

                rooted!(in(cx) let value = ObjectValue(obj.get_attrs(cx)));
                let mut iterator = JS::ForOfIterator {
                    cx_: cx,
                    iterator: JS::RootedObject::new_unrooted(),
                    index: ::std::u32::MAX, // NOT_ARRAY
                };
                let mut iterator = ForOfIteratorGuard::new(cx, &mut iterator);
                let iterator = &mut *iterator.root;

                if !iterator.init(value.handle(), JS::ForOfIterator_NonIterableBehavior::AllowNonIterable) {
                    return false;
                }

                if iterator.iterator.ptr.is_null() {
                    return false;
                }

                let mut ret: *mut JSString = ptr::null_mut();
                let mut mat_res : i32 = 0;
                loop {
                    let mut done = false;
                    rooted!(in(cx) let mut val = UndefinedValue());
                    if !iterator.next(val.handle_mut(), &mut done) {
                        return false;
                    }

                    if done {
                        break;
                    }
                    let (arr_val, qualified_name) = get_attr_and_qualified_name!(cx, val);
                    JS_CompareStrings(cx, qualified_name, arg1, &mut mat_res);
                    if mat_res == 0 {
                        ret = arr_val.get_value(cx);
                        break;
                    }
                }
                ret.to_jsval(cx, call_args.rval());
                true
            };
            res
        }

        pub extern "C" fn js_setAttributes(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
            let res = unsafe {
                let call_args = CreateCallArgsFromVp(argc, vp);
                if call_args._base.argc_ != 2 {
                    JS_ReportErrorASCII(cx, b"getAttributes requires 2 argument\0".as_ptr()
                                        as *const libc::c_char);
                    return false;
                }
                let obj = match Element::check_this(cx, &call_args) {
                    Some(obj_) => obj_,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't convert JSObject\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    },
                };
                get_js_arg!(arg1, cx, call_args, 0, ());
                get_js_arg!(arg2, cx, call_args, 1, ());
                rooted!(in(cx) let attrs = obj.get_attrs(cx));
                rooted!(in(cx) let value = ObjectValue(attrs.get()));
                let mut iterator = JS::ForOfIterator {
                    cx_: cx,
                    iterator: JS::RootedObject::new_unrooted(),
                    index: ::std::u32::MAX, // NOT_ARRAY
                };
                let mut iterator = ForOfIteratorGuard::new(cx, &mut iterator);
                let iterator = &mut *iterator.root;

                if !iterator.init(value.handle(), JS::ForOfIterator_NonIterableBehavior::AllowNonIterable) {
                    return false;
                }

                if iterator.iterator.ptr.is_null() {
                    return false;
                }

                let mut found = false;
                let mut mat_res : i32 = 0;
                loop {
                    let mut done = false;
                    rooted!(in(cx) let mut val = UndefinedValue());
                    if !iterator.next(val.handle_mut(), &mut done) {
                        return false;
                    }

                    if done {
                        break;
                    }
                    let (arr_val, qualified_name) = get_attr_and_qualified_name!(cx, val);
                    JS_CompareStrings(cx, qualified_name, arg1, &mut mat_res);
                    if mat_res == 0 {
                        found = true;
                        arr_val.set_value(cx, arg2);
                        break;
                    }
                }
                if !found {
                    rooted!(in(cx) let jsobj = JS_NewObjectForConstructor(cx, &ATTR_CLASS as *const _,
                                                                          &call_args as *const _));
                    if jsobj.is_null() {
                        JS_ReportErrorASCII(cx, b"Fail to construct JS object\0".as_ptr() as *const libc::c_char);
                        return false;
                    }
                    let obj = match Attr::from_object(jsobj.get()) {
                        Some(o) => o,
                        None => {
                            JS_ReportErrorASCII(cx, b"Fail to construct Attr from JS \
                                                      object\0" as *const u8 as *const
                                                libc::c_char);
                            return false;
                        }
                    };
                    obj.set_identifier_local_name(cx, arg1);
                    obj.set_value(cx, arg2);
                    obj.set_identifier_name(cx, ptr::null_mut());
                    obj.set_identifier_prefix(cx, ptr::null_mut());
                    obj.set_identifier_namespace(cx, ptr::null_mut());
                    let mut length: u32 = 0;
                    JS_GetArrayLength(cx, attrs.handle(), &mut length);
                    JS_SetArrayLength(cx, attrs.handle(), length + 1);
                    rooted!(in(cx) let mut val1 = UndefinedValue());
                    obj.to_jsval(cx, val1.handle_mut());
                    JS_SetElement(cx, attrs.handle(), length, val1.handle());
                    JS_GetArrayLength(cx, attrs.handle(), &mut length);
                }
                true
            };
            res
        }
    }
    #[cfg(not(feature = "native_array"))]
    pub use self::js_array_getsetattr::*;

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

    lazy_static! {
        pub static ref ELEMENT_FN_ARR: [JSFunctionSpec; 3] = [
            JSFunctionSpec::js_fs(b"getAttributes\0".as_ptr() as *const libc::c_char,
                                  Some(js_getAttributes),
                                  1,
                                  0
            ),
            JSFunctionSpec::js_fs(b"setAttributes\0".as_ptr() as *const libc::c_char,
                                  Some(js_setAttributes),
                                  2,
                                  0
            ),
            JSFunctionSpec::end_spec(),
        ];
    }
}
#[cfg(any(feature = "native_method",feature = "native_array"))]
pub use self::native::*;

// self hosted getter and setter
#[cfg(not(any(feature = "native_method",feature = "native_array")))]
mod selfhosted {
    use jsapi::root::*;
    extern crate libc;

    lazy_static! {
        pub static ref ELEMENT_PS_ARR: [JSPropertySpec; 7] = [
            JSPropertySpec::getter_selfhosted(b"local_name\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              "Element_get_local_name\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_selfhosted(b"tag_name\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              "Element_get_tag_name\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_selfhosted(b"namespace\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              "Element_get_namespace\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_selfhosted(b"prefix\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              "Element_get_prefix\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_setter_selfhosted(b"id\0".as_ptr() as *const libc::c_char,
                                                     JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                     "Element_get_id\0".as_ptr() as *const libc::c_char,
                                                     "Element_set_id\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::getter_selfhosted(b"attrs\0".as_ptr() as *const libc::c_char,
                                              JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                              "Element_get_attrs\0".as_ptr() as *const libc::c_char,
            ),
            JSPropertySpec::end_spec(),
        ];
    }

    lazy_static! {
        pub static ref ELEMENT_FN_ARR: [JSFunctionSpec; 3] = [
            JSFunctionSpec::js_selfhosted_fn(b"getAttributes\0".as_ptr() as *const libc::c_char,
                                             b"Element_getAttributes\0".as_ptr() as *const libc::c_char,
                                             1,
                                             0,
            ),
            JSFunctionSpec::js_selfhosted_fn(b"setAttributes\0".as_ptr() as *const libc::c_char,
                                             b"Element_setAttributes\0".as_ptr() as *const libc::c_char,
                                             2,
                                             0,
            ),
            JSFunctionSpec::end_spec(),
        ];
    }
}
#[cfg(not(any(feature = "native_method",feature = "native_array")))]
pub use self::selfhosted::*;
