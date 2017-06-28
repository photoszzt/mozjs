/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate js;
extern crate libc;

use js::rust::{Runtime, SIMPLE_GLOBAL_CLASS};
use js::jsapi::root::{JS_NewGlobalObject, JS_InitClass};
use js::jsapi::root::JS::CompartmentOptions;
use js::jsapi::root::JS::OnNewGlobalHookOption;
use js::jsval::UndefinedValue;
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

        rooted!(in(cx) let _node_proto =
                JS_InitClass(cx, global.handle(), proto.handle(), &NODE_CLASS, Some(Node_constructor),
                             5, NODE_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), r#"
let node = new Node(1, "Node", "mozilla/en", false, "n", "h1");
if (Object.getPrototypeOf(node) != Node.prototype) {
    throw Error("node prototype is wrong");
}
if (!(node instanceof Node)) {
    throw Error("is not instance of Node?");
}
if (node.node_type != 1) {
    throw Error("node.node_type is not 1");
}
if (node.node_name != "Node") {
    throw Error("node.node_name is not Node");
}
if (node.base_uri != "mozilla/en") {
    throw Error("node.base_uri is not mozilla/en");
}
if (node.is_connected != false) {
    throw error("node.is_connected is not false");
}
if (node.node_value != "n") {
    throw error("node.node_value is not n");
}
if (node.text_content != "h1") {
    throw error("node.text_content is not h1");
}
node.node_value = "h6";
node.text_content = "<b>";
if (node.node_value != "h6") {
    throw error("node.node_value is not h6");
}
if (node.text_content != "<b>") {
    throw error("node.text_content is not <b>");
}
"#,
                                   "test", 33, rval.handle_mut()).is_ok());
    }
}
