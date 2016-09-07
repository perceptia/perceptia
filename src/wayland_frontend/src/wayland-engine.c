// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-engine.h"

#include "wayland-output.h"

#include "wayland-protocol-compositor.h"
#include "wayland-protocol-subcompositor.h"
#include "wayland-protocol-shell.h"
#include "wayland-protocol-device-manager.h"
#include "wayland-protocol-seat.h"
#include "wayland-protocol-xdg-shell.h"
#include "wayland-protocol-output.h"
#include "wayland-protocol-screenshooter.h"

#include "xdg-shell-server-protocol.h"
#include "screenshooter-server-protocol.h"

#include "utils-log.h"
#include "utils-environment.h"
#include "global-macros.h"

#include "perceptia.h"

#include <wayland-server.h>
#include <stdlib.h>
#include <string.h>

//------------------------------------------------------------------------------

struct NoiaWaylandEngineStruct {
    pthread_t thread;
    struct wl_display* display;
    struct wl_event_source* src;
    NoiaStore* outputs;
    bool running;
};

//------------------------------------------------------------------------------
// PRIVATE

void* noia_wayland_engine_run(void* data)
{
    noia_environment_on_enter_new_thread(0, "noia:wayland");

    LOG_INFO1("Threads: Wayland thread started");
    NoiaWaylandEngine* engine = (NoiaWaylandEngine*) data;
    wl_display_run(engine->display);
    return NULL;
}

//------------------------------------------------------------------------------

int noia_wayland_engine_event_loop_feeder(void* data)
{
    LOG_WAYL5("--- Wayland loop feeder ---");
    NoiaWaylandEngine* engine = (NoiaWaylandEngine*) data;
    wl_event_source_timer_update(engine->src, 60);
    return 0;
}

//------------------------------------------------------------------------------
// PUBLIC

NoiaWaylandEngine* noia_wayland_engine_new(void)
{
    NoiaWaylandEngine* self = calloc(1, sizeof(*self));
    self->display = NULL;
    self->src = NULL;
    self->running = false;
    self->outputs = noia_store_new_for_str();
    return self;
}

//------------------------------------------------------------------------------

void noia_wayland_engine_free(NoiaWaylandEngine* self)
{
    NOIA_ENSURE(self, return);
    noia_store_free_with_items(self->outputs,
                               (NoiaFreeFunc) noia_wayland_output_destroy);
    free(self);
}

//------------------------------------------------------------------------------

NoiaResult noia_wayland_engine_initialize(NoiaWaylandEngine* self)
{
    NOIA_ENSURE(self, return NOIA_RESULT_INCORRECT_ARGUMENT);
    NoiaResult result = NOIA_RESULT_ERROR;

    // Init Wayland
    self->display = wl_display_create();
    if (self->display) {
        /// @note WORKAROUND:
        /// Wayland main loop must be fed with some kind of epoll events,
        /// otherwise it blocks. Here Wayland timer is used.
        self->src =
           wl_event_loop_add_timer(wl_display_get_event_loop(self->display),
                                   noia_wayland_engine_event_loop_feeder, self);
        noia_wayland_engine_event_loop_feeder(self);

        // Add socket
        if (wl_display_add_socket(self->display, "wayland-0")) {
            LOG_ERROR("Failed to add Wayland socket 'wayland-0': %m");
        } else {
            LOG_WAYL1("Wayland socket name: 'wayland-0'");
            result = NOIA_RESULT_SUCCESS;
        }
    } else {
        LOG_ERROR("Could not initialize Wayland!");
    }
    return result;
}

//------------------------------------------------------------------------------

void noia_wayland_engine_finalize(NoiaWaylandEngine* self)
{
    if (self) {
        if (self->display) {
            wl_display_destroy(self->display);
            self->display = NULL;
        }
        if (self->src) {
            free(self->src);
            self->src = NULL;
        }
    }
}

//------------------------------------------------------------------------------

NoiaResult noia_wayland_engine_start(NoiaWaylandEngine* self)
{
    // Create global objects
    if (!wl_global_create(self->display, &wl_compositor_interface, 3,
                          NULL, noia_wayland_compositor_bind)) {
        LOG_ERROR("Could not create global display!");
    }

    if (!wl_global_create(self->display, &wl_subcompositor_interface, 1,
                          NULL, noia_wayland_subcompositor_bind)) {
        LOG_ERROR("Could not create global display!");
    }

    if (!wl_global_create(self->display, &wl_data_device_manager_interface, 2,
                          NULL, noia_wayland_device_manager_bind)) {
        LOG_ERROR("Could not create global device manager!");
    }

    if (!wl_global_create(self->display, &wl_shell_interface, 1,
                          NULL, noia_wayland_shell_bind)) {
        LOG_ERROR("Could not create global shell!");
    }

    if (!wl_global_create(self->display, &xdg_shell_interface, 1,
                          NULL, noia_wayland_xdg_shell_bind)) {
        LOG_ERROR("Could not create global XDG shell!");
    }

    if (!wl_global_create(self->display, &wl_seat_interface, 4,
                          NULL, noia_wayland_seat_bind)) {
        LOG_ERROR("Could not create global seat!");
    }

    if (!wl_global_create(self->display, &screenshooter_interface, 1,
                          NULL, noia_wayland_screenshooter_bind)) {
        LOG_ERROR("Could not create global screenshooter!");
    }

    wl_display_init_shm(self->display);

    // Start thread
    NoiaResult result = NOIA_RESULT_ERROR;
    if (pthread_create(&self->thread, NULL, noia_wayland_engine_run, self)) {
        LOG_ERROR("Could not run Wayland display!");
    } else {
        result = NOIA_RESULT_SUCCESS;
        self->running = true;
    }

    return result;
}

//------------------------------------------------------------------------------

void noia_wayland_engine_stop(NoiaWaylandEngine* self)
{
    if (self->running) {
        LOG_INFO1("Wayland: waiting for thread to exit");
        wl_display_terminate(self->display);
        pthread_join(self->thread, NULL);
        self->running = false;
        LOG_INFO1("Wayland: thread joined");
    }
}

//------------------------------------------------------------------------------

int noia_wayland_engine_next_serial(NoiaWaylandEngine* self)
{
    return wl_display_next_serial(self->display);
}

//------------------------------------------------------------------------------

void noia_wayland_engine_advertise_output(NoiaWaylandEngine* self,
                                          NoiaOutput* output)
{
    struct wl_global* global =
                        wl_global_create(self->display, &wl_output_interface, 2,
                                         output, noia_wayland_output_bind);
    if (not global) {
        LOG_ERROR("Could not create global output!");
    }

    NoiaWaylandOutput* wayland_output =
                                     noia_wayland_output_create(global, output);
    noia_store_add(self->outputs,
                   strdup(noia_output_get_name(output)),
                   wayland_output);
}

//------------------------------------------------------------------------------

void noia_wayland_engine_destroy_output(NoiaWaylandEngine* self,
                                        NoiaOutput* output)
{
    NoiaWaylandOutput* wayland_output =
                 noia_store_delete(self->outputs, noia_output_get_name(output));
    wl_global_destroy(wayland_output->global_output);
    noia_wayland_output_destroy(wayland_output);
}

//------------------------------------------------------------------------------

