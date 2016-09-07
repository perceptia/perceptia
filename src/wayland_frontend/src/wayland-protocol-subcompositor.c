// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-subcompositor.h"
#include "wayland-protocol-subsurface.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

//------------------------------------------------------------------------------

/// Handle destruction of subcompositor resource.
void noia_wayland_subcompositor_unbind(struct wl_resource* resource)
{
    LOG_WAYL3("Wayland: unbind subcompositor");
    noia_wayland_facade_remove_general_resource(NOIA_RESOURCE_OTHER, resource);
}

//------------------------------------------------------------------------------

/// Wayland protocol: Handle request for destroying subcompositor.
void noia_wayland_subcompositor_destroy
                                      (struct wl_client* client     NOIA_UNUSED,
                                       struct wl_resource* resource NOIA_UNUSED)
{
    LOG_NYIMP("Wayland: subcompositor destroy");
}

//------------------------------------------------------------------------------

/// Wayland protocol: Handle request subsurface object.
void noia_wayland_subcompositor_get_subsurface
                                          (struct wl_client* client,
                                           struct wl_resource* resource,
                                           uint32_t id,
                                           struct wl_resource* surface_resource,
                                           struct wl_resource* parent_resource)
{
    uint32_t version = wl_resource_get_version(resource);
    NoiaSurfaceId sid =
                    (NoiaSurfaceId) wl_resource_get_user_data(surface_resource);
    NoiaSurfaceId parent_sid =
                     (NoiaSurfaceId) wl_resource_get_user_data(parent_resource);

    LOG_WAYL3("Wayland > get subsurface "
              "(sid: %u, parent sid: %u)",
              sid, parent_sid);

    noia_wayland_subsurface_bind(client, (void*) sid, version, id);
    noia_wayland_facade_add_subsurface(sid, parent_sid, 0, 0);
}

//------------------------------------------------------------------------------

const struct wl_subcompositor_interface scSubcompositorImplementation = {
        noia_wayland_subcompositor_destroy,
        noia_wayland_subcompositor_get_subsurface
    };

//------------------------------------------------------------------------------

/// Wayland protocol: Handle request for interface to compositor object.
void noia_wayland_subcompositor_bind(struct wl_client* client,
                                     void* data,
                                     uint32_t version,
                                     uint32_t id)
{
    LOG_WAYL2("Binding Wayland subcompositor "
              "(version: %u, id: %u)", version, id);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_subcompositor_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scSubcompositorImplementation,
                                   data, noia_wayland_subcompositor_unbind);

    noia_wayland_facade_add_general_resource(NOIA_RESOURCE_OTHER, rc);
}

//------------------------------------------------------------------------------

