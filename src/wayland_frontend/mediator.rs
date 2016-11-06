// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code used for madiating information between `Proxy`s and `Engine`.

// -------------------------------------------------------------------------------------------------

use std;
use std::collections::HashMap;

use dharma;
use qualia::SurfaceId;

// -------------------------------------------------------------------------------------------------

/// `Mediator` stores information about which surface was created by which client.
///
/// For information about its place among other structures see crate-level documentation.
pub struct Mediator {
    sid_to_cid_dictionary: HashMap<SurfaceId, dharma::EventHandlerId>,
}

define_ref!(Mediator, MediatorRef);

// -------------------------------------------------------------------------------------------------

impl Mediator {
    pub fn new() -> Self {
        Mediator { sid_to_cid_dictionary: HashMap::new() }
    }
}

// -------------------------------------------------------------------------------------------------

impl Mediator {
    pub fn relate_sid_to_client(&mut self, sid: SurfaceId, hid: dharma::EventHandlerId) {
        self.sid_to_cid_dictionary.insert(sid, hid);
    }

    pub fn get_client_for_sid(&self, sid: SurfaceId) -> Option<&dharma::EventHandlerId> {
        self.sid_to_cid_dictionary.get(&sid)
    }
}

// -------------------------------------------------------------------------------------------------
