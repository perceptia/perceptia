// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-transfer.h"

#include "utils-list.h"

#include <string.h>

//------------------------------------------------------------------------------

struct NoiaWaylandTransferStruct {
    struct wl_resource* source_resource;
    NoiaList* mime_types;
};

//------------------------------------------------------------------------------

NoiaWaylandTransfer* noia_wayland_transfer_create(struct wl_resource* rc)
{
    NoiaWaylandTransfer* self = malloc(sizeof(*self));
    self->source_resource = rc;
    self->mime_types = noia_list_new(free);
    return self;
}

//------------------------------------------------------------------------------

void noia_wayland_transfer_destroy(NoiaWaylandTransfer* self)
{
    NOIA_ENSURE(self, return);
    noia_list_free(self->mime_types);
    memset(self, 0, sizeof(*self));
    free(self);
}

//------------------------------------------------------------------------------

void noia_wayland_transfer_add_offer(NoiaWaylandTransfer* self,
                                     const char* type)
{
    NOIA_ENSURE(self, return);
    NOIA_ENSURE(type, return);
    noia_list_append(self->mime_types, (void*) strdup(type));
}

//------------------------------------------------------------------------------

struct wl_resource* noia_wayland_transfer_get_rc(NoiaWaylandTransfer* self)
{
    NOIA_ENSURE(self, return NULL);
    return self->source_resource;
}

//------------------------------------------------------------------------------

NoiaList* noia_wayland_transfer_get_mimetypes(NoiaWaylandTransfer* self)
{
    NOIA_ENSURE(self, return NULL);
    return self->mime_types;
}

//------------------------------------------------------------------------------

