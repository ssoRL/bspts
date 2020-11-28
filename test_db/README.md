## Test Database

This is a database used for testing or running the application with a clean state. To build it go to the `test_db` directory and run:

```
docker build . -t bspts_test_db
```

This will build the image with the tag `bspts_test_db`. From now on from the root directory, you can run:

```
cargo make test-db
```