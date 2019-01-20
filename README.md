# emscripten-rs

## Overview

This is a simple wrapper around the emscripten library for rust. 

Documentation for emscripten is availible on the
[offical emscripten site](http://kripken.github.io/emscripten-site/docs/getting_started/index.html).

To target emscripten, a new target must be added to cargo.

```
    rustup target add wasm32-unknown-emscripten
```

The easiest way to build for emscripten is to use [cargo web](https://github.com/koute/cargo-web).

See the `examples/` directory for full examples on applying this package.

Note: if you use glutin, you will not need this package. Glutin handles emscripten setup itself.
This package was intended for the SDL2 + GL usecase in rust.
