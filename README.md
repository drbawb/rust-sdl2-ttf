rust-sdl2-ttf
===

Provides bindings to `SDL_ttf`.

This particular crate is linked against `SDL2` / `rust-sdl2` -- and is suitable for use w/ SDL 2.0 applications.

Pre-Requisities
===

* [`mozilla/rust@master`](https://github.com/mozilla/rust)
    * This crate is meant to build against the latest version of `rust`.

* [`rust-sdl2`](https://github.com/AngryLawyer/rust-sdl2)
    * This crate provides bindings to SDL2. You will need this to build & link `rust-sdl2-ttf`.

* SDL2_ttf 
    * Available as binaries through your OS's package management tool
    * Available from http://libsdl.org

Building
===

    $ git clone https://github.com/drbawb/rust-sdl2-ttf
    $ cd rust-sdl2-ttf
    $ make deps && make
    
Targets
===

* `make deps` will initialize, fetch, and build `rust-sdl2` from the lib/rust-sdl2 submodule.
    * The resulting `.rlib` is placed in `lib/`
* `make` or `make all` will clean the `build/` directory and run `make compile`
* `make compile` will build an unoptimized version of the library and place the `.rlib` in `build/`
* `make veyron` will enable optimizations prior to completing the `compile` target.
* (WIP) `make test` will build the `tests` module and run them.
* `make doc` will run the `rustdoc` against `src/lib.rs` and store the docs as HTML in `docs/`

