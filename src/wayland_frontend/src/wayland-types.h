// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_TYPES_H
#define NOIA_WAYLAND_TYPES_H

typedef enum {
    NOIA_RESOURCE_SURFACE,
    NOIA_RESOURCE_BUFFER,
    NOIA_RESOURCE_FRAME,
    NOIA_RESOURCE_SHELL_SURFACE,
    NOIA_RESOURCE_XDG_SHELL_SURFACE,
    NOIA_NUM_SURFACE_RESOURCE_TYPES,
} NoiaWaylandSurfaceResourceType;

typedef enum {
    NOIA_RESOURCE_KEYBOARD,
    NOIA_RESOURCE_POINTER,
    NOIA_RESOURCE_DATA_DEVICE,
    NOIA_RESOURCE_OTHER,
    NOIA_NUM_GENERAL_RESOURCE_TYPES,
} NoiaWaylandGeneralResourceType;

#endif // NOIA_WAYLAND_TYPES_H

