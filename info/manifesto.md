Perceptia Manifesto
===================

The philosophy of `perceptia` is simplicity. **From simplicity comes power**. And this simplicity is
going to be achieved from both user and developer perspective.

User point of view
------------------

"Surface compositor" or "tilling window manager" may seem to be complicated concepts at first sight.
While `perceptia` draws ideas from many tilling window managers and as such is meant for power
users, its goal is actually to seamlessly introduce unexperienced user into tilling world. This is
achieved by providing familiar settings from stacking window managers as default but putting all the
other features in the range of sight. I hope `perceptia` will be simple enough for regular user but
also become powerful tool in hands of experienced one while retaining all the simplicity.

Developer point of view
-----------------------

Code is meant to be clean. It is meant to be read and maintained. If it wasn't we would use
assembler everywhere. Instead Rust - language focused on safety, speed, and concurrency - was
chosen. Rust has simple, clean syntax and it helps to avoid many classes of problems. I might even
say when using Rust good architecture pops up spontaneously. Once again: code is meant to be read so
should be easy to understand - functions should be short and do one task, bigger parts should be
solid and with well defined interface fit where needed - like blocks. And documentation! Lack of
documentation is a bug! When every part of application has it own small task to do and it's know how
this task fits in bigger structure - that is a good, simple design.

Note that "simple" here does not mean "easy". "Simple" means "basing on or composed of some
uncomplicated ideas". Such simple ideas can be then flexibly combined by user to solve the problem
at hand.
