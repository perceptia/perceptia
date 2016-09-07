// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_UTILS_KEYBOARD_STATE_H
#define NOIA_UTILS_KEYBOARD_STATE_H

#include "global-types.h"

/// Set of masks of key modifier states
typedef struct {
    int32_t depressed;
    int32_t latched;
    int32_t locked;
    int32_t effective;
} NoiaKeyMods;

/// Compare key mods.
/// @return `True` if key mods are identical, `false` otherwise.
bool noia_keymods_are_equal(NoiaKeyMods* km1, NoiaKeyMods* km2);

/// Opaque structure containing keyboard state.
typedef struct NoiaKeyboardStateInternal NoiaKeyboardState;

/// NoiaKeyboardState constructor.
NoiaKeyboardState* noia_keyboard_state_new(void);

/// NoiaKeyboardState destructor.
void noia_keyboard_state_free(NoiaKeyboardState* self);

/// NoiaKeyboardState initializer.
void noia_keyboard_state_initialize(NoiaKeyboardState* self);

/// NoiaKeyboardState finalizer.
void noia_keyboard_state_finalize(NoiaKeyboardState* self);

/// Notify about pressed of released key.
/// Keyboard state will be updated accordingly.
void noia_keyboard_state_update_key(NoiaKeyboardState* self,
                                    int code,
                                    NoiaKeyState state);

/// Serialize masks of key modifiers.
NoiaKeyMods noia_keyboard_state_get_modifiers(NoiaKeyboardState* self);

#endif // NOIA_UTILS_KEYBOARD_STATE_H

