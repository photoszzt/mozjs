#![feature(proc_macro)]
#![recursion_limit="256"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{Body, VariantData};
use proc_macro::TokenStream;

/// The `#[derive(MagicDom)]` implementation.
///
/// Takes a struct spec and creates the magic dom version where the types are
/// stored in an underlying JSObject's slots.
///
/// The struct name must have an underscore, such as `DOMPoint_spec`. Then,
/// the macro will use everything up until the first '_' as the name of the generated
/// magic dom struct. With our earlier example, we'd get `DOMPoint`.
#[proc_macro_derive(MagicDom)]
pub fn magic_dom(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let gen = match ast.body {
        Body::Enum(_) => panic!("#[derive(magic-dom)] can only be used with structs"),
        Body::Struct(ref data) => match_struct(&ast, data),
    };
    gen.to_string().parse().unwrap()
}

fn match_struct(ast: &syn::DeriveInput, variant: &syn::VariantData) -> quote::Tokens {
    let name_spec = &ast.ident;
    let name_spec_str = name_spec.to_string();
    let name = name_spec_str
        .split('_')
        .next()
        .expect("Should have a '_' in the magic dom struct name");
    let from_obj_err = quote::Ident::from(format!("b\"Fail to construct {} from JS \
                                                   object\\0\" as *const u8 as *const \
                                                   libc::c_char", name));
    let constructor = quote::Ident::from(format!("{}_constructor", name));
    let name_field = quote::Ident::from(format!("b\"{}\\0\"", name));
    let name_upper = name.to_uppercase();
    let js_class = quote::Ident::from(format!("{}_CLASS", name_upper));
    let name = quote::Ident::from(name);
    let MagicStructCode {
        getters,
        setters,
        get_callargs,
        setter_invocations,
    } = get_magic_struct_code(&name, variant);
    let test_fn_name = quote::Ident::from(format!("test_{}_magic_layout()", name));
    let num_reserved_slots = quote::Ident::from(format!("{}", getters.len()));
    let arg_num_err = quote::Ident::from(format!("b\"constructor requires exactly {} \
                                                  arguments\\0\".as_ptr() as *const \
                                                  libc::c_char", getters.len()));

    quote! {
        extern crate libc;
        use magicdom::*;
        use jsapi;
        use jsapi::root::*;
        use rust::{GCMethods, RootKind, maybe_wrap_value};
        use glue::CreateCallArgsFromVp;
        use jsval::{DoubleValue, ObjectOrNullValue, ObjectValue};
        use rust::ToNumber;
        use conversions::{ConversionResult, FromJSValConvertible, ToJSValConvertible};
        use jsval;

        use std::mem;
        use std::ptr;

        #[allow(non_camel_case_types)]
        pub struct #name {
            object: *mut JSObject,
        }

        pub static #js_class : JSClass = JSClass {
            name: #name_field as *const u8 as *const libc::c_char,
            flags: (#num_reserved_slots &
                    JSCLASS_RESERVED_SLOTS_MASK) << JSCLASS_RESERVED_SLOTS_SHIFT |
            JSCLASS_HAS_PRIVATE,
            cOps: 0 as *const _,
            reserved: [0 as *mut _; 3],
        };

        impl RootKind for #name  {
            #[inline(always)]
            fn rootKind() -> JS::RootKind  {
                JS::RootKind::Object
            }
        }

        impl #name {
            fn as_jsobject(&self) -> *mut JSObject {
                self.object
            }

            pub unsafe fn from_object(obj: *mut JSObject) -> Option<#name> {
                if JS_GetClass(obj) as *const _ as usize == &#js_class as *const _ as usize {
                    Some(#name {
                        object: obj,
                    })
                } else {
                    None
                }
            }

            pub unsafe fn check_this(cx: *mut JSContext, args: &JS::CallArgs) -> Option<#name> {
                rooted!(in(cx) let thisv = args.thisv().get());
                if !thisv.is_object() {
                    return None;
                }
                #name::from_object(thisv.to_object())
            }

            #(#getters)*

            #(#setters)*

        }

        impl GCMethods for #name  {
            unsafe fn initial() -> #name {
                #name {
                    object: ptr::null_mut(),
                }
            }

            unsafe fn post_barrier(v: *mut #name, prev: #name, next: #name) {
                let v = &mut (*v).as_jsobject() as *mut *mut JSObject;
                let prev = prev.as_jsobject();
                let next = next.as_jsobject();
                <*mut JSObject as GCMethods>::post_barrier(v, prev, next);
            }
        }

        impl ToJSValConvertible for #name {
            #[inline]
            unsafe fn to_jsval(&self, cx: *mut JSContext, rval: JS::MutableHandleValue) {
                rval.set(ObjectOrNullValue(self.object));
                maybe_wrap_value(cx, rval);
            }
        }

        impl FromJSValConvertible for #name {
            type Config = ();
            #[inline]
            unsafe fn from_jsval(_cx: *mut JSContext, val: JS::HandleValue, _option: ()) ->
                Result<ConversionResult<#name>, ()> {
                let obj = val.get().to_object();
                if JS_GetClass(obj) as *const _ as usize == &#js_class as *const _ as usize {
                    Ok(ConversionResult::Success(#name {
                        object: obj,
                    }))
                } else {
                    Err(())
                }
            }
        }

        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #constructor(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
            let call_args = CreateCallArgsFromVp(argc, vp);
            if call_args._base.argc_ != #num_reserved_slots {
                JS_ReportErrorASCII(cx, #arg_num_err);
                return false;
            }

            #(#get_callargs)*

            rooted!(in(cx) let jsobj = jsapi::JS_NewObjectForConstructor(cx,
                                                                         &#js_class as *const _,
                                                                         &call_args as *const _));
            if jsobj.is_null() {
                JS_ReportErrorASCII(cx, b"Fail to construct JS object\0".as_ptr() as *const libc::c_char);
                return false;
            }
            let obj = match #name::from_object(jsobj.get()) {
                Some(o) => o,
                None => {
                    JS_ReportErrorASCII(cx, #from_obj_err);
                    return false;
                }
            };

            #(obj.#setter_invocations;)*

            call_args.rval().set(ObjectValue(jsobj.get()));
            true
        }

        #[test]
        fn it_compiles() {
            assert!(true);
        }

        #[allow(non_snake_case)]
        #[test]
        fn #test_fn_name {
            assert_eq!(mem::size_of::<#name>(), mem::size_of::<*mut JSObject>());
            assert_eq!(mem::align_of::<#name>(), mem::align_of::<*mut JSObject>());

            let instance: #name = unsafe { mem::zeroed() };
            assert_eq!(&instance as *const _ as usize, &instance.object as *const _ as usize);
        }
    }
}

