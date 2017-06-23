/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
use jsapi;
use jsval::ObjectValue;
use conversions::{ConversionResult, ConversionBehavior, FromJSValConvertible,
                  ToJSValConvertible};
use glue::CreateCallArgsFromVp;
use magicdom::*;
use jsslotconversions::ToFromJsSlots;

extern crate libc;

magic_dom! {
    HtmlElement,
    HTMLELEMENT_CLASS,
    HtmlElement_constructor,
    magic_dom_spec_HtmlElement,
    struct HtmlElement_spec {
        _inherit: element::Element,
        title: String,
        lang: String,
        translate: bool,
        dir: String,
        hidden: bool,
        tabIndex: i32,
        accessKey: String,
        accessKeyLabel: String,
        draggable: bool,
        spellcheck: bool,
    }
}

impl HtmlElement {
    gen_getter_inherit!(get_local_name, String, as_Element);
    gen_getter_inherit!(get_tag_name, String, as_Element);
    gen_getter_inherit!(get_namespace, String, as_Element);
    gen_getter_inherit!(get_prefix, String, as_Element);
    gen_getter_inherit!(get_id, String, as_Element);
    gen_getter_inherit!(get_attrs, Vec<attr::Attr>, as_Element);

    gen_setter_inherit!(set_local_name, String, as_Element);
    gen_setter_inherit!(set_tag_name, String, as_Element);
    gen_setter_inherit!(set_namespace, String, as_Element);
    gen_setter_inherit!(set_prefix, String, as_Element);
    gen_setter_inherit!(set_id, String, as_Element);
    gen_setter_inherit!(set_attrs, Vec<attr::Attr>, as_Element);
}

js_getter!(js_get_title, get_title, HtmlElement);
js_getter!(js_get_lang, get_lang, HtmlElement);
js_getter!(js_get_translate, get_translate, HtmlElement);
js_getter!(js_get_dir, get_dir, HtmlElement);
js_getter!(js_get_hidden, get_hidden, HtmlElement);
js_getter!(js_get_tabIndex, get_tabIndex, HtmlElement);
js_getter!(js_get_accessKey, get_accessKey, HtmlElement);
js_getter!(js_get_accessKeyLabel, get_accessKeyLabel, HtmlElement);
js_getter!(js_get_draggable, get_draggable, HtmlElement);
js_getter!(js_get_spellcheck, get_spellcheck, HtmlElement);

js_setter!(js_set_title, set_title, HtmlElement, ());
js_setter!(js_set_lang, set_lang, HtmlElement, ());
js_setter!(js_set_translate, set_translate, HtmlElement, ());
js_setter!(js_set_dir, set_dir, HtmlElement, ());
js_setter!(js_set_hidden, set_hidden, HtmlElement, ());
js_setter!(js_set_tabIndex, set_tabIndex, HtmlElement, ConversionBehavior::Default);
js_setter!(js_set_accessKey, set_accessKey, HtmlElement, ());
js_setter!(js_set_draggable, set_draggable, HtmlElement, ());
js_setter!(js_set_spellcheck, set_spellcheck, HtmlElement, ());

lazy_static! {
    pub static ref HTMLELEMENT_PS_ARR: [JSPropertySpec; 11] = [
        JSPropertySpec::getter_setter(b"title\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_title), Some(js_set_title)),
        JSPropertySpec::getter_setter(b"lang\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_lang), Some(js_set_lang)),
        JSPropertySpec::getter_setter(b"translate\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_translate), Some(js_set_translate)),
        JSPropertySpec::getter_setter(b"dir\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_dir), Some(js_set_dir)),
        JSPropertySpec::getter_setter(b"hidden\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_hidden), Some(js_set_hidden)),
        JSPropertySpec::getter_setter(b"tabIndex\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_tabIndex), Some(js_set_tabIndex)),
        JSPropertySpec::getter_setter(b"accessKey\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_accessKey), Some(js_set_accessKey)),
        JSPropertySpec::getter(b"accessKeyLabel\0".as_ptr() as *const libc::c_char,
                               JSPROP_ENUMERATE | JSPROP_PERMANENT,
                               Some(js_get_accessKeyLabel)),
        JSPropertySpec::getter_setter(b"draggable\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_draggable), Some(js_set_draggable)),
        JSPropertySpec::getter_setter(b"spellcheck\0".as_ptr() as *const libc::c_char,
                                      JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                      Some(js_get_spellcheck), Some(js_set_spellcheck)),
        JSPropertySpec::end_spec(),
    ];
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn HtmlElement_constructor(cx: *mut JSContext, argc: u32, vp: *mut JS::Value) -> bool {
    let call_args = CreateCallArgsFromVp(argc, vp);
    if call_args._base.argc_ != 16 {
        JS_ReportErrorASCII(cx, b"constructor requires exactly 16 \
                                  arguments\0".as_ptr() as *const
                            libc::c_char);
        return false;
    }

    rooted!(in(cx) let jsobj = jsapi::JS_NewObjectForConstructor(cx,
                                                                 &HTMLELEMENT_CLASS as *const _,
                                                                 &call_args as *const _));
    if jsobj.is_null() {
        JS_ReportErrorASCII(cx, b"Fail to construct JS object\0".as_ptr() as *const libc::c_char);
        return false;
    }
    let obj = match HtmlElement::from_object(jsobj.get()) {
        Some(o) => o,
        None => {
            JS_ReportErrorASCII(cx, b"Fail to construct DOMPoint from JS \
                                object\0" as *const u8 as *const
                                libc::c_char);
            return false;
        }
    };

    get_js_arg!(local_name, cx, call_args, 0, ());
    get_js_arg!(tag_name, cx, call_args, 1, ());
    get_js_arg!(namespace, cx, call_args, 2, ());
    get_js_arg!(prefix, cx, call_args, 3, ());
    get_js_arg!(id, cx, call_args, 4, ());
    get_js_arg!(attrs, cx, call_args, 5, ());
    get_js_arg!(title, cx, call_args, 6, ());
    get_js_arg!(lang, cx, call_args, 7, ());
    get_js_arg!(translate, cx, call_args, 8, ());
    get_js_arg!(dir, cx, call_args, 9, ());
    get_js_arg!(hidden, cx, call_args, 10, ());
    get_js_arg!(tabIndex, cx, call_args, 11, ConversionBehavior::Default);
    get_js_arg!(accessKey, cx, call_args, 12, ());
    get_js_arg!(accessKeyLabel, cx, call_args, 13, ());
    get_js_arg!(draggable, cx, call_args, 14, ());
    get_js_arg!(spellcheck, cx, call_args, 15, ());

    obj.set_local_name(cx, local_name);
    obj.set_tag_name(cx, tag_name);
    obj.set_namespace(cx, namespace);
    obj.set_prefix(cx, prefix);
    obj.set_id(cx, id);
    obj.set_attrs(cx, attrs);
    obj.set_title(cx, title);
    obj.set_lang(cx, lang);
    obj.set_translate(cx, translate);
    obj.set_dir(cx, dir);
    obj.set_hidden(cx, hidden);
    obj.set_tabIndex(cx, tabIndex);
    obj.set_accessKey(cx, accessKey);
    obj.set_accessKeyLabel(cx, accessKeyLabel);
    obj.set_draggable(cx, draggable);
    obj.set_spellcheck(cx, spellcheck);

    call_args.rval().set(ObjectValue(jsobj.get()));
    true
}
