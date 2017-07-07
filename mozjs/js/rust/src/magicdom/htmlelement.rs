/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
#[cfg(feature = "native_method")]
use conversions::{ConversionResult, ConversionBehavior, FromJSValConvertible,
                  ToJSValConvertible};
#[cfg(feature = "native_method")]
use glue::CreateCallArgsFromVp;
use jsslotconversions::ToFromJsSlots;

extern crate libc;

magic_dom! {
    HtmlElement,
    HTMLELEMENT_CLASS,
    HtmlElement_constructor,
    magic_dom_spec_HtmlElement,
    struct HtmlElement_spec {
        _inherit: element::Element,
        title: *mut JSString,
        lang: *mut JSString,
        translate: bool,
        dir: *mut JSString,
        hidden: bool,
        tabIndex: i32,
        accessKey: *mut JSString,
        accessKeyLabel: *mut JSString,
        draggable: bool,
        spellcheck: bool,
    }
}

impl HtmlElement {
    gen_getter_inherit!(get_local_name, *mut JSString, as_Element);
    gen_getter_inherit!(get_tag_name, *mut JSString, as_Element);
    gen_getter_inherit!(get_namespace, *mut JSString, as_Element);
    gen_getter_inherit!(get_prefix, *mut JSString, as_Element);
    gen_getter_inherit!(get_id, *mut JSString, as_Element);
    gen_getter_inherit!(get_attrs, *mut JSObject, as_Element);

    gen_setter_inherit!(set_local_name, *mut JSString, as_Element);
    gen_setter_inherit!(set_tag_name, *mut JSString, as_Element);
    gen_setter_inherit!(set_namespace, *mut JSString, as_Element);
    gen_setter_inherit!(set_prefix, *mut JSString, as_Element);
    gen_setter_inherit!(set_id, *mut JSString, as_Element);
    gen_setter_inherit!(set_attrs, *mut JSObject, as_Element);
}

// Exposing native rust method to js side
#[cfg(feature = "native_method")]
js_getter!(js_get_title, get_title, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_lang, get_lang, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_translate, get_translate, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_dir, get_dir, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_hidden, get_hidden, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_tabIndex, get_tabIndex, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_accessKey, get_accessKey, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_accessKeyLabel, get_accessKeyLabel, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_draggable, get_draggable, HtmlElement);
#[cfg(feature = "native_method")]
js_getter!(js_get_spellcheck, get_spellcheck, HtmlElement);

#[cfg(feature = "native_method")]
js_setter!(js_set_title, set_title, HtmlElement, ());
#[cfg(feature = "native_method")]
js_setter!(js_set_lang, set_lang, HtmlElement, ());
#[cfg(feature = "native_method")]
js_setter!(js_set_translate, set_translate, HtmlElement, ());
#[cfg(feature = "native_method")]
js_setter!(js_set_dir, set_dir, HtmlElement, ());
#[cfg(feature = "native_method")]
js_setter!(js_set_hidden, set_hidden, HtmlElement, ());
#[cfg(feature = "native_method")]
js_setter!(js_set_tabIndex, set_tabIndex, HtmlElement, ConversionBehavior::Default);
#[cfg(feature = "native_method")]
js_setter!(js_set_accessKey, set_accessKey, HtmlElement, ());
#[cfg(feature = "native_method")]
js_setter!(js_set_draggable, set_draggable, HtmlElement, ());
#[cfg(feature = "native_method")]
js_setter!(js_set_spellcheck, set_spellcheck, HtmlElement, ());

#[cfg(feature = "native_method")]
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

// self hosted getter and setter
#[cfg(not(feature = "native_method"))]
lazy_static! {
    pub static ref HTMLELEMENT_PS_ARR: [JSPropertySpec; 11] = [
        JSPropertySpec::getter_setter_selfhosted(b"title\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_title\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_title\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_setter_selfhosted(b"lang\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_lang\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_lang\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_setter_selfhosted(b"translate\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_translate\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_translate\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_setter_selfhosted(b"dir\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_dir\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_dir\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_setter_selfhosted(b"hidden\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_hidden\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_hidden\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_setter_selfhosted(b"tabIndex\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_tabIndex\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_tabIndex\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_setter_selfhosted(b"accessKey\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_accessKey\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_accessKey\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_selfhosted(b"accessKeyLabel\0".as_ptr() as *const libc::c_char,
                                          JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                          "HtmlElement_get_accessKeyLabel\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_setter_selfhosted(b"draggable\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_draggable\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_draggable\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::getter_setter_selfhosted(b"spellcheck\0".as_ptr() as *const libc::c_char,
                                                 JSPROP_ENUMERATE | JSPROP_PERMANENT,
                                                 "HtmlElement_get_spellcheck\0".as_ptr() as *const libc::c_char,
                                                 "HtmlElement_set_spellcheck\0".as_ptr() as *const libc::c_char,
        ),
        JSPropertySpec::end_spec(),
    ];
}
