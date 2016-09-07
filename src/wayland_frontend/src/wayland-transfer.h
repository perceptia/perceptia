// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_TRANSFER_H
#define NOIA_WAYLAND_TRANSFER_H

#include "utils-list.h"

#include <wayland-server.h>

/// @file
/// Transfer is used to exchange data between clients.

/// Structure used for storing data source offers.
typedef struct NoiaWaylandTransferStruct NoiaWaylandTransfer;

/// Wayland data transfer constructor.
NoiaWaylandTransfer* noia_wayland_transfer_create(struct wl_resource* rc);

/// Wayland data transfer destructor.
void noia_wayland_transfer_destroy(NoiaWaylandTransfer* self);

/// Add data type offer.
void noia_wayland_transfer_add_offer(NoiaWaylandTransfer* self,
                                     const char* type);

/// Get resource of data source.
struct wl_resource* noia_wayland_transfer_get_rc(NoiaWaylandTransfer* self);

/// Get list of mime types associated with the transfer.
NoiaList* noia_wayland_transfer_get_mimetypes(NoiaWaylandTransfer* self);

#endif // NOIA_WAYLAND_TRANSFER_H

