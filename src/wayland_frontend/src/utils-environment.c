// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

/// @file
/// No unit tests needed.

#include "utils-environment.h"
#include "utils-log.h"
#include "global-macros.h"

#include <stdlib.h>
#include <memory.h>
#include <string.h>
#include <fcntl.h>
#include <signal.h>
#include <sys/stat.h>
#include <sys/types.h>

static const char scRuntimeDirTemplate[] = "/noia-XXXXXX";
static const char scDataDirTemplate[] = "/noia";

static char* sNoiaDataPath = NULL;
static char* sNoiaRuntimePath = NULL;

//------------------------------------------------------------------------------

void noia_environment_block_system_signals(void)
{
    sigset_t mask;
    sigemptyset(&mask);
    sigaddset(&mask, SIGINT);
    sigaddset(&mask, SIGTERM);
    sigprocmask(SIG_BLOCK, &mask, NULL);
}

//------------------------------------------------------------------------------

void noia_environment_unblock_system_signals(void)
{
    sigset_t mask;
    sigemptyset(&mask);
    sigaddset(&mask, SIGINT);
    sigaddset(&mask, SIGTERM);
    sigprocmask(SIG_UNBLOCK, &mask, NULL);
}

//------------------------------------------------------------------------------

void noia_environment_set_thread_name(pthread_t thread, char* name)
{
    if (not thread) {
        thread = pthread_self();
    }
    /// @note Linux thread name is up to 15 characters.
    if (strlen(name) > 15) {
        LOG_WARN1("Thread name '%s' is too long!", name);
    }
    pthread_setname_np(thread, name);
}

//------------------------------------------------------------------------------

void noia_environment_on_enter_new_thread(pthread_t thread, char* name)
{
    noia_environment_block_system_signals();
    noia_environment_set_thread_name(thread, name);
}

//------------------------------------------------------------------------------

/// Handle system signals.
/// @see noia_event_dispatcher_default_signal_handler
void noia_environment_async_signal_handler(int sig,
                                           siginfo_t* si NOIA_UNUSED,
                                           void* arg     NOIA_UNUSED)
{
    switch (sig) {
        case SIGINT:
        case SIGTERM:
        case SIGSEGV:
        case SIGABRT:
            LOG_INFO1("Signal '%d' received asynchronously", sig);
            noia_log_backtrace();
            exit(1);
        default:
            LOG_INFO2("Unhandled signal: '%d'", sig);
    }
}

//------------------------------------------------------------------------------

/// Set up signal handlers.
void noia_environment_signal_handler_set_up(void)
{
    struct sigaction sa;
    memset(&sa, 0, sizeof(struct sigaction));
    sigemptyset(&sa.sa_mask);

    sa.sa_sigaction = noia_environment_async_signal_handler;
    sa.sa_flags = SA_SIGINFO;

    sigaction(SIGINT,  &sa, NULL);
    sigaction(SIGTERM, &sa, NULL);
    sigaction(SIGSEGV, &sa, NULL);
    sigaction(SIGABRT, &sa, NULL);
}

//------------------------------------------------------------------------------

/// Make a directory if not exists.
NoiaResult noia_environment_mkdir(char* dir_name)
{
    struct stat st;
    NoiaResult result = NOIA_RESULT_SUCCESS;
    if (stat(dir_name, &st) == -1) {
        if (mkdir(dir_name, 0700) == -1) {
            LOG_ERROR("Failed to make directory '%s'! (%m)", dir_name);
            result = NOIA_RESULT_ERROR;
        }
    }
    return result;
}

//------------------------------------------------------------------------------

