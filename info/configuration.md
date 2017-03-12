Configuration
=============

There will be three levels of configuration:

 * build time configuration - where user can chose program configuration which will be compiled into
   the program without need for additional config files.
   (Ultimately similar to `dwm`'s configuration.)

 * plain text config file - **not yet implemented** - basically the same options as in build time
   configuration but read from text file in run time.

 * plugable modules written in scripting language (Python? Lua?) - **not yet implemented**.
   (Ultimately similar to `awesome`'s configuration.)

Build-time
----------

Currently there is only a few simple options that can be set in `src/qualia/config.rs` file. In
future it is planned to add compositor strategies.

Plain text
----------

**Not yet implemented**.

Scripting language
------------------

**Not yet implemented**.

