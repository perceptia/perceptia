// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-shell-surface.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_unbind(struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_WAYL2("Wayland: unbind shell surface (sid: %u)", sid);
    noia_wayland_facade_remove_surface_resource
                                   (sid, NOIA_RESOURCE_SHELL_SURFACE, resource);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_pong(struct wl_client* client NOIA_UNUSED,
                                     struct wl_resource* resource,
                                     uint32_t serial)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > pong (sid: %u, serial: %d)", sid, serial);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_move
                                 (struct wl_client* client          NOIA_UNUSED,
                                  struct wl_resource* resource,
                                  struct wl_resource* seat_resource NOIA_UNUSED,
                                  uint32_t serial)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > move (sid: %u, serial: %u)", sid, serial);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_resize
                                 (struct wl_client* client          NOIA_UNUSED,
                                  struct wl_resource* resource,
                                  struct wl_resource* seat_resource NOIA_UNUSED,
                                  uint32_t serial,
                                  uint32_t edges)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > resize (sid: %u, serial: %u, edges: 0x%x)",
              sid, serial, edges);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_set_toplevel
                                      (struct wl_client* client NOIA_UNUSED,
                                       struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > set toplevel (sid: %u)", sid);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_set_transient
                               (struct wl_client* client            NOIA_UNUSED,
                                struct wl_resource* resource,
                                struct wl_resource* parent_resource NOIA_UNUSED,
                                int32_t x,
                                int32_t y,
                                uint32_t f)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > set transient (sid: %u, x: %d, y: %d, flags: 0x%x)",
              sid, x, y, f);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_set_fullscreen
                               (struct wl_client* client            NOIA_UNUSED,
                                struct wl_resource* resource,
                                uint32_t method,
                                uint32_t framerate,
                                struct wl_resource* output_resource NOIA_UNUSED)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > set fullscreen (sid: %u, method: %u, framerate: %u)",
              sid, method, framerate);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_set_popup
                               (struct wl_client* client            NOIA_UNUSED,
                                struct wl_resource* resource,
                                struct wl_resource* seat_resource   NOIA_UNUSED,
                                uint32_t serial,
                                struct wl_resource* parent_resource NOIA_UNUSED,
                                int32_t x,
                                int32_t y,
                                uint32_t flags)
{
    NoiaSurfaceId popup_sid =
                     (NoiaSurfaceId) wl_resource_get_user_data(resource);
    NoiaSurfaceId parent_sid =
                     (NoiaSurfaceId) wl_resource_get_user_data(parent_resource);

    LOG_NYIMP("Wayland > set popup (popup_sid: %u, parent_sid: %u, "
              "serial: %u, x: %d, y: %d, flags: 0x%x)",
              popup_sid, parent_sid, serial, x, y, flags);

    noia_wayland_facade_add_subsurface(popup_sid, parent_sid, x, y);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_set_maximized
                               (struct wl_client* client            NOIA_UNUSED,
                                struct wl_resource* resource,
                                struct wl_resource* output_resource NOIA_UNUSED)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > set maximized (sid: %u)", sid);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_set_title
                                      (struct wl_client* client NOIA_UNUSED,
                                       struct wl_resource* resource,
                                       const char* title)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > set title (sid: %u, title: '%s')", sid, title);
}

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_set_class
                                      (struct wl_client* client NOIA_UNUSED,
                                       struct wl_resource* resource,
                                       const char* class)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > set class (sid: %u, class: '%s')", sid, class);
}

//------------------------------------------------------------------------------

const struct wl_shell_surface_interface scShellSurfaceImplementation = {
        noia_wayland_shell_surface_pong,
        noia_wayland_shell_surface_move,
        noia_wayland_shell_surface_resize,
        noia_wayland_shell_surface_set_toplevel,
        noia_wayland_shell_surface_set_transient,
        noia_wayland_shell_surface_set_fullscreen,
        noia_wayland_shell_surface_set_popup,
        noia_wayland_shell_surface_set_maximized,
        noia_wayland_shell_surface_set_title,
        noia_wayland_shell_surface_set_class
    };

//------------------------------------------------------------------------------

void noia_wayland_shell_surface_bind(struct wl_client* client,
                                     void* data,
                                     uint32_t version,
                                     uint32_t id)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) data;
    LOG_WAYL2("Binding Wayland shell surface "
              "(version: %u, id: %u, sid: %u)", version, id, sid);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_shell_surface_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scShellSurfaceImplementation, data,
                                   noia_wayland_shell_surface_unbind);

    noia_wayland_facade_add_shell_surface(sid, NOIA_RESOURCE_SHELL_SURFACE, rc);
}

//------------------------------------------------------------------------------

