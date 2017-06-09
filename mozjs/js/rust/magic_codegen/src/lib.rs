#![feature(proc_macro)]
#![recursion_limit="256"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;

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
    let name_field = quote::Ident::from(format!("b\"{}\\0\"", name));
    let js_class = quote::Ident::from(format!("{}_CLASS", name.to_uppercase()));
    let name = quote::Ident::from(name);
    let MagicStructCode {
        getters,
        setters,
        js_getters,
        js_setters,
        js_prop_spec
    } = get_magic_struct_code(&name, variant);
    let spec_size = js_prop_spec.len() + 1;
    let test_fn_name = quote::Ident::from(format!("test_{}_magic_layout()", name));
    let num_reserved_slots = quote::Ident::from(format!("{}", js_prop_spec.len()));

    quote! {
        extern crate libc;
        use js::jsapi;
        use js::jsapi::root::*;
        use js::rust::{GCMethods, RootKind};
        use js::glue::CreateCallArgsFromVp;
        use js::jsval::DoubleValue;
        use js::rust::ToNumber;
        use js::conversions::{ConversionResult, FromJSValConvertible, ToJSValConvertible};
        use js::jsval;

        use std::mem;
        use std::ptr;

        #[allow(non_camel_case_types)]
        pub struct #name {
            object: *mut jsapi::JSObject,
        }

        pub static #js_class : jsapi::JSClass = jsapi::JSClass {
            name: #name_field as *const u8 as *const libc::c_char,
            flags: (#num_reserved_slots &
                    jsapi::JSCLASS_RESERVED_SLOTS_MASK) << jsapi::JSCLASS_RESERVED_SLOTS_SHIFT |
            jsapi::JSCLASS_HAS_PRIVATE,
            cOps: 0 as *const _,
            reserved: [0 as *mut _; 3],
        };

        impl RootKind for #name  {
            #[inline(always)]
            fn rootKind() -> ::js::jsapi::JS::RootKind {
                jsapi::JS::RootKind::Object
            }
        }

        impl #name {
            fn as_jsobject(&self) -> *mut jsapi::JSObject {
                self.object
            }

            pub unsafe fn from_object(obj: *mut JSObject) -> Option<#name> {
                if jsapi::JS_GetClass(obj) as *const _ as usize == &#js_class as *const _ as usize {
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
                let v = &mut (*v).as_jsobject() as *mut *mut jsapi::JSObject;
                let prev = prev.as_jsobject();
                let next = next.as_jsobject();
                <*mut jsapi::JSObject as GCMethods>::post_barrier(v, prev, next);
            }
        }

        #(#js_getters)*

        #(#js_setters)*

        lazy_static! {
            pub static ref PS_ARR: [JSPropertySpec; #spec_size] = [
                #(#js_prop_spec,)*
                JSPropertySpec::end_spec(),
            ];
        }

        #[test]
        fn it_compiles() {
            assert!(true);
        }

        #[allow(non_snake_case)]
        #[test]
        fn #test_fn_name {
            assert_eq!(mem::size_of::<#name>(), mem::size_of::<*mut jsapi::JSObject>());
            assert_eq!(mem::align_of::<#name>(), mem::align_of::<*mut jsapi::JSObject>());

            let instance: #name = unsafe { mem::zeroed() };
            assert_eq!(&instance as *const _ as usize, &instance.object as *const _ as usize);
        }
    }
}

/// This hashmap stores the mapping of the symbol of a Rust type to the symbol of a function
/// that converts the Rust type into the corresponding JS type.
lazy_static! {
    static ref JSVAL: std::collections::HashMap<syn::Ident, syn::Ident> = {
        let mut m = std::collections::HashMap::new();
        m.insert(syn::Ident::from("f64"), syn::Ident::from("DoubleValue"));
        m.insert(syn::Ident::from("u32"), syn::Ident::from("UInt32Value"));
        m.insert(syn::Ident::from("i32"), syn::Ident::from("Int32Value"));
        m.insert(syn::Ident::from("JSString"), syn::Ident::from("StringValue"));
        m.insert(syn::Ident::from("bool"), syn::Ident::from("BooleanValue"));
        m.insert(syn::Ident::from("JSObject"), syn::Ident::from("ObjectValue"));
        m
    };
}

struct MagicStructCode {
    /// Rust getters for the data stored in slots.
    getters: Vec<quote::Tokens>,

    /// Rust setters for the data stored in slots.
    setters: Vec<quote::Tokens>,

    /// `JSNative` getter functions exposed to JavaScript.
    js_getters: Vec<quote::Tokens>,

    /// `JSNative` setter functions exposed to JavaScript.
    js_setters: Vec<quote::Tokens>,

    /// The `JSPropertySpec` definition that exposes the `JSNative`s to JavaScript.
    js_prop_spec: Vec<quote::Tokens>,
}

