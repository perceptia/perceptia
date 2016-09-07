// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-surface.h"
#include "utils-log.h"

#include <malloc.h>
#include <memory.h>

//------------------------------------------------------------------------------

struct NoiaWaylandSurfaceStruct {
    NoiaItem base;
    NoiaList* frame_resources;
    struct wl_resource* resources[NOIA_NUM_SURFACE_RESOURCE_TYPES];
};

//------------------------------------------------------------------------------

int noia_wayland_surface_compare_resources(struct wl_resource* rc1,
                                           struct wl_resource* rc2)
{
    return rc1 != rc2;
}

//------------------------------------------------------------------------------

NoiaWaylandSurface* noia_wayland_surface_new(void)
{
    NoiaWaylandSurface* self = calloc(1, sizeof(NoiaWaylandSurface));
    self->frame_resources = noia_list_new(NULL);
    return self;
}

//------------------------------------------------------------------------------

void noia_wayland_surface_free(NoiaWaylandSurface* self)
{
    NOIA_ENSURE(self, return);

    int len = noia_list_len(self->frame_resources);
    if (len > 2) {
        LOG_WARN1("Wayland: %d surface frame resources not released!", len);
    }
    noia_list_free(self->frame_resources);

    memset(self, 0, sizeof(NoiaWaylandSurface));
    free(self);
}

//------------------------------------------------------------------------------

struct wl_resource* noia_wayland_surface_get_resource
                                  (NoiaWaylandSurface* self,
                                   NoiaWaylandSurfaceResourceType resource_type)
{
    NOIA_ENSURE(self, return NULL);
    NOIA_ENSURE(resource_type < NOIA_NUM_SURFACE_RESOURCE_TYPES, return NULL);

    return self->resources[resource_type];
}

//------------------------------------------------------------------------------

NoiaList* noia_wayland_surface_get_frame_resources(NoiaWaylandSurface* self)
{
    NOIA_ENSURE(self, return NULL);
    return self->frame_resources;
}

//------------------------------------------------------------------------------

void noia_wayland_surface_add_resource
                                  (NoiaWaylandSurface* self,
                                   NoiaWaylandSurfaceResourceType resource_type,
                                   struct wl_resource* resource)
{
    NOIA_ENSURE(self, return);
    NOIA_ENSURE(resource_type < NOIA_NUM_SURFACE_RESOURCE_TYPES, return);

    if (resource_type == NOIA_RESOURCE_FRAME) {
        noia_list_append(self->frame_resources, resource);
    } else if (self->resources[resource_type]) {
        LOG_WAYL3("Wayland: surface resource of type '%d' "
                  "already here!", resource_type);
    }

    self->resources[resource_type] = resource;
}

//------------------------------------------------------------------------------

void noia_wayland_surface_remove_resource
                                  (NoiaWaylandSurface* self,
                                   NoiaWaylandSurfaceResourceType resource_type,
                                   struct wl_resource* resource)
{
    NOIA_ENSURE(self, return);
    NOIA_ENSURE(resource_type < NOIA_NUM_SURFACE_RESOURCE_TYPES, return);

    self->resources[resource_type] = NULL;

    if (resource_type == NOIA_RESOURCE_FRAME) {
        noia_list_remove(self->frame_resources, resource,
                      (NoiaCompareFunc) noia_wayland_surface_compare_resources);
        self->resources[resource_type] = noia_list_first(self->frame_resources);
    }
}

//------------------------------------------------------------------------------

