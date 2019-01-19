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
