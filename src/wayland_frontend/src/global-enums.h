// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_GLOBAL_ENUMS_H
#define NOIA_GLOBAL_ENUMS_H

#include <stdbool.h>

/// Enumerate key states
typedef enum {
    NOIA_KEY_RELEASED,
    NOIA_KEY_PRESSED,
} NoiaKeyState;

/// Enumerate all used modifiers
typedef enum {
    NOIA_KEY_NONE  = 0x0000,
    NOIA_KEY_CTRL  = 0x0001,
    NOIA_KEY_SHIFT = 0x0002,
    NOIA_KEY_ALT   = 0x0004,
    NOIA_KEY_META  = 0x0008,
} NoiaKeyModifierFlag;

/// Enumerate key binding modes.
/// The mode is set of key bindings that can be active.
typedef enum {
    NOIA_MODE_COMMON,
    NOIA_MODE_NORMAL,
    NOIA_MODE_INSERT,
    NOIA_MODE_NUM, ///< Guard
} NoiaModeEnum;

/// Action type for Exhibitor.
typedef enum {
    NOIA_ACTION_NONE = 0,  ///< Dummy; do/parametrize nothing
    NOIA_ACTION_ANCHOR,    ///< Anchorize; de-anchorize
    NOIA_ACTION_CONF,      ///< Change configuration
    NOIA_ACTION_FOCUS,     ///< Change focus
    NOIA_ACTION_SWAP,      ///< Swap
    NOIA_ACTION_MOVE,      ///< Change position
    NOIA_ACTION_JUMP,      ///< Change placement by jumping over
    NOIA_ACTION_DIVE,      ///< Change placement by diving in
    NOIA_ACTION_RESIZE,    ///< Change size
} NoiaAction;

/// Enum representing directions on screen, in time and beetwen frames.
typedef enum {
    NOIA_DIRECTION_NONE = 0,  ///< Dummy; point nowhere
    NOIA_DIRECTION_N,         ///< North; up; above
    NOIA_DIRECTION_E,         ///< East; right
    NOIA_DIRECTION_S,         ///< South; down; below
    NOIA_DIRECTION_W,         ///< West; left
    NOIA_DIRECTION_BACK,      ///< Back in time; most recently used
    NOIA_DIRECTION_FORWARD,   ///< Forward in time; the oldest used
    NOIA_DIRECTION_BEGIN,     ///< Begin; start; head
    NOIA_DIRECTION_END,       ///< End; finish; tail
    NOIA_DIRECTION_TRUNK,     ///< Trunk; up in frame hierarchy
    NOIA_DIRECTION_WORKSPACE, ///< Workspace
} NoiaDirection;

/// Frame types.
typedef enum {
    NOIA_FRAME_TYPE_NONE       = 0x0000,
    NOIA_FRAME_TYPE_STACKED    = 0x0001,
    NOIA_FRAME_TYPE_HORIZONTAL = 0x0002,
    NOIA_FRAME_TYPE_VERTICAL   = 0x0004,
    NOIA_FRAME_TYPE_FLOATING   = 0x0010,
    NOIA_FRAME_TYPE_FIXED      = 0x0020,
    NOIA_FRAME_TYPE_LEAF       = 0x0100,
    NOIA_FRAME_TYPE_SPECIAL    = 0x1000,

    NOIA_FRAME_TYPE_DIRECTED   = NOIA_FRAME_TYPE_HORIZONTAL
                               | NOIA_FRAME_TYPE_VERTICAL
                               | NOIA_FRAME_TYPE_STACKED,

    // Workspace has to be directed to let relaxing work
    NOIA_FRAME_TYPE_WORKSPACE  = NOIA_FRAME_TYPE_SPECIAL
                               | NOIA_FRAME_TYPE_FIXED
                               | NOIA_FRAME_TYPE_STACKED,

    NOIA_FRAME_TYPE_DISPLAY    = NOIA_FRAME_TYPE_SPECIAL
                               | NOIA_FRAME_TYPE_FLOATING
                               | NOIA_FRAME_TYPE_STACKED,
} NoiaFrameType;

/// Type of transformation used for background image.
typedef enum {
    NOIA_BG_TRANS_REPEAT,
    NOIA_BG_TRANS_CENTER,
    NOIA_BG_TRANS_SCALE,
    NOIA_BG_TRANS_STRETCH,
} NoiaBGTransform;

/// Function return values or error codes
typedef enum {
    NOIA_RESULT_SUCCESS = 0,        ///< Everything worked fine
    NOIA_RESULT_ERROR,              ///< Unspecified error
    NOIA_RESULT_INCORRECT_ARGUMENT, ///< Incorrect or invalid argument passed
    NOIA_RESULT_NOT_FOUND,          ///< Required data not found
} NoiaResult;

/// Asserts if the result is `NOIA_RESULT_SUCCESS`.
/// @see NoiaResult NOIA_TRY
#define NOIA_ASSERT_RESULT(RESULT) { if (RESULT != NOIA_RESULT_SUCCESS) break; }

/// Return opposite direction.
NoiaDirection noia_direction_reverse(NoiaDirection direction);

/// Translate direction to corresponding frame type.
NoiaFrameType noia_direction_translate_to_frame_type(NoiaDirection direction);

#endif // NOIA_GLOBAL_ENUMS_H

