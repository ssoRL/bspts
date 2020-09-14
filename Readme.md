# Skyler's Bullshit Point Tracking System

## Building

To build locally, should first get rust up and running, and install some crates

```
$ cargo install wasm-pack
$ cargo install cargo-make
$ cargo install simple-http-server
```

Then run

```
cargo make build
```

## Serving

In order to see the running example, first serve the site with

```
cargo make serve
```

and then navigate your browser to `http://localhost:3000/`

## Further references

This site is based off of [this Tutorial](http://www.sheshbabu.com/posts/rust-wasm-yew-single-page-application/)