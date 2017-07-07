/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Conversion of Rust values to and from JS Slots

#![deny(missing_docs)]

use conversions::{ConversionResult, ConversionBehavior, FromJSValConvertible,
                  ToJSValConvertible};
use jsapi::root::*;
use jsval;
use std::mem;
use std::ptr;
use std::fmt;

extern crate libc;

/// A trait to record the number of slots required for the current type
pub trait NumSlots {
    /// Number of slots required to store Self
    const NUM_SLOTS: u32 = 1;
}

/// A trait to record the number of slots required for the current type
/// if a type inherited the current type. It's mainly used for recording
/// DOM object inheritance.
pub trait InheritanceSlots {
    /// Number of slots reserved for inheritance from Self
    const INHERITANCE_SLOTS: u32 = 1;
}

/// A trait to store and retrieve data with Rust type to JS slots
///
/// When implementing this trait, you need to be careful about the property of the Rust type
/// and how they are stored.
///
/// 1. Rust type
/// If the Rust type can be copied(Copy), then the Target will be Self. A copy of the value
/// will be return from slots when calling from_slots. An example implementation for this
/// category would be the integer primitive type listed in this file.
///
/// If the Rust type doesn't implement Copy, then the value will be returned to the caller
/// and the caller also owns the data after calling the from_slots. The Target is
/// Option<Self> for this case. If the caller calls from_slots the second time, a None
/// will be returned. The reverse operation happened for into slots. When calling into_slots,
/// the caller loses ownership for the data.
///
/// Most of the type can be implemented using ToJSValConvertible and FromJSValConvertible trait.
/// For magic dom struct, the JS object pointer is stored in the slot. The implementation of the
/// trait is generated by the procedure macro. There are types which takes more than one slot to
/// store. One example is the Vec implemented in this file.
///
/// 2. Store in the slot
/// Pointer to JSObject, will be stored as ObjectValue which will be traced by the GC. For
/// a raw pointer that doesn't traced by the GC, you can stored it using PrivateValue.
pub trait ToFromJsSlots : NumSlots {
    /// Default type for Target, same for in/out
    type Target = Self;

    /// Whether this type needs finalization. Don't need finalization by default.
    const NEEDS_FINALIZE: bool = false;

    /// Whether this type requires custom tracing for the GC.
    const NEEDS_TRACE: bool = false;

    /// get the Self from the slots
    /// For Copy types, a copy of the value is returned.
    /// For non-Copy types, returns Option<Self>.
    /// returns None if it has already been taken out of slots.
    unsafe fn from_slots(object: *mut JSObject, cx: *mut JSContext, offset: u32)
                         -> Self::Target;

    /// puts the self into the slots
    unsafe fn into_slots(self, object: *mut JSObject, cx: *mut JSContext, offset: u32);

    /// Only called for types where `NEEDS_FINALIZE` is `true`.
    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
        // Provide a default no-op `finalize` method to make the procedural
        // macro's life easier and not have to figure out how to sort which fields
        // need finalization or not.
    }

    /// Custom GC tracing for these slots. Only called when `NEEDS_TRACE` is `true`.
    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
        // Once again, no-op by default to make `magic_codegen`'s job easier
    }
}

macro_rules! gen_from_slots {
    ($conversion_behavior:expr) => {
        unsafe fn from_slots(object: *mut JSObject, cx: *mut JSContext, offset: u32)
                             -> Self::Target {
            rooted!(in(cx) let val = JS_GetReservedSlot(object, offset));
            if val.is_undefined() {
                panic!("The value inside this slot has been taken out");
            }
            let conversion = FromJSValConvertible::from_jsval(cx, val.handle(),
                                                              $conversion_behavior)
                .expect("Should never put anything into a JS slot that we can't \
                         convert back out again");
            match conversion {
                ConversionResult::Success(v) => v,
                ConversionResult::Failure(why) => {
                    panic!("Should never put anything into a JS slot that we \
                            can't convert back out again: {}",
                           why);
                }
            }
        }
    }
}

macro_rules! gen_to_slots {
    () => {
        unsafe fn into_slots(self, object: *mut JSObject, cx: *mut JSContext,
                             offset: u32) {
            rooted!(in(cx) let mut val = jsval::UndefinedValue());
            self.to_jsval(cx, val.handle_mut());
            JS_SetReservedSlot(object, offset, &*val);
        }
    }
}

