// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef PERCEPTIA_H
#define PERCEPTIA_H

#include "global-types.h"

#define NOIA_SURFACE_STATE_MAXIMIZED 0x1 

//------------------------------------------------------------------------------

//typedef struct NoiaSurfaceDataStruct NoiaSurfaceData;

typedef enum {
    NOIA_SURFACE_SHOW_DRAWABLE = 0x1,
    NOIA_SURFACE_SHOW_IN_SHELL = 0x2,

    NOIA_SURFACE_SHOW_FULL = NOIA_SURFACE_SHOW_DRAWABLE
                           | NOIA_SURFACE_SHOW_IN_SHELL,
} NoiaSurfaceShowReason;

//------------------------------------------------------------------------------

typedef struct {
    uint32_t format;
    uint64_t size;
    uint32_t fd;
} NoiaKeymapSettings;

//------------------------------------------------------------------------------

typedef struct NoiaOutput NoiaOutput;

const char* noia_output_get_name(NoiaOutput* self);

NoiaArea noia_output_get_area(NoiaOutput* self);

NoiaSize noia_output_get_physical_size(NoiaOutput* self);

//------------------------------------------------------------------------------

typedef struct NoiaCoordinatorStruct NoiaCoordinator;

//------------------------------------------------------------------------------

NoiaSurfaceId noia_surface_create(NoiaCoordinator* coordinator);

void noia_surface_destroy(NoiaCoordinator* coordinator, NoiaSurfaceId sid);

/*NoiaSurfaceData* noia_surface_get(NoiaCoordinator* coordinator,
                                  NoiaSurfaceId sid);

*/
void noia_surface_attach(NoiaCoordinator* coordinator,
                         NoiaSurfaceId sid,
                         int width,
                         int height,
                         int stride,
                         uint8_t* buffer,
                         void* resource);

void noia_surface_commit(NoiaCoordinator* coordinator, NoiaSurfaceId sid);

void noia_surface_show(NoiaCoordinator* coordinator,
                       NoiaSurfaceId sid,
                       NoiaSurfaceShowReason reason);
/*
void noia_surface_reconfigure(NoiaCoordinator* coordinator,
                              NoiaSurfaceId sid,
                              NoiaSize size,
                              uint8_t state_flags);

void noia_surface_set_focus(NoiaCoordinator* coordinator, NoiaSurfaceId sid);
*/
void noia_surface_set_offset(NoiaCoordinator* coordinator,
                             NoiaSurfaceId sid,
                             NoiaPosition offset);

void noia_surface_set_requested_size(NoiaCoordinator* coordinator,
                                     NoiaSurfaceId sid,
                                     NoiaSize size);

void noia_surface_reset_offset_and_requested_size(NoiaCoordinator* coordinator,
                                                  NoiaSurfaceId sid);

void noia_surface_set_relative_position(NoiaCoordinator* coordinator,
                                        NoiaSurfaceId sid,
                                        NoiaPosition pos);

void noia_surface_relate(NoiaCoordinator* coordinator,
                         NoiaSurfaceId sid,
                         NoiaSurfaceId parent_sid);
/*
/// @todo Add unit tests for `noia_surface_to_array`.
void noia_surface_to_array(NoiaCoordinator* coordinator,
                           NoiaSurfaceId sid,
                           NoiaPosition parent_pos,
                           NoiaPool* surfaces);
*/
//------------------------------------------------------------------------------

void noia_surface_set_as_cursor(NoiaCoordinator* coordinator, NoiaSurfaceId sid);
/*
int noia_surface_compare(NoiaSurfaceId first, NoiaSurfaceId second);
*/
//------------------------------------------------------------------------------

#endif // PERCEPTIA_H