struct MagicStructCode {
    /// Rust getters for the data stored in slots.
    getters: Vec<quote::Tokens>,

    /// Rust setters for the data stored in slots.
    setters: Vec<quote::Tokens>,

    /// Code to turn JS Value back to Rust value. Uses get_js_arg! macro
    get_callargs: Vec<quote::Tokens>,

    /// Code to invoke setter to set value
    setter_invocations: Vec<quote::Tokens>,
}

fn gen_getter(id: &Option<syn::Ident>,
              ty: &syn::Ty,
              idx: u32) -> quote::Tokens {
    let getter_str = match *id {
        Some(ref real_id) => format!("get_{}", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let getter_name = quote::Ident::from(getter_str);
    quote! {
        pub unsafe fn #getter_name (&self, cx: *mut JSContext) -> #ty {
            let jsobj = self.object;
            rooted!(in(cx) let val = JS_GetReservedSlot(jsobj, #idx));

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
    }
}

fn gen_setter(id: &Option<syn::Ident>,
              ty: &syn::Ty,
              idx: u32) -> (quote::Tokens, quote::Ident) {
    let setter_str = match *id {
        Some(ref real_id) => format!("set_{}", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let setter_name = quote::Ident::from(setter_str);
    let setter = quote! {
        pub fn #setter_name (&self, cx: *mut JSContext, t: #ty) {
            unsafe {
                let jsobj = self.object;
                rooted!(in(cx) let mut val = jsval::UndefinedValue());
                t.to_jsval(cx, val.handle_mut());
                JS_SetReservedSlot(jsobj, #idx, &*val);
            }
        }
    };
    (setter, setter_name)
}

/// This function generates a struct which consists the definition of the struct, the
/// definition of the slot number, Rust getters and setters for the data stored in slots,
/// `JSNative` getter and setters and JSPropertySpec array elements.
fn get_magic_struct_code(name: &quote::Ident,
                         variant: &syn::VariantData)
                         -> MagicStructCode {
    let res = match *variant {
        VariantData::Struct(ref fields) => {
            let mut getters = Vec::new();
            let mut setters = Vec::new();
            let mut get_callargs = Vec::new();
            let mut setter_invocations = Vec::new();
            for (idx, field) in fields.iter().enumerate() {
                let id = &field.ident;
                let ty = &field.ty;
                let field_name_str = match *id {
                    Some(ref real_id) => format!("{}", real_id.to_string()),
                    None => panic!("Encounter an empty field. Something wrong..."),
                };
                let field_name = quote::Ident::from(field_name_str);
                let getter = gen_getter(id, ty, idx as u32);
                let (setter, setter_name) = gen_setter(id, ty, idx as u32);
                let get_callarg = quote! {
                    get_js_arg!(#field_name, cx, call_args, #idx as u32);
                };
                let setter_invocation = quote! {
                    #setter_name(cx, #field_name)
                };
                getters.push(getter);
                setters.push(setter);
                get_callargs.push(get_callarg);
                setter_invocations.push(setter_invocation);
            }
            MagicStructCode {
                getters: getters,
                setters: setters,
                get_callargs: get_callargs,
                setter_invocations: setter_invocations,
            }
        },
        _ => panic!("Only struct is implemented"),
    };
    res
}