impl NumSlots for bool {}
impl NumSlots for str {}
impl NumSlots for String {}
impl NumSlots for u8 {}
impl NumSlots for i8 {}
impl NumSlots for u16 {}
impl NumSlots for i16 {}
impl NumSlots for u32 {}
impl NumSlots for i32 {}
impl NumSlots for u64 {}
impl NumSlots for i64 {}
impl NumSlots for f32 {}
impl NumSlots for f64 {}
impl<T> NumSlots for *const T {}
impl<T> NumSlots for *mut T {}

impl InheritanceSlots for bool {}
impl InheritanceSlots for str {}
impl InheritanceSlots for String {}
impl InheritanceSlots for u8 {}
impl InheritanceSlots for i8 {}
impl InheritanceSlots for u16 {}
impl InheritanceSlots for i16 {}
impl InheritanceSlots for u32 {}
impl InheritanceSlots for i32 {}
impl InheritanceSlots for u64 {}
impl InheritanceSlots for i64 {}
impl InheritanceSlots for f32 {}
impl InheritanceSlots for f64 {}

impl<T> NumSlots for Vec<T> {
    const NUM_SLOTS: u32 = 3;
}

impl<T> InheritanceSlots for Vec<T> {
    const INHERITANCE_SLOTS: u32 = 3;
}

