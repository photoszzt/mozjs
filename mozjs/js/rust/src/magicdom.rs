/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

magic_dom! {
    DOMPoint,
    DOMPOINT_CLASS,
    DOMPOINT_PS_ARR,
    DOMPoint_constructor,
    magic_dom_spec_DOMPoint,
    struct DOMPoint_spec {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }
}

magic_dom! {
    DOMQuad,
    DOMQUAD_CLASS,
    DOMQUAD_PS_ARR,
    DOMQuad_constructor,
    magic_dom_spec_DOMQuad,
    struct DOMQuad_spec {
        p1: DOMPoint,
        p2: DOMPoint,
        p3: DOMPoint,
        p4: DOMPoint,
    }
}
