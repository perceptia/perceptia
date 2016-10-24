// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-gateway.h"

#include "wayland-protocol-data-offer.h"

#include "xdg-shell-server-protocol.h"

#include "perceptia.h"

#include "utils-log.h"

//------------------------------------------------------------------------------

void noia_wayland_gateway_screen_refresh(NoiaWaylandCache* cache,
                                         NoiaSurfaceId sid,
                                         uint32_t milliseconds)
{
    LOG_WAYL4("Wayland: screen refresh (sid: %u)", sid);

    noia_wayland_cache_lock(cache);
    NoiaWaylandSurface* surface = noia_wayland_cache_find_surface(cache, sid);
    NoiaList* frcs = noia_wayland_surface_get_frame_resources(surface);

    // Release buffer if needed
    if (noia_list_len(frcs) > 0) {
        struct wl_resource* buffer_rc =
               noia_wayland_surface_get_resource(surface, NOIA_RESOURCE_BUFFER);
        if (buffer_rc) {
            wl_resource_queue_event(buffer_rc, WL_BUFFER_RELEASE);
            noia_wayland_surface_remove_resource
                                     (surface, NOIA_RESOURCE_BUFFER, buffer_rc);
        }
    }

    noia_wayland_cache_unlock(cache);

    // Notify frame
    while (noia_list_len(frcs) > 0) {
        LOG_WAYL3("Wayland < frame (sid: %u)", sid);
        struct wl_resource* rc = noia_list_pop(frcs);
        wl_callback_send_done(rc, milliseconds);
        wl_resource_destroy(rc);
    }
}

//------------------------------------------------------------------------------

void noia_wayland_gateway_send_selection(NoiaWaylandState* state,
                                         NoiaWaylandCache* cache)
{
    noia_wayland_cache_lock(cache);

    NoiaSurfaceId kfsid = state->keyboard_focused_sid;
    NoiaWaylandRc kfrc = noia_wayland_cache_get_rc_for_sid(cache, kfsid);
    NoiaWaylandTransfer* transfer = state->current_transfer;

    if (transfer) {
        LOG_WAYL3("Wayland < send selection (kfsid: %u)", kfsid);

        NoiaList* resources =
             noia_wayland_cache_get_resources(cache, NOIA_RESOURCE_DATA_DEVICE);
        FOR_EACH (resources, link) {
            struct wl_resource* data_device_rc = link->data;
            struct wl_client* client = wl_resource_get_client(data_device_rc);

            if (client == kfrc.cl) {
                int version = wl_resource_get_version(data_device_rc);
                struct wl_resource* data_offer_rc =
                     noia_wayland_data_offer_bind(client, transfer, version, 0);

                wl_data_device_send_data_offer(data_device_rc, data_offer_rc);

                FOR_EACH (noia_wayland_transfer_get_mimetypes(transfer), link) {
                    const char* mime = link->data;
                    wl_data_offer_send_offer(data_offer_rc, mime);
                }

                /// @todo Use more actions.
                uint32_t action = WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY;
                wl_data_offer_send_action(data_offer_rc, action);

                wl_data_device_send_selection(data_device_rc, data_offer_rc);
            }
        }
    }

    noia_wayland_cache_unlock(cache);
}

//------------------------------------------------------------------------------

void noia_wayland_gateway_keyboard_focus_update(NoiaWaylandState* state,
                                                NoiaWaylandCache* cache,
                                                NoiaWaylandEngine* engine,
                                                NoiaCoordinator* coordinator,
                                                NoiaSurfaceId old_sid,
                                                NoiaSize old_size,
                                                uint32_t old_state_flags,
                                                NoiaSurfaceId new_sid,
                                                NoiaSize new_size,
                                                uint32_t new_state_flags)
{
    noia_wayland_cache_lock(cache);

    NoiaWaylandRc new = noia_wayland_cache_get_rc_for_sid(cache, new_sid);
    NoiaWaylandRc old = noia_wayland_cache_get_rc_for_sid(cache, old_sid);

    // Check if new and old clients are different
    if (new.cl != old.cl) {
        LOG_WAYL2("Wayland < keyboard focus update "
                  "(oldsid: %u, newsid: %u)", old_sid, new_sid);

        // Clear current client
        state->keyboard_focused_sid = scInvalidItemId;

        // Send 'leave' and 'enter' event to all clients' keyboard objects
        struct wl_array array;
        wl_array_init(&array);
        NoiaList* resources =
                noia_wayland_cache_get_resources(cache, NOIA_RESOURCE_KEYBOARD);
        FOR_EACH (resources, link) {
            struct wl_resource* rc = link->data;
            struct wl_client* client = wl_resource_get_client(rc);

            if (client == old.cl) {
                wl_keyboard_send_leave(rc, 0, old.rc);
            }
            if (client == new.cl) {
                wl_keyboard_send_enter(rc, 0, new.rc, &array);
            }
        }

        // Update current client
        state->keyboard_focused_sid = new_sid;
        noia_wayland_cache_unlock(cache);

        // Send selection (clipboard data offer)
        noia_wayland_gateway_send_selection(state, cache);
    } else {
        noia_wayland_cache_unlock(cache);
    }

    // Inform surfaces their states changed
    noia_wayland_gateway_surface_reconfigured
                         (state, cache, engine, coordinator, old_sid, old_size, old_state_flags);
    noia_wayland_gateway_surface_reconfigured
                         (state, cache, engine, coordinator, new_sid, new_size, new_state_flags);
}

