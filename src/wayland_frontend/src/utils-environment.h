// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_UTILS_ENVIRONMENT_H
#define NOIA_UTILS_ENVIRONMENT_H

#include "global-constants.h"

#define __USE_GNU
#define _GNU_SOURCE
#include <pthread.h>

/// This enum defines a directories for storing files.
/// @see noia_environment_open_file
typedef enum NoiaFilePath {
    RUNTIME_PATH, ///< '$XDG_RUNTIME_DIR/noia-XXXXXX'
    DATA_PATH     ///< '$XDG_DATA_HOME/noia'
} NoiaPath;

/// Block SIGINT and SIGTERM to make sure these signals will be handled by main
/// thread.
/// @see `noia_environment_on_enter_new_thread`
void noia_environment_block_system_signals(void);

/// Unblock SIGINT and SIGTERM.
void noia_environment_unblock_system_signals(void);

/// Set thread name.
/// @see `noia_environment_on_enter_new_thread`
void noia_environment_set_thread_name(pthread_t thread, char* name);

/// This function call `noia_environment_block_system_signals` and
/// `noia_environment_set_thread_name`
/// @note This function should be called at entry to every newly created thread.
/// @see noia_environment_block_system_signals noia_environment_set_thread_name
void noia_environment_on_enter_new_thread(pthread_t thread, char* name);

/// Set up signal handlers; create data and runtime directories; open log file.
NoiaResult noia_environment_setup(const char* log_filename);

/// Free memory and close log file.
void noia_environment_cleanup(void);

/// Create and open file in predefined directory.
/// @param file_name - file name.
/// @param size - if not zero, memory of size `size` will be allocated for this
///               file (usefull for mmap).
/// @param path - describes where the file should be created.
/// @see NoiaPath
int noia_environment_open_file(const char *file_name,
                               unsigned size,
                               NoiaPath path);

#endif // NOIA_UTILS_ENVIRONMENT_H

