// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-xdg-popup.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

#include "xdg-shell-server-protocol.h"

//------------------------------------------------------------------------------

void noia_wayland_xdg_popup_unbind(struct wl_resource* resource NOIA_UNUSED)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_WAYL2("Wayland: unbind XDG shell popup (sid: %u)", sid);
}

//------------------------------------------------------------------------------

void noia_wayland_xdg_popup_destroy(struct wl_client* client NOIA_UNUSED,
                                    struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_NYIMP("Wayland > XDG popup destroy (sid: %u)", sid);
}

//------------------------------------------------------------------------------

const struct xdg_popup_interface scXdgPopupImplementation = {
        noia_wayland_xdg_popup_destroy,
    };

//------------------------------------------------------------------------------

void noia_wayland_xdg_popup_bind(struct wl_client* client,
                                 void* data,
                                 uint32_t version,
                                 uint32_t id)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) data;
    LOG_WAYL2("Binding XDG shell popup "
              "(version: %u, id: %u, sid: %u)", version, id, sid);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &xdg_popup_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scXdgPopupImplementation, data,
                                   noia_wayland_xdg_popup_unbind);

    noia_wayland_facade_add_general_resource(NOIA_RESOURCE_OTHER, rc);
}

//------------------------------------------------------------------------------

