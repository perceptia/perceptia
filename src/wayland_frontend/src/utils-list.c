// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "utils-list.h"

#include <malloc.h>
#include <memory.h>

//------------------------------------------------------------------------------

NoiaList* noia_list_new(NoiaFreeFunc free_data)
{
    NoiaList* self = malloc(sizeof(NoiaList));
    if (!self) {
        return NULL;
    }

    noia_chain_initialize(&self->base, (NoiaFreeFunc) noia_link_free);
    self->free_data = free_data;
    return self;
}

//------------------------------------------------------------------------------

void noia_list_free(NoiaList* self)
{
    if (!self) {
        return;
    }

    noia_list_clean(self);
    free(self);
}

//------------------------------------------------------------------------------

void noia_list_prepend(NoiaList* self, void* data)
{
    if (!self) {
        return;
    }

    noia_chain_prejoin(&self->base, noia_link_new(data));
}

//------------------------------------------------------------------------------

void noia_list_append(NoiaList* self, void* data)
{
    if (!self) {
        return;
    }

    noia_chain_adjoin(&self->base, noia_link_new(data));
}

//------------------------------------------------------------------------------

void* noia_list_pop(NoiaList* self)
{
    void* result;
    NoiaLink* next;

    if (!self || self->base.len == 0) {
        return NULL;
    }

    result = self->base.first->data;
    next = self->base.first->next;
    noia_link_free(self->base.first);
    self->base.first = next;
    self->base.len -= 1;

    if (self->base.len == 0) {
        self->base.last = NULL;
    }
    return result;
}

//------------------------------------------------------------------------------

void* noia_list_get_nth(const NoiaList* self, int n)
{
    NoiaLink* link = NULL;
    if (n < 0) {
        link = self->base.last;
        for (int i = 1; link && i < -n; ++i, link = link->prev);
    } else {
        link = self->base.first;
        for (int i = 0; link && i < n; ++i, link = link->next);
    }

    if (link) {
        return link->data;
    } else {
        return NULL;
    }
}

//------------------------------------------------------------------------------

NoiaResult noia_list_remove(NoiaList* self, void* data, NoiaCompareFunc compare)
{
    if (!self) {
        return NOIA_RESULT_INCORRECT_ARGUMENT;
    }

    int found = false;
    NoiaLink* link = NULL;
    for (link = self->base.first; link; link = link->next) {
        found = !compare(link->data, data);
        if (found) {
            break;
        }
    }

    if (!found) {
        return NOIA_RESULT_NOT_FOUND;
    }

    NoiaResult result = noia_chain_disjoin(&self->base, link);
    if (result == NOIA_RESULT_SUCCESS) {
        noia_link_destroy(link, self->free_data);
    }
    return result;
}

//------------------------------------------------------------------------------

NoiaResult noia_list_remove_all(NoiaList* self,
                                void* data,
                                NoiaCompareFunc compare)
{
    bool removed_all = false;
    while (!removed_all) {
        NoiaResult result = noia_list_remove(self, data, compare);
        if (result == NOIA_RESULT_NOT_FOUND) {
            removed_all = true;
        } else if (result != NOIA_RESULT_SUCCESS) {
            return result;
        }
    }
    return NOIA_RESULT_SUCCESS;
}

//------------------------------------------------------------------------------

void noia_list_clean(NoiaList* self)
{
    NoiaLink* iter = self->base.first;
    while (iter) {
        NoiaLink* next = iter->next;
        noia_link_destroy(iter, self->free_data);
        iter = next;
    }

    self->base.first = NULL;
    self->base.last = NULL;
    self->base.len = 0;
}

//------------------------------------------------------------------------------

NoiaList* noia_list_subtract(const NoiaList* minuend,
                             const NoiaList* subtrahent,
                             NoiaCompareFunc compare,
                             NoiaDuplicateFunc duplicate)
{
    NoiaList* difference = noia_list_new(minuend->free_data);

    FOR_EACH (minuend, mlink) {
        bool found = false;
        FOR_EACH (subtrahent, slink) {
            found = found || !compare(mlink->data, slink->data);
        }
        if (!found) {
            if (duplicate) {
                noia_list_append(difference, duplicate(mlink->data));
            } else {
                noia_list_append(difference, mlink->data);
            }
        }
    }

    return difference;
}

//------------------------------------------------------------------------------

