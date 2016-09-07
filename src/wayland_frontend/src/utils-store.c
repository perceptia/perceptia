// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "utils-store.h"
#include "global-macros.h"

#include <malloc.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#define __USE_GNU
#define _GNU_SOURCE
#include <search.h>

struct NoiaStorePriv {
    void* root;
    NoiaStoreValueCompareFunc compare_value;
    NoiaStoreKeyFreeFunc free_key;
    pthread_mutex_t mutex;
};

//------------------------------------------------------------------------------

/// Compare two NoiaItems using identifies.
int noia_store_id_compare(const void* data1, const void* data2)
{
    NoiaItemId id1 = ((NoiaItem*) data1)->id;
    NoiaItemId id2 = ((NoiaItem*) data2)->id;

    if (id1 < id2) return -1;
    if (id1 > id2) return  1;
    return 0;
}

//------------------------------------------------------------------------------

/// Compare two NoiaItems using strings.
int noia_store_str_compare(const void* data1, const void* data2)
{
    char* str1 = ((NoiaItem*) data1)->str;
    char* str2 = ((NoiaItem*) data2)->str;
    return strcmp(str1, str2);
}

//------------------------------------------------------------------------------

/// Tree destroy action used to free duplicated id-keys when freeing store.
void noia_store_destroy_id_key(void* data NOIA_UNUSED)
{
    // Nothing to do
}

//------------------------------------------------------------------------------

/// Tree destroy action used to free duplicated string-keys when freeing store.
void noia_store_destroy_string_key(void* data)
{
    NoiaItem* item = (NoiaItem*) data;
    if (item and item->str) {
        free(item->str);
    }
}

//------------------------------------------------------------------------------

/// Allocate memory for new NoiaStore with arbitrary compare function.
NoiaStore* noia_store_new(NoiaStoreValueCompareFunc value_compare_func,
                          NoiaStoreKeyFreeFunc key_free_func)
{
    NoiaStore* self = malloc(sizeof(NoiaStore));
    NOIA_ENSURE(self, abort());

    self->root = NULL;
    self->compare_value = value_compare_func;
    self->free_key = key_free_func;
    pthread_mutex_init(&self->mutex, NULL);
    return self;
}

//------------------------------------------------------------------------------

/// Allocate memory for new NoiaStore that uses IDs to distinguish items.
NoiaStore* noia_store_new_for_id(void)
{
    return noia_store_new(noia_store_id_compare, noia_store_destroy_id_key);
}

//------------------------------------------------------------------------------

/// Allocate memory for new NoiaStore that uses strings to distinguish items.
NoiaStore* noia_store_new_for_str(void)
{
    return noia_store_new(noia_store_str_compare,
                          noia_store_destroy_string_key);
}

//------------------------------------------------------------------------------

/// Free store without freeing stored items.
void noia_store_free(NoiaStore* self)
{
    NOIA_ENSURE(self, return);

    pthread_mutex_lock(&self->mutex);
    if (self->free_key) {
        tdestroy(self->root, self->free_key);
    }
    pthread_mutex_unlock(&self->mutex);
    memset(self, 0, sizeof(NoiaStore));
    free(self);
}

//------------------------------------------------------------------------------

/// Free store and stored items.
void noia_store_free_with_items(NoiaStore* self, NoiaFreeFunc free_func)
{
    NOIA_ENSURE(self, return);

    if (self->free_key) {
        tdestroy(self->root, free_func);
    }
    memset(self, 0, sizeof(NoiaStore));
    free(self);
}

//------------------------------------------------------------------------------

/// Generate new ID that is not yet present in store.
NoiaItemId noia_store_generate_new_id(NoiaStore* self)
{
    NOIA_ENSURE(self, return scInvalidItemId);

    pthread_mutex_lock(&self->mutex);
    NoiaItem item;
    do {
        item.id = (NoiaItemId) rand() & NOIA_RANDOM_MASK;
    } while ((item.id == scInvalidItemId)
      or (tfind((void *) &item, &self->root, self->compare_value) != NULL));

    pthread_mutex_unlock(&self->mutex);
    return item.id;
}

