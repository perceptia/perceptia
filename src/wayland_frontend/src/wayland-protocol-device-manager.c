// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-device-manager.h"
#include "wayland-protocol-data-source.h"
#include "wayland-protocol-data-device.h"

#include "utils-log.h"
#include "global-macros.h"

//------------------------------------------------------------------------------

/// @todo Handle destruction of device manager.
void noia_wayland_device_manager_unbind
                                      (struct wl_resource* resource NOIA_UNUSED)
{
    LOG_NYIMP("Wayland: unbind device manager");
}

//------------------------------------------------------------------------------

/// Wayland protocol: create data source.
void noia_wayland_create_data_source(struct wl_client* client,
                                     struct wl_resource* resource NOIA_UNUSED,
                                     uint32_t id)
{
    LOG_WAYL2("Wayland > create data source");

    noia_wayland_data_source_bind(client, NULL, 1, id);
}

//------------------------------------------------------------------------------

/// Wayland protocol: get data device.
void noia_wayland_get_data_device(struct wl_client* client,
                                  struct wl_resource* manager_resource,
                                  uint32_t id,
                                  struct wl_resource* seat_resource NOIA_UNUSED)
{
    LOG_WAYL2("Wayland > get data device");

    uint32_t version = wl_resource_get_version(manager_resource);
    noia_wayland_data_device_bind(client, NULL, version, id);
}

//------------------------------------------------------------------------------

static const struct wl_data_device_manager_interface scManagerImplementation = {
        noia_wayland_create_data_source,
        noia_wayland_get_data_device
    };

//------------------------------------------------------------------------------

void noia_wayland_device_manager_bind(struct wl_client* client,
                                      void* data,
                                      uint32_t version,
                                      uint32_t id)
{
    LOG_WAYL2("Binding Wayland device manager "
              "(version: %d, id: %d)", version, id);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_data_device_manager_interface,
                            version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scManagerImplementation,
                                   data, noia_wayland_device_manager_unbind);
}

//------------------------------------------------------------------------------

