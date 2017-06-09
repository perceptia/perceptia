Contributing
============

You probably have read [installation.md](./installation.md) and know how to build, run and configure
`perceptia`. Have suggestions? Want to help? Here's how to:

 * **bug reports**

   All users are encouraged to fill bug reports for everything: bugs, features, suggestions, request
   for more documentation, changes in documentation, enhancements... whatever that should be done.

   Bug tracker can be found on [github](https://github.com/perceptia/perceptia/issues).

 * **idea selling**

   Any idea for improvements, feature proposals are very welcome. `perceptia` is in very early stage
   of development, so ground breaking ideas are still relatively easy to implement. On the other
   hand feature you need may be already planned. On github you can brainstorm the ideas and vote for
   issues you are interested in to indicate they should be implemented with higher priority.

 * **artworks**

 * **coding**

   Rust was chosen as base language. It is modern, safe system language with relatively good access
   to system API, syntax similar to C/C++ and great community and tools.

   There is plan to be able to write configuration and plug-ins in Python or Lua.

   Source code is available on [github](https://github.com/perceptia/perceptia/) and can be fetched
   using command
   ```
   git clone git@github.com:perceptia/perceptia.git
   ```

   If you need quick overview of architecture see [concepts.md](./concepts.md).

Future
------

List of features planed for nearest releases can be found on [wiki TODO
page](https://github.com/perceptia/perceptia/wiki/TODO).

---

Later parts of this file focus on tips for developers.

Directories
-----------

Here is overview of directories (documentation in code will provide more details):

 * **cognitive** - source code of all general purpose crates for creating surface compositors

 * **perceptia** - source code of `perceptia`

 * **\*/\*/tests** - source code of unit tests

 * **info** - hand-written documentation

 * **target** - directory containing build artifacts created by `cargo`

Sources are split into several crats:

 * **cognitive/dharma** - managing threads and inter-thread communication

 * **cognitive/timber** - logging helper

 * **cognitive/graphics** - common tools related to hardware graphics

 * **cognitive/qualia** - common definitions and traits

 * **cognitive/renderer_gl** - rendering using GL

 * **cognitive/inputs** - input related functionality

 * **cognitive/outputs** - output related functionality

 * **cognitive/frames** - framing

 * **cognitive/exhibitor** - managing frames, displays and other compositing related stuff

 * **cognitive/aesthetics** - functionality related to visual appearance. Aesthetics uses the same
   API as exposed to client frontends.

 * **cognitive/device_manager** - device management functionality

 * **cognitive/wayland_frontend** - Wayland related functionality

 * **perceptia/skylane_simple_framework** - example how `skylane` client could be written; used by
   perceptiactl

 * **perceptia/gears** - various tools without better place to put them in

 * **perceptia/coordination** - management of shared state and inter-thread communication

 * **perceptia/perceptia** - `perceptia` main application

 * **perceptia/perceptiactl** - supportive, diagnostic application

Run unit tests
--------------

Running unit test is as simple as
```
cargo test --all
```

Generate documentation
----------------------

Command
```
cargo doc --all
```
will produce documentation for whole program and its dependencies.

Code conventions
----------------

Using code conventions is important because it homogenizes source code making it more predictable
for the reader and therefore easier to understand.

`perceptia` tries to adhere to Rusts coding guidelines. Here are some conventions:

 * keep line width up to 100 characters,
 * keep file length up to 999 lines,
 * use
   - `lower_snake_case` for functions, macros, variable and member names,
   - `UPPER_SNAKE_CASE` for constants,
   - `UpperCamelCase` for data types and structure names,
 * use four spaces for indentation (tabs make no sense with line width limit),
 * do not use block comments (these are reserved for debugging),
 * divide bigger code blocks with horizontal rule (this greatly increases readability),
 * even if code is clear use comments to separate code blocks visually,
 * document everything

Git work flow and versioning
----------------------------

`perceptia` is hosted on GitHub and uses GitHub flow (pull requests) but patches sent by mail will
also be accepted.

Active development takes place on subbranches. Main development branch is `development`. On `master`
land only working features; `master` should always be buildable and usable. Every release has
version and codename. Version is used to tag commit. Code name is used for branch. If bugs were
found fixes are made on this branch, then merged to `development`, then to `master`.

Version string consists of three numbers: EPOCH.MAJOR.MINOR
 * MINOR - bug-fixed release
 * MAJOR - normal release (codenamed)
 * EPOCH - version containing revolutionary changes opening new chapter in history

Make sure every commit compiles - this may make life easier when searching for commit that
introduced bug.

Related projects
----------------

[`skylane`](https://github.com/perceptia/skylane) is implementation of Wayland protocol written from
scratch in Rust. It originated in `perceptia` but was moved away as separate crate and repository
to be able to be used independently from `perceptia`.
