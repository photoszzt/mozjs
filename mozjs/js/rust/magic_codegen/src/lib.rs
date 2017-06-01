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
    println!("{}", gen.to_string());
    gen.to_string().parse().unwrap()
}

fn match_struct(ast: &syn::DeriveInput, variant: &syn::VariantData) -> quote::Tokens {
    let name_spec = &ast.ident;
    let name_spec_str = name_spec.to_string();
    let name = name_spec_str
        .split('_')
        .next()
        .expect("Should have a '_' in the magic dom struct name");
    let dom_flag = if name.starts_with("DOM") {
        quote::Ident::from("jsapi::JSCLASS_IS_DOMJSCLASS")
    } else {
        quote::Ident::from("")
    };
    let name_field = quote::Ident::from(format!("b\"{}\\0\"", name));
    let name = quote::Ident::from(name);
    let js_class = quote::Ident::from(format!("{}_class", name));
    let struct_init_gc = get_init_struct_field(variant);
    let init_copy = struct_init_gc.to_vec();
    let (struct_def, slot_num, getters, setters) =
        get_struct_def(&name, variant);
    let struct_field_tests = get_addr_test_struct_field(variant);
    let test_fn_name = quote::Ident::from(format!("test_{}_magic_layout()", name));
    let num_reserved_slots = quote::Ident::from(format!("{}", slot_num.len()));

    /// TODO: Need to generate the js class implementation
    quote! {
        extern crate libc;
        use js::jsapi;
        use js::jsapi::root::*;
        use js::magic::{MagicSlot, SlotIndex};
        use js::rust::{GCMethods, RootKind};

        use std::mem;
        use std::ptr;

        pub struct #name {
            #(#struct_def,)*
        }

        pub static #js_class : jsapi::JSClass = jsapi::JSClass {
            name: #name_field as *const u8 as *const libc::c_char,
            flags: (#num_reserved_slots &
                    jsapi::JSCLASS_RESERVED_SLOTS_MASK) << jsapi::JSCLASS_RESERVED_SLOTS_SHIFT |
            jsapi::JSCLASS_HAS_PRIVATE  |
            #dom_flag,
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

            fn from_object(obj: *mut JSObject) -> Option<#name> {
                if jsapi::JS_GetClass(obj) as usize == &#js_class as usize {
                    Some(#name {
                        object: obj,
                        #(#struct_init_gc,)*
                    })
                } else {
                    None
                }
            }
        }

        impl GCMethods for #name  {
            unsafe fn initial() -> #name {
                #name {
                    object: ptr::null_mut(),
                    #(#init_copy,)*
                }
            }

            unsafe fn post_barrier(v: *mut #name, prev: #name, next: #name) {
                let v = &mut (*v).as_jsobject() as *mut *mut jsapi::JSObject;
                let prev = prev.as_jsobject();
                let next = next.as_jsobject();
                <*mut jsapi::JSObject as GCMethods>::post_barrier(v, prev, next);
            }
        }

        #(#getters)*

        #(#setters)*

        #(#slot_num)*

        pub fn check_this(cx: *mut JSContext, args: &JS::CallArgs) -> Option<*mut JSObject> {
            rooted!(in(cx) let thisv = args.thisv());
            if !thisv.is_object() {
                return None;
            }
            let jsobj = thisv.to_object();
            if jsapi::JS_GetClass(jsobj) as usize != &#js_class as usize {
                return None;
            }
            Some(jsobj)
        }

        #[test]
        fn it_compiles() {
            assert!(true);
        }

        #[test]
        fn #test_fn_name {
            assert_eq!(mem::size_of::<#name>(), mem::size_of::<*mut jsapi::JSObject>());
            assert_eq!(mem::align_of::<#name>(), mem::align_of::<*mut jsapi::JSObject>());

            let instance: #name = unsafe { mem::zeroed() };
            let uptr_size = mem::size_of::<usize>();
            assert_eq!(&instance as *const _ as usize, &instance.object as *const _ as usize);
            #(#struct_field_tests)*
        }
    }
}

/// This function generates the code to initialize the struct.
fn get_init_struct_field(variant: &syn::VariantData) -> Vec<quote::Tokens> {
    let res = match *variant {
        VariantData::Struct(ref fields) => {
            let items: Vec<_> = fields
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    quote! {
                        #ident: MagicSlot::new()
                    }
                })
                .collect();
            items
        }
        _ => panic!("Only struct is implemented"),
    };
    res
}

/// This function generates test code to check the size and alignment for each field in the struct
fn get_addr_test_struct_field(variant: &syn::VariantData) -> Vec<quote::Tokens> {
    let res = match *variant {
        VariantData::Struct(ref fields) => {
            let items: Vec<_> = fields
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    quote! {
                    assert_eq!(((&instance.#ident as *const _ as usize) - uptr_size),
                               (&instance.object as *const _ as usize));
                    }
                })
                .collect();
            items
        }
        _ => panic!("Only struct is implemented"),
    };
    res
}

/// This function generates a tuple which consists the definition of the struct, the
/// definition of the slot number, getter and setter for the object and getters and
/// setters for the js
fn get_struct_def(name: &quote::Ident,
                  variant: &syn::VariantData)
                  -> (Vec<quote::Tokens>, Vec<quote::Tokens>, Vec<quote::Tokens>,
                      Vec<quote::Tokens>) {
    let res = match *variant {
        VariantData::Struct(ref fields) => {
            let mut result = Vec::new();
            let mut slot_num_res = Vec::new();
            let mut getters = Vec::new();
            let mut setters = Vec::new();
            result.push(quote! {
                object: *mut jsapi::JSObject
            });
            for (idx, field) in fields.iter().enumerate() {
                let id = &field.ident;
                let ty = &field.ty;
                let slot_num_type_name = quote::Ident::from(format!("{}SlotIndex{}", name, idx));
                let slot_num = quote! {
                    enum #slot_num_type_name {}
                    impl SlotIndex for #slot_num_type_name {
                        fn slot_index() -> u32 { #idx as u32 }
                    }
                };
                let getter_str = match *id {
                    Some(ref real_id) => format!("get_{}", real_id.to_string()),
                    None => panic!("Encounter a empty field. Something wrong..."),
                };
                let setter_str = match *id {
                    Some(ref real_id) => format!("set_{}", real_id.to_string()),
                    None => panic!("Encounter a empty field. Something wrong..."),
                };
                let getter_name = quote::Ident::from(getter_str);
                let setter_name = quote::Ident::from(setter_str);
                let new = quote! {
                    #id: MagicSlot<#ty, #slot_num_type_name>
                };

                // getter and setter for the obj #name
                let getter = quote! {
                    pub fn #getter_name (obj: #name, cx: *mut jsapi::JSContext) -> #ty {
                        unsafe {
                            obj.#id.get(cx)
                        }
                    }
                };
                let setter = quote! {
                    pub fn #setter_name (obj: #name, cx: *mut jsapi::JSContext, t: #ty) {
                        unsafe {
                            obj.#id.set(cx, t);
                        }
                    }
                };
                slot_num_res.push(slot_num);
                result.push(new);
                getters.push(getter);
                setters.push(setter);
            }
            (result, slot_num_res, getters, setters)
        },
        _ => panic!("Only struct is implemented"),
    };
    res
}
