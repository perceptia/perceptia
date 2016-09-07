// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-cache.h"

#include "utils-log.h"
#include "utils-store.h"

#include <pthread.h>

//------------------------------------------------------------------------------

struct NoiaWaylandCacheStruct {
    NoiaStore* surfaces;
    NoiaStore* regions;
    NoiaList* general_resource[NOIA_NUM_GENERAL_RESOURCE_TYPES];
    pthread_mutex_t mutex;
};

//------------------------------------------------------------------------------
// PRIVATE

int noia_wayland_cache_compare_resources(struct wl_resource* rc1,
                                         struct wl_resource* rc2)
{
    return rc1 != rc2;
}

//------------------------------------------------------------------------------
// PUBLIC

NoiaWaylandCache* noia_wayland_cache_new(void)
{
    NoiaWaylandCache* self = calloc(1, sizeof(*self));

    self->regions = noia_store_new_for_id();
    self->surfaces = noia_store_new_for_id();

    for (int type = 0; type < NOIA_NUM_GENERAL_RESOURCE_TYPES; ++type) {
        self->general_resource[type] = noia_list_new(NULL);
    }

    return self;
}

//------------------------------------------------------------------------------

void noia_wayland_cache_initialize(NoiaWaylandCache* self)
{
    NOIA_ENSURE(self, return);
    pthread_mutex_init(&self->mutex, NULL);
}

//------------------------------------------------------------------------------

void noia_wayland_cache_finalize(NoiaWaylandCache* self)
{
    NOIA_ENSURE(self, return);

    for (int type = 0; type < NOIA_NUM_GENERAL_RESOURCE_TYPES; ++type) {
        int len = noia_list_len(self->general_resource[type]);
        if (len > 0) {
            LOG_WARN1("Wayland: %d general resources of type '%d' "
                      "were not released!", len, type);
        }
        noia_list_free(self->general_resource[type]);
    }

    if (self->surfaces) {
        noia_store_free_with_items(self->surfaces,
                                  (NoiaFreeFunc) noia_wayland_surface_free);
        self->surfaces = NULL;
    }

    if (self->regions) {
        noia_store_free_with_items(self->regions,
                                  (NoiaFreeFunc) noia_wayland_region_free);
        self->regions = NULL;
    }
}

//------------------------------------------------------------------------------

void noia_wayland_cache_free(NoiaWaylandCache* self)
{
    NOIA_ENSURE(self, return);
    free(self);
}

//------------------------------------------------------------------------------

void noia_wayland_cache_lock(NoiaWaylandCache* self)
{
    LOG_MUTEX("Locking Wayland cache mutex");
    pthread_mutex_lock(&self->mutex);
}

//------------------------------------------------------------------------------

void noia_wayland_cache_unlock(NoiaWaylandCache* self)
{
    pthread_mutex_unlock(&self->mutex);
    LOG_MUTEX("Unlocked Wayland cache mutex");
}

//------------------------------------------------------------------------------

void noia_wayland_cache_create_surface(NoiaWaylandCache* self,
                                       NoiaSurfaceId sid)
{
    if (sid != scInvalidSurfaceId) {
        LOG_WAYL1("Wayland: creating surface (sid: %d)", sid);
        noia_store_add(self->surfaces, sid, noia_wayland_surface_new());
    }
}

//------------------------------------------------------------------------------

void noia_wayland_cache_remove_surface(NoiaWaylandCache* self,
                                       NoiaSurfaceId sid)
{
    if (sid != scInvalidSurfaceId) {
        LOG_WAYL1("Wayland: removing surface (sid: %d)", sid);
        noia_wayland_surface_free(noia_store_delete(self->surfaces, sid));
    }
}

//------------------------------------------------------------------------------

NoiaWaylandSurface* noia_wayland_cache_find_surface(NoiaWaylandCache* self,
                                                    NoiaSurfaceId sid)
{
    NoiaWaylandSurface* result = NULL;
    if (sid != scInvalidSurfaceId) {
        result = noia_store_find(self->surfaces, sid);
    }
    if (result == NULL) {
        LOG_ERROR("Wayland: Could not find surface (id: '%u')", sid);
    }
    return result;
}

//------------------------------------------------------------------------------

