#!/bin/bash

set -o errexit -o nounset

echo "Running $0"

if [ -z "${TRAVIS_BRANCH:-}" ]; then
    echo "This script may only be run from Travis!"
    exit 1
fi

# Returns 1 if program is installed and 0 otherwise
program_installed() {
    local return_=1

    type $1 >/dev/null 2>&1 || { local return_=0; }

    echo "$return_"
}

if [[ "${CONTENT_TESTS:-}" == 1 ]]; then
    # Ensure required programs are installed
    if [ $(program_installed mdbook) == 0 ]; then
        echo "Please install mdbook: cargo install mdbook."
        exit 1
    fi

    echo "Testing markup and descriptions"
    echo "Building site to book/"
    mdbook build
    echo "Checking spelling"
    ./ci/spellcheck.sh list

    if [[ "${CONTENT_TESTS_LINKS:-}" == 1 ]]; then
        if [ $(program_installed htmlproofer) == 0 ]; then
            echo "Please install htmlproofer: gem install html-proofer"
            exit 1
        fi
        echo "Checking links"
        htmlproofer --empty-alt-ignore ./book/
    fi
else
    echo "Testing code snippets"
    cargo build --verbose
    cargo test --verbose
fi
