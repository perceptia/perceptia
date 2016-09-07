// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "global-enums.h"

//------------------------------------------------------------------------------

NoiaDirection noia_direction_reverse(NoiaDirection direction)
{
    switch (direction) {
    case NOIA_DIRECTION_N:       return NOIA_DIRECTION_S;
    case NOIA_DIRECTION_S:       return NOIA_DIRECTION_N;
    case NOIA_DIRECTION_E:       return NOIA_DIRECTION_W;
    case NOIA_DIRECTION_W:       return NOIA_DIRECTION_E;
    case NOIA_DIRECTION_BACK:    return NOIA_DIRECTION_FORWARD;
    case NOIA_DIRECTION_FORWARD: return NOIA_DIRECTION_BACK;
    case NOIA_DIRECTION_BEGIN:   return NOIA_DIRECTION_END;
    case NOIA_DIRECTION_END:     return NOIA_DIRECTION_BEGIN;
    case NOIA_DIRECTION_TRUNK:   return NOIA_DIRECTION_TRUNK;
    default:                     return NOIA_DIRECTION_NONE;
    }
}

//------------------------------------------------------------------------------

NoiaFrameType noia_direction_translate_to_frame_type(NoiaDirection direction)
{
    NoiaFrameType type = NOIA_FRAME_TYPE_NONE;

    switch (direction) {
    case NOIA_DIRECTION_BEGIN:
    case NOIA_DIRECTION_END:
        type = NOIA_FRAME_TYPE_STACKED;
        break;

    case NOIA_DIRECTION_N:
    case NOIA_DIRECTION_S:
        type = NOIA_FRAME_TYPE_VERTICAL;
        break;

    case NOIA_DIRECTION_E:
    case NOIA_DIRECTION_W:
        type = NOIA_FRAME_TYPE_HORIZONTAL;
        break;

    default:
        break;
    }

    return type;
}

//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

