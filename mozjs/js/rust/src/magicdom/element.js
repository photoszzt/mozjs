/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

function Element_getAttributes(qualifiedName) {
    let attrs = callFunction(Element_get_attrs, this);
    let attrs_length = attrs.length;
    for (var i = 0; i < attrs_length; i++) {
        let attr = attrs[i];
        let local_name = callFunction(Attr_get_identifier_local_name, attr);
        let prefix = callFunction(Attr_get_identifier_prefix, attr);
        var qname = "";
        if (prefix == null) {
            qname = local_name;
        } else {
            qname = prefix + ":" + local_name;
        }
        if (qname == qualifiedName) {
            var value = callFunction(Attr_get_value, attr);
            return value;
        }
    }
    return null;
}

function Element_setAttributes(qualifiedName, value) {
    let attrs = callFunction(Element_get_attrs, this);
    let attrs_length = attrs.length;
    for (var i = 0; i < attrs_length; i++) {
        let attr = attrs[i];
        let local_name = callFunction(Attr_get_identifier_local_name, attr);
        let prefix = callFunction(Attr_get_identifier_prefix, attr);
        let qname = '';
        if (prefix == null) {
            qname = local_name;
        } else {
            qname = prefix + ':' + local_name;
        }
        if (qname == qualifiedName) {
            callFunction(Attr_set_value, attr, value);
            return;
        }
    }
    let new_attr = new Attr(qualifiedName, null, null, null, value);
    callFunction(std_Array_push, attrs, new_attr);
}
