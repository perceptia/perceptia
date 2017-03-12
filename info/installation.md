Installation
============

Build
-----

`perceptia` uses Rusts `cargo` package manager. To build `perceptia` go to `src/perceptia`
directory and then

```
cargo build
```

`perceptia` does not yet provide packages.

Dependences
-----------

Required runtime dependences:

 * libdbus
 * libdrm
 * libgbm
 * libgl, libegl
 * libinput
 * libudev
 * libxkbcommon

Buildtime dependences:

 * rustc
 * cargo

Configuration
-------------

Configuration is wider topic described in [configuration.md](./configuration.md).

Run
---

To run `perceptia` switch to Linux console e.g. using `ctrl+alt+F6` (make sure it's kernel console,
not userspace console like `kmscon`).

For GPU acceleration `perceptia` requires drivers which support KMS (Kernel Mode Setting). Without
acceleration performance will be very poor.

In case of **any** problems please make bug report in
[bugtracker](https://github.com/perceptia/perceptia/issues). Providing logs will be helpful. They
can be found in `$XDG_RUNTIME_DIR/perceptia/log`.

For list of options and environment variables you can refer to [manual page](./manual.adoc).
