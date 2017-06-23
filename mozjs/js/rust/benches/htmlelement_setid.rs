/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#![feature(test)]
#[macro_use]
extern crate js;
extern crate libc;
extern crate test;

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
use js::magicdom::htmlelement::HTMLELEMENT_CLASS;
use js::magicdom::htmlelement::HTMLELEMENT_PS_ARR;
use js::magicdom::htmlelement::HtmlElement_constructor;
use test::Bencher;

use std::ptr;

#[bench]
fn bench_htmlelement_setid(b: &mut Bencher) {
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

        rooted!(in(cx) let element_proto =
                JS_InitClass(cx, global.handle(), proto.handle(), &ELEMENT_CLASS, Some(Element_constructor),
                             5, ELEMENT_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let _attr_proto =
                JS_InitClass(cx, global.handle(), proto.handle(),
                             &ATTR_CLASS, Some(Attr_constructor),
                             5, ATTR_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let _html_element_proto =
                JS_InitClass(cx, global.handle(), element_proto.handle(),
                             &HTMLELEMENT_CLASS, Some(HtmlElement_constructor),
                             10, HTMLELEMENT_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );


        JS::SetWarningReporter(cx, Some(rust::report_warning));

        rooted!(in(cx) let mut rval = UndefinedValue());
        let _ = rt.evaluate_script(global.handle(), r#"
let attr1 = new Attr("l", "a", "l", "p", "f");
let attr2 = new Attr("l", "b", "l", "p", "b");
let element1 = new HtmlElement("la", "a", "l", "pp", "foo", [attr1, attr2], "title123", "en",
false, "dir12345", false, 1, "ackeylab", "ackeylab", false, false);
let attr3 = new Attr("l", "a", "l", "p", "f");
let attr4 = new Attr("l", "b", "l", "p", "b");
let element2 = new HtmlElement("lb", "b", "l", "pp", "foo", [attr3, attr4], "title456", "es",
false, "dir", false, 1, "ackey", "ackey456", false, false);
"#,
                                   "test", 9, rval.handle_mut());
        b.iter(|| {
            let _ = rt.evaluate_script(global.handle(), r#"
let num = 10240;
for ( var i = 0; i < num; i++) {
element1.id = "baz";
}
"#,
            "test", 5, rval.handle_mut());
        });
    }
}
