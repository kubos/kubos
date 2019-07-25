# File Transfer Large Upload Test

This crate performs a single unit test of the file transfer service.
It performs a upload operation with a 100MB file.

The test may take several minutes to complete.
As a result, it has been placed in its own folder so that it can be run as an
independent CircleCI test, rather than as part of the sequential `cargo test`
process which verifies the other Rust unit tests.