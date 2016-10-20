// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-module.h"
#include "wayland-context.h"
#include "wayland-gateway.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "utils-time.h"

static NoiaWaylandContext* ctx = NULL;

// FIXME
//#include "global-objects.h"

//------------------------------------------------------------------------------

void noia_wayland_module_on_surface_frame(NoiaSurfaceId sid)
{
    LOG_WAYL4("Wayland: handling screen refresh");
    NoiaMilliseconds ms = noia_time_get_monotonic_milliseconds();
    noia_wayland_gateway_screen_refresh(ctx->cache, sid, ms);
}

//------------------------------------------------------------------------------
/*
void noia_wayland_module_on_keyboard_focus_changed(void* edata, void* sdata)
{
    NoiaWaylandContext* ctx = (NoiaWaylandContext*) sdata;
    NoiaSurfaceId sid = noia_uint_unref_get((NoiaIntObject*) edata);
    LOG_WAYL2("Wayland: handling keyboard focus change (sid: %d)", sid);
    noia_wayland_gateway_keyboard_focus_update
                   (ctx->state, ctx->cache, ctx->engine, ctx->coordinator, sid);
}

//------------------------------------------------------------------------------

void noia_wayland_module_on_keyboard_event(void* edata, void* sdata)
{
    LOG_WAYL4("Wayland: handling keyboard event");

    NoiaWaylandContext* ctx = (NoiaWaylandContext*) sdata;
    NoiaKeyObject* object = (NoiaKeyObject*) edata;
    NOIA_ENSURE(object, return);
    noia_wayland_gateway_key(ctx->state,
                             ctx->cache,
                             ctx->engine,
                             object->keydata.time,
                             object->keydata.code,
                             object->keydata.value);
    noia_object_unref((NoiaObject*) object);
}
*/
//------------------------------------------------------------------------------

void noia_wayland_module_on_pointer_focus_changed(NoiaSurfaceId sid, NoiaPosition pos)
{
    LOG_WAYL4("Wayland: handling pointer focus change");
    noia_wayland_gateway_pointer_focus_update
                                (ctx->state, ctx->cache, ctx->engine, sid, pos);
}

//------------------------------------------------------------------------------

void noia_wayland_module_on_pointer_relative_motion(NoiaSurfaceId sid, NoiaPosition pos)
{
    LOG_WAYL4("Wayland: handling pointer motion");
    NoiaMilliseconds ms = noia_time_get_monotonic_milliseconds();
    noia_wayland_gateway_pointer_motion
                          (ctx->cache, sid, pos, ms);
}

//------------------------------------------------------------------------------
/*
void noia_wayland_module_on_pointer_button(void* edata, void* sdata)
{
    LOG_WAYL4("Wayland: handling pointer button");
    NoiaWaylandContext* ctx = (NoiaWaylandContext*) sdata;
    NoiaButtonObject* object = (NoiaButtonObject*) edata;
    NOIA_ENSURE(object, return);
    noia_wayland_gateway_pointer_button(ctx->state,
                                        ctx->cache,
                                        ctx->engine,
                                        object->buttondata.time,
                                        object->buttondata.code,
                                        object->buttondata.value);
    noia_object_unref((NoiaObject*) object);
}

//------------------------------------------------------------------------------

void noia_wayland_module_on_pointer_axis(void* edata, void* sdata)
{
    LOG_WAYL4("Wayland: handling pointer axis");
    NoiaWaylandContext* ctx = (NoiaWaylandContext*) sdata;
    NoiaAxisObject* object = (NoiaAxisObject*) edata;
    noia_wayland_gateway_pointer_axis(ctx->state, ctx->cache,
                                      wl_fixed_from_double(object->axisdata.h),
                                      wl_fixed_from_double(object->axisdata.v),
                                      object->axisdata.hd,
                                      object->axisdata.vd);
    noia_object_unref((NoiaObject*) object);
}

//------------------------------------------------------------------------------

void noia_wayland_module_on_surface_reconfigured(void* edata, void* sdata)
{
    NoiaWaylandContext* ctx = (NoiaWaylandContext*) sdata;
    NoiaSurfaceId sid = noia_uint_unref_get((NoiaIntObject*) edata);
    noia_wayland_gateway_surface_reconfigured
                   (ctx->state, ctx->cache, ctx->engine, ctx->coordinator, sid);
}

//------------------------------------------------------------------------------

void noia_wayland_module_on_output_found(void* edata, void* sdata)
{
    NoiaWaylandContext* ctx = (NoiaWaylandContext*) sdata;
    NoiaOutput* output = (NoiaOutput*) edata;
    NOIA_ENSURE(output, return);
    noia_wayland_engine_advertise_output(ctx->engine, output);
    noia_object_unref((NoiaObject*) output);
}

//------------------------------------------------------------------------------

void noia_wayland_module_on_output_lost(void* edata, void* sdata)
{
    NoiaWaylandContext* ctx = (NoiaWaylandContext*) sdata;
    NoiaOutput* output = (NoiaOutput*) edata;
    NOIA_ENSURE(output, return);
    noia_wayland_engine_destroy_output(ctx->engine, output);
}

//------------------------------------------------------------------------------

void noia_wayland_module_finalize(void* edata NOIA_UNUSED, void* sdata)
{
    NoiaWaylandContext* ctx = (NoiaWaylandContext*) sdata;
    noia_wayland_facade_finalize();
    noia_wayland_context_finalize(ctx);
    noia_wayland_context_free(ctx);
}

//------------------------------------------------------------------------------
*/

void noia_wayland_advertise_output()
{
    noia_wayland_engine_advertise_output(ctx->engine);
}

//------------------------------------------------------------------------------

void noia_wayland_initialize(NoiaCoordinator* coordinator)
{
    LOG_INFO1("Initializing Wayland...");

    // Init Wayland
    ctx = noia_wayland_context_new();
    NoiaResult result = noia_wayland_context_initialize(ctx, coordinator);
    if (result != NOIA_RESULT_SUCCESS) {
        LOG_ERROR("Initializing Wayland: Failed to create context!");
        noia_wayland_context_finalize(ctx);
        return;
    }
    noia_wayland_facade_initialize(ctx);
}


