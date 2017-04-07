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
    screenshoter_cid: Option<dharma::EventHandlerId>,
}

define_ref!(struct Mediator as MediatorRef);

// -------------------------------------------------------------------------------------------------

impl Mediator {
    pub fn new() -> Self {
        Mediator {
            sid_to_cid_dictionary: HashMap::new(),
            screenshoter_cid: None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Mediator {
    pub fn relate_sid_to_client(&mut self, sid: SurfaceId, cid: dharma::EventHandlerId) {
        self.sid_to_cid_dictionary.insert(sid, cid);
    }

    pub fn get_client_for_sid(&self, sid: SurfaceId) -> Option<&dharma::EventHandlerId> {
        self.sid_to_cid_dictionary.get(&sid)
    }

    pub fn remove(&mut self, sid: SurfaceId) {
        self.sid_to_cid_dictionary.remove(&sid);
    }

    pub fn register_screenshoter(&mut self, cid: Option<dharma::EventHandlerId>) {
        self.screenshoter_cid = cid;
    }

    pub fn get_screenshooter(&self) -> Option<dharma::EventHandlerId> {
        self.screenshoter_cid
    }
}

// -------------------------------------------------------------------------------------------------
