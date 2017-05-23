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
exhibitor:
  compositor:
    move_step: 10
  strategist:
    choose_target: anchored_but_popups
input:
  touchpad_scale: 0.5
  touchpad_pressure_threshold: 50
  mouse_scale: 0.5
keyboard:
  layout: pl
keybindings:
  insert:
    - key: W
      mods: [lctl, lalt]
      execute: [epiphany]
    - key: S
      mods: [lctl, lalt]
      execute: ["perceptiactl", "screenshot"]
    - key: space
      mods: [lalt]
      action: toggle_anchorization
```

List of all available options:

 * `aesthetics`
    - `background_path` - path to background image file
 * `exhibitor`
    * `compositor`
       - `move_step` - distance in pixels by which frames are moved by `move` command
    * `strategist` - changes strategies `compositor` uses to manager surfaces
       - `choose_target` - decides if new surface should be anchored or not and where to be placed.
         Possible values:
          - `always_floating` - (default) every surface will be floating
          - `anchored_but_popups` - every surface except popups should be anchored
       - `choose_floating` - is surface is mean to be floating - decide where to place it. Possible
         values:
          - `always_centered` - always at the center of current worspace
          - `random` - (default) random place on current workspace
 * `input`
    - `touchpad_scale` - value by which touchpad move events will be scaled (the smaller the pointer
      moves slower)
    - `touchpad_pressure_threshold` - touchpad events with pressure below this value will be ignored
    - `mouse_scale` - value by which mouse move events will be scaled (the smaller the pointer moves
      slower)
 * `keyboard` - keyboard configuration for clients
    - `layout` - keyboard layout (e.g. "us", "pl", "de", etc.)
    - `variant` - keyboard variant (e.g. "dvorak", "colemak", etc.)
 * `keybindings`
    - `insert` - list of key bindings in `insert` mode (only this mode can be modified via
      configuration file)

Key binding entry consists of `key`, `mods` and either `action` or `execute`:
 * `key` - name of the key (currently only numbers, letters and `space`). `key` is case insensitive
 * `mods` - list of modifiers: `lctl`, `rctl`, `lshift`, `rshift`, `lalt`, `ralt`, `lmeta`, `rmeta`.
   `mods` are case insensitive
 * `action` - name of predefined action
 * `execute` - command to be executed (in form of list)

Note that `perceptia` differentiates between left and right mod keys (`lctl`, `lalt`, `lshift`,
`lmeta` vs. `rctl`, `ralt`, `rshift`, `rmeta`)

TODO: Add more information about actions.

Scripting language
------------------

**Not yet implemented**.

Default configuration
=====================

This section presents set of default settings. Reader should be familiar with
[concepts.md](./concepts.md) (frame layouts, key modes, framing).

Global bindings:

 * `[lmeta]+[lctrl]+[esc]` - quit application

Insert mode:

 * `[lmeta]+[esc]` - swap to normal mode

 * `[lmeta]+[space]` - toggles anchorization

 * `[lmeta]+[_X_ arrow]` - focus frame in `_X_` direction from currently focused one

 * `[lmeta]+[tab]` - circle history forward

 * `[lmeta]+[lshift]+[tab]` - circle history backward

 * `[lmeta]+[_N_]` - focus workspace number `_N_`

 * `[lmeta]+[lshift]+[_X_ arrow]` - jump focused frame in `_X_` direction

 * `[lmeta]+[lshift]+[lctrl]+[_N_]` - jump focused frame to workspace number `_N_`

 * `[lmeta]+[lalt]+[_X_ arrow]` - dive focused frame in `_X_` direction

 * `[lmeta]+[lalt]+[lshift]+[lctrl]+[_N_]` - dive focused frame to workspace number `_N_`

 * `[lmeta]+[home]`, `[lmeta]+[end]` - exalt/ramify focused frame

 * `[lctrl]+[lmeta]+T` - spawn `weston-terminal`

Normal mode:

 * `[esc]` - clean the command

 * `[i]`, `[space]` - swap to insert mode

 * `[h]`, `[v]`, `[s]` - make layout of focused frame horizontal, vertical or stacked

 * `[f]`, `[lshift]+[f]`, `[j]`, `[d]` - indicate focus/swap/jump/dive action

 * `[home]`, `[end]` - indicate begin/end directions

 * `[_X_ arrow]` - indicate `_X_` direction

 * `[page up]`, `[page down]` - indicate direction forward/backward in time

 * from `[0]` to `[9]` - indicate magnitude of command

For example `[f] [right arrow]` will focus surface on the right from focussed one or `[5] [m] [arrow
down]` will move floating frame 5 steps down.

Built-ins:

 * `[lctrl]+[lalt]+[F_X_]` - switch to virtual terminal `_X_`