NoiaItemId noia_wayland_cache_create_region(NoiaWaylandCache* self)
{
    NoiaWaylandRegion* region = noia_wayland_region_new();
    NoiaItemId rid = noia_store_generate_new_id(self->regions);
    LOG_WAYL3("Wayland: creating region (rid: %d)", rid);
    noia_store_add(self->regions, rid, region);
    return rid;
}

//------------------------------------------------------------------------------

NoiaWaylandRegion* noia_wayland_cache_find_region(NoiaWaylandCache* self,
                                                  NoiaItemId rid)
{
    NoiaWaylandRegion* result = NULL;
    if (rid != scInvalidItemId) {
        result = noia_store_find(self->regions, rid);
    }
    if (result == NULL) {
        LOG_ERROR("Wayland: Could not find region (id: '%u')", rid);
    }
    return result;
}

//------------------------------------------------------------------------------

void noia_wayland_cache_remove_region(NoiaWaylandCache* self NOIA_UNUSED,
                                      NoiaItemId rid)
{
    if (rid != scInvalidItemId) {
        LOG_WAYL3("Wayland: removing region (rid: %d)", rid);
        noia_wayland_region_free(noia_store_delete(self->regions, rid));
    }
}

//------------------------------------------------------------------------------

void noia_wayland_cache_add_surface_resource
                                  (NoiaWaylandCache* self,
                                   NoiaSurfaceId sid,
                                   NoiaWaylandSurfaceResourceType resource_type,
                                   struct wl_resource* resource)
{
    NoiaWaylandSurface* surface = noia_wayland_cache_find_surface(self, sid);
    noia_wayland_surface_add_resource(surface, resource_type, resource);
}

//------------------------------------------------------------------------------

void noia_wayland_cache_add_general_resource
                                   (NoiaWaylandCache* self,
                                    NoiaWaylandGeneralResourceType resource_type,
                                    struct wl_resource* resource)
{
    NOIA_ENSURE(resource_type < NOIA_NUM_GENERAL_RESOURCE_TYPES, return);
    noia_list_append(self->general_resource[resource_type], resource);
}

//------------------------------------------------------------------------------

void noia_wayland_cache_remove_surface_resource
                                  (NoiaWaylandCache* self,
                                   NoiaSurfaceId sid,
                                   NoiaWaylandSurfaceResourceType resource_type,
                                   struct wl_resource* resource)
{
    NOIA_ENSURE(resource_type < NOIA_NUM_SURFACE_RESOURCE_TYPES, return);
    NoiaWaylandSurface* surface = noia_wayland_cache_find_surface(self, sid);
    if (surface) {
        noia_wayland_surface_remove_resource(surface, resource_type, resource);
    } else {
        // This is not error. Some clients remove surface before XDG surface.
        LOG_WARN1("Wayland: surface not found (sid: %u)", sid);
    }
}

//------------------------------------------------------------------------------

void noia_wayland_cache_remove_general_resource
                                  (NoiaWaylandCache* self,
                                   NoiaWaylandGeneralResourceType resource_type,
                                   struct wl_resource* resource)
{
    NOIA_ENSURE(resource_type < NOIA_NUM_GENERAL_RESOURCE_TYPES, return);
    noia_list_remove(self->general_resource[resource_type], resource,
                        (NoiaCompareFunc) noia_wayland_cache_compare_resources);
}

//------------------------------------------------------------------------------

NoiaList* noia_wayland_cache_get_resources
                                  (NoiaWaylandCache* self,
                                   NoiaWaylandGeneralResourceType resource_type)
{
    NOIA_ENSURE(resource_type < NOIA_NUM_GENERAL_RESOURCE_TYPES, return NULL);
    return self->general_resource[resource_type];
}

//------------------------------------------------------------------------------

NoiaWaylandRc noia_wayland_cache_get_rc_for_sid(NoiaWaylandCache* self,
                                                NoiaSurfaceId sid)
{
    NoiaWaylandRc result = {NULL, NULL};
    NoiaWaylandSurface* surface = noia_wayland_cache_find_surface(self, sid);
    if (surface) {
        result.rc = noia_wayland_surface_get_resource(surface,
                                                      NOIA_RESOURCE_SURFACE);
        if (result.rc) {
            result.cl = wl_resource_get_client(result.rc);
        }
    }
    return result;
}

//------------------------------------------------------------------------------

