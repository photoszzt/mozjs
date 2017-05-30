/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate magic_codegen;
#[macro_use]
extern crate js;

magic_dom! {
    DOMPoint,
    struct DOMPoint_spec {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }
}
