// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-xdg-surface.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

#include "xdg-shell-server-protocol.h"

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_unbind(struct wl_resource* resource NOIA_UNUSED)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_WAYL2("Wayland: unbind XDG shell surface (sid: %u)", sid);
    noia_wayland_facade_remove_surface_resource
                               (sid, NOIA_RESOURCE_XDG_SHELL_SURFACE, resource);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_destroy(struct wl_client* client NOIA_UNUSED,
                                      struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface destroy (sid: %u)", sid);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_set_parent(struct wl_client* client NOIA_UNUSED,
                                         struct wl_resource* surface_resource,
                                         struct wl_resource* parent_resource)
{
    NoiaSurfaceId popup_sid =
                    (NoiaSurfaceId) wl_resource_get_user_data(surface_resource);
    NoiaSurfaceId parent_sid = scInvalidSurfaceId;
    if (parent_resource) {
        parent_sid = (NoiaSurfaceId) wl_resource_get_user_data(parent_resource);
    }

    LOG_NYIMP("Wayland > XDG surface set parent "
              "(popup sid: %u, parent sid: %p)", popup_sid, parent_sid);

    if (parent_sid != scInvalidSurfaceId) {
        /// @todo When setting parent for XDG surface the same machanism as for
        ///       popus is used. This should be replaced with more inteligent
        ///       behavior.
        noia_wayland_facade_add_subsurface(popup_sid, parent_sid, 0, 0);
    }
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_set_title(struct wl_client* client NOIA_UNUSED,
                                        struct wl_resource* resource,
                                        const char* title)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface set title "
              "(sid: %u, title: '%s')", sid, title);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_set_app_id(struct wl_client* client NOIA_UNUSED,
                                         struct wl_resource* resource,
                                         const char* app_id)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface set app id "
              "(sid: %u, id: '%s')", sid, app_id);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_show_window_menu
                                 (struct wl_client* client          NOIA_UNUSED,
                                  struct wl_resource* resource,
                                  struct wl_resource* seat_resource NOIA_UNUSED,
                                  uint32_t serial,
                                  int32_t x,
                                  int32_t y)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface show window menu "
              "(sid: %u, serial: %u, x: %u, y: %u)", sid, serial, x, y);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_move
                                 (struct wl_client* client          NOIA_UNUSED,
                                  struct wl_resource* resource,
                                  struct wl_resource* seat_resource NOIA_UNUSED,
                                  uint32_t serial)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface move (sid: %u, serial: %u)", sid, serial);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_resize
                                 (struct wl_client* client          NOIA_UNUSED,
                                  struct wl_resource* resource,
                                  struct wl_resource* seat_resource NOIA_UNUSED,
                                  uint32_t serial,
                                  uint32_t edges)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface resize (sid: %u, serial: %u, edges: %u)",
              sid, serial, edges);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_ack_configure
                                          (struct wl_client* client NOIA_UNUSED,
                                           struct wl_resource* resource,
                                           uint32_t serial)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface ack configure "
              "(sid: %u, serial: %u)", sid, serial);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_set_window_geometry
                                          (struct wl_client* client NOIA_UNUSED,
                                           struct wl_resource* resource,
                                           int32_t x,
                                           int32_t y,
                                           int32_t width,
                                           int32_t height)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);

    LOG_WAYL3("Wayland > XDG surface set window geometry "
              "(sid: %u, x: %d, y: %d, w: %d, h: %d)",
               sid, x, y, width, height);

    NoiaSize size = {width, height};
    noia_wayland_facade_set_requested_size(sid, size);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_set_maximized
                                          (struct wl_client* client NOIA_UNUSED,
                                           struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface set maximized (sid: %u)", sid);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_unset_maximized
                                          (struct wl_client* client NOIA_UNUSED,
                                           struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface uset maximized (sid: %u)", sid);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_set_fullscreen
                               (struct wl_client* client            NOIA_UNUSED,
                                struct wl_resource* resource,
                                struct wl_resource* output_resource NOIA_UNUSED)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface set fullscreen (sid: %u)", sid);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_unset_fullscreen
                                          (struct wl_client* client NOIA_UNUSED,
                                           struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface unset fullscreen (sid: %u)", sid);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_set_minimized
                                          (struct wl_client* client NOIA_UNUSED,
                                           struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG surface set minimized (sid: %u)", sid);
}

//------------------------------------------------------------------------------

const struct xdg_surface_interface scXdgSurfaceImplementation = {
        noia_wayland_xdg_surface_destroy,
        noia_wayland_xdg_surface_set_parent,
        noia_wayland_xdg_surface_set_title,
        noia_wayland_xdg_surface_set_app_id,
        noia_wayland_xdg_surface_show_window_menu,
        noia_wayland_xdg_surface_move,
        noia_wayland_xdg_surface_resize,
        noia_wayland_xdg_surface_ack_configure,
        noia_wayland_xdg_surface_set_window_geometry,
        noia_wayland_xdg_surface_set_maximized,
        noia_wayland_xdg_surface_unset_maximized,
        noia_wayland_xdg_surface_set_fullscreen,
        noia_wayland_xdg_surface_unset_fullscreen,
        noia_wayland_xdg_surface_set_minimized
    };

//------------------------------------------------------------------------------

void noia_wayland_xdg_surface_bind(struct wl_client* client,
                                   void* data,
                                   uint32_t version,
                                   uint32_t id)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) data;
    LOG_WAYL2("Binding XDG shell surface "
              "(version: %u, id: %u, sid: %u)", version, id, sid);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &xdg_surface_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scXdgSurfaceImplementation, data,
                                   noia_wayland_xdg_surface_unbind);

    noia_wayland_facade_add_shell_surface
                           (sid, NOIA_RESOURCE_XDG_SHELL_SURFACE, rc);
}

//------------------------------------------------------------------------------

