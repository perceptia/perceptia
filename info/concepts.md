Concepts
========

Nomenclature and key concepts needed for new-coming developers to dive in easier:

 * **surface** - a window (in Wayland language)

 * **sid** - in `perceptia` every surface have ID, `sid` is a Surface ID

 * **frame** - tree-like structure containing information about position and size of surface. Set of
   all frames is similar concept to `i3`'s tree

 * **selection** - currently selected frame. Selection may include many surfaces. It is guaranteed
   there always exists a selection

 * **anchored frame** - frame which is under automatic management. `perceptia` will take care about
   resizing or moving it

 * **floating frame** - any frame which is not anchored

 * frame layouts:

   - **horizontal** - frames are places aside another horizontally (on the right or left of each
     other)

   - **vertical** - frames are places aside another vertically (above or below of each other)

   - **stacked** - frames have the same size and position; only one frame is visible at a time

 * **exhibitor** - logical part of program which take care about compositing, drawing, frame
   management, input handling (if you wanted make `perceptia` an X compositor you would probably
   remove everything except exhibitor)

 * **output** - monitor understood as physical object

 * **display** - monitor understood as abstract concept (e.g. a drawing thread)

 * **workspace** - special frame containing all surfaces exhibited on given display. There may be
   other workspaces on the same displays but will be invisible.

 * **root frame** - a single frame containing all known frames

 * **global coordinates** - coordinates describing position of display

 * **workspace coordinates** - coordinates of surfaces within given display (workspace)

 * **key modes** - concept similar to `vim` modes which in basics allows to change set of active key
   bindings.

   - **insert mode** - (default mode) most keys are passed to clients, rare key bindings should
     containt `[meta]` or `[alt]` key.

   - **normal mode** - provides fast, composable bindings in `vim` style. For example depending on
     configuration `[j] [right]` could make focused surface jump to the right, `[w] [1]` focus
     worspace `1` or `[f] [down] [down]` focus surface two frames below currently focused one. Keys
     pressed in normal mode are not passed to clients.

 * **config** - not mutable user configuration (compiled in or read from text file)

 * **settings** - not mutable information gathered from system (wayland socket, environment
   variables, program arguments)

 * **gears** - mutable user configuration (keymap, key modes, etc.)

Framing
-------

Framing in `perceptia` mostly resembles concepts that can be found in `i3`. All frames are placed in
one tree-like structure where frames can be organized in horizontal, vertical or stacked layouts.

In future per-workspace strategies will be added to allow more automatic management like in
`Awesome`.

Exhibitor provides commands composed of action, direction and magnitude. Some available actions are:

 * **jumping** - jumping over neighbouring frame

 * **diving** - incorporating one frame into another

 * **focusing** - changing focus of frames

Some available directions are:

 * north (up, above), east (right), south (down, below), west (left) - to perform actions in
   output plane

 * backward (back in time, most recently used), forward (forward in time, the oldest used) - to
   perform actions in history order

 * begin (start,  head), end (finish, tail) - to perform actions perpendicular to output plane (for
   example **ramification** and **exaltation** is jumping to begin and end respectively)

---

Now you know theory. For implementation details you can refer to code.