fn gen_getter(id: &Option<syn::Ident>,
              ty: &syn::Ty,
              idx: u32) -> (quote::Tokens, quote::Ident) {
    let getter_str = match *id {
        Some(ref real_id) => format!("get_{}", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let getter_name = quote::Ident::from(getter_str);
    let getter = quote! {
        pub unsafe fn #getter_name (&self, cx: *mut jsapi::JSContext) -> #ty {
            let jsobj = self.object;
            rooted!(in(cx) let val = jsapi::JS_GetReservedSlot(jsobj, #idx));

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
    };
    (getter, getter_name)
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
        pub fn #setter_name (&self, cx: *mut jsapi::JSContext, t: #ty) {
            unsafe {
                let jsobj = self.object;
                rooted!(in(cx) let mut val = jsval::UndefinedValue());
                t.to_jsval(cx, val.handle_mut());
                jsapi::JS_SetReservedSlot(jsobj, #idx, &*val);
            }
        }
    };
    (setter, setter_name)
}

fn gen_js_getter(id: &Option<syn::Ident>,
                 name: &quote::Ident,
                 ty: &syn::Ty,
                 getter_name: &quote::Ident) -> (quote::Tokens, quote::Ident) {
    let js_getter_str = match *id {
        Some(ref real_id) => format!("js_get_{}", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let js_getter_name = quote::Ident::from(js_getter_str);
    let jsval = match *ty {
        syn::Ty::Path(ref qself, ref path) => {
            if let &Some(ref q) = qself {
                panic!("Check the qself: {:?}", q);
            }
            let seg = &path.segments[0];
            match JSVAL.get(&seg.ident) {
                Some(jsval_func) => jsval_func,
                None => {
                    panic!("No corresponding js val function for {}", seg.ident);
                }
            }
        },
        _ => {
            panic!("This type {:?} hasn't been handled", ty);
        }
    };
    let js_getter = quote! {
        pub extern "C" fn #js_getter_name (cx: *mut jsapi::JSContext, argc: u32, vp: *mut JS::Value)
                                           -> bool {
            let res = unsafe {
                let call_args = CreateCallArgsFromVp(argc, vp);
                if call_args._base.argc_ != 0 {
                    JS_ReportErrorASCII(cx, b"getter doesn't require any arguments\0".as_ptr()
                                        as *const libc::c_char);
                    return false;
                }
                let obj = match #name::check_this(cx, &call_args) {
                    Some(obj_) => obj_,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't convert JSObject\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    },
                };
                let val = obj.#getter_name(cx);
                call_args.rval().set(#jsval(val));
                true
            };
            res
        }
    };
    (js_getter, js_getter_name)
}

fn gen_js_setter(id: &Option<syn::Ident>,
                 name: &quote::Ident,
                 setter_name: &quote::Ident) -> (quote::Tokens, quote::Ident) {
    let js_setter_str = match *id {
        Some(ref real_id) => format!("js_set_{}", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let js_setter_name = quote::Ident::from(js_setter_str);
    let js_setter = quote! {
        pub extern "C" fn #js_setter_name (cx: *mut jsapi::JSContext, argc: u32, vp: *mut JS::Value)
                                           -> bool {
            let res = unsafe {
                let call_args = CreateCallArgsFromVp(argc, vp);
                if call_args._base.argc_ != 1 {
                    JS_ReportErrorASCII(cx, b"setter requires exactly 1 arguments\0".as_ptr()
                                        as *const libc::c_char);
                    return false;
                }
                let obj = match #name::check_this(cx, &call_args) {
                    Some(obj_) => obj_,
                    None => {
                        JS_ReportErrorASCII(cx, b"Can't convert JSObject\0".as_ptr()
                                            as *const libc::c_char);
                        return false;
                    },
                };
                let v = match ToNumber(cx, call_args.index(0)) {
                    Ok(num) => num,
                    Err(_) => {
                        JS_ReportErrorASCII(cx, b"Can't recognize arg 0\0".as_ptr() as *const libc::c_char);
                        return false;
                    },
                };
                obj.#setter_name(cx, v);
                true
            };
            res
        }
    };
    (js_setter, js_setter_name)
}

fn gen_js_prop_spec(id: &Option<syn::Ident>,
                    js_getter_name: &quote::Ident,
                    js_setter_name: &quote::Ident) -> quote::Tokens {
    let js_prop_spec_str = match *id {
        Some(ref real_id) => format!("b\"{}\\0\" as *const u8 as *const libc::c_char", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let js_prop_spec_name = quote::Ident::from(js_prop_spec_str);
    quote! {
        JSPropertySpec::getter_setter(#js_prop_spec_name,
                                      jsapi::JSPROP_ENUMERATE | jsapi::JSPROP_PERMANENT,
                                      Some(#js_getter_name), Some(#js_setter_name))
    }
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
            let mut js_getters = Vec::new();
            let mut js_setters = Vec::new();
            let mut js_prop_specs = Vec::new();
            for (idx, field) in fields.iter().enumerate() {
                let id = &field.ident;
                let ty = &field.ty;
                let (getter, getter_name) = gen_getter(id, ty, idx as u32);
                let (setter, setter_name) = gen_setter(id, ty, idx as u32);
                let (js_getter, js_getter_name) = gen_js_getter(id, name, ty, &getter_name);
                let (js_setter, js_setter_name) = gen_js_setter(id, name, &setter_name);
                let js_prop_spec = gen_js_prop_spec(id, &js_getter_name, &js_setter_name);
                getters.push(getter);
                setters.push(setter);
                js_setters.push(js_setter);
                js_getters.push(js_getter);
                js_prop_specs.push(js_prop_spec);
            }
            MagicStructCode {
                getters: getters,
                setters: setters,
                js_getters: js_getters,
                js_setters: js_setters,
                js_prop_spec: js_prop_specs,
            }
        },
        _ => panic!("Only struct is implemented"),
    };
    res
}
