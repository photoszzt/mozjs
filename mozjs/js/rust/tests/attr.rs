/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate js;
extern crate libc;

use js::rust::{Runtime, SIMPLE_GLOBAL_CLASS};
use js::rust;
use js::jsapi::root::JS;
use js::jsapi::root::{JS_NewGlobalObject, JS_InitClass};
use js::jsapi::root::JS::CompartmentOptions;
use js::jsapi::root::JS::OnNewGlobalHookOption;
use js::jsval::UndefinedValue;
use js::magicdom::attr::ATTR_CLASS;
use js::magicdom::attr::ATTR_PS_ARR;
use js::magicdom::attr::Attr_constructor;
use js::magicdom::node::NODE_CLASS;
use js::magicdom::node::NODE_PS_ARR;
use js::magicdom::node::Node_constructor;

use std::ptr;
use std::str;

#[test]
fn get_and_set() {
    let rt = Runtime::new().unwrap();
    let cx = rt.cx();

    unsafe {
        rooted!(in(cx) let global =
                JS_NewGlobalObject(cx, &SIMPLE_GLOBAL_CLASS, std::ptr::null_mut(),
                                   OnNewGlobalHookOption::FireOnNewGlobalHook,
                                   &CompartmentOptions::default())
        );

        let _ac = js::ac::AutoCompartment::with_obj(cx, global.get());

        rooted!(in(cx) let proto = ptr::null_mut());

        rooted!(in(cx) let node_proto =
                JS_InitClass(cx, global.handle(), proto.handle(), &NODE_CLASS, Some(Node_constructor),
                             5, NODE_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let _attr_proto =
                JS_InitClass(cx, global.handle(), node_proto.handle(),
                             &ATTR_CLASS, Some(Attr_constructor),
                             11, ATTR_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );
        JS::SetWarningReporter(cx, Some(rust::report_warning));

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), r#"
let attr = new Attr(1, "Node", "mozilla/en", false, "n", "h1", "la", "a", "l", "pp", "foo");
if (Object.getPrototypeOf(attr) != Attr.prototype) {
    throw Error("attr prototype is wrong");
}
if (!(attr instanceof Attr)) {
    throw Error("is not instance of Attr?");
}
if (attr.node_type != 1) {
    throw Error("attr.node_type is not 1");
}
if (attr.node_name != "Node") {
    throw Error("attr.node_name is not Node");
}
if (attr.base_uri != "mozilla/en") {
    throw Error("attr.base_uri is not mozilla/en");
}
if (attr.is_connected != false) {
    throw error("attr.is_connected is not false");
}
if (attr.node_value != "n") {
    throw error("attr.node_value is not n");
}
if (attr.text_content != "h1") {
    throw error("attr.text_content is not h1");
}
attr.node_value = "h6";
attr.text_content = "<b>";
if (attr.node_value != "h6") {
    throw error("attr.node_value is not h6");
}
if (attr.text_content != "<b>") {
    throw error("attr.text_content is not <b>");
}
if (attr.local_name != "la") {
    throw Error("attr.local_name is not la");
}
if (attr.name != "a") {
    throw Error("attr.name is not a");
}
if (attr.namespace != "l") {
    throw Error("attr.namespace is not l");
}
if (attr.prefix != "pp") {
    throw Error("attr.prefix is not pp");
}
if (attr.value != "foo") {
    throw Error("attr.value is not foo");
}
"#,
                                   "test", 20, rval.handle_mut()).is_ok());
    }
}
