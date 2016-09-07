// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_FACADE_H
#define NOIA_WAYLAND_FACADE_H

#include "wayland-context.h"
#include "wayland-transfer.h"

//------------------------------------------------------------------------------

/// @file
/// Wayland Gateway is set of functions used to react on request from clients.
///
/// @see NoiaWaylandState, NoiaWaylandCache, NoiaWaylandEngine,
///      wayland-facade.h

//------------------------------------------------------------------------------

/// Before call to this function facade is invalid.
void noia_wayland_facade_initialize(NoiaWaylandContext* context);

/// After call to this function whole memory is free and facade is invalid.
void noia_wayland_facade_finalize();

//------------------------------------------------------------------------------

/// Tell coordinator to create new surface.
NoiaSurfaceId noia_wayland_facade_create_surface();

/// Tell coordinator surface was commited.
void noia_wayland_facade_commit(NoiaSurfaceId sid);

/// Set offset of a given surface.
void noia_wayland_facade_set_offset(NoiaSurfaceId sid, NoiaPosition pos);

/// Set size of a given surface.
void noia_wayland_facade_set_requested_size(NoiaSurfaceId sid, NoiaSize size);

/// Set position of a given surface (used by subsurfaces).
void noia_wayland_facade_set_subsurface_position(NoiaSurfaceId sid,
                                                 int x, int y);

/// Tell coordinator to use given surface as cursor image.
void noia_wayland_facade_set_cursor(int serial,
                                    int hotspot_x,
                                    int hotspot_y,
                                    NoiaSurfaceId sid);

//------------------------------------------------------------------------------

/// Cache surface resource.
void noia_wayland_facade_add_surface_resource
                                        (NoiaSurfaceId sid,
                                         NoiaWaylandSurfaceResourceType rc_type,
                                         struct wl_resource* rc);

/// Cache general resource.
void noia_wayland_facade_add_general_resource
                                        (NoiaWaylandGeneralResourceType rc_type,
                                         struct wl_resource* rc);

/// Remove surface resource from cache.
void noia_wayland_facade_remove_surface_resource
                                        (NoiaSurfaceId sid,
                                         NoiaWaylandSurfaceResourceType rc_type,
                                         struct wl_resource* rc);

/// Remove general resource from cache.
void noia_wayland_facade_remove_general_resource
                                        (NoiaWaylandGeneralResourceType rc_type,
                                         struct wl_resource* rc);

//------------------------------------------------------------------------------

/// Create surface in cache.
void noia_wayland_facade_add_surface(NoiaItemId sid, struct wl_resource* rc);

/// Store surface resource and tell coordinator it can be displayed.
void noia_wayland_facade_add_shell_surface(NoiaSurfaceId sid,
                                           NoiaWaylandSurfaceResourceType type,
                                           struct wl_resource* rc);

/// Tell coordinator surface defined by `sid` is subsurface of `parent_sid`.
void noia_wayland_facade_add_subsurface(NoiaSurfaceId sid,
                                        NoiaSurfaceId parent_sid,
                                        int x, int y);

/// Cache buffer resource and tell coordinator what surface data should be used
/// after commit.
void noia_wayland_facade_surface_attach(NoiaSurfaceId sid,
                                        struct wl_resource* rc,
                                        struct wl_resource* brc,
                                        int width,
                                        int height,
                                        int stride,
                                        uint8_t* buffer);

/// Tell coordinator to destroy given surface and remove it from cache.
void noia_wayland_facade_remove_surface(NoiaSurfaceId sid,
                                        struct wl_resource* rc);

/// Reorder satellite surfaces.
/// If `above` is `true` surface with `sid` will be placed just above surface
/// with sid `sibling_sid`, otherwise it will be placed just below.
void noia_wayland_facade_reorder_satellites(NoiaSurfaceId sid,
                                            NoiaSurfaceId sibling_sid,
                                            bool above);

//------------------------------------------------------------------------------

/// Create new region.
NoiaItemId noia_wayland_facade_create_region();

/// Inflate given region to make it contain given area.
void noia_wayland_facade_inflate_region(NoiaItemId rid,
                                        int x, int y,
                                        int width, int height);

/// Tell coordinator about given surfaces size and offset basing on given region
void noia_wayland_facade_set_input_region(NoiaSurfaceId sid, NoiaItemId rid);

/// Remove region from cache.
void noia_wayland_facade_remove_region(NoiaItemId rid);

//------------------------------------------------------------------------------

/// Add given keyboard resource to cache and send enter event if it is focused.
void noia_wayland_facade_add_keyboard_resource(struct wl_resource* rc);

//------------------------------------------------------------------------------

/// Create new data transfer for exchanging data between clients.
void noia_wayland_facade_create_transfer(struct wl_resource* rc);

/// Destroy data transfer.
void noia_wayland_facade_destroy_transfer(NoiaWaylandTransfer* transfer);

/// Add mime type to data transfer.
void noia_wayland_facade_add_mime_type(NoiaWaylandTransfer* transfer,
                                       const char* mime_type);

/// Inform focused client about current selection.
void noia_wayland_facade_send_selection(NoiaWaylandTransfer* transfer);

/// Send current clipboard contents to requesting client.
void noia_wayland_facade_receive_data_offer(NoiaWaylandTransfer* transfer,
                                            const char* mime_type,
                                            int fd);

//------------------------------------------------------------------------------

#endif // NOIA_WAYLAND_FACADE_H

