#!/bin/bash

set -o errexit -o nounset

if [ -z "${TRAVIS_BRANCH:-}" ]; then
    echo "This script may only be run from Travis!"
    exit 1
fi

if [ "$TRAVIS_BRANCH" != "master" ]; then
    echo "This commit was made against '$TRAVIS_BRANCH' and not master! No deploy!"
    exit 0
fi

# Returns 1 if program is installed and 0 otherwise
program_installed() {
    local return_=1

    type $1 >/dev/null 2>&1 || { local return_=0; }

    echo "$return_"
}

# Ensure required programs are installed
if [ $(program_installed git) == 0 ]; then
    echo "Please install Git."
    exit 1
elif [ $(program_installed mdbook) == 0 ]; then
    echo "Please install mdbook: cargo install mdbook."
    exit 1
fi

echo "Building site to book/"
mdbook build

echo "Copying assets/* to book/"
cp -r assets/ book/

echo "Committing book directory to gh-pages branch"
REV=$(git rev-parse --short HEAD)
cd book
git init
git remote add upstream "https://$GH_TOKEN@github.com/brson/rust-cookbook.git"
git config user.name "Rust Cookbook"
git config user.email "cookbook@rust-lang.org"
git add -A .
git commit -qm "Build cookbook at ${TRAVIS_REPO_SLUG}@${REV}"

echo "Pushing gh-pages to GitHub"
git push -q upstream HEAD:refs/heads/gh-pages --force
