// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_GATEWAY_H
#define NOIA_WAYLAND_GATEWAY_H

#include "wayland-state.h"
#include "wayland-cache.h"
#include "wayland-engine.h"
#include "wayland-transfer.h"

//------------------------------------------------------------------------------

/// @file
/// Wayland Gateway is set of functions used to inform clients about events like
/// keyboard input or pointer motion.
///
/// @see NoiaWaylandState, NoiaWaylandCache, NoiaWaylandEngine,
///      wayland-facade.h, wayland-module.c

//------------------------------------------------------------------------------

/// Send frame events to given client.
void noia_wayland_gateway_screen_refresh(NoiaWaylandCache* cache,
                                         NoiaSurfaceId sid,
                                         uint32_t milliseconds);

//------------------------------------------------------------------------------

/// Send selection (clipboard data offer).
void noia_wayland_gateway_send_selection(NoiaWaylandState* state,
                                         NoiaWaylandCache* cache);

//------------------------------------------------------------------------------

/// Send keyboard leave and enter event to interested clients.
void noia_wayland_gateway_keyboard_focus_update(NoiaWaylandState* state,
                                                NoiaWaylandCache* cache,
                                                NoiaWaylandEngine* engine,
                                                NoiaCoordinator* coordinator,
                                                NoiaSurfaceId new_sid);

/// Send key event and modifiers to focused surface.
void noia_wayland_gateway_key(NoiaWaylandState* state,
                              NoiaWaylandCache* cache,
                              NoiaWaylandEngine* engine,
                              uint32_t time,
                              uint32_t key_code,
                              uint32_t key_state);

//------------------------------------------------------------------------------

/// Send pointer leave and enter event to interested clients.
void noia_wayland_gateway_pointer_focus_update(NoiaWaylandState* state,
                                               NoiaWaylandCache* cache,
                                               NoiaWaylandEngine* engine,
                                               NoiaSurfaceId new_sid,
                                               NoiaPosition pos);

/// Send pointer motion event to hovered surface.
void noia_wayland_gateway_pointer_motion(NoiaWaylandCache* cache,
                                         NoiaSurfaceId sid,
                                         NoiaPosition pos,
                                         int32_t milliseconds);

/// Send pointer button event to clicked surface.
void noia_wayland_gateway_pointer_button(NoiaWaylandState* state,
                                         NoiaWaylandCache* cache,
                                         NoiaWaylandEngine* engine,
                                         uint32_t time,
                                         uint32_t button,
                                         uint32_t button_state);

/// Send pointer axis event to scrolled surface.
void noia_wayland_gateway_pointer_axis(NoiaWaylandState* state,
                                       NoiaWaylandCache* cache,
                                       wl_fixed_t horiz,
                                       wl_fixed_t vert,
                                       int32_t horiz_descrete,
                                       int32_t vert_descrete);

//------------------------------------------------------------------------------

/// Send reconfiguration event (size or state change) to given surface.
void noia_wayland_gateway_surface_reconfigured(NoiaWaylandState* state,
                                               NoiaWaylandCache* cache,
                                               NoiaWaylandEngine* engine,
                                               NoiaCoordinator* coordinator,
                                               NoiaSurfaceId sid);

//------------------------------------------------------------------------------

#endif // NOIA_WAYLAND_GATEWAY_H

