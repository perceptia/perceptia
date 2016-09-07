// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-data-offer.h"

#include "wayland-facade.h"

#include "global-macros.h"
#include "utils-log.h"

//------------------------------------------------------------------------------

/// @todo Handle destruction of data offer.
void noia_wayland_data_offer_unbind(struct wl_resource* resource NOIA_UNUSED)
{
    LOG_NYIMP("Wayland: unbind data offer");
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: accept data offer.
void noia_wayland_data_offer_accept(struct wl_client* client     NOIA_UNUSED,
                                    struct wl_resource* resource NOIA_UNUSED,
                                    uint32_t serial,
                                    const char* mime_type)
{
    LOG_NYIMP("Wayland > accept data offer "
              "(serial: %u, mime type: '%s')",
              serial, mime_type);
}

//------------------------------------------------------------------------------

/// Wayland protocol: receive data offer.
void noia_wayland_data_offer_receive(struct wl_client* client NOIA_UNUSED,
                                     struct wl_resource* resource,
                                     const char* mime_type,
                                     int32_t fd)
{
    LOG_WAYL3("Wayland > receive data offer"
              "(mime type: '%s', fd: %d)",
              mime_type, fd);

    NoiaWaylandTransfer* transfer = wl_resource_get_user_data(resource);
    noia_wayland_facade_receive_data_offer(transfer, mime_type, fd);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: destroy data offer.
void noia_wayland_data_offer_destroy(struct wl_client* client NOIA_UNUSED,
                                     struct wl_resource* resource)
{
    LOG_NYIMP("Wayland > destroy data offer");
    wl_resource_destroy(resource);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: finish data offer.
void noia_wayland_data_offer_finish(struct wl_client* client     NOIA_UNUSED,
                                    struct wl_resource* resource NOIA_UNUSED)
{
    LOG_NYIMP("Wayland > finish data offer");
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: set data offer actions.
void noia_wayland_data_offer_set_actions
                                      (struct wl_client* client     NOIA_UNUSED,
                                       struct wl_resource* resource NOIA_UNUSED,
                                       uint32_t dnd_actions,
                                       uint32_t preferred_action)
{
    LOG_NYIMP("Wayland > set data offer actions "
              "(dnd actions: 0x%x, preferred action: 0x%x)",
              dnd_actions, preferred_action);
}

//------------------------------------------------------------------------------

static const struct wl_data_offer_interface scDataOfferImplementation = {
        noia_wayland_data_offer_accept,
        noia_wayland_data_offer_receive,
        noia_wayland_data_offer_destroy,
        noia_wayland_data_offer_finish,
        noia_wayland_data_offer_set_actions,
    };

//------------------------------------------------------------------------------

struct wl_resource* noia_wayland_data_offer_bind(struct wl_client* client,
                                                 void* data,
                                                 uint32_t version,
                                                 uint32_t id)
{
    LOG_WAYL2("Binding Wayland data offer (version: %d, id: %d)", version, id);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_data_offer_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return NULL);

    wl_resource_set_implementation(rc, &scDataOfferImplementation,
                                   data, noia_wayland_data_offer_unbind);
    return rc;
}

//------------------------------------------------------------------------------

