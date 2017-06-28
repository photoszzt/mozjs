#![feature(proc_macro)]
#![feature(box_patterns)]
#![recursion_limit="256"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use syn::{Body, VariantData};
use proc_macro::TokenStream;
use std::collections::HashSet;
use quote::ToTokens;

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

    let name_spec = &ast.ident;
    let name_spec_str = name_spec.to_string();
    let name_str = name_spec_str
        .split('_')
        .next()
        .expect("Should have a '_' in the magic dom struct name");
    let rust_gen = match ast.body {
        Body::Enum(_) => panic!("#[derive(magic-dom)] can only be used with structs"),
        Body::Struct(ref data) => match_struct(name_str, data),
    };
    rust_gen.to_string().parse().unwrap()
}

fn gen_constructor(name: &quote::Ident,
                   js_class: &quote::Ident) -> quote::Tokens {
    let constructor = quote::Ident::from(format!("{}_constructor", name));
    let from_obj_err = quote::Ident::from(format!("b\"Fail to construct {} from JS \
                                                   object\\0\" as *const u8 as *const \
                                                   libc::c_char", name));
    let arg_num_err = quote::Ident::from(format!("b\"constructor requires exact number of \
                                                  arguments\\0\".as_ptr() as *const \
                                                  libc::c_char"));
    quote! {
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #constructor(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
            let call_args = CreateCallArgsFromVp(argc, vp);

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
            if call_args._base.argc_ != obj.num_fields() {
                JS_ReportErrorASCII(cx, #arg_num_err);
                return false;
            }

            if !obj.set_fields(cx, &call_args) {
                return false;
            }

            call_args.rval().set(ObjectValue(jsobj.get()));
            true
        }
    }
}

fn match_struct(name_str: &str, variant: &syn::VariantData) -> quote::Tokens {
    let name_upper = name_str.to_uppercase();
    let name = quote::Ident::from(name_str);
    let name_field = quote::Ident::from(format!("b\"{}\\0\"", name));
    let js_class = quote::Ident::from(format!("{}_CLASS", name_upper));
    let MagicStructCode {
        getters,
        setters,
        slot_counters,
        set_fields,
        num_fields,
        upcast,
    } = get_magic_struct_code(variant);
    let test_fn_name = quote::Ident::from(format!("test_{}_magic_layout()", name));
    let num_reserved_slots = quote::Ident::from(format!("<{} as InheritanceSlots>::INHERITANCE_SLOTS",
                                                        name));
    let constructor_quote = gen_constructor(&name, &js_class);

    let rust_gen = quote! {
        extern crate libc;
        use magicdom::*;
        use jsapi;
        use jsapi::root::*;
        use rust::{GCMethods, RootKind, maybe_wrap_value};
        use glue::CreateCallArgsFromVp;
        use jsval::{DoubleValue, ObjectOrNullValue, ObjectValue};
        use conversions::{ConversionResult, ConversionBehavior, FromJSValConvertible,
                          ToJSValConvertible};
        use jsslotconversions::{InheritanceSlots, NumSlots, ToFromJsSlots};
        use jsval;

        use std::mem;
        use std::ptr;

        #[allow(non_camel_case_types)]
        #[derive(Debug)]
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

        impl InheritanceSlots for #name {
            const INHERITANCE_SLOTS: u32 = #(#slot_counters)* 0;
        }

        impl NumSlots for #name {
            const NUM_SLOTS: u32 = 1;
        }

        impl ToFromJsSlots for #name {
            type Target = Option<Self>;
            const NEEDS_FINALIZE: bool = false;
            const NEEDS_TRACE: bool = false;

            unsafe fn from_slots(object: *mut JSObject, cx: *mut JSContext, offset: u32) -> Self::Target {
                get_slot_val!(val1, obj, 0, cx, object, offset, ());
                obj
            }

            unsafe fn into_slots(self, object: *mut JSObject, cx: *mut JSContext, offset: u32) {
                let jsobj = self.as_jsobject();
                set_slot_val!(prev_val, jsobj, 0, cx, object, offset);
            }

            fn finalize(_fop: *mut js::FreeOp, _obj: *mut JSObject, _offset: u32) {
            }

            fn trace(_trc: *mut JSTracer, _obj: *mut JSObject, _offset: u32) {
            }
        }

        impl RootKind for #name  {
            #[inline(always)]
            fn rootKind() -> JS::RootKind {
                JS::RootKind::Object
            }
        }

        impl #name {
            pub fn as_jsobject(&self) -> *mut JSObject {
                self.object
            }

            pub unsafe fn from_object(obj: *mut JSObject) -> Option<#name> {
                if JS_GetClass(obj) as *const _ as usize == &#js_class as *const _ as usize {
                    Some(#name {
                        object: obj,
                    })
                } else {
                    debug!("Fail to match from_object");
                    None
                }
            }

            pub fn new(obj: *mut JSObject) -> #name {
                #name {
                    object: obj,
                }
            }

            pub unsafe fn check_this(cx: *mut JSContext, args: &JS::CallArgs) -> Option<#name> {
                rooted!(in(cx) let thisv = args.thisv().get());
                if !thisv.is_object() {
                    return None;
                }
                // Drop the class checking to let inheritance pass through
                Some(#name::new(thisv.to_object()))
            }


            #(#getters)*

            #(#setters)*

            #upcast

            #set_fields

            #num_fields
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

        #constructor_quote

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
    };
    rust_gen
}

