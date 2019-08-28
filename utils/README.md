# Testing Utilities Library

This library allows us to build and run actual, external service
(ex. monitor service) which can then be queried during unit tests.

The services will be automatically build and run during test bringup, and then
killed during test cleanup