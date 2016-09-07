// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "utils-keyboard-state.h"
#include "global-macros.h"

#include <malloc.h>
#include <memory.h>
#include <xkbcommon/xkbcommon.h>

//------------------------------------------------------------------------------

struct NoiaKeyboardStateInternal {
    struct xkb_context* context;
    struct xkb_keymap* keymap;
    struct xkb_state* state;
};

//------------------------------------------------------------------------------

bool noia_keymods_are_equal(NoiaKeyMods* km1, NoiaKeyMods* km2)
{
    return (km1->depressed == km2->depressed)
       and (km1->latched   == km2->latched)
       and (km1->locked    == km2->locked)
       and (km1->effective == km2->effective);
}

//------------------------------------------------------------------------------

NoiaKeyboardState* noia_keyboard_state_new(void)
{
    NoiaKeyboardState* self = malloc(sizeof(NoiaKeyboardState));
    memset(self, 0, sizeof(NoiaKeyboardState));
    return self;
}

//------------------------------------------------------------------------------

void noia_keyboard_state_free(NoiaKeyboardState* self)
{
    NOIA_ENSURE(self, return);
    free(self);
}

//------------------------------------------------------------------------------

void noia_keyboard_state_initialize(NoiaKeyboardState* self)
{
    NOIA_ENSURE(self, return);

    // Create context
    self->context = xkb_context_new(0x0);

    // Create keymap from names
    struct xkb_rule_names names;
    names.rules = "evdev";
    names.model = "evdev";
    names.layout = "us";
    names.variant = NULL;
    names.options = NULL;

    self->keymap = xkb_keymap_new_from_names(self->context, &names, 0x0);

    // Create keyboard state
    self->state = xkb_state_new(self->keymap);
}

//------------------------------------------------------------------------------

void noia_keyboard_state_finalize(NoiaKeyboardState* self)
{
    NOIA_ENSURE(self, return);

    if (self->state) {
        xkb_state_unref(self->state);
        self->state = NULL;
    }

    if (self->keymap) {
        xkb_map_unref(self->keymap);
        self->keymap = NULL;
    }

    if (self->context) {
        xkb_context_unref(self->context);
        self->context = NULL;
    }
}

//------------------------------------------------------------------------------

void noia_keyboard_state_update_key(NoiaKeyboardState* self,
                                    int code,
                                    NoiaKeyState state)
{
    enum xkb_key_direction direction =
                        (state == NOIA_KEY_PRESSED) ? XKB_KEY_DOWN : XKB_KEY_UP;

    // Offset the keycode by 8, as the evdev XKB rules reflect X's
    // broken keycode system, which starts at 8.
    xkb_state_update_key(self->state, code + 8, direction);
}

//------------------------------------------------------------------------------

NoiaKeyMods noia_keyboard_state_get_modifiers(NoiaKeyboardState* self)
{
    NoiaKeyMods mods;
    mods.depressed = xkb_state_serialize_mods(self->state, XKB_STATE_DEPRESSED);
    mods.latched   = xkb_state_serialize_mods(self->state, XKB_STATE_LATCHED);
    mods.locked    = xkb_state_serialize_mods(self->state, XKB_STATE_LOCKED);
    mods.effective = xkb_state_serialize_group(self->state,XKB_STATE_EFFECTIVE);
    return mods;
}

//------------------------------------------------------------------------------