struct MagicStructCode {
    /// Rust getters for the data stored in slots.
    getters: Vec<quote::Tokens>,

    /// Rust setters for the data stored in slots.
    setters: Vec<quote::Tokens>,

    /// Arithmetic statements for calculating the total number of slots for
    slot_counters: Vec<quote::Ident>,

    /// Method to set the fields of the struct
    set_fields: quote::Tokens,

    /// Method to get number of fields in the struct
    num_fields: quote::Tokens,

    /// Method to cast back to parent. Only available when there's object inheritance
    upcast: quote::Tokens,
}

lazy_static! {
    static ref INTTYPESET: HashSet<syn::Ident> = {
        let mut m = HashSet::new();
        m.insert(syn::Ident::from("i8"));
        m.insert(syn::Ident::from("u8"));
        m.insert(syn::Ident::from("i16"));
        m.insert(syn::Ident::from("u16"));
        m.insert(syn::Ident::from("i32"));
        m.insert(syn::Ident::from("u32"));
        m.insert(syn::Ident::from("i64"));
        m.insert(syn::Ident::from("u64"));
        m
    };
}

fn gen_getter(id: &Option<syn::Ident>,
              ty: &syn::Ty,
              effective_idx: &quote::Ident,
              inherit: bool,
              idx: usize) -> quote::Tokens {
    let getter_str = match *id {
        Some(ref real_id) => format!("get_{}", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let getter_name = quote::Ident::from(getter_str.as_str());
    let getter = if idx == 0 && inherit {
        quote!{}
    } else {
        quote! {
            pub unsafe fn #getter_name (&self, cx: *mut JSContext) -> <#ty as ToFromJsSlots>::Target {
                let jsobj = self.object;
                <#ty as ToFromJsSlots>::from_slots(jsobj, cx, #effective_idx)
            }
        }
    };
    getter
}

fn gen_setter(id: &Option<syn::Ident>,
              ty: &syn::Ty,
              effective_idx: &quote::Ident,
              inherit: bool,
              idx: usize) -> (quote::Tokens, quote::Ident) {
    let setter_str = match *id {
        Some(ref real_id) => format!("set_{}", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let setter_name = quote::Ident::from(setter_str.as_str());
    let setter = if idx == 0 && inherit {
        quote!{}
    } else {
        quote! {
            pub fn #setter_name (&self, cx: *mut JSContext, t: #ty) {
                unsafe {
                    let jsobj = self.object;
                    t.into_slots(jsobj, cx, #effective_idx);
                }
            }
        }
    };
    (setter, setter_name)
}

fn gen_get_callargs(ty: &syn::Ty,
                    field_name: &quote::Ident,
                    upcast_name: &quote::Ident,
                    inherit: bool,
                    idx: usize) -> quote::Tokens {
    if inherit && idx == 0 {
        quote!{}
    } else {
        let conversion_behavior = match *ty {
            syn::Ty::Path(ref qself, ref path) => {
                if let &Some(ref q) = qself {
                    panic!("Check the qself: {:?}", q);
                }
                let seg = &path.segments[0];
                if INTTYPESET.contains(&seg.ident) {
                    quote::Ident::from("ConversionBehavior::Default")
                } else {
                    quote::Ident::from("()")
                }
            },
            _ => {
                quote::Ident::from("()")
            }
        };
        if inherit {
            quote! {
                get_js_arg!(#field_name, cx, call_args, self.#upcast_name().num_fields() + #idx as u32 - 1,
                            #conversion_behavior);
            }
        } else {
            quote! {
                get_js_arg!(#field_name, cx, call_args, #idx as u32,
                            #conversion_behavior);
            }
        }
    }
}

/// This function generates a struct which consists the definition of the struct, the
/// definition of the slot number and Rust getters and setters for the data stored in slots
fn get_magic_struct_code(variant: &syn::VariantData)
                         -> MagicStructCode {
    let res = match *variant {
        VariantData::Struct(ref fields) => {
            let mut getters = Vec::new();
            let mut setters = Vec::new();
            let mut get_callargs = Vec::new();
            let mut setter_invocations = Vec::new();
            let mut slot_counters = Vec::new();
            let mut inherit = false;
            let mut upcast = quote!{};
            let mut inherit_set_fields = quote!{};
            let mut inherit_num_fields = quote!{0};
            let mut inherit_type = &syn::Ty::Infer;
            let mut upcast_name = quote::Ident::from("");
            for (idx, field) in fields.iter().enumerate() {
                let id = &field.ident;
                let ty = &field.ty;
                let field_name_str = match *id {
                    Some(ref real_id) => format!("{}", real_id.to_string()),
                    None => panic!("Encounter an empty field. Something wrong..."),
                };
                if idx == 0 {
                    inherit = field_name_str == "_inherit";
                    if inherit {
                        inherit_type = ty;
                        upcast_name = match *ty {
                            syn::Ty::Path(ref qself, ref path) => {
                                if let &Some(ref q) = qself {
                                    panic!("Check the qself: {:?}", q);
                                }
                                let mut t = quote::Tokens::new();
                                let seg = &path.segments[path.segments.len()-1];
                                seg.to_tokens(&mut t);
                                quote::Ident::from(format!("as_{}", t.as_str()))
                            },
                            _ => {
                                debug!("Generating empty upcast name for {:?}", ty);
                                quote::Ident::from("")
                            }
                        };
                        upcast = quote! {
                            pub fn #upcast_name(&self) -> #ty {
                                #ty::new(self.object)
                            }
                        };
                        inherit_set_fields = quote!{
                            self.#upcast_name().set_fields(cx, call_args);
                        };
                        inherit_num_fields = quote!{
                            self.#upcast_name().num_fields()
                        };
                    }
                }
                let effective_idx = if inherit {
                    match *inherit_type {
                        syn::Ty::Path(ref qself, ref path) => {
                            if let &Some(ref q) = qself {
                                panic!("Check the qself: {:?}", q);
                            }
                            let mut t = quote::Tokens::new();
                            path.to_tokens(&mut t);
                            quote::Ident::from(format!(
                                "<{} as InheritanceSlots>::INHERITANCE_SLOTS - 1 + {}",
                                t.as_str(), idx))
                        },
                        _ => {
                            debug!("Generating 0 inheritance slot for {:?}", inherit_type);
                            quote::Ident::from("0 + ")
                        }
                    }
                } else {
                    quote::Ident::from(format!("{}", idx))
                };
                let field_name = quote::Ident::from(field_name_str);
                let mut tt = quote::Tokens::new();
                ty.to_tokens(&mut tt);
                let slot_counter = if inherit && idx == 0 {
                    quote::Ident::from(format!("<{} as InheritanceSlots>::INHERITANCE_SLOTS + ", tt.as_str()))
                } else {
                    quote::Ident::from(format!("<{} as NumSlots>::NUM_SLOTS + ", tt.as_str()))
                };
                let getter = gen_getter(id, ty, &effective_idx, inherit, idx);
                let (setter, setter_name) = gen_setter(id, ty, &effective_idx, inherit, idx);
                let get_callarg = gen_get_callargs(ty, &field_name, &upcast_name, inherit, idx);
                let setter_invocation = if inherit && idx == 0 {
                    quote!{}
                } else {
                    quote! {
                        self.#setter_name(cx, #field_name);
                    }
                };
                getters.push(getter);
                setters.push(setter);
                slot_counters.push(slot_counter);
                get_callargs.push(get_callarg);
                setter_invocations.push(setter_invocation);
            }
            let set_fields = quote! {
                pub unsafe fn set_fields(&self, cx: *mut JSContext, call_args: &JS::CallArgs) -> bool {
                    #inherit_set_fields
                    #(#get_callargs)*
                    #(#setter_invocations)*
                    true
                }
            };
            let cur_field_len = if inherit {
                getters.len() as u32 - 1
            } else {
                getters.len() as u32
            };
            let num_fields = quote! {
                pub unsafe fn num_fields(&self) -> u32 {
                    #inherit_num_fields + #cur_field_len
                }
            };
            MagicStructCode {
                getters: getters,
                setters: setters,
                slot_counters: slot_counters,
                set_fields: set_fields,
                num_fields: num_fields,
                upcast: upcast,
            }
        },
        _ => panic!("Only struct is implemented"),
    };
    res
}
