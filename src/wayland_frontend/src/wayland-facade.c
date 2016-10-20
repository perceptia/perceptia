// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-facade.h"
#include "wayland-gateway.h"
#include "wayland-transfer.h"

#include <unistd.h>

static NoiaWaylandContext* C;

//------------------------------------------------------------------------------

void noia_wayland_facade_initialize(NoiaWaylandContext* context)
{
    C = context;
}

//------------------------------------------------------------------------------

void noia_wayland_facade_finalize() {}

//------------------------------------------------------------------------------

NoiaSurfaceId noia_wayland_facade_create_surface()
{
    return noia_surface_create(C->coordinator);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_commit(NoiaSurfaceId sid)
{
    noia_surface_commit(C->coordinator, sid);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_set_offset(NoiaSurfaceId sid, NoiaPosition pos)
{
    noia_surface_set_offset(C->coordinator, sid, pos);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_set_requested_size(NoiaSurfaceId sid, NoiaSize size)
{
    noia_surface_set_requested_size(C->coordinator, sid, size);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_set_subsurface_position(NoiaSurfaceId sid,
                                                 int x, int y)
{
    NoiaPosition pos = {x, y};
    noia_surface_set_relative_position(C->coordinator, sid, pos);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_set_cursor(int serial NOIA_UNUSED,
                                    int hotspot_x,
                                    int hotspot_y,
                                    NoiaSurfaceId sid)
{
    NoiaPosition hotspot = {hotspot_x, hotspot_y};
    noia_surface_set_offset(C->coordinator, sid, hotspot);
    noia_surface_set_as_cursor(C->coordinator, sid);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_add_surface_resource
                                        (NoiaSurfaceId sid,
                                         NoiaWaylandSurfaceResourceType rc_type,
                                         struct wl_resource* rc)
{
    noia_wayland_cache_lock(C->cache);
    noia_wayland_cache_add_surface_resource(C->cache, sid, rc_type, rc);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_add_general_resource
                                        (NoiaWaylandGeneralResourceType rc_type,
                                         struct wl_resource* rc)
{
    noia_wayland_cache_lock(C->cache);
    noia_wayland_cache_add_general_resource(C->cache, rc_type, rc);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_remove_surface_resource
                                        (NoiaSurfaceId sid,
                                         NoiaWaylandSurfaceResourceType rc_type,
                                         struct wl_resource* rc)
{
    noia_wayland_cache_lock(C->cache);
    noia_wayland_cache_remove_surface_resource(C->cache, sid, rc_type, rc);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_remove_general_resource
                                        (NoiaWaylandGeneralResourceType rc_type,
                                         struct wl_resource* rc)
{
    noia_wayland_cache_lock(C->cache);
    noia_wayland_cache_remove_general_resource(C->cache, rc_type, rc);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_add_surface(NoiaSurfaceId sid, struct wl_resource* rc)
{
    noia_wayland_cache_lock(C->cache);
    noia_wayland_cache_create_surface(C->cache, sid);
    noia_wayland_cache_add_surface_resource
                       (C->cache, sid, NOIA_RESOURCE_SURFACE, rc);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_add_shell_surface(NoiaSurfaceId sid,
                                           NoiaWaylandSurfaceResourceType type,
                                           struct wl_resource* rc)
{
    noia_wayland_cache_lock(C->cache);
    noia_wayland_cache_add_surface_resource(C->cache, sid, type, rc);
    noia_surface_show(C->coordinator, sid, NOIA_SURFACE_SHOW_IN_SHELL);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_add_subsurface(NoiaSurfaceId sid,
                                        NoiaSurfaceId parent_sid,
                                        int x, int y)
{
    noia_surface_relate(C->coordinator, sid, parent_sid);
    noia_wayland_facade_set_subsurface_position(sid, x, y);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_surface_attach(NoiaSurfaceId sid,
                                        struct wl_resource* rc,
                                        struct wl_resource* brc,
                                        int width,
                                        int height,
                                        int stride,
                                        uint8_t* data)
{
    noia_wayland_cache_lock(C->cache);
    noia_wayland_cache_add_surface_resource
                       (C->cache, sid, NOIA_RESOURCE_BUFFER, brc);
    noia_surface_attach(C->coordinator, sid, width, height, stride, data, rc);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_remove_surface(NoiaSurfaceId sid,
                                        struct wl_resource* rc)
{
    noia_wayland_cache_lock(C->cache);
    noia_surface_destroy(C->coordinator, sid);
    noia_wayland_cache_remove_surface_resource
                       (C->cache, sid, NOIA_RESOURCE_SURFACE, rc);
    noia_wayland_cache_remove_surface(C->cache, sid);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_reorder_satellites(NoiaSurfaceId sid,
                                            NoiaSurfaceId sibling_sid,
                                            bool above)
{
// FIXME
    /*NoiaSurfaceData* surface = noia_surface_get(C->coordinator, sid);
    NOIA_ENSURE(surface, return);

    NoiaSurfaceData* parent =
                          noia_surface_get(C->coordinator, surface->parent_sid);
    NOIA_ENSURE(parent, return);

    NoiaLink* surface_link = NULL;
    NoiaLink* sibling_link = NULL;
    FOR_EACH(parent->satellites, link) {
        NoiaSurfaceId satellite_sid = (NoiaSurfaceId) link->data;
        if (satellite_sid == sibling_sid) {
            sibling_link = link;
        } else if (satellite_sid == sid) {
            surface_link = link;
        }
    }

    NOIA_ENSURE(surface_link and sibling_link, return);

    NoiaChain* satellites = &parent->satellites->base;
    noia_chain_disjoin(satellites, surface_link);
    if (above) {
        noia_chain_adjoin_onto(satellites, surface_link, sibling_link);
    } else {
        noia_chain_prejoin_onto(satellites, surface_link, sibling_link);
    }*/
}

//------------------------------------------------------------------------------

NoiaItemId noia_wayland_facade_create_region()
{
    noia_wayland_cache_lock(C->cache);
    NoiaItemId result = noia_wayland_cache_create_region(C->cache);
    noia_wayland_cache_unlock(C->cache);
    return result;
}

//------------------------------------------------------------------------------

void noia_wayland_facade_inflate_region(NoiaItemId rid,
                                        int x, int y,
                                        int width, int height)
{
    noia_wayland_cache_lock(C->cache);
    NoiaWaylandRegion* region = noia_wayland_cache_find_region(C->cache, rid);
    noia_wayland_region_inflate(region, x, y, width, height);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_set_input_region(NoiaSurfaceId sid, NoiaItemId rid)
{
    noia_wayland_cache_lock(C->cache);
    NoiaWaylandRegion* region = noia_wayland_cache_find_region(C->cache, rid);
    if (region) {
        noia_surface_set_offset(C->coordinator, sid, region->pos);
        noia_surface_set_requested_size(C->coordinator, sid, region->size);
    } else {
        noia_surface_reset_offset_and_requested_size(C->coordinator, sid);
    }
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_remove_region(NoiaItemId rid)
{
    noia_wayland_cache_lock(C->cache);
    noia_wayland_cache_remove_region(C->cache, rid);
    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_add_keyboard_resource(struct wl_resource* rc)
{
    noia_wayland_cache_lock(C->cache);

    // Store new resource
    noia_wayland_cache_add_general_resource
                       (C->cache, NOIA_RESOURCE_KEYBOARD, rc);

    // If client is focused, send enter event
    NoiaWaylandRc focused =
              noia_wayland_cache_get_rc_for_sid(C->cache,
                                                C->state->keyboard_focused_sid);

    struct wl_client* rc_client = wl_resource_get_client(rc);
    if (focused.cl and rc_client and (rc_client == focused.cl)) {
        struct wl_array array;
        wl_array_init(&array);
        int serial = noia_wayland_engine_next_serial(C->engine);
        wl_keyboard_send_enter(rc, serial, focused.rc, &array);
    }

    noia_wayland_cache_unlock(C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_create_transfer(struct wl_resource* rc)
{
    NoiaWaylandTransfer* transfer = noia_wayland_transfer_create(rc);
    wl_resource_set_user_data(rc, (void*) transfer);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_destroy_transfer(NoiaWaylandTransfer* transfer)
{
    noia_wayland_transfer_destroy(transfer);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_add_mime_type(NoiaWaylandTransfer* transfer,
                                       const char* mime_type)
{
    noia_wayland_transfer_add_offer(transfer, mime_type);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_send_selection(NoiaWaylandTransfer* transfer)
{
    C->state->current_transfer = transfer;
    noia_wayland_gateway_send_selection(C->state, C->cache);
}

//------------------------------------------------------------------------------

void noia_wayland_facade_receive_data_offer(NoiaWaylandTransfer* transfer,
                                            const char* mime_type,
                                            int fd)
{
    struct wl_resource* data_source_rc = noia_wayland_transfer_get_rc(transfer);
    wl_data_source_send_send(data_source_rc, mime_type, fd);
    close(fd);
}

//------------------------------------------------------------------------------

NoiaKeymapSettings noia_wayland_facade_get_keymap_settings()
{
    return C->keymap_settings;
}

//------------------------------------------------------------------------------
