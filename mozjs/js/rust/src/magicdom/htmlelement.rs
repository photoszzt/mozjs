/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::root::*;
use conversions::{ConversionResult, ConversionBehavior, FromJSValConvertible, ToJSValConvertible};
use glue::CreateCallArgsFromVp;
extern crate libc;

magic_dom! {
    HtmlElement,
    HTMLELEMENT_CLASS,
    HtmlElement_constructor,
    magic_dom_spec_HtmlElement,
    struct HtmlElement_spec {
        // TODO put Element here
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
