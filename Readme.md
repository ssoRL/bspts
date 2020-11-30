# Skyler's Bullshit Point Tracking System

## Building the Frontend

To build locally, should first get rust up and running, and install some crates

```
cargo install wasm-pack
cargo install cargo-make
```

Then from the frontend run

```
cargo make build
```

## Serving the Backend

In order to see the running example, first build the site as above. Then, from the backend directory, run:

```
cargo run
```

and then navigate your browser to `http://localhost:3030/`

## Database

The database is a postgres db managed by [diesel](http://diesel.rs/). To download postgres for mac run:

```
brew install postgres
```

and to start the db run:

```
brew services start postgres
```

All changes to the database should be handled via diesel's migration pattern. The migrations are stored under `backend/migrations/`. To download the cli tools to manage the migrations, run:

```
cargo install diesel_cli --no-default-features --features "postgres"
```

### Docker

To start postgres running in a container, use the command:

```
docker run --rm --name postgres -e POSTGRES_USER=bspts_user -e POSTGRES_PASSWORD=pw -e POSTGRES_DB=bspts_db -p 5433:5432 -d postgres
```

## Further references

This site is based off of [this Tutorial](http://www.sheshbabu.com/posts/rust-wasm-yew-single-page-application/)

The server is implemented with [warp](https://github.com/seanmonstar/warp)

An [ineresting blog post](https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471) from a man who's done something similar to what my end goal is. See the latest on [github](https://github.com/saschagrunert/webapp.rs/tree/master)