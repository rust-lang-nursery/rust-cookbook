#!/bin/bash

set -o errexit -o nounset

echo "Running $0"

if [ -z "${TRAVIS_BRANCH:-}" ]; then
    echo "This script may only be run from Travis!"
    exit 1
fi

if [[ "${CONTENT_TESTS:-}" == 1 ]]; then
    echo "Installing additional dependencies"

    if [[ "${CONTENT_TESTS_LINKS:-}" == 1 ]]; then
        gem install html-proofer
    fi
    cargo install mdbook --vers '0.1.7' --debug
fi

exit 0

