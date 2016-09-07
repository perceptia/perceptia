// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_UTILS_CHAIN_H
#define NOIA_UTILS_CHAIN_H

#include "global-constants.h"

/// Link for storing data and joining in chain.
/// @see NoiaChain
typedef struct NoiaLink NoiaLink;
struct NoiaLink {
    NoiaLink* prev;
    NoiaLink* next;
    void* data;
};

/// Construct new link.
NoiaLink* noia_link_new(void* data);

/// Free link without destroying data.
/// @see noia_link_destroy
void noia_link_free(NoiaLink* link);

/// Free link together with stored data.
void noia_link_destroy(NoiaLink* link, NoiaFreeFunc free_data);

/// Initialize link.
void noia_link_initialize(NoiaLink* link, void* data);

/// Data type for storing data as doubly linked list.
/// Data should be stored in NoiaLink.data or inherit NoiaLink.
/// @see NoiaList NoiaBranch
typedef struct {
    NoiaLink* first;
    NoiaLink* last;
    unsigned len;
    NoiaFreeFunc free_link;
} NoiaChain;

/// Create new chain.
/// @param free_link - function used to free data. Can be `NULL`. If `free_link`
///                    is not `NULL` all data should have the same type.
NoiaChain* noia_chain_new(NoiaFreeFunc free_link);

/// Free chain, links and all data using `free_link` function passed to
/// `noia_chain_new`.
void noia_chain_free(NoiaChain* self);

/// Initialize newly created chain.
void noia_chain_initialize(NoiaChain* self, NoiaFreeFunc free_link);

/// Get length of the chain.
unsigned noia_chain_len(const NoiaChain* self);

/// Recalculate length of the chain (for debugging purposes).
unsigned noia_chain_recalculate_length(NoiaChain* self);

/// Add new link `link` at the begin of the chain `self`.
NoiaResult noia_chain_prejoin(NoiaChain* self, NoiaLink* link);

/// Add new link `link` at the end of the chain `self`.
NoiaResult noia_chain_adjoin(NoiaChain* self, NoiaLink* link);

/// Add new link `link` just after existing link `onto`.
/// If length of chain is zero then `onto` parameter is ignored,
/// otherwise if must be part of the chain.
NoiaResult noia_chain_prejoin_onto(NoiaChain* self,
                                   NoiaLink* link,
                                   NoiaLink* onto);

/// Add new link `link` just after existing link `onto`.
/// If length of chain is zero then `onto` is ignored,
/// otherwise if must be part of the chain.
NoiaResult noia_chain_adjoin_onto(NoiaChain* self,
                                  NoiaLink* link,
                                  NoiaLink* onto);

/// Check if link `link` is contained in chain `self` and remove it.
NoiaResult noia_chain_unjoin(NoiaChain* self, NoiaLink* link);

/// Remove link `link` from chain `self` without any safety checks.
NoiaResult noia_chain_disjoin(NoiaChain* self, NoiaLink* link);

/// Free all links contained in chain if free function provided.
NoiaResult noia_chain_clean(NoiaChain* self);

#endif // NOIA_UTILS_CHAIN_H