impl ToFromJsSlots for bool {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(());
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for String {
    type Target = Option<Self>;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    unsafe fn from_slots(object: *mut JSObject, cx: *mut JSContext, offset: u32)
                         -> Self::Target {
        rooted!(in(cx) let val = JS_GetReservedSlot(object, offset));
        if val.is_undefined() {
            return None;
        }
        let conversion = FromJSValConvertible::from_jsval(cx, val.handle(), ())
            .expect("Should never put anything into a JS slot that we can't \
                     convert back out again");
        rooted!(in(cx) let undef_val = jsval::UndefinedValue());
        JS_SetReservedSlot(object, offset, &*undef_val);
        match conversion {
            ConversionResult::Success(v) => Some(v),
            ConversionResult::Failure(why) => {
                panic!("Should never put anything into a JS slot that we \
                        can't convert back out again: {}",
                       why);
            }
        }
    }

    unsafe fn into_slots(self, object: *mut JSObject, cx: *mut JSContext, offset: u32) {
        let prev_val = JS_GetReservedSlot(object, offset);
        if !prev_val.is_undefined() {
            mem::drop(prev_val);
        }
        rooted!(in(cx) let mut val = jsval::UndefinedValue());
        self.to_jsval(cx, val.handle_mut());
        JS_SetReservedSlot(object, offset, &*val);
    }

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for u8 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(ConversionBehavior::Default);
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for i8 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(ConversionBehavior::Default);
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for u16 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(ConversionBehavior::Default);
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for i16 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(ConversionBehavior::Default);
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for u32 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(ConversionBehavior::Default);
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for i32 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(ConversionBehavior::Default);
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for u64 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(ConversionBehavior::Default);
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for i64 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(ConversionBehavior::Default);
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for f32 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(());
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for f64 {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    gen_from_slots!(());
    gen_to_slots!();

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

#[macro_export]
macro_rules! get_slot_val {
    ($val:ident, $conv_val:ident, $idx:expr, $cx:ident, $object:ident,
     $offset:ident, $conversion_behavior:expr) => {
        rooted!(in($cx) let $val = JS_GetReservedSlot($object, $offset + $idx));
        if $val.is_undefined() {
            panic!("The value inside this slot has been taken out");
        }
        let conversion = FromJSValConvertible::from_jsval($cx, $val.handle(),
                                                          $conversion_behavior)
            .expect("Should never put anything into a JS slot that we can't \
                     convert back out again");
        rooted!(in($cx) let undef_val = jsval::UndefinedValue());
        JS_SetReservedSlot($object, $offset + $idx, &*undef_val);
        let $conv_val = match conversion {
            ConversionResult::Success(v) => v,
            ConversionResult::Failure(why) => {
                panic!("Should never put anything into a JS slot that we \
                        can't convert back out again: {}",
                       why);
            }
        };
    }
}

#[macro_export]
macro_rules! set_slot_val {
    ($prev_val:ident,
     $rust_val:ident,
     $idx:expr,
     $cx:ident,
     $object:ident,
     $offset:ident
    ) => {
        let $prev_val = JS_GetReservedSlot($object, $offset + $idx);
        if !$prev_val.is_undefined() {
            debug!("Dropping value originally there");
            mem::drop($prev_val);
        }
        rooted!(in($cx) let mut val = jsval::UndefinedValue());
        $rust_val.to_jsval($cx, val.handle_mut());
        JS_SetReservedSlot($object, $offset + $idx, &*val);
    }
}

trait VecExt<T>: Sized {
    fn into_raw_parts(self) -> (*mut T, usize, usize);
}

impl<T> VecExt<T> for Vec<T> {
    fn into_raw_parts(mut self) -> (*mut T, usize, usize) {
        let len = self.len();
        let cap = self.capacity();
        let ptr = self.as_mut_ptr();
        mem::forget(self);
        (ptr, len, cap)
    }
}

/// Notice:
/// Vec takes up 3 slots which stores the components of the fat pointer separately. It's
/// structured to avoid a deep copy of the Vec as implemented in the ToJSValConvertible.
impl<T> ToFromJsSlots for Vec<T> where T: fmt::Debug {
    type Target = Option<Self>;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    unsafe fn from_slots(object: *mut JSObject, cx: *mut JSContext, offset: u32)
                         -> Self::Target {
        rooted!(in(cx) let val0 = JS_GetReservedSlot(object, offset));
        if val0.is_undefined() {
            return None;
        }
        let conversion0 =
            FromJSValConvertible::from_jsval(cx, val0.handle(), ())
            .expect("Should never put anything into a JS slot that we can't \
                     convert back out again");
        rooted!(in(cx) let undef_val = jsval::UndefinedValue());
        JS_SetReservedSlot(object, offset, &*undef_val);
        let ptr_val: JS::Value = match conversion0 {
            ConversionResult::Success(v) => v,
            ConversionResult::Failure(why) => {
                panic!("Should never put anything into a JS slot that we \
                        can't convert back out again: {}",
                       why);
            }
        };
        get_slot_val!(val1, len, 1, cx, object, offset, ConversionBehavior::Default);
        get_slot_val!(val2, cap, 2, cx, object, offset, ConversionBehavior::Default);
        let ptr = ptr_val.to_private() as *mut _;
        Some(Vec::from_raw_parts(ptr, len, cap))
    }

    unsafe fn into_slots(self, object: *mut JSObject, cx: *mut JSContext, offset: u32) {
        let (ptr, len, cap) = self.into_raw_parts();
        let ptr = jsval::PrivateValue(ptr as *const _ as *const libc::c_void);
        set_slot_val!(prev_val0, ptr, 0, cx, object, offset);
        set_slot_val!(prev_val1, len, 1, cx, object, offset);
        set_slot_val!(prev_val2, cap, 2, cx, object, offset);
    }

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for *mut JSString {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    unsafe fn from_slots(object: *mut JSObject, cx: *mut JSContext, offset: u32)
                         -> Self::Target {
        rooted!(in(cx) let val = JS_GetReservedSlot(object, offset));
        if val.is_string() {
            val.to_string()
        } else if val.is_null() {
            ptr::null_mut()
        } else {
            panic!("<*mut JSString as ToFromJsSlots>::from_slots called on non-string/null slot");
        }
    }

    unsafe fn into_slots(self, object: *mut JSObject, cx: *mut JSContext, offset: u32) {
        let ptr = if self.is_null() {
            jsval::NullValue()
        } else {
            jsval::StringValue(&*self)
        };
        set_slot_val!(prev_val0, ptr, 0, cx, object, offset);
    }

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}

impl ToFromJsSlots for *mut JSObject {
    type Target = Self;
    const NEEDS_FINALIZE: bool = false;
    const NEEDS_TRACE: bool = false;

    unsafe fn from_slots(object: *mut JSObject, cx: *mut JSContext, offset: u32)
                         -> Self::Target {
        rooted!(in(cx) let val = JS_GetReservedSlot(object, offset));
        if val.is_object() {
            val.to_object()
        } else if val.is_null() {
            ptr::null_mut()
        } else {
            panic!("<*mut JSString as ToFromJsSlots>::from_slots called on non-string/null slot");
        }
    }

    unsafe fn into_slots(self, object: *mut JSObject, cx: *mut JSContext, offset: u32) {
        let ptr = if self.is_null() {
            jsval::NullValue()
        } else {
            jsval::ObjectValue(self)
        };
        set_slot_val!(prev_val0, ptr, 0, cx, object, offset);
    }

    fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
    }

    fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
    }
}