//------------------------------------------------------------------------------

void noia_wayland_gateway_key(NoiaWaylandState* state,
                              NoiaWaylandCache* cache,
                              NoiaWaylandEngine* engine,
                              uint32_t time,
                              uint32_t key_code,
                              uint32_t key_state)
{
    // Update keyboard state
    NoiaKeyMods old_mods =
                       noia_keyboard_state_get_modifiers(state->keyboard_state);
    noia_keyboard_state_update_key(state->keyboard_state, key_code, key_state);
    NoiaKeyMods new_mods =
                       noia_keyboard_state_get_modifiers(state->keyboard_state);
    bool mods_changed = not noia_keymods_are_equal(&new_mods, &old_mods);

    // Check if there is someone to be notified
    if (state->keyboard_focused_sid == scInvalidItemId) {
        return;
    }

    LOG_WAYL4("Wayland < key (sid: %u, time: %u, key: %u, state: %u)",
              state->keyboard_focused_sid, time, key_code, key_state);

    noia_wayland_cache_lock(cache);

    NOIA_BLOCK {
        // Get client
        NoiaWaylandRc focused =
          noia_wayland_cache_get_rc_for_sid(cache, state->keyboard_focused_sid);
        if (not focused.cl) {
            break;
        }

        // Notify the client
        int serial = noia_wayland_engine_next_serial(engine);
        NoiaList* resources =
                noia_wayland_cache_get_resources(cache, NOIA_RESOURCE_KEYBOARD);
        FOR_EACH (resources, link) {
            struct wl_resource* rc = link->data;
            if (focused.cl == wl_resource_get_client(rc)) {
                // Send key
                wl_keyboard_send_key(rc, serial, time, key_code, key_state);

                // Send modifiers if needed
                if (mods_changed) {
                    wl_keyboard_send_modifiers(rc, serial,
                                               new_mods.depressed,
                                               new_mods.latched,
                                               new_mods.locked,
                                               new_mods.effective);
                }
            }
        }
    }

    noia_wayland_cache_unlock(cache);
}

//------------------------------------------------------------------------------

void noia_wayland_gateway_pointer_focus_update(NoiaWaylandState* state,
                                               NoiaWaylandCache* cache,
                                               NoiaWaylandEngine* engine,
                                               NoiaSurfaceId new_sid,
                                               NoiaPosition pos)
{
    noia_wayland_cache_lock(cache);

    NoiaList* resources = NULL;
    NoiaSurfaceId old_sid = state->pointer_focused_sid;
    NoiaWaylandRc new = noia_wayland_cache_get_rc_for_sid(cache, new_sid);
    NoiaWaylandRc old = noia_wayland_cache_get_rc_for_sid(cache, old_sid);
    int serial = noia_wayland_engine_next_serial(engine);
    resources = noia_wayland_cache_get_resources(cache, NOIA_RESOURCE_POINTER);

    state->pointer_focused_sid = scInvalidSurfaceId;

    FOR_EACH (resources, link) {
        struct wl_resource* rc = link->data;
        struct wl_client* client = wl_resource_get_client(rc);

        if (client == old.cl) {
            wl_pointer_send_leave(rc, serial, old.rc);
        }
        if (client == new.cl) {
            wl_pointer_send_enter(rc, serial, new.rc,
                                  wl_fixed_from_int(pos.x),
                                  wl_fixed_from_int(pos.y));
        }
    }

    state->pointer_focused_sid = new_sid;

    noia_wayland_cache_unlock(cache);
}

//------------------------------------------------------------------------------

// For each pointer resource matching currently focussed surface send
// appropriate motion events.
void noia_wayland_gateway_pointer_motion(NoiaWaylandCache* cache,
                                         NoiaSurfaceId sid,
                                         NoiaPosition pos,
                                         int32_t milliseconds)
{
    noia_wayland_cache_lock(cache);

    NoiaList* resources = NULL;
    NoiaWaylandRc info = noia_wayland_cache_get_rc_for_sid(cache, sid);
    resources = noia_wayland_cache_get_resources(cache, NOIA_RESOURCE_POINTER);

    if (info.cl) {
        FOR_EACH (resources, link) {
            struct wl_resource* rc = link->data;
            if (info.cl == wl_resource_get_client(rc)) {
                wl_pointer_send_motion(rc, milliseconds,
                                       wl_fixed_from_int(pos.x),
                                       wl_fixed_from_int(pos.y));
            }
        }
    }

    noia_wayland_cache_unlock(cache);
}

//------------------------------------------------------------------------------

