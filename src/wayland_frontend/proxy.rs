// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains common state between handlers belonging to one client.

// -------------------------------------------------------------------------------------------------

use std;
use std::collections::{HashMap, HashSet};

use dharma;
use skylane as wl;
use skylane_protocols::server::wayland::{wl_display, wl_callback, wl_buffer};
use skylane_protocols::server::xdg_shell_unstable_v6::{zxdg_toplevel_v6, zxdg_surface_v6};

use qualia::{Coordinator, MappedMemory, Milliseconds};
use qualia::{Area, Button, Key, Size, SurfacePosition, Vector};
use qualia::{MemoryPoolId, MemoryViewId, SurfaceId};
use qualia::{show_reason, surface_state};

use facade::{Facade, ShellSurfaceOid};
use gateway::Gateway;
use global::Global;
use mediator::MediatorRef;

// -------------------------------------------------------------------------------------------------

/// Helper macro for relating surface information with surface ID
macro_rules! relate_sid_with {
    ($member:ident, $dict:expr, $sid:ident, $obj:ident) => {
        if {
            if let Some(info) = $dict.get_mut(&$sid) {
                if info.$member.is_none() {
                    info.$member = Some($obj);
                } else {
                    log_warn3!("Already have {} for surface {}", stringify!($member), $sid);
                }
                false
            } else {
                true
            }
        } {
            let mut info = SurfaceInfo::new();
            info.$member = Some($obj);
            $dict.insert($sid, info);
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for aggregating information about surface.
struct SurfaceInfo {
    // For releasing buffer in `on_surface_frame`
    buffer_oid: Option<wl::common::ObjectId>,

    // For sending frame callback in `on_surface_frame`
    frame_oid: Option<wl::common::ObjectId>,

    // For send reconfiguration events in `on_surface_reconfigured`
    shell_surface_oid: Option<ShellSurfaceOid>,
}

// -------------------------------------------------------------------------------------------------

impl SurfaceInfo {
    pub fn new() -> Self {
        SurfaceInfo {
            shell_surface_oid: None,
            buffer_oid: None,
            frame_oid: None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for aggregating information about buffers.
#[derive(Clone, Copy)]
struct BufferInfo {
    mvid: MemoryViewId,
}

// -------------------------------------------------------------------------------------------------

impl BufferInfo {
    pub fn new(mvid: MemoryViewId) -> Self {
        BufferInfo { mvid: mvid }
    }
}

// -------------------------------------------------------------------------------------------------

/// `Proxy` holds information common between handlers of one client. It constitutes for them facade
/// for rest of the crate/application and gateway from `Engine` to clients.
///
/// For information about its place among other structures see crate-level documentation.
pub struct Proxy {
    id: dharma::EventHandlerId,
    coordinator: Coordinator,
    mediator: MediatorRef,
    socket: wl::server::ClientSocket,
    globals: HashMap<u32, Global>,
    regions: HashMap<wl::common::ObjectId, Area>,
    memory_pools: HashSet<MemoryPoolId>,
    surface_oid_to_sid_dictionary: HashMap<wl::common::ObjectId, SurfaceId>,
    sid_to_surface_info_dictionary: HashMap<SurfaceId, SurfaceInfo>,
    buffer_oid_to_buffer_info_dictionary: HashMap<wl::common::ObjectId, BufferInfo>,
    last_global_id: u32,
}

define_ref!(Proxy, ProxyRef);

// -------------------------------------------------------------------------------------------------

impl Proxy {
    /// Creates new `Proxy`.
    pub fn new(id: dharma::EventHandlerId,
               coordinator: Coordinator,
               mediator: MediatorRef,
               socket: wl::server::ClientSocket)
               -> Self {
        Proxy {
            id: id,
            coordinator: coordinator,
            mediator: mediator,
            socket: socket,
            globals: HashMap::new(),
            regions: HashMap::new(),
            memory_pools: HashSet::new(),
            surface_oid_to_sid_dictionary: HashMap::new(),
            sid_to_surface_info_dictionary: HashMap::new(),
            buffer_oid_to_buffer_info_dictionary: HashMap::new(),
            last_global_id: 0,
        }
    }

    /// Returns client connection socket.
    pub fn get_socket(&self) -> wl::server::ClientSocket {
        self.socket.clone()
    }

    /// Return list of current globals.
    pub fn get_globals(&self) -> &HashMap<u32, Global> {
        &self.globals
    }

    /// Registers new global.
    pub fn register_global(&mut self, mut global: Global) {
        self.last_global_id += 1;
        global.name = self.last_global_id;
        self.globals.insert(self.last_global_id, global);
    }

    /// Handles termination of client by destroing its resources.
    pub fn terminate(&mut self) {
        for mpid in self.memory_pools.iter() {
            self.coordinator.destroy_memory_pool(*mpid);
        }

        for (_, sid) in self.surface_oid_to_sid_dictionary.iter() {
            self.mediator.borrow_mut().remove(*sid);
            self.coordinator.destroy_surface(*sid);
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Proxy {
    /// Helper method for setting shell information for surface.
    fn relate_sid_with_shell_surface(&mut self, sid: SurfaceId, oid: ShellSurfaceOid) {
        relate_sid_with!(shell_surface_oid,
                         self.sid_to_surface_info_dictionary,
                         sid,
                         oid);
    }

    /// Helper method for setting buffer information for surface.
    fn relate_sid_with_buffer(&mut self, sid: SurfaceId, oid: wl::common::ObjectId) {
        relate_sid_with!(buffer_oid, self.sid_to_surface_info_dictionary, sid, oid);
    }

    /// Helper method for setting frame callback ID information for surface.
    fn relate_sid_with_frame(&mut self, sid: SurfaceId, oid: wl::common::ObjectId) {
        relate_sid_with!(frame_oid, self.sid_to_surface_info_dictionary, sid, oid);
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl Facade for Proxy {
    fn create_memory_pool(&mut self, memory: MappedMemory) -> MemoryPoolId {
        let mpid = self.coordinator.create_pool_from_memory(memory);
        self.memory_pools.insert(mpid);
        mpid
    }

    fn destroy_memory_pool(&mut self, mpid: MemoryPoolId) {
        self.memory_pools.remove(&mpid);
        self.coordinator.destroy_memory_pool(mpid);
    }

    fn create_memory_view(&mut self,
                          mpid: MemoryPoolId,
                          buffer_oid: wl::common::ObjectId,
                          offset: usize,
                          width: usize,
                          height: usize,
                          stride: usize)
                          -> Option<MemoryViewId> {
        let result = self.coordinator.create_memory_view(mpid, offset, width, height, stride);
        if let Some(mvid) = result {
            self.buffer_oid_to_buffer_info_dictionary.insert(buffer_oid, BufferInfo::new(mvid));
        }
        result
    }

    fn define_region(&mut self, region_id: wl::common::ObjectId, region: Area) {
        self.regions.insert(region_id, region);
    }

    fn undefine_region(&mut self, region_id: wl::common::ObjectId) {
        self.regions.remove(&region_id);
    }

    fn set_input_region(&self, sid: SurfaceId, region_id: wl::common::ObjectId) {
        if let Some(region) = self.regions.get(&region_id) {
            self.coordinator.set_surface_offset(sid, region.pos);
            self.coordinator.set_surface_requested_size(sid, region.size);
        } else {
            // TODO: Implement reseting input region.
        }
    }

    fn create_surface(&mut self, oid: wl::common::ObjectId) -> SurfaceId {
        let sid = self.coordinator.create_surface();
        self.surface_oid_to_sid_dictionary.insert(oid, sid);
        self.mediator.borrow_mut().relate_sid_to_client(sid, self.id);
        sid
    }

    fn destroy_surface(&self, sid: SurfaceId) {
        self.coordinator.destroy_surface(sid)
    }

    fn attach(&mut self, buffer_oid: wl::common::ObjectId, sid: SurfaceId, x: i32, y: i32) {
        if let Some(&info) = self.buffer_oid_to_buffer_info_dictionary.get(&buffer_oid) {
            self.relate_sid_with_buffer(sid, buffer_oid);
            self.coordinator.attach(info.mvid, sid);
        } else {
            log_error!("Unknown buffer object ID: {}", buffer_oid);
        }
    }

    fn commit(&self, sid: SurfaceId) {
        self.coordinator.commit_surface(sid);
    }

    fn set_frame(&mut self, sid: SurfaceId, frame_oid: wl::common::ObjectId) {
        self.relate_sid_with_frame(sid, frame_oid);
    }

    fn show(&mut self,
            surface_oid: wl::common::ObjectId,
            shell_surface_oid: ShellSurfaceOid,
            reason: show_reason::ShowReason) {
        if let Some(&sid) = self.surface_oid_to_sid_dictionary.get(&surface_oid) {
            self.relate_sid_with_shell_surface(sid, shell_surface_oid);
            self.coordinator.show_surface(sid, reason);
        } else {
            log_error!("Unknown surface object ID: {}", surface_oid);
        }
    }

    fn set_offset(&self, sid: SurfaceId, offset: Vector) {
        self.coordinator.set_surface_offset(sid, offset);
    }

    fn set_requested_size(&self, sid: SurfaceId, size: Size) {
        self.coordinator.set_surface_requested_size(sid, size);
    }

    fn set_relative_position(&self, sid: SurfaceId, offset: Vector) {
        self.coordinator.set_surface_relative_position(sid, offset);
    }

    fn relate(&self, sid: SurfaceId, parent_sid: SurfaceId) {
        self.coordinator.relate_surfaces(sid, parent_sid);
    }

    fn set_as_cursor(&self, sid: SurfaceId) {
        self.coordinator.set_surface_as_cursor(sid);
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
impl Gateway for Proxy {
    fn on_output_found(&self) {}

    fn on_keyboard_input(&self, key: Key) {}

    fn on_pointer_button(&self, btn: Button) {}

    fn on_pointer_axis(&self, axis: Vector) {}

    fn on_surface_frame(&mut self, sid: SurfaceId, milliseconds: Milliseconds) {
        if let Some(info) = self.sid_to_surface_info_dictionary.get_mut(&sid) {
            if let Some(frame_oid) = info.frame_oid {
                send!(wl_callback::done(&mut self.socket,
                                        frame_oid,
                                        milliseconds.get_value() as u32));
                send!(wl_display::delete_id(&mut self.socket,
                                            wl::common::DISPLAY_ID,
                                            frame_oid.get_value()));
            }
            info.frame_oid = None;

            if let Some(buffer_oid) = info.buffer_oid {
                send!(wl_buffer::release(&mut self.socket, buffer_oid));
            }
            info.buffer_oid = None;
        }
    }

    fn on_pointer_focus_changed(&self, surface_position: SurfacePosition) {}

    fn on_pointer_relative_motion(&self, surface_position: SurfacePosition) {}

    fn on_keyboard_focus_changed(&self, old_sid: SurfaceId, new_sid: SurfaceId) {}

    fn on_surface_reconfigured(&mut self,
                               sid: SurfaceId,
                               size: Size,
                               state_flags: surface_state::SurfaceState) {
        if let Some(info) = self.sid_to_surface_info_dictionary.get(&sid) {
            if let Some(surface) = info.shell_surface_oid {
                match surface {
                    ShellSurfaceOid::Shell(oid) => {
                        // TODO: Finish implementation of Wayland shell protocol.
                        log_nyimp!("Wayland shell protocol is not implemented yet");
                    }
                    ShellSurfaceOid::ZxdgToplevelV6(shell_surface_oid, shell_toplevel_oid) => {
                        let mut pos = 0;
                        let mut states: [u8; 2] = [0; 2];
                        if state_flags.intersects(surface_state::MAXIMIZED) {
                            states[pos] = zxdg_toplevel_v6::state::MAXIMIZED as u8;
                            pos += 1;
                        }
                        if sid == self.coordinator.get_keyboard_focused_sid() {
                            states[pos] = zxdg_toplevel_v6::state::ACTIVATED as u8;
                            pos += 1;
                        }
                        send!(zxdg_toplevel_v6::configure(&mut self.socket,
                                                          shell_toplevel_oid,
                                                          size.width as i32,
                                                          size.height as i32,
                                                          &states[..pos]));
                        let serial = self.socket.get_next_serial();
                        send!(zxdg_surface_v6::configure(&mut self.socket,
                                                         shell_surface_oid,
                                                         serial));
                    }
                }
            } else {
                log_warn3!("Received reconfiguration request for surface {:?} \
                           which is not in shell",
                           sid);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Drop for Proxy {
    fn drop(&mut self) {
        self.terminate();
    }
}

// -------------------------------------------------------------------------------------------------
