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

        pyenv install 3.6.0
        pyenv local 3.6.0
        pip3 install --user link-checker==0.1.0
    fi
    MDBOOK_VERSION="${MDBOOK_VERSION:-$(cat "$(dirname "$0")/../.mdbook-version")}"
    cargo install mdbook --version "=${MDBOOK_VERSION}" --debug
fi

exit 0
