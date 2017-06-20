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
use js::magicdom::element::Element_constructor;

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
                             &ELEMENT_CLASS, Some(Element_constructor),
                             5, ELEMENT_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let _attr_proto =
                JS_InitClass(cx, global.handle(), proto.handle(),
                             &ATTR_CLASS, Some(Attr_constructor),
                             5, ATTR_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        JS::SetWarningReporter(cx, Some(rust::report_warning));

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), r#"
let attr1 = new Attr("la", "a", "l", "pp", "foo");
let attr2 = new Attr("lb", "b", "l", "pp", "bar");
let element = new Element("la", "a", "l", "pp", "foo", [attr1, attr2]);
if (Object.getPrototypeOf(element) != Element.prototype) {
    throw Error("element prototype is wrong");
}
if (!(element instanceof Element)) {
    throw Error("is not instance of Element?");
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
if (attrss[0].local_name != "la") {
    throw Error("attr.local_name is not la");
}
if (attrss[0].name != "a") {
    throw Error("attrss[0].name is not a");
}
if (attrss[0].namespace != "l") {
    throw Error("attrss[0].namespace is not l");
}
if (attrss[0].prefix != "pp") {
    throw Error("attrss[0].prefix is not pp");
}
if (attrss[0].value != "foo") {
    throw Error("attrss[0].value is not foo");
}
if (attrss[1].local_name != "lb") {
    throw Error("attr.local_name is not lb");
}
if (attrss[1].name != "b") {
    throw Error("attrss[1].name is not b");
}
if (attrss[1].namespace != "l") {
    throw Error("attrss[1].namespace is not l");
}
if (attrss[1].prefix != "pp") {
    throw Error("attrss[1].prefix is not pp");
}
if (attrss[1].value != "bar") {
    throw Error("attrss[1].value is not boo");
}
element.id = "bar";
if (element.id != "bar") {
    throw Error("element.id is not bar");
}
"#,
                                   "test", 60, rval.handle_mut()).is_ok());
    }
}
