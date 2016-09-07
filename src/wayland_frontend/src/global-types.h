// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_GLOBAL_TYPES_H
#define NOIA_GLOBAL_TYPES_H

#include <stdint.h>
#include <limits.h>

#include "global-enums.h"

/// Id for NoiaItem
typedef uintptr_t NoiaItemId;

/// Milliseconds
typedef uint_least64_t NoiaMilliseconds;

/// Free function definition
typedef void (*NoiaFreeFunc) (void*);

/// Compare function definition
typedef int (*NoiaCompareFunc) (const void*, const void*);

/// Duplicate function definition
typedef void* (*NoiaDuplicateFunc) (void*);

/// Print formatted string function
typedef int (*NoiaPrintFunc) (const char*, ...);

/// Structure to be inherited by all types that want to be stored in NoiaStore
typedef struct {
    union {
        NoiaItemId id;
        char* str;
    };
} NoiaItem;

/// Identifier of a surface
typedef NoiaItemId NoiaSurfaceId;

/// Callback used in NoiaBinding structure
typedef void (*NoiaKeyCallback) (void);

/// Type defining position, point coordinates or 2D vector
typedef struct {
    uint32_t x;
    uint32_t y;
} NoiaPosition;

/// Type defining 2D size, dimensions or resolution
typedef struct {
    uint32_t width;
    uint32_t height;
} NoiaSize;

/// Type defining 2D area
typedef struct {
    NoiaPosition pos;
    NoiaSize size;
} NoiaArea;

/// Key event data
typedef struct {
    unsigned time;
    int code;
    NoiaKeyState value;
} NoiaKeyData;

/// Button event data
typedef struct {
    unsigned time;
    int code;
    bool value;
} NoiaButtonData;

/// Axis event data
typedef struct {
    double h; ///< Horizontal
    double v; ///< Vertical
    int hd;   ///< Horizontal descrete
    int vd;   ///< Vertical descrete
} NoiaAxisData;

/// Container for color data.
typedef struct {
    uint8_t b;
    uint8_t g;
    uint8_t r;
    uint8_t a;
} NoiaColor;

/// Data needed by Renderer to draw surface
typedef struct {
    NoiaSurfaceId sid;
    NoiaPosition pos;
} NoiaSurfaceContext;

/// Data needed by Renderer to draw layout
typedef struct {
    NoiaSurfaceContext pointer;
    NoiaSurfaceId background_sid;
    NoiaBGTransform background_transform;
    NoiaColor background_color;
} NoiaLayoutContext;

/// Check if point `position` is inside area `area`.
bool noia_position_is_inside(NoiaPosition position, NoiaArea area);

/// If point `position` is outside area `area` return a point inside area `area`
/// that is the closest to point `position`.
NoiaPosition noia_position_cast(NoiaPosition position, NoiaArea area);

/// Invalidate area by seting negative dimensions.
void noia_area_invalidate(NoiaArea* area);

/// Check if two areas are equal.
/// @return `true` if areas are equal or
///         `false` if not or at least one area is invalid.
bool noia_area_is_equal(NoiaArea area1, NoiaArea area2);

#endif // NOIA_GLOBAL_TYPES_H

