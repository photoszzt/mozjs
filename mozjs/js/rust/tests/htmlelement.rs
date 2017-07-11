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
use js::magicdom::element::ELEMENT_CLASS;
use js::magicdom::element::ELEMENT_PS_ARR;
use js::magicdom::element::ELEMENT_FN_ARR;
use js::magicdom::element::Element_constructor;
use js::magicdom::htmlelement::HTMLELEMENT_CLASS;
use js::magicdom::htmlelement::HTMLELEMENT_PS_ARR;
use js::magicdom::htmlelement::HtmlElement_constructor;
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
                             6, NODE_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let element_proto =
                JS_InitClass(cx, global.handle(), node_proto.handle(), &ELEMENT_CLASS, Some(Element_constructor),
                             12, ELEMENT_PS_ARR.as_ptr(), ELEMENT_FN_ARR.as_ptr(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let _attr_proto =
                JS_InitClass(cx, global.handle(), node_proto.handle(),
                             &ATTR_CLASS, Some(Attr_constructor),
                             11, ATTR_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let _html_element_proto =
                JS_InitClass(cx, global.handle(), element_proto.handle(),
                             &HTMLELEMENT_CLASS, Some(HtmlElement_constructor),
                             22, HTMLELEMENT_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        JS::SetWarningReporter(cx, Some(rust::report_warning));

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), r#"
let attr1 = new Attr(1, "Node", "mozilla/en", false, "n", "h1", "l", "a", "l", "p", "f");
let attr2 = new Attr(1, "Node", "mozilla/en", false, "n", "h1", "l", "b", "l", "p", "b");
let element = new HtmlElement(1, "Node", "mozilla/en", false, "n", "h1", "la", "a", "l", "pp", "foo", [attr1, attr2], "title",
"en", false, "dir", false, 1, "ackey", "ackeylabel", false, false);
if (Object.getPrototypeOf(element) != HtmlElement.prototype) {
    throw Error("element prototype is wrong");
}
if (!(element instanceof HtmlElement)) {
    throw Error("is not instance of HtmlElement?");
}
if (element.node_type != 1) {
    throw Error("element.node_type is not 1");
}
if (element.node_name != "Node") {
    throw Error("element.node_name is not Node");
}
if (element.base_uri != "mozilla/en") {
    throw Error("element.base_uri is not mozilla/en");
}
if (element.is_connected != false) {
    throw error("element.is_connected is not false");
}
if (element.node_value != "n") {
    throw error("element.node_value is not n");
}
if (element.text_content != "h1") {
    throw error("element.text_content is not h1");
}
element.node_value = "h6";
element.text_content = "<b>";
if (element.node_value != "h6") {
    throw error("element.node_value is not h6");
}
if (element.text_content != "<b>") {
    throw error("element.text_content is not <b>");
}
if (element.local_name != "la") {
    throw Error("element.local_name is not la");
}
if (element.tag_name != "a") {
    throw Error("element.tag_name is not a");
}
if (element.namespace != "l") {
    throw Error("element.namespace is not l");
}
if (element.prefix != "pp") {
    throw Error("element.prefix is not l");
}
if (element.id != "foo") {
    throw Error("element.id is not foo");
}
let attrss = element.attrs;
if (attrss[0].node_type != 1) {
    throw Error("attrss[0].node_type is not 1");
}
if (attrss[0].node_name != "Node") {
    throw Error("attrss[0].node_name is not Node");
}
if (attrss[0].base_uri != "mozilla/en") {
    throw Error("attrss[0].base_uri is not mozilla/en");
}
if (attrss[0].is_connected != false) {
    throw error("attrss[0].is_connected is not false");
}
if (attrss[0].node_value != "n") {
    throw error("attrss[0].node_value is not n");
}
if (attrss[0].text_content != "h1") {
    throw error("attrss[0].text_content is not h1");
}
attrss[0].node_value = "h6";
attrss[0].text_content = "<b>";
if (attrss[0].node_value != "h6") {
    throw error("attrss[0].node_value is not h6");
}
if (attrss[0].text_content != "<b>") {
    throw error("attrss[0].text_content is not <b>");
}
if (attrss[0].local_name != "l") {
    throw Error("attr.local_name is not la");
}
if (attrss[0].name != "a") {
    throw Error("attrss[0].name is not a");
}
if (attrss[0].namespace != "l") {
    throw Error("attrss[0].namespace is not l");
}
if (attrss[0].prefix != "p") {
    throw Error("attrss[0].prefix is not pp");
}
if (attrss[0].value != "f") {
    throw Error("attrss[0].value is not foo");
}
if (attrss[1].node_type != 1) {
    throw Error("attrss[1].node_type is not 1");
}
if (attrss[1].node_name != "Node") {
    throw Error("attrss[1].node_name is not Node");
}
if (attrss[1].base_uri != "mozilla/en") {
    throw Error("attrss[1].base_uri is not mozilla/en");
}
if (attrss[1].is_connected != false) {
    throw error("attrss[1].is_connected is not false");
}
if (attrss[1].node_value != "n") {
    throw error("attrss[1].node_value is not n");
}
if (attrss[1].text_content != "h1") {
    throw error("attrss[1].text_content is not h1");
}
attrss[1].node_value = "h6";
attrss[1].text_content = "<b>";
if (attrss[1].node_value != "h6") {
    throw error("attrss[1].node_value is not h6");
}
if (attrss[1].text_content != "<b>") {
    throw error("attrss[1].text_content is not <b>");
}
if (attrss[1].local_name != "l") {
    throw Error("attr.local_name is not lb");
}
if (attrss[1].name != "b") {
    throw Error("attrss[1].name is not b");
}
if (attrss[1].namespace != "l") {
    throw Error("attrss[1].namespace is not l");
}
if (attrss[1].prefix != "p") {
    throw Error("attrss[1].prefix is not pp");
}
if (attrss[1].value != "b") {
    throw Error("attrss[1].value is not boo");
}
let value = element.getAttributes("p:l");
if (value != "f") {
    throw Error("value is not foo");
}
element.setAttributes("p:l", "baz");
let value2 = element.getAttributes("p:l");
if (value2 != "baz") {
    throw Error("value is not baz");
}
element.setAttributes("id", "idbaz");
let value3 = element.getAttributes("id");
if (value3 != "idbaz") {
    throw Error("value is not idbaz");
}
element.id = "bar";
if (element.id != "bar") {
    throw Error("element.id is not bar");
}
if (element.title != "title") {
    throw Error("element.title is not title");
}
if (element.lang != "en") {
    throw Error("element.lang is not en");
}
if (element.translate != false) {
    throw Error("element.translate is not false");
}
if (element.dir != "dir") {
    throw Error("element.dir is not dir");
}
if (element.hidden != false) {
    throw Error("element.hidden is not false");
}
if (element.tabIndex != 1) {
    throw Error("element.tabIndex is not 1");
}
if (element.accessKey != "ackey") {
    throw Error("element.accessKey is not ackey");
}
if (element.accessKeyLabel != "ackeylabel") {
    throw Error("element.accessKeyLabel is not ackeylabel");
}
if (element.draggable != false) {
    throw Error("element.draggable is not false");
}
if (element.spellcheck != false) {
    throw Error("element.spellcheck is not false");
}
element.title = "title2";
element.lang = "es";
element.translate = true;
element.dir = "dir1";
element.hidden = true;
element.tabIndex = 3;
element.accessKey = "ackey2";
element.draggable = true;
element.spellcheck = true;
if (element.title != "title2") {
    throw Error("element.title is not title2");
}
if (element.lang != "es") {
    throw Error("element.lang is not es");
}
if (element.translate != true) {
    throw Error("element.translate is not true");
}
if (element.dir != "dir1") {
    throw Error("element.dir is not dir1");
}
if (element.hidden != true) {
    throw Error("element.hidden is not true");
}
if (element.tabIndex != 3) {
    throw Error("element.tabIndex is not 3");
}
if (element.accessKey != "ackey2") {
    throw Error("element.accessKey is not ackey2");
}
if (element.draggable != true) {
    throw Error("element.draggable is not true");
}
if (element.spellcheck != true) {
    throw Error("element.spellcheck is not true");
}
"#,
                                   "test", 218, rval.handle_mut()).is_ok());
    }
}
