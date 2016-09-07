// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_STORE_H
#define NOIA_STORE_H

#include "global-constants.h"

typedef int (*NoiaStoreValueCompareFunc) (const void*, const void*);
typedef void (*NoiaStoreKeyFreeFunc) (void*);

typedef struct NoiaStorePriv NoiaStore;

NoiaStore* noia_store_new(NoiaStoreValueCompareFunc value_compare_func,
                          NoiaStoreKeyFreeFunc key_free_func);
NoiaStore* noia_store_new_for_id(void);
NoiaStore* noia_store_new_for_str(void);

void noia_store_free(NoiaStore* self);
void noia_store_free_with_items(NoiaStore* self, NoiaFreeFunc free_func);

NoiaItemId noia_store_generate_new_id(NoiaStore* self);

NoiaResult noia_store_add_with_id(NoiaStore* self, NoiaItemId key, void* data);
NoiaResult noia_store_add_with_str(NoiaStore* self,
                                   const char* key,
                                   void* data);

#define noia_store_add(store, key, data) _Generic(key, \
        NoiaItemId:  noia_store_add_with_id, \
        char*:       noia_store_add_with_str, \
        const char*: noia_store_add_with_str) (store, key, data)

void* noia_store_find_with_id(NoiaStore* self, NoiaItemId key);
void* noia_store_find_with_str(NoiaStore* self, const char* key);

#define noia_store_find(store, key) _Generic(key, \
        NoiaItemId:  noia_store_find_with_id, \
        const char*: noia_store_find_with_str) (store, key)

void* noia_store_delete_with_id(NoiaStore* self, NoiaItemId key);
void* noia_store_delete_with_str(NoiaStore* self, const char* key);

#define noia_store_delete(store, key) _Generic(key, \
        NoiaItemId:  noia_store_delete_with_id, \
        const char*: noia_store_delete_with_str) (store, key)

#endif // NOIA_STORE_H

