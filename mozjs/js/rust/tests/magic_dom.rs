/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate js;

use js::jsapi;
use js::magic::{MagicSlot, SlotIndex};
use js::rust::{GCMethods, RootKind};

use std::mem;
use std::ptr;

// Here is pretty simple DOM interface:
//
//     // http://dev.w3.org/fxtf/geometry/Overview.html#dompointreadonly
//     interface DOMPointReadOnly {
//         readonly attribute unrestricted double x;
//         readonly attribute unrestricted double y;
//         readonly attribute unrestricted double z;
//         readonly attribute unrestricted double w;
//     };
//
// The magic dom macro invocation would like
//
//     magic_dom_struct! {
//         struct DOMPoint {
//             x: f64,
//             y: f64,
//             z: f64,
//             w: f64,
//         }
//     }
//
// or perhaps like
//
//     #[derive(MagicDomStruct)]
//     struct DOMPoint {
//         x: f64,
//         y: f64,
//         z: f64,
//         w: f64,
//     }
//
// What follows is a straw man for what the macro's generated code should look
// like.

struct DOMPoint {
    object: *mut jsapi::JSObject,
    x: MagicSlot<f64, DOMPointSlotIndex0>,
    y: MagicSlot<f64, DOMPointSlotIndex1>,
    z: MagicSlot<f64, DOMPointSlotIndex2>,
    w: MagicSlot<f64, DOMPointSlotIndex3>,
}

// TODO
const DOMPoint_class: jsapi::js::Class = {
};

impl RootKind for DOMPoint {
    #[inline(always)]
    fn rootKind() -> jsapi::JS::RootKind {
        jsapi::JS::RootKind::Object
    }
}

impl GCMethods for DOMPoint {
    unsafe fn initial() -> DOMPoint {
        DOMPoint {
            x: MagicSlot::new(),
            y: MagicSlot::new(),
            z: MagicSlot::new(),
            w: MagicSlot::new(),
            object: ptr::null_mut(),
        }
    }

    unsafe fn post_barrier(v: *mut DOMPoint, prev: DOMPoint, next: DOMPoint) {
        unimplemented!()
    }
}

#[test]
fn test_DOMPoint_magic_layout() {
    // Assert that a magic dom struct is physically just a pointer to a
    // jsapi::JSObject.
    assert_eq!(mem::size_of::<DOMPoint>(), mem::size_of::<*mut jsapi::JSObject>());
    assert_eq!(mem::align_of::<DOMPoint>(), mem::align_of::<*mut jsapi::JSObject>());

    // Assert that all the `MagicSlot` fields' `self` pointer is a pointer to
    // the `*mut jsapi::JSObject`.
    let instance: DOMPoint = unsafe { mem::zeroed() };
    assert_eq!(&instance as *const _ as usize, &instance.object as *const _ as usize);
    assert_eq!(&instance.x as *const _ as usize, &instance.object as *const _ as usize);
    assert_eq!(&instance.y as *const _ as usize, &instance.object as *const _ as usize);
    assert_eq!(&instance.z as *const _ as usize, &instance.object as *const _ as usize);
    assert_eq!(&instance.w as *const _ as usize, &instance.object as *const _ as usize);
}

enum DOMPointSlotIndex0 {}
impl SlotIndex for DOMPointSlotIndex0 {
    fn slot_index() -> u32 { 0 }
}

enum DOMPointSlotIndex1 {}
impl SlotIndex for DOMPointSlotIndex1 {
    fn slot_index() -> u32 { 1 }
}

enum DOMPointSlotIndex2 {}
impl SlotIndex for DOMPointSlotIndex2 {
    fn slot_index() -> u32 { 2 }
}

enum DOMPointSlotIndex3 {}
impl SlotIndex for DOMPointSlotIndex3 {
    fn slot_index() -> u32 { 3 }
}

#[test]
fn it_compiles() {
    assert!(true);
}
