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
use js::magicdom::domquad::DOMQUAD_CLASS;
use js::magicdom::domquad::DOMQUAD_PS_ARR;
use js::magicdom::domquad::DOMQuad_constructor;
use js::magicdom::dompoint::DOMPOINT_CLASS;
use js::magicdom::dompoint::DOMPOINT_PS_ARR;
use js::magicdom::dompoint::DOMPoint_constructor;

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

        rooted!(in(cx) let _dom_quad_proto =
                JS_InitClass(cx, global.handle(), proto.handle(),
                             &DOMQUAD_CLASS, Some(DOMQuad_constructor),
                             4, DOMQUAD_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let _dom_point_proto =
                JS_InitClass(cx, global.handle(), proto.handle(),
                             &DOMPOINT_CLASS, Some(DOMPoint_constructor),
                             4, DOMPOINT_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        JS::SetWarningReporter(cx, Some(rust::report_warning));

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), r#"
let p1 = new DOMPoint(1,2,3,4);
let p2 = new DOMPoint(5,6,7,8);
let p3 = new DOMPoint(9,10,11,12);
let p4 = new DOMPoint(13,14,15,16);
let qd = new DOMQuad(p1,p2,p3,p4);
if (Object.getPrototypeOf(qd) != DOMQuad.prototype) {
    throw Error("qd prototype is wrong");
}
if (!(qd instanceof DOMQuad)) {
    throw Error("is not instance of DOMQuad?");
}
let p11 = qd.p1;
if (p11.x != 1) {
    throw Error("p1.x is not 1");
}
if (p11.y != 2) {
    throw Error("p1.y is not 2");
}
if (p11.z != 3) {
    throw Error("p1.z is not 3");
}
if (p11.w != 4) {
    throw Error("p1.w is not 4");
}
let p22 = qd.p2;
if (p22.x != 5) {
    throw Error("p2.x is not 5");
}
if (p22.y != 6) {
    throw Error("p2.y is not 6");
}
if (p22.z != 7) {
    throw Error("p2.z is not 7");
}
if (p22.w != 8) {
    throw Error("p2.w is not 8");
}
let p33 = qd.p3;
if (p33.x != 9) {
    throw Error("p3.x is not 9");
}
if (p33.y != 10) {
    throw Error("p3.y is not 10");
}
if (p33.z != 11) {
    throw Error("p3.z is not 11");
}
if (p33.w != 12) {
    throw Error("p3.w is not 12");
}
let p44 = qd.p4;
if (p44.x != 13) {
    throw Error("p4.x is not 13");
}
if (p44.y != 14) {
    throw Error("p4.y is not 14");
}
if (p44.z != 15) {
    throw Error("p4.z is not 15");
}
if (p44.w != 16) {
    throw Error("p4.w is not 16");
}
"#,
                                   "test", 64, rval.handle_mut()).is_ok());
    }
}
