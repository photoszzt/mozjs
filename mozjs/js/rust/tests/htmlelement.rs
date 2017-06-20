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
use js::magicdom::htmlelement::HTMLELEMENT_CLASS;
use js::magicdom::htmlelement::HTMLELEMENT_PS_ARR;
use js::magicdom::htmlelement::HtmlElement_constructor;

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

        rooted!(in(cx) let _element_proto =
                JS_InitClass(cx, global.handle(), proto.handle(),
                             &HTMLELEMENT_CLASS, Some(HtmlElement_constructor),
                             10, HTMLELEMENT_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        JS::SetWarningReporter(cx, Some(rust::report_warning));

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), r#"
let element = new HtmlElement("title", "en", false, "dir", false, 1, "ackey", "ackeylabel", false, false);
if (Object.getPrototypeOf(element) != HtmlElement.prototype) {
    throw Error("element prototype is wrong");
}
if (!(element instanceof HtmlElement)) {
    throw Error("is not instance of HtmlElement?");
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
                                   "test", 77, rval.handle_mut()).is_ok());
    }
}