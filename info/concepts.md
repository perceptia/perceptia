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

 * **config** - not mutable user configuration (compiled in or read from text file)

 * **settings** - not mutable information gathered from system (wayland socket, environment
   variables, program arguments)

 * **gears** - mutable user configuration (keymap, key modes, etc.)

Framing
-------

TODO

---

Now you know theory. For implementation details you can refer to code.