// For each pointer resource matching currently focussed surface send
// appropriate button events.
void noia_wayland_gateway_pointer_button(NoiaWaylandState* state,
                                         NoiaWaylandCache* cache,
                                         NoiaWaylandEngine* engine,
                                         uint32_t time,
                                         uint32_t button,
                                         uint32_t button_state)
{
    noia_wayland_cache_lock(cache);

    NoiaWaylandRc info =
           noia_wayland_cache_get_rc_for_sid(cache, state->pointer_focused_sid);
    NoiaList* resources =
                 noia_wayland_cache_get_resources(cache, NOIA_RESOURCE_POINTER);

    if (info.cl) {
        FOR_EACH (resources, link) {
            struct wl_resource* rc = link->data;
            if (info.cl == wl_resource_get_client(rc)) {
                int serial = noia_wayland_engine_next_serial(engine);
                wl_pointer_send_button(rc, serial, time, button, button_state);
            }
        }
    }

    noia_wayland_cache_unlock(cache);
}

//------------------------------------------------------------------------------

// For each pointer resource matching currently focussed surface send
// appropriate axis events.
void noia_wayland_gateway_pointer_axis(NoiaWaylandState* state,
                                       NoiaWaylandCache* cache,
                                       wl_fixed_t horiz,
                                       wl_fixed_t vert,
                                       int32_t horiz_descrete,
                                       int32_t vert_descrete)
{
    noia_wayland_cache_lock(cache);

    NoiaWaylandRc info =
           noia_wayland_cache_get_rc_for_sid(cache, state->pointer_focused_sid);
    NoiaList* resources =
                 noia_wayland_cache_get_resources(cache, NOIA_RESOURCE_POINTER);

    if (info.cl) {
        FOR_EACH (resources, link) {
            struct wl_resource* rc = link->data;
            if (info.cl == wl_resource_get_client(rc)) {
                // Send horizontal events
                uint32_t axis = WL_POINTER_AXIS_HORIZONTAL_SCROLL;
                if (horiz_descrete != 0) {
                    wl_fixed_t fvalue = wl_fixed_from_double(horiz_descrete);
                    wl_pointer_send_axis_discrete(rc, axis, fvalue);
                }

                if (horiz != 0.0) {
                    wl_fixed_t fvalue = wl_fixed_from_double(horiz);
                    wl_pointer_send_axis(rc, 0, axis, fvalue);
                } else {
                    wl_pointer_send_axis_stop(rc, 0, axis);
                }

                // Send vertical events
                axis = WL_POINTER_AXIS_VERTICAL_SCROLL;
                if (vert_descrete != 0) {
                    wl_fixed_t fvalue = wl_fixed_from_double(vert_descrete);
                    wl_pointer_send_axis_discrete(rc, axis, fvalue);
                }

                if (vert != 0.0) {
                    wl_fixed_t fvalue = wl_fixed_from_double(vert);
                    wl_pointer_send_axis(rc, 0, axis, fvalue);
                } else {
                    wl_pointer_send_axis_stop(rc, 0, axis);
                }
            }
        }
    }

    noia_wayland_cache_unlock(cache);
}

//------------------------------------------------------------------------------

void noia_wayland_gateway_surface_reconfigured(NoiaWaylandState* state,
                                               NoiaWaylandCache* cache,
                                               NoiaWaylandEngine* engine,
                                               NoiaCoordinator* coordinator,
                                               NoiaSurfaceId sid,
                                               NoiaSize size,
                                               uint32_t state_flags)
{
    noia_wayland_cache_lock(cache);
    NoiaWaylandSurface* surface = noia_wayland_cache_find_surface(cache, sid);

    LOG_WAYL3("Wayland < surface reconfiguration "
              "(sid: %d, width: %d, height: %d)",
               sid, size.width, size.height);

    if (surface) {
        struct wl_resource* shell_surface_rc =
                            noia_wayland_surface_get_resource
                                         (surface, NOIA_RESOURCE_SHELL_SURFACE);
        struct wl_resource* xdg_shell_surface_rc =
                            noia_wayland_surface_get_resource
                                     (surface, NOIA_RESOURCE_XDG_SHELL_SURFACE);

        if (shell_surface_rc) {
            wl_shell_surface_send_configure(shell_surface_rc,
                                            0x0,
                                            size.width,
                                            size.height);
        } else if (xdg_shell_surface_rc) {
            struct wl_array states;
            uint32_t* s;
            wl_array_init(&states);
            if (state_flags & NOIA_SURFACE_STATE_MAXIMIZED) {
                s = wl_array_add(&states, sizeof(*s));
                *s = XDG_SURFACE_STATE_MAXIMIZED;
            }
            if (sid == state->keyboard_focused_sid) {
                s = wl_array_add(&states, sizeof(*s));
                *s = XDG_SURFACE_STATE_ACTIVATED;
            }

            int serial = noia_wayland_engine_next_serial(engine);
            xdg_surface_send_configure(xdg_shell_surface_rc,
                                       size.width,
                                       size.height,
                                       &states, serial);
            wl_array_release(&states);
        }
    }

    noia_wayland_cache_unlock(cache);
}

//------------------------------------------------------------------------------

