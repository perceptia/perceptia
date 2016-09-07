// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "utils-chain.h"

#include <malloc.h>
#include <memory.h>

//------------------------------------------------------------------------------

NoiaLink* noia_link_new(void* data)
{
    NoiaLink* self = malloc(sizeof(NoiaLink));
    noia_link_initialize(self, data);
    return self;
}

//------------------------------------------------------------------------------

void noia_link_free(NoiaLink* self)
{
    if (!self) {
        return;
    }
    memset(self, 0, sizeof(NoiaLink));
    free(self);
}

//------------------------------------------------------------------------------

void noia_link_destroy(NoiaLink* self, NoiaFreeFunc free_data)
{
    if (!self) {
        return;
    }
    if (free_data) {
        free_data(self->data);
    }
    memset(self, 0, sizeof(NoiaLink));
    free(self);
}

//------------------------------------------------------------------------------

void noia_link_initialize(NoiaLink* self, void* data)
{
    if (!self) {
        return;
    }
    self->prev = NULL;
    self->next = NULL;
    self->data = data;
}

//------------------------------------------------------------------------------

NoiaChain* noia_chain_new(NoiaFreeFunc free_link)
{
    NoiaChain* self = malloc(sizeof(NoiaChain));
    if (!self) {
        return NULL;
    }

    noia_chain_initialize(self, free_link);
    return self;
}

//------------------------------------------------------------------------------

void noia_chain_free(NoiaChain* self)
{
    if (!self) {
        return;
    }

    noia_chain_clean(self);
    free(self);
}

//------------------------------------------------------------------------------

void noia_chain_initialize(NoiaChain* self, NoiaFreeFunc free_link)
{
    if (!self) {
        return;
    }

    self->first = NULL;
    self->last = NULL;
    self->len = 0;
    self->free_link = free_link;
}

//------------------------------------------------------------------------------

unsigned noia_chain_len(const NoiaChain* self)
{
    if (!self) {
        return 0;
    }

    return self->len;
}

//------------------------------------------------------------------------------

unsigned noia_chain_recalculate_length(NoiaChain* self)
{
    unsigned len = 0;
    for (NoiaLink* link = self->first; link; link = link->next) {
        len += 1;
    }
    self->len = len;
    return len;
}

//------------------------------------------------------------------------------

NoiaResult noia_chain_add_first(NoiaChain* self, NoiaLink* link)
{
    NoiaResult result = NOIA_RESULT_SUCCESS;

    if (!self) {
        result = NOIA_RESULT_INCORRECT_ARGUMENT;
    } else {
        self->first = link;
        self->last = link;
        link->prev = NULL;
        link->next = NULL;
        self->len = 1;
    }

    return result;
}

//------------------------------------------------------------------------------

NoiaResult noia_chain_prejoin(NoiaChain* self, NoiaLink* link)
{
    NoiaResult result = NOIA_RESULT_SUCCESS;

    if (!self || !link) {
        result = NOIA_RESULT_INCORRECT_ARGUMENT;
    } else if (self->len == 0) {
        result = noia_chain_add_first(self, link);
    } else {
        link->next = self->first;
        link->prev = NULL;
        self->first->prev = link;
        self->first = link;
        self->len += 1;
    }

    return result;
}

//------------------------------------------------------------------------------

NoiaResult noia_chain_adjoin(NoiaChain* self, NoiaLink* link)
{
    NoiaResult result = NOIA_RESULT_SUCCESS;

    if (!self || !link) {
        result = NOIA_RESULT_INCORRECT_ARGUMENT;
    } else if (self->len == 0) {
        result = noia_chain_add_first(self, link);
    } else {
        link->next = NULL;
        link->prev = self->last;
        self->last->next = link;
        self->last = link;
        self->len += 1;
    }

    return result;
}

//------------------------------------------------------------------------------

NoiaResult noia_chain_prejoin_onto(NoiaChain* self,
                                   NoiaLink* link,
                                   NoiaLink* onto)
{
    NoiaResult result = NOIA_RESULT_SUCCESS;

    if (!self || !link || (self->len != 0 && !onto)) {
        result = NOIA_RESULT_INCORRECT_ARGUMENT;
    } else if (self->len == 0 || onto == self->first) {
        result = noia_chain_prejoin(self, link);
    } else {
        link->prev = onto->prev;
        link->next = onto;
        onto->prev->next = link;
        onto->prev = link;
        self->len += 1;
    }

    return result;
}

//------------------------------------------------------------------------------

NoiaResult noia_chain_adjoin_onto(NoiaChain* self,
                                  NoiaLink* link,
                                  NoiaLink* onto)
{
    NoiaResult result = NOIA_RESULT_SUCCESS;

    if (!self || !link || (self->len != 0 && !onto)) {
        result = NOIA_RESULT_INCORRECT_ARGUMENT;
    } else if (self->len == 0 || onto == self->last) {
        result = noia_chain_adjoin(self, link);
    } else {
        link->next = onto->next;
        link->prev = onto;
        onto->next->prev = link;
        onto->next = link;
        self->len += 1;
    }

    return result;
}

//------------------------------------------------------------------------------

NoiaResult noia_chain_unjoin(NoiaChain* self, NoiaLink* unjoinee)
{
    if (!self || !unjoinee) {
        return NOIA_RESULT_INCORRECT_ARGUMENT;
    }

    int found = false;
    NoiaLink* link = NULL;
    for (link = self->first; link; link = link->next) {
        found = (link == unjoinee);
        if (found) {
            break;
        }
    }

    if (!found) {
        return NOIA_RESULT_NOT_FOUND;
    }

    return noia_chain_disjoin(self, link);
}

//------------------------------------------------------------------------------

NoiaResult noia_chain_disjoin(NoiaChain* self, NoiaLink* link)
{
    if (!self || !link) {
        return NOIA_RESULT_INCORRECT_ARGUMENT;
    }

    NoiaLink* prev = link->prev;
    NoiaLink* next = link->next;

    if (prev) {
        prev->next = next;
    } else {
        self->first = next;
    }

    if (next) {
        next->prev = prev;
    } else {
        self->last = prev;
    }

    link->prev = NULL;
    link->next = NULL;
    self->len = noia_chain_recalculate_length(self);
    return NOIA_RESULT_SUCCESS;
}

//------------------------------------------------------------------------------

NoiaResult noia_chain_clean(NoiaChain* self)
{
    if (!self) {
        return NOIA_RESULT_INCORRECT_ARGUMENT;
    }

    if (self->free_link) {
        NoiaLink* link = self->first;
        while (link) {
            NoiaLink* next = link->next;
            self->free_link(link);
            link = next;
        }
    }

    self->first = NULL;
    self->last = NULL;
    self->len = 0;

    return NOIA_RESULT_SUCCESS;
}

//------------------------------------------------------------------------------

