// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_REGION_H
#define NOIA_WAYLAND_REGION_H

#include "global-types.h"

typedef struct {
    NoiaItem base;
    NoiaPosition pos;
    NoiaSize size;
} NoiaWaylandRegion;

NoiaWaylandRegion* noia_wayland_region_new(void);

void noia_wayland_region_free(NoiaWaylandRegion* self);

void noia_wayland_region_inflate(NoiaWaylandRegion* self,
                                 int x, int y,
                                 int width, int height);

#endif // NOIA_WAYLAND_REGION_H

