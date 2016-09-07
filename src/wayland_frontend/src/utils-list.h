// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_UTILS_LIST_H
#define NOIA_UTILS_LIST_H

#include "utils-chain.h"
#include "global-macros.h"

#include <stddef.h>

#define FOR_EACH(LIST,LINK) \
    for (NoiaLink* LINK = LIST->base.first; LINK; LINK = LINK->next)

#define FOR_EACH_REVERSE(LIST,LINK) \
    for (NoiaLink* LINK = LIST->base.last; LINK; LINK = LINK->prev)

/// Simple implementations of double linked list.
/// This list is meant for storing data of the same type and provide
/// destruction mechanism using data-destructor passed in constructor of list.
///
/// @note When frequent allocation and deallocation of memory is needed,
///       NoiaPool may be a better choise.
/// @see NoiaChain, NoiaPool
typedef struct {
    NoiaChain base;
    NoiaFreeFunc free_data;
} NoiaList;

/// Constructor of NoiaList.
/// @param free_data - destructor for data stored in the list.
NoiaList* noia_list_new(NoiaFreeFunc free_data);

/// Destructor of NoiaList.
void noia_list_free(NoiaList* self);

/// Adds a new element to the beginning of the list.
void noia_list_prepend(NoiaList* self, void* data);

/// Adds a new element to the ending of the list.
void noia_list_append(NoiaList* self, void* data);

/// Removes and return last element of the list.
void* noia_list_pop(NoiaList* self);

/// Get `n`-th element of the list.
void* noia_list_get_nth(const NoiaList* self, int n);

/// Searches for first occurrence of given data using given compare function and
/// removes it.
NoiaResult noia_list_remove(NoiaList* self,
                            void* data,
                            NoiaCompareFunc compare);

/// Searches for all occurrences of given data using given compare function and
/// removes all of them.
NoiaResult noia_list_remove_all(NoiaList* self,
                                void* data,
                                NoiaCompareFunc compare);

/// Removes all elements and destroys them if data-destructor function was
/// given in list constructor.
/// @see noia_list_new
void noia_list_clean(NoiaList* self);

/// Return a new list composed of elements of `minuent` that are not contained
/// in `subtrahent`.
/// Uses `compare` function to compare elements and `duplicate` to copy them.
/// `duplicate` can be NULL.
NoiaList* noia_list_subtract(const NoiaList* minuend,
                             const NoiaList* subtrahent,
                             NoiaCompareFunc compare,
                             NoiaDuplicateFunc duplicate);

/// Return the length of the list.
static inline unsigned noia_list_len(const NoiaList* self)
{
    NOIA_ENSURE(self, return 0);
    return noia_chain_len(&self->base);
}

/// Recalculate and return the length of the list.
static inline unsigned noia_list_recalculate_length(NoiaList* self)
{
    NOIA_ENSURE(self, return 0);
    return noia_chain_recalculate_length(&self->base);
}

/// Return first element of the list.
static inline void* noia_list_first(const NoiaList* self)
{
    if (self->base.first) {
        return self->base.first->data;
    } else {
        return NULL;
    }
}

/// Return last element of the list.
static inline void* noia_list_last(const NoiaList* self)
{
    if (self->base.last) {
        return self->base.last->data;
    } else {
        return NULL;
    }
}

#endif // NOIA_UTILS_LIST_H

