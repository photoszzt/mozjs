//! Common traits and types for Magic DOM structs.

use conversions::{ConversionResult, FromJSValConvertible, ToJSValConvertible};
use jsapi;
use jsval;

use std::marker::PhantomData;
use std::mem;
use std::ptr;

/// TODO FITZGEN
pub trait SlotIndex {
    fn slot_index() -> u32;
}

/// TODO FITZGEN
pub struct MagicSlot<T, I> {
    slot_type: PhantomData<T>,
    slot_index: PhantomData<I>,
}

impl<T, I> MagicSlot<T, I> {
    /// Construct a new `MagicSlot`.
    pub fn new() -> MagicSlot<T, I> {
        MagicSlot {
            slot_type: PhantomData,
            slot_index: PhantomData,
        }
    }
}

impl<T, I> MagicSlot<T, I>
    where T: ToJSValConvertible + FromJSValConvertible<Config=()>,
          I: SlotIndex
{
    unsafe fn get_object(&self) -> *mut jsapi::JSObject {
        let obj_ptr: *const *mut jsapi::JSObject = mem::transmute(self);
        debug_assert!(obj_ptr != ptr::null());
        *obj_ptr
    }

    /// Get the value stored in this slot.
    ///
    /// # Unsafety
    ///
    /// Requires that this `MagicSlot`'s self ptr is the same as the containing
    /// magic DOM struct's pointer to the `JSObject` that has the appropriate
    /// slots.
    pub unsafe fn get(&self, cx: *mut jsapi::JSContext) -> T {
        let obj = self.get_object();
        rooted!(in(cx) let val = jsapi::JS_GetReservedSlot(obj, I::slot_index()));

        let conversion = FromJSValConvertible::from_jsval(cx, val.handle(), ())
            .expect("Should never put anything into a MagicSlot that we can't \
                     convert back out again");

        match conversion {
            ConversionResult::Success(v) => v,
            ConversionResult::Failure(why) => {
                panic!("Should never put anything into a MagicSlot that we \
                        can't convert back out again: {}",
                       why);
            }
        }
    }

    /// Store a value into this slot
    ///
    /// # Unsafety
    ///
    /// The same pitfalls as `get`.
    pub unsafe fn set(&self, cx: *mut jsapi::JSContext, t: T) {
        rooted!(in(cx) let obj = self.get_object());
        // TODO: read + drop old slot value.

        rooted!(in(cx) let mut val = jsval::UndefinedValue());
        t.to_jsval(cx, val.handle_mut());
        jsapi::JS_SetReservedSlot(obj.get(), I::slot_index(), &*val);
    }

    /// TODO FITZGEN
    pub unsafe fn initialize(&self, cx: *mut jsapi::JSContext, t: T) {
        // TODO: same as `set` but assert that the slot is `undefined` and don't
        // read + drop the slot value.
    }
}
