/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate js;
extern crate libc;

use js::debug::{val_to_str, puts};
use js::rust::{Runtime, SIMPLE_GLOBAL_CLASS};
use js::rust;
use js::jsapi::root::JS;
use js::jsapi::root::{JS_NewGlobalObject,
                      JS_InitClass, JS_DefineFunction,};
use js::jsapi::root::JS::CompartmentOptions;
use js::jsapi::root::JS::OnNewGlobalHookOption;
use js::jsval::{UndefinedValue};
use js::magicdom::dompoint::DOMPOINT_CLASS;
use js::magicdom::dompoint::DOMPOINT_PS_ARR;
use js::magicdom::dompoint::DOMPoint_constructor;
use js::magicdom::dompointreadonly::DOMPOINTREADONLY_CLASS;
use js::magicdom::dompointreadonly::DOMPOINTREADONLY_PS_ARR;
use js::magicdom::dompointreadonly::DOMPointReadOnly_constructor;

use std::ptr;

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

        rooted!(in(cx) let dom_point_readonly_proto =
                JS_InitClass(cx, global.handle(), proto.handle(),
                             &DOMPOINTREADONLY_CLASS, Some(DOMPointReadOnly_constructor),
                             4, DOMPOINTREADONLY_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        rooted!(in(cx) let _dom_point_proto =
                JS_InitClass(cx, global.handle(), dom_point_readonly_proto.handle(),
                             &DOMPOINT_CLASS, Some(DOMPoint_constructor),
                             4, DOMPOINT_PS_ARR.as_ptr(), std::ptr::null(),
                             std::ptr::null(), std::ptr::null())
        );

        let print_function = JS_DefineFunction(cx, global.handle(), b"puts\0".as_ptr() as *const libc::c_char,
                                         Some(puts), 1, 0);
        assert!(!print_function.is_null());
        let to_str_function = JS_DefineFunction(cx, global.handle(), b"val_to_str\0".as_ptr() as *const libc::c_char,
                                                Some(val_to_str), 1, 0);
        assert!(!to_str_function.is_null());

        JS::SetWarningReporter(cx, Some(rust::report_warning));

        rooted!(in(cx) let mut rval = UndefinedValue());
        assert!(rt.evaluate_script(global.handle(), r#"
let dp = new DOMPoint(100,2,3,4);
if (Object.getPrototypeOf(dp) != DOMPoint.prototype) {
    throw Error("dp prototype is wrong");
}
if (!(dp instanceof DOMPoint)) {
    throw Error("is not instance of DOMPoint?");
}
if (dp.x != 100) {
    throw Error("dp.x is not 100");
}
if (dp.y != 2) {
    throw Error("dp.y is not 2");
}
if (dp.z != 3) {
    throw Error("dp.z is not 3");
}
if (dp.w != 4) {
    throw Error("dp.w is not 4");
}
dp.x = 2000;
dp.y = 3000;
dp.z = 4000;
dp.w = 5000;
if (dp.x != 2000) {
    throw Error("dp.x is not 2000");
}
if (dp.y != 3000) {
    throw Error("dp.y is not 3000");
}
if (dp.z != 4000) {
    throw Error("dp.z is not 4000");
}
if (dp.w != 5000) {
    throw Error("dp.w is not 5000");
}
"#,
                                   "test", 36, rval.handle_mut()).is_ok());
    }
}
