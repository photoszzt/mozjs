/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate magic_codegen;
#[macro_use]
extern crate js;
extern crate libc;
#[macro_use]
extern crate lazy_static;

use js::rust::{Runtime, SIMPLE_GLOBAL_CLASS, ToNumber};
use js::rust;
use js::glue::CreateCallArgsFromVp;
use js::jsapi;
use js::jsapi::root::JS;
use js::jsapi::root::{JS_ReportErrorASCII, JS_NewGlobalObject,
                      JS_InitClass, JSContext, JS_EncodeStringToUTF8, JS_DefineFunction,};
use js::jsapi::root::JS::CompartmentOptions;
use js::jsapi::root::JS::OnNewGlobalHookOption;
use js::jsval::{ObjectValue, StringValue, UndefinedValue};

use std::ffi::CStr;
use std::ptr;
use std::str;

magic_dom! {
    DOMPoint,
    struct DOMPoint_spec {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }
}

use magic_dom_spec::set_x as set_x;
use magic_dom_spec::set_y as set_y;
use magic_dom_spec::set_z as set_z;
use magic_dom_spec::set_w as set_w;

unsafe extern "C" fn DOMPoint_constructor(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
    let call_args = CreateCallArgsFromVp(argc, vp);
    if call_args._base.argc_ != 4 {
        JS_ReportErrorASCII(cx, b"constructor requires exactly 4 arguments\0".as_ptr() as *const libc::c_char);
        return false;
    }
    get_js_val_number!(x, cx, call_args, 0);
    get_js_val_number!(y, cx, call_args, 1);
    get_js_val_number!(z, cx, call_args, 2);
    get_js_val_number!(w, cx, call_args, 3);
    rooted!(in(cx) let jsobj = jsapi::JS_NewObjectForConstructor(cx,
                                                                 &magic_dom_spec::DOMPOINT_CLASS as *const _,
                                                                 &call_args as *const _));
    if jsobj.is_null() {
        JS_ReportErrorASCII(cx, b"Fail to construct JS object\0".as_ptr() as *const libc::c_char);
        return false;
    }
    let d = match DOMPoint::from_object(jsobj.get()) {
        Some(o) => o,
        None => {
            JS_ReportErrorASCII(cx, b"Fail to construct DOMPoint from JS object\0".as_ptr()
                                as *const libc::c_char);
            return false;
        }
    };
    set_x(&d, cx, x);
    set_y(&d, cx, y);
    set_z(&d, cx, z);
    set_w(&d, cx, w);
    call_args.rval().set(ObjectValue(jsobj.get()));
    true
}

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

        rooted!(in(cx) let dom_point_proto =
                JS_InitClass(cx, global.handle(), proto.handle(),
                             &magic_dom_spec::DOMPOINT_CLASS, Some(DOMPoint_constructor),
                             4, magic_dom_spec::PS_ARR.as_ptr(), std::ptr::null(),
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

// val_to_str: debug function
// Can turn a JSValue to JSString
unsafe extern "C" fn val_to_str(context: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
    let args = JS::CallArgs::from_vp(vp, argc);

    if args._base.argc_ != 1 {
        JS_ReportErrorASCII(context, b"val_to_str() requires exactly 1 argument\0".as_ptr() as *const libc::c_char);
        return false;
    }

    let arg = args.get(0);
    let jsstr = jsapi::JS_ValueToSource(context, arg);
    args.rval().set(StringValue(&*jsstr));
    return true;
}

// print a JSString to terminal
unsafe extern "C" fn puts(context: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
    let args = JS::CallArgs::from_vp(vp, argc);

    if args._base.argc_ != 1 {
        JS_ReportErrorASCII(context, b"puts() requires exactly 1 argument\0".as_ptr() as *const libc::c_char);
        return false;
    }

    let arg = args.get(0);
    let js = js::rust::ToString(context, arg);
    rooted!(in(context) let message_root = js);
    let message = JS_EncodeStringToUTF8(context, message_root.handle());
    let message = CStr::from_ptr(message);
    println!("{}", str::from_utf8(message.to_bytes()).unwrap());

    args.rval().set(UndefinedValue());
    return true;
}
