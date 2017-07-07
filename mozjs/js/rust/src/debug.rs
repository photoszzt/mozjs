/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate libc;

use rust::ToString;
use jsapi::JS_ValueToSource;
use jsapi::root::{JS_ReportErrorASCII, JS_EncodeStringToUTF8, JSContext, };
use jsval::{StringValue, UndefinedValue};
use jsapi::root::JS;
use std::ffi::CStr;
use std::str;

// val_to_str: debug function
// Can turn a JSValue to JSString
pub unsafe extern "C" fn val_to_str(context: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
    let args = JS::CallArgs::from_vp(vp, argc);

    if args._base.argc_ != 1 {
        JS_ReportErrorASCII(context, b"val_to_str() requires exactly 1 argument\0".as_ptr() as *const libc::c_char);
        return false;
    }

    let arg = args.get(0);
    let jsstr = JS_ValueToSource(context, arg);
    args.rval().set(StringValue(&*jsstr));
    return true;
}

// print a JSString to terminal
pub unsafe extern "C" fn puts(context: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
    let args = JS::CallArgs::from_vp(vp, argc);

    if args._base.argc_ != 1 {
        JS_ReportErrorASCII(context, b"puts() requires exactly 1 argument\0".as_ptr() as *const libc::c_char);
        return false;
    }

    let arg = args.get(0);
    let js = ToString(context, arg);
    rooted!(in(context) let message_root = js);
    let message = JS_EncodeStringToUTF8(context, message_root.handle());
    let message = CStr::from_ptr(message);
    println!("{}", str::from_utf8(message.to_bytes()).unwrap());

    args.rval().set(UndefinedValue());
    return true;
}
