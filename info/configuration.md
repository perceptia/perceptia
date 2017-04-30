Configuration
=============

There will be three levels of configuration:

 * build time configuration - where user can chose program configuration which will be compiled into
   the program without need for additional config files.
   (Ultimately similar to `dwm`'s configuration.)

 * static text config file - basically the same options as in build time configuration but read from
   text file in run time.

 * plugable modules written in scripting language (Python? Lua?) - **not yet implemented**.
   (Ultimately similar to `awesome`'s configuration.)

Build-time
----------

Currently there is only a few simple options that can be set in `src/qualia/config_defaults.rs`
file. In future it is planned to add compositor strategies.

Static text
-----------

Text config files use YAML format. YAML is human friendly data serialization standard.

`perceptiactl` provides `verify-config` subcommand which verifies correctness of configuration files
and prints effective configuration.

Here is example configuration:

```
aesthetics:
  background_path: /home/user/bg.jpg
input:
  touchpad_scale: 0.5
  touchpad_pressure_threshold: 50
  mouse_scale: 0.5
```

List of all available options:

 * `aesthetics`
   - `background_path` - path to background image file
 * `input`
   - `touchpad_scale` - value by which touchpad move events will be scaled (the smaller the pointer
     moves slower)
   - `touchpad_pressure_threshold` - touchpad events with pressure below this value will be ignored
   - `mouse_scale` - value by which mouse move events will be scaled (the smaller the pointer moves
     slower)

Scripting language
------------------

**Not yet implemented**.

Default configuration
=====================

This section presents set of default settings. Reader should be familiar with
[concepts.md](./concepts.md) (frame layouts, key modes, framing).

Global bindings:

 * `[meta]+[ctrl]+[esc]` - quit application

Insert mode:

 * `[meta]+[esc]` - swap to normal mode

 * `[meta]+[_X_ arrow]` - focus frame in `_X_` direction from currently focused one

 * `[meta]+[tab]` - circle history forward

 * `[meta]+[shift]+[tab]` - circle history backward

 * `[meta]+[_N_]` - focus workspace number `_N_`

 * `[meta]+[shift]+[_X_ arrow]` - jump focused frame in `_X_` direction

 * `[meta]+[shift]+[ctrl]+[_N_]` - jump focused frame to workspace number `_N_`

 * `[meta]+[alt]+[_X_ arrow]` - dive focused frame in `_X_` direction

 * `[meta]+[alt]+[shift]+[ctrl]+[_N_]` - dive focused frame to workspace number `_N_`

 * `[meta]+[home]`, `[meta]+[end]` - exalt/ramify focused frame

Normal mode:

 * `[esc]` - clean the command

 * `[i]`, `[space]` - swap to insert mode

 * `[h]`, `[v]`, `[s]` - make layout of focused frame horizontal, vertical or stacked

 * `[f]`, `[shift]+[f]`, `[j]`, `[d]` - indicate focus/swap/jump/dive action

 * `[home]`, `[end]` - indicate begin/end directions

 * `[_X_ arrow]` - indicate `_X_` direction

 * `[page up]`, `[page down]` - indicate direction forward/backward in time

For example `[f] [right arrow]` will focus surface on the right from focussed one.

Built-ins:

 * `[ctrl]+[alt]+[F_X_]` - switch to virtual terminal `_X_`
