// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_PROTOCOL_POINTER_H
#define NOIA_WAYLAND_PROTOCOL_POINTER_H

#include <wayland-server.h>

void noia_wayland_pointer_bind(struct wl_client* client,
                               void* data,
                               uint32_t version,
                               uint32_t id);

#endif // NOIA_WAYLAND_PROTOCOL_POINTER_H

