// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementations of Wayland `wl_data_device_manager`, `wl_data_device`, `wl_data_source` and
//! `wl_data_offer` objects.

use std::os::unix::io::RawFd;

use std::rc::Rc;

use skylane::server::{Bundle, Object, ObjectId, Task};
use skylane_protocols::server::Handler;
use skylane_protocols::server::wayland::wl_data_device_manager;
use skylane_protocols::server::wayland::wl_data_source;
use skylane_protocols::server::wayland::wl_data_device;
use skylane_protocols::server::wayland::wl_data_offer;

use qualia::Transfer;

use global::Global;
use proxy::ProxyRef;
use facade::Facade;

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_data_device_manager` object.
struct DataDeviceManager {
    proxy_ref: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

pub fn get_global() -> Global {
    Global::new(wl_data_device_manager::NAME,
                wl_data_device_manager::VERSION,
                Rc::new(DataDeviceManager::new_object))
}

// -------------------------------------------------------------------------------------------------

impl DataDeviceManager {
    /// Creates new `DataDeviceManager`.
    fn new(_oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        DataDeviceManager {
            proxy_ref: proxy_ref,
        }
    }

    fn new_object(oid: ObjectId, _version: u32, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_data_device_manager::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_data_device_manager::Interface for DataDeviceManager {
    fn create_data_source(&mut self,
                          _this_object_id: ObjectId,
                          _bundle: &mut Bundle,
                          new_data_source_id: ObjectId)
                          -> Task {
        Task::Create {
            id: new_data_source_id,
            object: DataSource::new_object(new_data_source_id, self.proxy_ref.clone()),
        }
    }

    fn get_data_device(&mut self,
                       _this_object_id: ObjectId,
                       _bundle: &mut Bundle,
                       new_data_device_id: ObjectId,
                       _seat: ObjectId)
                       -> Task {
        Task::Create {
            id: new_data_device_id,
            object: DataDevice::new_object(new_data_device_id, self.proxy_ref.clone()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_data_source` object.
struct DataSource {
    proxy_ref: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl DataSource {
    /// Creates new `DataSource`.
    fn new(oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            let mut proxy = proxy_ref.borrow_mut();
            let transfer = Transfer::new();
            proxy.set_transfer(oid, transfer);
        }

        DataSource {
            proxy_ref: proxy_ref,
        }
    }

    fn new_object(oid: ObjectId, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_data_source::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_data_source::Interface for DataSource {
    fn offer(&mut self,
             this_object_id: ObjectId,
             _bundle: &mut Bundle,
             mime_type: String)
             -> Task {
        let mut proxy = self.proxy_ref.borrow_mut();
        if let Some(ref mut transfer) = proxy.get_transfer(this_object_id) {
            transfer.add_mime_type(mime_type);
            proxy.set_transfer(this_object_id, transfer.clone());
        }
        Task::None
    }

    fn destroy(&mut self,
               this_object_id: ObjectId,
               _bundle: &mut Bundle)
               -> Task {
        let mut proxy = self.proxy_ref.borrow_mut();
        proxy.remove_transfer(this_object_id);
        Task::Destroy { id: this_object_id }
    }

    fn set_actions(&mut self,
                   _this_object_id: ObjectId,
                   _bundle: &mut Bundle,
                   _dnd_actions: u32)
                   -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_data_device` object.
struct DataDevice {
    proxy_ref: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl DataDevice {
    /// Creates new `DataDevice`.
    fn new(oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        {
            let mut proxy = proxy_ref.borrow_mut();
            proxy.add_data_device_oid(oid);
        }

        DataDevice {
            proxy_ref: proxy_ref,
        }
    }

    fn new_object(oid: ObjectId, proxy_ref: ProxyRef) -> Box<Object> {
        let data_device = Self::new(oid, proxy_ref);
        Box::new(Handler::<_, wl_data_device::Dispatcher>::new(data_device))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_data_device::Interface for DataDevice {
    fn start_drag(&mut self,
                  _this_object_id: ObjectId,
                  _bundle: &mut Bundle,
                  _source: ObjectId,
                  _origin: ObjectId,
                  _icon: ObjectId,
                  _serial: u32)
                  -> Task {
        Task::None
    }

    fn set_selection(&mut self,
                     _this_object_id: ObjectId,
                     _bundle: &mut Bundle,
                     data_source_oid: ObjectId,
                     _serial: u32)
                     -> Task {
        let mut proxy = self.proxy_ref.borrow_mut();
        proxy.select_transfer(data_source_oid);
        Task::None
    }

    fn release(&mut self,
               this_object_id: ObjectId,
               _bundle: &mut Bundle)
               -> Task {
        let mut proxy = self.proxy_ref.borrow_mut();
        proxy.remove_data_device_oid(this_object_id);
        Task::Destroy { id: this_object_id }
    }
}

// -------------------------------------------------------------------------------------------------

/// Wayland `wl_data_offer` object.
pub struct DataOffer {
    proxy_ref: ProxyRef,
}

// -------------------------------------------------------------------------------------------------

impl DataOffer {
    /// Creates new `DataOffer`.
    fn new(_oid: ObjectId, proxy_ref: ProxyRef) -> Self {
        DataOffer {
            proxy_ref: proxy_ref,
        }
    }

    pub fn new_object(oid: ObjectId, proxy_ref: ProxyRef) -> Box<Object> {
        Box::new(Handler::<_, wl_data_offer::Dispatcher>::new(Self::new(oid, proxy_ref)))
    }
}

// -------------------------------------------------------------------------------------------------

impl wl_data_offer::Interface for DataOffer {
    fn accept(&mut self,
              _this_object_id: ObjectId,
              _bundle: &mut Bundle,
              _serial: u32,
              _mime_type: String)
              -> Task {
        Task::None
    }

    fn receive(&mut self,
               _this_object_id: ObjectId,
               _bundle: &mut Bundle,
               mime_type: String,
               fd: RawFd)
               -> Task {
        let mut proxy = self.proxy_ref.borrow_mut();
        proxy.request_transfer(mime_type, fd);
        Task::None
    }

    fn destroy(&mut self,
               this_object_id: ObjectId,
               _bundle: &mut Bundle)
               -> Task {
        Task::Destroy { id: this_object_id }
    }

    fn finish(&mut self,
              _this_object_id: ObjectId,
              _bundle: &mut Bundle)
              -> Task {
        Task::None
    }

    fn set_actions(&mut self,
                   _this_object_id: ObjectId,
                   _bundle: &mut Bundle,
                   _dnd_actions: u32,
                   _preferred_action: u32)
                   -> Task {
        Task::None
    }
}

// -------------------------------------------------------------------------------------------------
