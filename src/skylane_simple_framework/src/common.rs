// Copyright 2017 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Functions implementing requests for server common between handlers and `Controller`.
//!
//! TODO: All requests to server should be moved here.
//! FIXME: To avoid duplications trait implemented by both `Controller` and `Bundle` should be
//! introduced in `skylane`.

use skylane::client as wl;
use skylane_protocols::client::wayland::{wl_compositor, wl_surface};
use skylane_protocols::client::wayland::{wl_shell, wl_shell_surface};

use proxy::ProxyRef;
use protocol::{compositor, shell};

// -------------------------------------------------------------------------------------------------

/// Requests creation of surface and shows it in shell.
pub fn create_shell_surface(connection_controller: &mut wl::Controller,
                            proxy: ProxyRef,
                            compositor_oid: wl::ObjectId,
                            shell_oid: wl::ObjectId,
                            buffer_oid: wl::ObjectId,
                            width: usize,
                            height: usize) {
    // Create surface
    let surface_oid = connection_controller.get_next_available_client_object_id();
    let surface_object = compositor::Surface::new_object(proxy.clone());
    connection_controller.add_object(surface_oid, surface_object);

    send!(wl_compositor::create_surface(&connection_controller.get_socket(),
                                        compositor_oid,
                                        surface_oid));

    // Create shell surface
    let shell_surface_oid = connection_controller.get_next_available_client_object_id();
    let shell_surface_object = shell::ShellSurface::new_object(proxy.clone());
    connection_controller.add_object(shell_surface_oid, shell_surface_object);

    send!(wl_shell::get_shell_surface(&connection_controller.get_socket(),
                                      shell_oid,
                                      shell_surface_oid,
                                      surface_oid));

    // Perform first redraw
    send!(wl_surface::attach(&connection_controller.get_socket(),
                             surface_oid,
                             buffer_oid,
                             width as i32,
                             height as i32));

    send!(wl_surface::commit(&connection_controller.get_socket(),
                             surface_oid));

    // Show surface
    send!(wl_shell_surface::set_toplevel(&connection_controller.get_socket(),
                                         shell_surface_oid));
}

/// Requests creation of surface and shows it in shell.
pub fn create_shell_surface2(connection_controller: &mut wl::Bundle,
                            proxy: ProxyRef,
                            compositor_oid: wl::ObjectId,
                            shell_oid: wl::ObjectId,
                            buffer_oid: wl::ObjectId,
                            width: usize,
                            height: usize) {
    // Create surface
    let surface_oid = connection_controller.get_next_available_client_object_id();
    let surface_object = compositor::Surface::new_object(proxy.clone());
    connection_controller.add_object(surface_oid, surface_object);

    send!(wl_compositor::create_surface(&connection_controller.get_socket(),
                                        compositor_oid,
                                        surface_oid));

    // Create shell surface
    let shell_surface_oid = connection_controller.get_next_available_client_object_id();
    let shell_surface_object = shell::ShellSurface::new_object(proxy.clone());
    connection_controller.add_object(shell_surface_oid, shell_surface_object);

    send!(wl_shell::get_shell_surface(&connection_controller.get_socket(),
                                      shell_oid,
                                      shell_surface_oid,
                                      surface_oid));

    // Perform first redraw
    send!(wl_surface::attach(&connection_controller.get_socket(),
                             surface_oid,
                             buffer_oid,
                             width as i32,
                             height as i32));

    send!(wl_surface::commit(&connection_controller.get_socket(),
                             surface_oid));

    // Show surface
    send!(wl_shell_surface::set_toplevel(&connection_controller.get_socket(),
                                         shell_surface_oid));
}

// -------------------------------------------------------------------------------------------------
