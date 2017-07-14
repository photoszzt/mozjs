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
use js::magicdom::node::NODE_FN_ARR;
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
                7, NODE_PS_ARR.as_ptr(), NODE_FN_ARR.as_ptr(),
                std::ptr::null(), std::ptr::null())
               );

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), r#"
let node = new Node(1, "Node", "mozilla/en", false, "n", "h1", []);
let node1 = new Node(1, "Node2", "mozilla/en", false, "x", "div", []);
let node2 = new Node(1, "Node3", "mozilla/en", false, "p", "div", []);
node.appendChild(node1);
node.appendChild(node2);
let childs = node.child_nodes;
if (childs[0].node_type != 1) {
    throw Error("childs[0].node_type is not 1");
}
if (childs[0].node_name != "Node2") {
    throw Error("childs[0].node_name is not Node");
}
if (childs[0].base_uri != "mozilla/en") {
    throw Error("childs[0].base_uri is not mozilla/en");
}
if (childs[0].is_connected != false) {
    throw Error("childs[0].is_connected is not false");
}
if (childs[0].node_value != "x") {
    throw Error("childs[0].node_value is not n");
}
if (childs[0].text_content != "div") {
    throw Error("childs[0].text_content is not h1");
}
if (childs[0].child_nodes.length != 0) {
    throw Error("childs[0].child_nodes is not empty array");
}
if (childs[1].node_type != 1) {
    throw Error("childs[1].node_type is not 1");
}
if (childs[1].node_name != "Node3") {
    throw Error("childs[1].node_name is not Node");
}
if (childs[1].base_uri != "mozilla/en") {
    throw Error("childs[1].base_uri is not mozilla/en");
}
if (childs[1].is_connected != false) {
    throw Error("childs[1].is_connected is not false");
}
if (childs[1].node_value != "p") {
    throw Error("childs[1].node_value is not n");
}
if (childs[1].text_content != "div") {
    throw Error("childs[1].text_content is not h1");
}
if (childs[1].child_nodes.length != 0) {
    throw Error("childs[1].child_nodes is not empty array");
}
for (var i = 0; i < 4; i++) {
    let node3 = new Node(1, "Node"+i, "mozilla/es", false, "q"+i, "<a>", []);
    node1.appendChild(node3);
}
let newchilds = childs[0].child_nodes;
for (var i = 0; i < 4; i++) {
    if (newchilds[i].node_type != 1) {
        throw Error("newchilds[i].node_type is not 1");
    }
    if (newchilds[i].node_name != ("Node"+i)) {
        throw Error("newchilds[i].node_name is not Node" + i);
    }
    if (newchilds[i].base_uri != "mozilla/es") {
        throw Error("newchilds[i].base_uri is not mozilla/es");
    }
    if (newchilds[i].is_connected != false) {
        throw Error("newchilds[i].is_connected is not false");
    }
    if (newchilds[i].node_value != ("q" + i)) {
        throw Error("newchilds[i].node_value is not a" + i);
    }
    if (newchilds[i].text_content != "<a>") {
        throw Error("newchilds[i].text_content is not <a>");
    }
    if (newchilds[i].child_nodes.length != 0) {
        throw Error("newchilds[i].child_nodes is not empty array");
    }
}
"#,
                                   "test", 33, rval.handle_mut()).is_ok());
    }
}