/// Create data directory.
/// '$XDG_DATA_HOME/noia' or '/tmp' if environment variable not provided.
NoiaResult noia_environment_data_path_setup(void)
{
    // Choose directory
    char* data_path = getenv("XDG_DATA_HOME");
    if (not data_path) {
        data_path = "/tmp";
    }

    sNoiaDataPath = malloc(strlen(data_path) + sizeof(scDataDirTemplate));
    NOIA_ENSURE(sNoiaDataPath, abort());

    strcpy(sNoiaDataPath, data_path);
    strcat(sNoiaDataPath, scDataDirTemplate);

    // Create subdirectories
    return noia_environment_mkdir(sNoiaDataPath);
}

//------------------------------------------------------------------------------

/// Create runtime directory.
/// '$XDG_RUNTIME_DIR/noia' or '/tmp' if environment variable not provided.
NoiaResult noia_environment_runtime_path_setup(void)
{
    NoiaResult result = NOIA_RESULT_SUCCESS;

    const char* runtime_path = getenv("XDG_RUNTIME_DIR");
    if (not runtime_path) {
        runtime_path = "/tmp";
    }

    char* noia_runtime_path_template =
                    malloc(strlen(runtime_path) + sizeof(scRuntimeDirTemplate));
    NOIA_ENSURE(noia_runtime_path_template, abort());

    strcpy(noia_runtime_path_template, runtime_path);
    strcat(noia_runtime_path_template, scRuntimeDirTemplate);

    sNoiaRuntimePath = mkdtemp(noia_runtime_path_template);
    if (not sNoiaRuntimePath) {
        LOG_WARN1("Failed to create runtime directory (template: '%s')",
                  noia_runtime_path_template);
        free(noia_runtime_path_template);
        result = NOIA_RESULT_ERROR;
    }

    return result;
}

//------------------------------------------------------------------------------

NoiaResult noia_environment_setup(const char* log_filename)
{
    // Set up async signal handler
    noia_environment_signal_handler_set_up();

    // Create $XDG_DATA_HOME/noia directory
    NoiaResult result1 = noia_environment_data_path_setup();

    // Create temporary $XDG_RUNTIME_DIR/noia-XXXXXX directory
    NoiaResult result2 = noia_environment_runtime_path_setup();

    // Open log file
    noia_log_initialize(log_filename);

    LOG_INFO1("Data path: '%s'", sNoiaDataPath);
    LOG_INFO1("Runtime path: '%s'", sNoiaRuntimePath);

    if ((result1 != NOIA_RESULT_SUCCESS)
     or (result2 != NOIA_RESULT_SUCCESS)) {
        return NOIA_RESULT_ERROR;
    }
    return NOIA_RESULT_SUCCESS;
}

//------------------------------------------------------------------------------

void noia_environment_cleanup(void)
{
    if (sNoiaRuntimePath) {
        free(sNoiaRuntimePath);
        sNoiaRuntimePath = NULL;
    }

    if (sNoiaDataPath) {
        free(sNoiaDataPath);
        sNoiaDataPath = NULL;
    }

    noia_log_finalize();
}

//------------------------------------------------------------------------------

int noia_environment_open_file(const char *file_name,
                               unsigned size,
                               NoiaPath path)
{
    int fd = -1;
    char* file_path = NULL;

    NOIA_BLOCK {
        char* base_path;
        switch (path) {
        case RUNTIME_PATH:       base_path = sNoiaRuntimePath; break;
        case DATA_PATH: default: base_path = sNoiaDataPath;    break;
        }

        file_path = malloc(strlen(base_path) + strlen(file_name) + 2);
        NOIA_ENSURE(file_path, abort());

        strcpy(file_path, base_path);
        strcat(file_path, "/");
        strcat(file_path, file_name);

        fd = open(file_path, O_RDWR|O_CREAT|O_APPEND, S_IRUSR|S_IWUSR);
        if (fd < 0) {
            LOG_ERROR("Creating file '%s' failed! (%m)", file_path);
            break;
        }

        if (size > 0) {
            posix_fallocate(fd, 0, size);
        }
    }

    if (file_path) {
        free(file_path);
    }
    return fd;
}

//------------------------------------------------------------------------------

