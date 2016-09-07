// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-screenshooter.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

#include "perceptia.h"

#include "screenshooter-server-protocol.h"

//------------------------------------------------------------------------------

void noia_wayland_screenshooter_unbind(struct wl_resource* resource NOIA_UNUSED)
{
    LOG_WAYL2("Wayland: unbind screenshooter");
    noia_wayland_facade_remove_general_resource(NOIA_RESOURCE_OTHER, resource);
}

//------------------------------------------------------------------------------

/// Wayland protocol: Take a screenshot.
/// @todo Implement support for screenshot format.
void noia_wayland_screenshooter_shoot(struct wl_client* client NOIA_UNUSED,
                                      struct wl_resource* resource,
                                      struct wl_resource* output_resource,
                                      struct wl_resource* buffer_resource)
{
    // FIXME
    /*NoiaOutput* output = wl_resource_get_user_data(output_resource);

    struct wl_shm_buffer* shm_buffer = wl_shm_buffer_get(buffer_resource);
    if (shm_buffer) {
        int width  = wl_shm_buffer_get_width(shm_buffer);
        int height = wl_shm_buffer_get_height(shm_buffer);
        int stride = wl_shm_buffer_get_stride(shm_buffer);
        uint8_t* data = wl_shm_buffer_get_data(shm_buffer);

        NoiaBuffer buffer;
        noia_buffer_setup(&buffer, width, height, stride,
                          NOIA_FORMAT_UNKNOWN, data);
        NoiaArea area = {.pos={0, 0}, .size={width, height}};

        noia_output_take_screenshot(output, area, &buffer);
        noia_buffer_reformat(&buffer, NOIA_FORMAT_RGBA);
    } else {
        LOG_ERROR("Wrong shared memory buffer!");
        wl_resource_post_no_memory(resource);
    }

    screenshooter_send_done(resource);*/
}

//------------------------------------------------------------------------------

const struct screenshooter_interface scScreenshooterImplementation = {
        noia_wayland_screenshooter_shoot
    };

//------------------------------------------------------------------------------

/// Wayland protocol: Handle request for interface to screenshooter object.
void noia_wayland_screenshooter_bind(struct wl_client* client,
                                     void* data,
                                     uint32_t version,
                                     uint32_t id)
{
    LOG_WAYL2("Binding Wayland screenshooter "
              "(version: %d, id: %d)", version, id);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &screenshooter_interface, 1, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scScreenshooterImplementation,
                                   data, noia_wayland_screenshooter_unbind);

    noia_wayland_facade_add_general_resource(NOIA_RESOURCE_OTHER, rc);
}

//------------------------------------------------------------------------------

