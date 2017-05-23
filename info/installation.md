Installation
============

Packages
--------

Up to date list of downloads is available on [wiki downloads
page](https://github.com/perceptia/perceptia/wiki/Downloads).

Build
-----

`perceptia` uses Rusts `cargo` package manager. To build `perceptia` run

```
cargo build --all --release
```

in repo root directory. Binaries will land in `target/release/`. You should be able to find there:

 * `perceptia` - the main display compositor
 * `perceptiactl` - helper diagnostic tool

Dependences
-----------

Required runtime dependencies:

 * libdbus
 * libdrm
 * libgbm
 * libgl, libegl
 * libinput
 * libudev
 * libxkbcommon

Buildtime dependencies:

 * rustc
 * cargo

Configuration
-------------

Configuration is wider topic described in [configuration.md](./configuration.md).

Run
---

For GPU acceleration `perceptia` requires drivers which support KMS (Kernel Mode Setting).

`perceptia` should be run as normal user. It will get access to required resources from `logind`.

To run `perceptia` switch to Linux console e.g. using `ctrl+alt+F6` (make sure it's kernel console,
not userspace console like `kmscon`).

For list of options and environment variables you can refer to [manual page](./manual.adoc).

Bugs
----

In case of **any** problems please make bug report in
[bugtracker](https://github.com/perceptia/perceptia/issues). Providing logs will be helpful. They
can be found in `$XDG_CACHE_HOME/perceptia/`.
