/* This Source Code Form is subject to the terms of the Mozilla Wlic
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

function Node_appendChild(node) {
    let child_nodes = callFunction(Node_get_child_nodes, this);
    callFunction(std_Array_push, child_nodes, node);
    return node;
}
