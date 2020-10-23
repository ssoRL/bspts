# Skyler's Bullshit Point Tracking System

## Building

To build locally, should first get rust up and running, and install some crates

```
cargo install wasm-pack
cargo install cargo-make
```

Then run

```
cargo make build
```

## Serving

In order to see the running example, first build the site as above. Then, from the backend directory, run:

```
cargo run
```

and then navigate your browser to `http://localhost:3030/`

## Further references

This site is based off of [this Tutorial](http://www.sheshbabu.com/posts/rust-wasm-yew-single-page-application/)

The server is implemented with [warp](https://github.com/seanmonstar/warp)

An [ineresting blog post](https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471) from a man who's done something similar to what my end goal is.