#!/bin/bash
set -ex

pushd libs/kubos
poetry install --no-interaction --no-ansi
popd

pushd apis/app-api/python
poetry install --no-interaction --no-ansi
popd

cargo install cargo2junit
mkdir test-results

CARGO_TEST_REPORT_ARGS="-Z unstable-options --format json --report-time"

cargo test --workspace --release -- $CARGO_TEST_REPORT_ARGS | tee test-results/main.json
cat test-results/main.json | cargo2junit > test-results/main.xml