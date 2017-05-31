// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains code used for madiating information between `Proxy`s and `Engine`.

// -------------------------------------------------------------------------------------------------

use std;
use std::collections::HashMap;
use std::os::unix::io::RawFd;
use std::path::PathBuf;

use libdrm;

use dharma;

use qualia::SurfaceId;

// -------------------------------------------------------------------------------------------------

/// `Mediator` stores information about which surface was created by which client.
///
/// For information about its place among other structures see crate-level documentation.
pub struct Mediator {
    sid_to_cid_dictionary: HashMap<SurfaceId, dharma::EventHandlerId>,
    transfer_offerer: Option<dharma::EventHandlerId>,
    screenshoter_cid: Option<dharma::EventHandlerId>,
    drm_device_path: Option<PathBuf>,
    drm_device_fd: Option<RawFd>,
}

define_ref!(struct Mediator as MediatorRef);

// -------------------------------------------------------------------------------------------------

impl Mediator {
    pub fn new() -> Self {
        Mediator {
            sid_to_cid_dictionary: HashMap::new(),
            transfer_offerer: None,
            screenshoter_cid: None,
            drm_device_fd: None,
            drm_device_path: None,
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

    pub fn register_transfer_offerer(&mut self, transfer_offerer: Option<dharma::EventHandlerId>) {
        self.transfer_offerer = transfer_offerer;
    }

    pub fn get_transfer_offerer(&self) -> Option<dharma::EventHandlerId> {
        self.transfer_offerer
    }

    pub fn register_screenshoter(&mut self, cid: Option<dharma::EventHandlerId>) {
        self.screenshoter_cid = cid;
    }

    pub fn get_screenshooter(&self) -> Option<dharma::EventHandlerId> {
        self.screenshoter_cid
    }

    pub fn set_drm_device(&mut self, fd: RawFd, path: PathBuf) {
        self.drm_device_fd = Some(fd);
        self.drm_device_path = Some(path);
    }

    pub fn get_drm_device_path(&self) -> Option<PathBuf> {
        self.drm_device_path.clone()
    }

    pub fn authenticate_drm_device(&self, magic: u32) {
        if let Some(fd) = self.drm_device_fd {
            // TODO: Add safe `drmAuthMagic` to lidrm bindings.
            let result = unsafe { libdrm::ffi::xf86drm::drmAuthMagic(fd, magic) };
            if result != 0 {
                log_warn3!("Failed to authenticate clients DRM device");
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
