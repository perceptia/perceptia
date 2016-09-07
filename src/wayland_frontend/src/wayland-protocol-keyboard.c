// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-keyboard.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

//------------------------------------------------------------------------------

void noia_wayland_keyboard_unbind(struct wl_resource* resource)
{
    LOG_WAYL3("Wayland: unbind keyboard");
    noia_wayland_facade_remove_general_resource(NOIA_RESOURCE_KEYBOARD,
                                                resource);
}

//------------------------------------------------------------------------------

void noia_wayland_keyboard_release(struct wl_client* client NOIA_UNUSED,
                                   struct wl_resource* resource)
{
    LOG_WAYL2("Wayland: keyboard release");
    wl_resource_destroy(resource);
}

//------------------------------------------------------------------------------

const struct wl_keyboard_interface scKeyboardImplementation = {
        noia_wayland_keyboard_release
    };

//------------------------------------------------------------------------------

void noia_wayland_keyboard_bind(struct wl_client* client,
                                void* data,
                                uint32_t version,
                                uint32_t id)
{
    LOG_WAYL2("Binding Wayland keyboard (version: %d, id: %d)", version, id);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_keyboard_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scKeyboardImplementation,
                                   data, noia_wayland_keyboard_unbind);

    // Store resource
    noia_wayland_facade_add_keyboard_resource(rc);

    // FIXME
    // Send keymap to client
    /*NoiaKeymap* keymap = noia_gears()->keymap;
    NOIA_ENSURE(keymap, return);

    LOG_WAYL2("Wayland < keyboard map send (format: %d, fd: %d, size: %d)",
              keymap->format, keymap->keymap_fd, keymap->keymap_size);
    wl_keyboard_send_keymap(rc, (uint32_t) keymap->format,
                                (uint32_t) keymap->keymap_fd,
                                (uint32_t) keymap->keymap_size);*/
}

//------------------------------------------------------------------------------

