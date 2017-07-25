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
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::fs::create_dir_all;
use std::path::Path;

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
    let (rust_gen, js_gen) = match ast.body {
        Body::Enum(_) => panic!("#[derive(magic-dom)] can only be used with structs"),
        Body::Struct(ref data) => match_struct(name_str, data),
    };
    let manifest_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/magicdom/js");
    let dir_path = Path::new(manifest_dir);
    if !dir_path.exists() {
        match create_dir_all(dir_path) {
            Err(why) => {
                panic!("couldn't create dir: {}", why.description())
            },
            Ok(_) => (),
        }
    }
    let path_string = format!("{}/{}.js", manifest_dir, name_str);
    let path = Path::new(path_string.as_str());
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };
    match file.write_all(js_gen.to_string().as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   why.description())
        },
        Ok(_) => (),
    };
    match file.sync_all() {
        Err(why) => {
            panic!("couldn't sync: {}", why.description())
        },
        Ok(_) => (),
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

fn match_struct(name_str: &str, variant: &syn::VariantData) -> (quote::Tokens, quote::Tokens) {
    let name_upper = name_str.to_uppercase();
    let name = quote::Ident::from(name_str);
    let name_field = quote::Ident::from(format!("b\"{}\\0\"", name));
    let js_class = quote::Ident::from(format!("{}_CLASS", name_upper));
    let MagicStructCode {
        rust_getters,
        rust_setters,
        js_getters,
        js_setters,
        slot_counters,
        js_slot_counters,
        set_fields,
        num_fields,
        upcast,
    } = get_magic_struct_code(name_str, variant);
    let test_fn_name = quote::Ident::from(format!("test_{}_magic_layout()", name));
    let num_reserved_slots = quote::Ident::from(format!("<{} as InheritanceSlots>::INHERITANCE_SLOTS",
                                                        name));
    let constructor_quote = gen_constructor(&name, &js_class);
    let js_inherit_slot_name = quote::Ident::from(format!("{}_INHERIT_SLOT", name_str));
    let js_numslot_name = quote::Ident::from(format!("{}_NUMSLOT", name_str));
    let js_define_symbol = quote::Ident::from(format!("#define"));

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


            #(#rust_getters)*

            #(#rust_setters)*

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
    let js_gen = quote!{
        #js_define_symbol #js_inherit_slot_name #(#js_slot_counters)* 0

        #js_define_symbol #js_numslot_name 1

        #(#js_getters)*

        #(#js_setters)*
    };
    (rust_gen, js_gen)
}

struct MagicStructCode {
    /// Rust getters for the data stored in slots.
    rust_getters: Vec<quote::Tokens>,

    /// Rust setters for the data stored in slots.
    rust_setters: Vec<quote::Tokens>,

    /// JS getters for the data stored in slots.
    js_getters: Vec<quote::Tokens>,

    /// JS setters for the data stored in slots.
    js_setters: Vec<quote::Tokens>,

    /// Arithmetic statements for calculating the total number of slots
    slot_counters: Vec<quote::Ident>,

    /// Arithmetic statements for calculating the total number of slots (JS side)
    js_slot_counters: Vec<quote::Ident>,

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
              js_effective_idx: &quote::Ident,
              inherit: bool,
              idx: usize,
              name_str: &str) -> (quote::Tokens, quote::Tokens) {
    let (rust_getter_str, js_getter_str) = match *id {
        Some(ref real_id) => {
            let id_str = real_id.to_string();
            (format!("get_{}", id_str), format!("{}_get_{}", name_str, id_str))
        },
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let rust_getter_name = quote::Ident::from(rust_getter_str.as_str());
    let js_getter_name = quote::Ident::from(js_getter_str.as_str());
    let (rust_getter, js_getter) = if idx == 0 && inherit {
        (quote!{}, quote!{})
    } else {
        let rust_getter = quote! {
            pub unsafe fn #rust_getter_name (&self, cx: *mut JSContext) -> <#ty as ToFromJsSlots>::Target {
                let jsobj = self.object;
                <#ty as ToFromJsSlots>::from_slots(jsobj, cx, #effective_idx)
            }
        };
        let js_getter = quote! {
            function #js_getter_name() {
                let res = UnsafeGetReservedSlot(this, #js_effective_idx);
                return res;
            }
        };
        (rust_getter, js_getter)
    };
    (rust_getter, js_getter)
}

fn gen_setter(id: &Option<syn::Ident>,
              ty: &syn::Ty,
              effective_idx: &quote::Ident,
              js_effective_idx: &quote::Ident,
              inherit: bool,
              idx: usize,
              name_str: &str) -> (quote::Tokens, quote::Tokens, quote::Ident) {
    let (rust_setter_str, js_setter_str) = match *id {
        Some(ref real_id) => {
            let id_str = real_id.to_string();
            (format!("set_{}", id_str), format!("{}_set_{}", name_str, id_str))
        },
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    let rust_setter_name = quote::Ident::from(rust_setter_str.as_str());
    let js_setter_name = quote::Ident::from(js_setter_str.as_str());
    let (rust_setter, js_setter) = if idx == 0 && inherit {
        (quote!{}, quote!{})
    } else {
        let rust_setter = quote! {
            pub fn #rust_setter_name (&self, cx: *mut JSContext, t: #ty) {
                unsafe {
                    let jsobj = self.object;
                    t.into_slots(jsobj, cx, #effective_idx);
                }
            }
        };
        let js_setter = quote! {
            function #js_setter_name(v) {
                UnsafeSetReservedSlot(this, #js_effective_idx, v);
            }
        };
        (rust_setter, js_setter)
    };
    (rust_setter, js_setter, rust_setter_name)
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

fn gen_slot_counter(ty: &syn::Ty,
                    parent_name: &quote::Ident,
                    inherit: bool,
                    idx: usize,
                    ) -> (quote::Ident, quote::Ident) {
    let mut tt = quote::Tokens::new();
    ty.to_tokens(&mut tt);
    let type_str = tt.as_str();
    if inherit && idx == 0 {
        (quote::Ident::from(format!("<{} as InheritanceSlots>::INHERITANCE_SLOTS + ", type_str)),
         quote::Ident::from(format!("{}_INHERIT_SLOT + ", parent_name)))
    } else {
        let (type_first_part, type_last_part) = match *ty {
            syn::Ty::Path(ref qself, ref path) => {
                if let &Some(ref q) = qself {
                    panic!("Check the qself: {:?}", q);
                }
                let mut t = quote::Tokens::new();
                let mut t2 = quote::Tokens::new();
                let seg = &path.segments[0];
                let seg2 = &path.segments[path.segments.len() - 1];
                seg.to_tokens(&mut t);
                seg2.to_tokens(&mut t2);
                (t.to_string(), t2.to_string())
            },
            _ => {
                ("".to_string(), "".to_string())
            }
        };
        let js_numslot = if type_first_part.starts_with("Vec") {
            quote::Ident::from("VEC_NUMSLOT + ")
        } else if type_last_part != "" {
            match type_last_part.as_str() {
                "bool" | "str" | "String" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" |
                "i64" | "f32" | "f64" => quote::Ident::from("1 + "),
                _ => quote::Ident::from(format!("{}_NUMSLOT + ", type_last_part)),
            }
        } else if type_str.contains("String") {
            quote::Ident::from("1 + ")
        } else if type_str.contains("JSObject") {
            quote::Ident::from("1 + ")
        } else {
            quote::Ident::from(format!("{}_NUMSLOT + ", type_str))
        };
        (quote::Ident::from(format!("<{} as NumSlots>::NUM_SLOTS + ", type_str)),
         js_numslot)

    }
}

fn gen_effective_idx(first_field: &syn::Field,
                     parent_name: &quote::Ident,
                     inherit: bool,
                     idx: usize) -> (quote::Ident, quote::Ident) {
    let (effective_idx, js_effective_idx) = if inherit {
        let mut t = quote::Tokens::new();
        first_field.ty.to_tokens(&mut t);
        (quote::Ident::from(format!(
            "<{} as InheritanceSlots>::INHERITANCE_SLOTS - 1 + {}",
            t.as_str(), idx)),
         quote::Ident::from(format!(
             "{}_INHERIT_SLOT - 1 + {}",
             parent_name, idx))
        )
    } else {
        (quote::Ident::from(format!("{}", idx)),
         quote::Ident::from(format!("{}", idx)))
    };
    (effective_idx, js_effective_idx)
}

struct InheritanceStruct {
    /// Whether there's inheritance in the struct
    inherit: bool,

    /// Name of the parent class
    parent_name: quote::Ident,

    /// Name of the upcast method to the parent class
    upcast_name: quote::Ident,

    /// Method to upcast to parent class
    upcast: quote::Tokens,

    /// Method code to set fields using parent class's method
    inherit_set_fields: quote::Tokens,

    /// Method code to call num_fields method of the parent class
    inherit_num_fields: quote::Tokens,
}

fn check_inheritance(field: &syn::Field) -> InheritanceStruct {
    let id = &field.ident;
    let ty = &field.ty;
    let field_name_str = match *id {
        Some(ref real_id) => format!("{}", real_id.to_string()),
        None => panic!("Encounter an empty field. Something wrong..."),
    };
    if field_name_str == "_inherit" {
        let parent_name = match *ty {
            syn::Ty::Path(ref qself, ref path) => {
                if let &Some(ref q) = qself {
                    panic!("Check the qself: {:?}", q);
                }
                let mut t = quote::Tokens::new();
                let seg = &path.segments[path.segments.len()-1];
                seg.to_tokens(&mut t);
                quote::Ident::from(format!("{}", t.as_str()))
            },
            _ => {
                debug!("Generating empty upcast name for {:?}", ty);
                quote::Ident::from("")
            }
        };
        let upcast_name = quote::Ident::from(format!("as_{}", parent_name));
        let upcast = quote! {
            pub fn #upcast_name(&self) -> #ty {
                #ty::new(self.object)
            }
        };
        let inherit_set_fields = quote!{
            self.#upcast_name().set_fields(cx, call_args);
        };
        let inherit_num_fields = quote!{
            self.#upcast_name().num_fields()
        };
        InheritanceStruct {
            inherit: true,
            parent_name,
            upcast_name,
            upcast,
            inherit_set_fields,
            inherit_num_fields,
        }
    } else {
        InheritanceStruct {
            inherit: false,
            parent_name: quote::Ident::from(""),
            upcast_name: quote::Ident::from(""),
            upcast: quote!{},
            inherit_set_fields: quote!{},
            inherit_num_fields: quote!{0},
        }
    }
}

/// This function generates a struct which consists the definition of the struct, the
/// definition of the slot number and Rust getters and setters for the data stored in slots
fn get_magic_struct_code(name_str: &str, variant: &syn::VariantData)
                         -> MagicStructCode {
    let res = match *variant {
        VariantData::Struct(ref fields) => {
            let mut rust_getters = Vec::new();
            let mut rust_setters = Vec::new();
            let mut js_getters = Vec::new();
            let mut js_setters = Vec::new();
            let mut get_callargs = Vec::new();
            let mut setter_invocations = Vec::new();
            let mut slot_counters = Vec::new();
            let mut js_slot_counters = Vec::new();
            let first_field = &fields[0];
            let InheritanceStruct{
                inherit,
                parent_name,
                upcast_name,
                upcast,
                inherit_set_fields,
                inherit_num_fields,
            } = check_inheritance(first_field);
            for (idx, field) in fields.iter().enumerate() {
                let id = &field.ident;
                let ty = &field.ty;
                let field_name_str = match *id {
                    Some(ref real_id) => format!("{}", real_id.to_string()),
                    None => panic!("Encounter an empty field. Something wrong..."),
                };
                let (effective_idx, js_effective_idx) = gen_effective_idx(first_field,
                                                                          &parent_name,
                                                                          inherit,
                                                                          idx);
                let field_name = quote::Ident::from(field_name_str);
                let (slot_counter, js_slot_counter) = gen_slot_counter(ty, &parent_name, inherit, idx);
                let (rust_getter, js_getter) = gen_getter(id, ty, &effective_idx, &js_effective_idx,
                                                          inherit, idx, name_str);
                let (rust_setter, js_setter, setter_name) = gen_setter(id, ty, &effective_idx, &js_effective_idx,
                                                                       inherit, idx, name_str);
                let get_callarg = gen_get_callargs(ty, &field_name, &upcast_name, inherit, idx);
                let setter_invocation = if inherit && idx == 0 {
                    quote!{}
                } else {
                    quote! {
                        self.#setter_name(cx, #field_name);
                    }
                };
                rust_getters.push(rust_getter);
                rust_setters.push(rust_setter);
                js_getters.push(js_getter);
                js_setters.push(js_setter);
                slot_counters.push(slot_counter);
                js_slot_counters.push(js_slot_counter);
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
                rust_getters.len() as u32 - 1
            } else {
                rust_getters.len() as u32
            };
            let num_fields = quote! {
                pub unsafe fn num_fields(&self) -> u32 {
                    #inherit_num_fields + #cur_field_len
                }
            };
            MagicStructCode {
                rust_getters: rust_getters,
                rust_setters: rust_setters,
                js_getters: js_getters,
                js_setters: js_setters,
                slot_counters: slot_counters,
                js_slot_counters: js_slot_counters,
                set_fields: set_fields,
                num_fields: num_fields,
                upcast: upcast,
            }
        },
        _ => panic!("Only struct is implemented"),
    };
    res
}
