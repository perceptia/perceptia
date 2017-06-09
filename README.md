Perceptia
=========

Do you like configurability and lightweightness of tiling window managers? Do you like cutting-edge
technology and want to try a tiling window manager running on Wayland? `perceptia` is for you!
`perceptia` tries to merge concepts from most popular tiling window managers like `i3`, `awesome`,
`ratpoison` or `dwm` with the freshness of Wayland.

For main objectives of the project see [the Perceptia Manifesto](./info/manifesto.md).

Status
------

There are still many things to work on. If you are looking for a working compositor it is a bit too
early to choose `perceptia`, but if you have programming skills and want to join the development team, your help will be appreciated.

Backbone of every project is community. User input is especially valuable for young projects. It is
the users that shape what the project will later become. Please go to
[github](https://github.com/perceptia/perceptia/issues) and vote for features you want to see in
`perceptia` or share your own ideas. Your input will help to decide which features should be
implemented first and how they should be designed.

This repository consists of:

 * `cognitive` - a loose set of (Rust) crates helping creating surface compositors or other low
   level graphic applications

 * `perceptia` - a dynamic (tilling or stacking) Wayland surface compositor basing on `cognitive`.

Getting Started
---------------

For instructions on building and running `perceptia` see [installation.md](./info/installation.md).

For configuration options see [configuration.md](./info/configuration.md).

If you want to see features from your favourite window manager in `perceptia` you are encouraged to
contribute in brain-storming and development. For details see
[contributing.md](./info/contributing.md).

Contact via mailing list `perceptia@freelists.org` or directly with [authors](./info/authors.md).

`perceptia` is licensed on `MPL-2.0`. For more information see [license.md](./info/license.md).
