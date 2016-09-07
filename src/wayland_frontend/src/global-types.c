// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "global-types.h"
#include "global-macros.h"

#include <memory.h>

//------------------------------------------------------------------------------

bool noia_position_is_inside(NoiaPosition position, NoiaArea area)
{
    int margin_top    = area.pos.y;
    int margin_bottom = area.size.height + margin_top;
    int margin_left   = area.pos.x;
    int margin_right  = area.size.width + margin_left;

    return (margin_top  <= position.y) and (position.y < margin_bottom)
       and (margin_left <= position.x) and (position.x < margin_right);
}

//------------------------------------------------------------------------------

NoiaPosition noia_position_cast(NoiaPosition position, NoiaArea area)
{
    NOIA_BLOCK {
        if (noia_position_is_inside(position, area)) {
            break;
        }

        if (position.x < area.pos.x) {
            position.x = area.pos.x;
        }
        if (position.x > (area.pos.x + area.size.width - 1)) {
            position.x = area.pos.x + area.size.width - 1;
        }
        if (position.y < area.pos.y) {
            position.y = area.pos.y;
        }
        if (position.y > (area.pos.y + area.size.height - 1)) {
            position.y = area.pos.y + area.size.height - 1;
        }
    }
    return position;
}

//------------------------------------------------------------------------------

void noia_area_invalidate(NoiaArea* area)
{
    area->pos.x = 0;
    area->pos.y = 0;
    area->size.width = -1;
    area->size.height = -1;
}

//------------------------------------------------------------------------------

bool noia_area_is_equal(NoiaArea area1, NoiaArea area2)
{
    if ((area1.size.width < 0) or (area1.size.height < 0)
     or (area2.size.width < 0) or (area2.size.height < 0)) {
        return false;
    }

    return (area1.size.width  == area2.size.width)
       and (area1.size.height == area2.size.height)
       and (area1.pos.x       == area2.pos.x)
       and (area1.pos.y       == area2.pos.y);
}

//------------------------------------------------------------------------------