//------------------------------------------------------------------------------

/// Store item using ID.
/// @param key - ID used as a key
/// @param data - item to be stored
NoiaResult noia_store_add_with_id(NoiaStore* self, NoiaItemId key, void* data)
{
    NoiaResult result = NOIA_RESULT_NOT_FOUND;
    NOIA_ENSURE(self, return NOIA_RESULT_INCORRECT_ARGUMENT);

    NoiaItem* item = (NoiaItem*) data;
    item->id = key;

    pthread_mutex_lock(&self->mutex);
    if (tsearch(item, &self->root, self->compare_value)) {
        result = NOIA_RESULT_SUCCESS;
    }
    pthread_mutex_unlock(&self->mutex);
    return result;
}

//------------------------------------------------------------------------------

/// Store item using string.
/// The string used as a key is duplicated.
/// @param key - string used as a key
/// @param data - item to be stored
NoiaResult noia_store_add_with_str(NoiaStore* self, const char* key, void* data)
{
    NoiaResult result = NOIA_RESULT_NOT_FOUND;
    NOIA_ENSURE(self, return NOIA_RESULT_INCORRECT_ARGUMENT);

    NoiaItem* item = (NoiaItem*) data;
    item->str = strdup(key);

    pthread_mutex_lock(&self->mutex);
    if (tsearch(item, &self->root, self->compare_value)) {
        result = NOIA_RESULT_SUCCESS;
    }
    pthread_mutex_unlock(&self->mutex);
    return result;
}

//------------------------------------------------------------------------------

#define noia_store_find_template(KEYTYPE) \
    if (!self) { return NULL; } \
    NoiaItem item; item.KEYTYPE = key; \
    pthread_mutex_lock(&self->mutex); \
    void** result = tfind((void*) &item, &self->root, self->compare_value); \
    pthread_mutex_unlock(&self->mutex); \
    if (result) { return *result; } \
    return NULL;

//------------------------------------------------------------------------------

/// Store item using ID.
/// @param key - ID used to reference an item
/// @return pointer to found item or null if nothing found
void* noia_store_find_with_id(NoiaStore* self, NoiaItemId key)
{
    noia_store_find_template(id);
}

//------------------------------------------------------------------------------

/// Store item using string.
/// @param key - string used to reference an item
/// @return pointer to found item or null if nothing found
void* noia_store_find_with_str(NoiaStore* self, const char* const_key)
{
    // Const cats just tu satify compiler.
    char* key = (char*) const_key;
    noia_store_find_template(str);
}

//------------------------------------------------------------------------------

/// Delete an item using ID.
/// @param key - ID used to reference an item
/// @return pointer to found item or null if nothing found
void* noia_store_delete_with_id(NoiaStore* self, NoiaItemId key)
{
    void* result = NULL;
    NOIA_ENSURE(self, return result);

    NoiaItem* item = noia_store_find(self, key);
    pthread_mutex_lock(&self->mutex);
    if (tdelete(item, &self->root, self->compare_value)) {
        result = item;
    }
    pthread_mutex_unlock(&self->mutex);
    return item;
}

//------------------------------------------------------------------------------

/// Delete an item using string.
/// @param key - string used to reference an item
/// @return pointer to found item or null if nothing found
void* noia_store_delete_with_str(NoiaStore* self, const char* key)
{
    void* result = NULL;
    NOIA_ENSURE(self, return result);

    NoiaItem* item = noia_store_find(self, key);
    pthread_mutex_lock(&self->mutex);
    if (tdelete(item, &self->root, self->compare_value)) {
        result = item;
    }
    pthread_mutex_unlock(&self->mutex);

    if (result) {
        free(item->str);
        item->str = NULL;
    }
    return result;
}

//------------------------------------------------------------------------------

