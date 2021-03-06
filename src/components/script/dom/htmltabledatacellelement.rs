/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLTableDataCellElementBinding;
use dom::bindings::codegen::InheritTypes::HTMLTableDataCellElementDerived;
use dom::bindings::js::{JSRef, Temporary};
use dom::document::Document;
use dom::element::HTMLTableDataCellElementTypeId;
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmltablecellelement::HTMLTableCellElement;
use dom::node::{Node, ElementNodeTypeId};
use servo_util::str::DOMString;

#[deriving(Encodable)]
pub struct HTMLTableDataCellElement {
    pub htmltablecellelement: HTMLTableCellElement,
}

impl HTMLTableDataCellElementDerived for EventTarget {
    fn is_htmltabledatacellelement(&self) -> bool {
        self.type_id == NodeTargetTypeId(ElementNodeTypeId(HTMLTableDataCellElementTypeId))
    }
}

impl HTMLTableDataCellElement {
    pub fn new_inherited(localName: DOMString, document: &JSRef<Document>) -> HTMLTableDataCellElement {
        HTMLTableDataCellElement {
            htmltablecellelement: HTMLTableCellElement::new_inherited(HTMLTableDataCellElementTypeId, localName, document)
        }
    }

    pub fn new(localName: DOMString, document: &JSRef<Document>) -> Temporary<HTMLTableDataCellElement> {
        let element = HTMLTableDataCellElement::new_inherited(localName, document);
        Node::reflect_node(box element, document, HTMLTableDataCellElementBinding::Wrap)
    }
}

pub trait HTMLTableDataCellElementMethods {
}
